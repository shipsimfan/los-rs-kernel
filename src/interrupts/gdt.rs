use crate::locks;
use core::ffi::c_void;

#[repr(packed(1))]
struct GlobalDescriptorTable(pub [Entry; 7]);

#[repr(packed(1))]
struct Entry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub access: u8,
    pub flags_limit_high: u8,
    pub base_high: u8,
}

#[repr(packed(1))]
struct TSS {
    _reserved0: u32,
    _rsp0: usize,
    _rsp1: usize,
    _rsp2: usize,
    _reserved1: usize,
    _ist1: usize,
    _ist2: usize,
    _ist3: usize,
    _ist4: usize,
    _ist5: usize,
    _ist6: usize,
    _ist7: usize,
    _reserved2: usize,
    _reserved3: u32,
}

const GDT_ACCESS_ACCESSED: u8 = 1;
const GDT_ACCESS_READ_WRITE: u8 = 2;
const GDT_ACCESS_EXECUTABLE: u8 = 8;
const GDT_ACCESS_TYPE: u8 = 16;
const GDT_ACCESS_DPL_3: u8 = 96;
const GDT_ACCESS_PRESENT: u8 = 128;

const GDT_FLAGS_64_CODE: u8 = 32;
const GDT_FLAGS_SIZE: u8 = 64;
const GDT_FLAGS_GRANULARITY: u8 = 128;

static GDT: locks::Mutex<GlobalDescriptorTable> = locks::Mutex::new(GlobalDescriptorTable::null());
static TSS: locks::Mutex<TSS> = locks::Mutex::new(TSS::null());

extern "C" {
    fn install_gdt(gdtr: *const c_void, data0: u16, tss: u16, code0: u16);
}

pub fn initialize() {
    let mut gdt = GDT.lock();

    // CODE 0
    gdt.0[1] = Entry::new(true, false, false, true);

    // DATA 0
    gdt.0[2] = Entry::new(false, false, true, false);

    // CODE 3
    gdt.0[4] = Entry::new(true, true, false, true);

    // DATA 0
    gdt.0[3] = Entry::new(false, true, true, false);

    // TSS
    let tss_lock = TSS.lock();
    let tss_ptr = &tss_lock as *const _ as usize;

    gdt.0[5] = super::gdt::Entry {
        limit_low: 104,
        base_low: (tss_ptr & 0xFFFF) as u16,
        base_mid: ((tss_ptr >> 16) & 0xFF) as u8,
        access: super::gdt::GDT_ACCESS_ACCESSED
            | super::gdt::GDT_ACCESS_EXECUTABLE
            | super::gdt::GDT_ACCESS_DPL_3
            | super::gdt::GDT_ACCESS_PRESENT,
        flags_limit_high: super::gdt::GDT_FLAGS_GRANULARITY,
        base_high: ((tss_ptr >> 24) & 0xFF) as u8,
    };

    gdt.0[6] = super::gdt::Entry {
        limit_low: ((tss_ptr >> 32) & 0xFFFF) as u16,
        base_low: ((tss_ptr >> 48) & 0xFFFF) as u16,
        base_mid: 0,
        access: 0,
        flags_limit_high: 0,
        base_high: 0,
    };

    // Prepare the GDTR
    let gdt_ptr = &(*gdt) as *const super::gdt::GlobalDescriptorTable as usize;

    let gdtr = super::CPUPointer {
        _size: 55,
        _ptr: gdt_ptr,
    };

    // Install GDT
    unsafe {
        install_gdt(
            &gdtr as *const super::CPUPointer as *const c_void,
            0x10,
            0x28,
            0x8,
        );
    }
}

pub fn set_interrupt_stack(stack_pointer: usize) {
    (*TSS.lock()).set_stack(stack_pointer);
}

impl GlobalDescriptorTable {
    pub const fn null() -> Self {
        GlobalDescriptorTable([
            Entry::null(),
            Entry::null(),
            Entry::null(),
            Entry::null(),
            Entry::null(),
            Entry::null(),
            Entry::null(),
        ])
    }
}

impl Entry {
    pub const fn new(executable: bool, user: bool, size: bool, l: bool) -> Self {
        let mut access = GDT_ACCESS_READ_WRITE | GDT_ACCESS_TYPE | GDT_ACCESS_PRESENT;
        let mut flags = GDT_FLAGS_GRANULARITY;

        if executable {
            access |= GDT_ACCESS_EXECUTABLE;
        }

        if user {
            access |= GDT_ACCESS_DPL_3;
        }

        if size {
            flags |= GDT_FLAGS_SIZE;
        }

        if l {
            flags |= GDT_FLAGS_64_CODE;
        }

        Entry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access: access,
            flags_limit_high: flags | 0x0F,
            base_high: 0,
        }
    }

    pub const fn null() -> Self {
        Entry {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            flags_limit_high: 0,
            base_high: 0,
        }
    }
}

impl TSS {
    pub const fn null() -> Self {
        TSS {
            _reserved0: 0,
            _reserved1: 0,
            _reserved2: 0,
            _reserved3: 0,
            _rsp0: 0,
            _rsp1: 0,
            _rsp2: 0,
            _ist1: 0,
            _ist2: 0,
            _ist3: 0,
            _ist4: 0,
            _ist5: 0,
            _ist6: 0,
            _ist7: 0,
        }
    }

    pub fn set_stack(&mut self, stack: usize) {
        self._rsp0 = stack;
    }
}
