use crate::{get_table, MADT};
use base::log_info;
use core::{mem::size_of, ptr::null_mut};
use device::outb;
use memory::KERNEL_VMA;
use process::ProcessTypes;

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

extern "C" {
    fn spurious_irq_handler();
}

pub fn initialize_io_apic<T: ProcessTypes + 'static>() {
    log_info!("Initializing 8259 PICs . . .");

    // Get the MADT table
    let madt = get_table::<MADT, T>().unwrap();

    // Verify 8259 PICs
    if madt.flags & 1 == 0 {
        panic!("No 8259 PICs installed! I/O APIC not currently supported!");
    }

    // Save local apic address
    unsafe { LOCAL_APIC = (madt.apic_address as usize + KERNEL_VMA) as *mut u32 };

    // Find and mask all IO APICs
    // Also look for a APIC address override
    let mut entry = unsafe { (madt as *const _ as *const u8).offset(size_of::<MADT>() as isize) }
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
    memory::map_virtual_memory(addr, addr - KERNEL_VMA);

    // Disable the APIC timer
    unsafe { *LOCAL_APIC.offset(LAPIC_TIMER_LVT) = 0x10000 };

    // Initialize the local APIC
    unsafe {
        interrupts::install_interrupt_handler(
            SPURIOUS_INTERRUPT_VECTOR,
            spurious_irq_handler as usize,
        );
        *LOCAL_APIC.offset(LAPIC_SPURIOUS_INTERRUPT_VECTOR) =
            0x100 | SPURIOUS_INTERRUPT_VECTOR as u32;
        *LOCAL_APIC.offset(LAPIC_TASK_PRIORITY) = 0;
    }

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

    log_info!("Initialized 8259 PICs!");
}

pub unsafe fn end_irq(irq: u8) {
    if irq > 15 {
        return;
    }

    if irq >= 8 {
        outb(SLAVE_PIC_COMMAND, 0x20);
    }

    outb(MASTER_PIC_COMMAND, 0x20);
}

pub unsafe fn end_interrupt() {
    if LOCAL_APIC != null_mut() {
        *LOCAL_APIC.offset(LAPIC_EOI) = 0;
    }
}
