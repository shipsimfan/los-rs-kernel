use base::{critical::CriticalLock, log_info};
use core::ffi::c_void;

#[repr(packed(1))]
pub struct InterruptDescriptorTable(pub [Entry; 256]);

#[repr(packed(1))]
#[derive(Clone, Copy)]
pub struct Entry {
    _offset1: u16,
    _selector: u16,
    _ist: u8,
    _type_attribute: u8,
    _offset2: u16,
    _offset3: u32,
    _zero: u32,
}

static mut IDT_INITIALIZED: bool = false;

// IDT will be shared between all cores
static IDT: CriticalLock<InterruptDescriptorTable> =
    CriticalLock::new(InterruptDescriptorTable::null());

extern "C" {
    fn install_idt(idtr: *const c_void);
}

pub fn install_interrupt_handler(interrupt: u8, handler: usize) {
    let mut idt = IDT.lock();
    idt.0[interrupt as usize] = Entry::new(handler);
}

pub fn initialize() {
    log_info!("Initializing IDT . . . ");

    unsafe {
        assert!(!IDT_INITIALIZED);
        IDT_INITIALIZED = true;
    }

    // Prepare the IDTR
    let idt = IDT.lock();
    let idt_ptr = &(*idt) as *const super::idt::InterruptDescriptorTable as usize;

    let idtr = super::CPUPointer {
        _size: 4095,
        _ptr: idt_ptr,
    };

    // Install IDT
    unsafe {
        install_idt(&idtr as *const super::CPUPointer as *const c_void);
    }

    log_info!("Initialized IDT!");
}

impl InterruptDescriptorTable {
    pub const fn null() -> Self {
        InterruptDescriptorTable([Entry::null(); 256])
    }
}

impl Entry {
    pub fn new(offset: usize) -> Self {
        Entry {
            _offset1: (offset & 0xFFFF) as u16,
            _selector: 0x08,
            _ist: 0,
            _type_attribute: 0b11101110,
            _offset2: ((offset >> 16) & 0xFFFF) as u16,
            _offset3: ((offset >> 32) & 0xFFFFFFFF) as u32,
            _zero: 0,
        }
    }

    pub const fn null() -> Self {
        Entry {
            _offset1: 0,
            _selector: 0,
            _ist: 0,
            _type_attribute: 0,
            _offset2: 0,
            _offset3: 0,
            _zero: 0,
        }
    }
}
