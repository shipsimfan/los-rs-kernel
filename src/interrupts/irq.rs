use crate::{
    device::{acpi, outb},
    log, logln,
    memory::KERNEL_VMA,
};
use core::{mem::size_of, ptr::null_mut};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum MADTEntryType {
    ProcessorLocalAPIC = 0x00,
    IOAPIC = 0x01,
    IOAPICInterruptSourceOverride = 0x02,
    IOAPICNMISource = 0x03,
    LocalAPICNMI = 0x04,
    LocalAPICAddressOverride = 0x05,
    ProcessorLocalx2APIC = 0x09,
}

#[repr(packed(1))]
struct MADTEntry {
    entry_type: MADTEntryType,
    length: u8,
}

#[repr(packed(1))]
struct IOAPICEntry {
    _entry_type: MADTEntryType,
    _length: u8,
    _apic_id: u8,
    _reserved: u8,
    address: u32,
    _irq_base: u32,
}

#[repr(packed(1))]
struct LocalAPICAddressOverrideEntry {
    _entry_type: MADTEntryType,
    _length: u8,
    _reserved: u16,
    address: u64,
}

#[derive(Debug, Clone, Copy)]
struct HandlerWithContext {
    handler: Handler,
    context: usize,
}

pub type Handler = unsafe fn(context: usize);

const IRQ_BASE: u8 = 32;

const MASTER_PIC_COMMAND: u16 = 0x20;
const MASTER_PIC_DATA: u16 = 0x21;
const SLAVE_PIC_COMMAND: u16 = 0xA0;
const SLAVE_PIC_DATA: u16 = 0xA1;

const LAPIC_TASK_PRIORITY: isize = 0x80 / 4;
const LAPIC_EOI: isize = 0xB0 / 4;
const LAPIC_SPURIOUS_INTERRUPT_VECTOR: isize = 0xF0 / 4;
const LAPIC_TIMER_LVT: isize = 0x320 / 4;

const SPURIOUS_INTERRUPT_VECTOR: u8 = 0xFF;

static mut LOCAL_APIC: *mut u32 = null_mut();
static mut IRQ_HANDLERS: [Option<HandlerWithContext>; 16] = [None; 16];

extern "C" {
    fn spurious_irq_handler();

    fn irq_handler_0();
    fn irq_handler_1();
    fn irq_handler_2();
    fn irq_handler_3();
    fn irq_handler_4();
    fn irq_handler_5();
    fn irq_handler_6();
    fn irq_handler_7();
    fn irq_handler_8();
    fn irq_handler_9();
    fn irq_handler_10();
    fn irq_handler_11();
    fn irq_handler_12();
    fn irq_handler_13();
    fn irq_handler_14();
    fn irq_handler_15();
}

pub fn initialize() {
    log!("Initializing IRQs . . . ");

    // Get the MADTlint
    let madt: &acpi::MADT = match acpi::get_table() {
        Err(err) => panic!("{}", err),
        Ok(table) => table,
    };

    // Verify 8259 PICs
    if madt.flags & 1 == 0 {
        panic!("No 8259 PICs installed! I/O APIC not currently supported!");
    }

    // Save local apic address
    unsafe { LOCAL_APIC = (madt.apic_address as usize + KERNEL_VMA) as *mut u32 };

    // Find and mask all IO APICs
    // Also look for a APIC address override
    let mut entry =
        unsafe { (madt as *const _ as *const u8).offset(size_of::<acpi::MADT>() as isize) }
            as *const MADTEntry;
    let mut i = entry as usize;
    let top = madt as *const _ as usize + madt.header.length as usize;
    while i < top {
        match unsafe { (*entry).entry_type } {
            MADTEntryType::IOAPIC => {
                let ioapic = entry as *const IOAPICEntry;
                let select_register =
                    (unsafe { (*ioapic).address as usize } + KERNEL_VMA) as *mut u32;
                let data_register = (select_register as usize + 0x10) as *mut u32;

                unsafe { *select_register = 1 }; // Select IOAPICVER
                let num_irq = unsafe { ((*data_register) >> 16) + 1 };
                let mut i = 0;
                while i < num_irq {
                    unsafe {
                        *select_register = 0x10 + 2 * i;
                        *data_register = 0x10000; // Disable
                    }

                    i += 1;
                }
            }
            MADTEntryType::LocalAPICAddressOverride => unsafe {
                LOCAL_APIC = ((*(entry as *const LocalAPICAddressOverrideEntry)).address as usize
                    + KERNEL_VMA) as *mut u32;
            },
            _ => {}
        }

        i += unsafe { (*entry).length } as usize;
        entry = i as *const MADTEntry;
    }

    // Allocate the LAPIC
    let addr = unsafe { LOCAL_APIC as usize };
    crate::memory::map_virtual_memory(addr, addr - KERNEL_VMA);

    // Disable the APIC timer
    unsafe { *LOCAL_APIC.offset(LAPIC_TIMER_LVT) = 0x10000 };

    // Initialize the local APIC
    super::idt::install_interrupt_handler(SPURIOUS_INTERRUPT_VECTOR, spurious_irq_handler as usize);
    unsafe {
        *LOCAL_APIC.offset(LAPIC_SPURIOUS_INTERRUPT_VECTOR) =
            0x100 | SPURIOUS_INTERRUPT_VECTOR as u32;
        *LOCAL_APIC.offset(LAPIC_TASK_PRIORITY) = 0;
    }

    // Install IRQ handlers
    super::idt::install_interrupt_handler(IRQ_BASE + 0, irq_handler_0 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 1, irq_handler_1 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 2, irq_handler_2 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 3, irq_handler_3 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 4, irq_handler_4 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 5, irq_handler_5 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 6, irq_handler_6 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 7, irq_handler_7 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 8, irq_handler_8 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 9, irq_handler_9 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 10, irq_handler_10 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 11, irq_handler_11 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 12, irq_handler_12 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 13, irq_handler_13 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 14, irq_handler_14 as usize);
    super::idt::install_interrupt_handler(IRQ_BASE + 15, irq_handler_15 as usize);

    // Initialize the 8259 PICs
    outb(MASTER_PIC_COMMAND, 0x11);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(SLAVE_PIC_COMMAND, 0x11);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(MASTER_PIC_DATA, IRQ_BASE + 0);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(SLAVE_PIC_DATA, IRQ_BASE + 8);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(MASTER_PIC_DATA, 4);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(SLAVE_PIC_DATA, 2);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(MASTER_PIC_DATA, 0x01);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(SLAVE_PIC_DATA, 0x01);
    outb(0x80, 0);
    outb(0x80, 0);
    outb(MASTER_PIC_DATA, 0);
    outb(SLAVE_PIC_DATA, 0);

    logln!("OK!");
}

#[no_mangle]
unsafe extern "C" fn common_irq_handler(irq: usize) {
    end_irq(irq as u8);
    end_interrupt();

    match IRQ_HANDLERS[irq] {
        None => {}
        Some(handler) => (handler.handler)(handler.context),
    }
}

pub fn install_irq_handler(irq: u8, handler: Handler, context: usize) -> bool {
    if irq > 15 {
        return false;
    }

    match unsafe { IRQ_HANDLERS[irq as usize] } {
        None => unsafe {
            IRQ_HANDLERS[irq as usize] = Some(HandlerWithContext {
                handler: handler,
                context: context,
            });
            true
        },
        Some(_) => false,
    }
}

fn end_irq(irq: u8) {
    if irq > 15 {
        return;
    }

    if irq >= 8 {
        outb(SLAVE_PIC_COMMAND, 0x20);
    }

    outb(MASTER_PIC_COMMAND, 0x20);
}

fn end_interrupt() {
    unsafe { *LOCAL_APIC.offset(LAPIC_EOI) = 0 };
}
