use core::ffi::c_void;

#[repr(packed(1))]
struct GlobalDescriptorTable(pub [Entry; 7]);

#[repr(packed(1))]
#[derive(Clone, Copy)]
struct Entry {
    _limit_low: u16,
    _base_low: u16,
    _base_mid: u8,
    _access: u8,
    _flags_limit_high: u8,
    _base_high: u8,
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

static mut GDT_INITIALIZED: bool = false;

// GDT doesn't require a critical lock because it never changes after boot
// TODO: Make a GDT for each core
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::null();

// TSS doesn't require a critical lock because it is local to each core, it will need to be accessed critically though
// TODO: Make a TSS for each core
static mut TSS: TSS = TSS::null();

// CURRENT_KERNEL_STACK is similar to the TSS but for system calls instead of interrupts
// TODO: Move CURRENT_KERNEL_STACK into per-thread structure that changes with threads and is pointed to by gs
#[no_mangle]
static mut CURRENT_KERNEL_STACK: usize = 0;

extern "C" {
    fn install_gdt(gdtr: *const c_void, data0: u16, tss: u16, code0: u16);
}

pub fn initialize() {
    unsafe {
        assert!(!GDT_INITIALIZED);
        GDT_INITIALIZED = true;

        // CODE 0
        GDT.0[1] = Entry::new(true, false, false, true);

        // DATA 0
        GDT.0[2] = Entry::new(false, false, true, false);

        // CODE 3
        GDT.0[4] = Entry::new(true, true, false, true);

        // DATA 3
        GDT.0[3] = Entry::new(false, true, true, false);

        // TSS
        let tss_ptr = &TSS as *const _ as usize;

        GDT.0[5] = super::gdt::Entry {
            _limit_low: 104,
            _base_low: (tss_ptr & 0xFFFF) as u16,
            _base_mid: ((tss_ptr >> 16) & 0xFF) as u8,
            _access: super::gdt::GDT_ACCESS_ACCESSED
                | super::gdt::GDT_ACCESS_EXECUTABLE
                | super::gdt::GDT_ACCESS_DPL_3
                | super::gdt::GDT_ACCESS_PRESENT,
            _flags_limit_high: super::gdt::GDT_FLAGS_GRANULARITY,
            _base_high: ((tss_ptr >> 24) & 0xFF) as u8,
        };

        GDT.0[6] = super::gdt::Entry {
            _limit_low: ((tss_ptr >> 32) & 0xFFFF) as u16,
            _base_low: ((tss_ptr >> 48) & 0xFFFF) as u16,
            _base_mid: 0,
            _access: 0,
            _flags_limit_high: 0,
            _base_high: 0,
        };

        // Prepare the GDTR
        let gdt_ptr = &GDT as *const super::gdt::GlobalDescriptorTable as usize;

        let gdtr = super::CPUPointer {
            _size: 55,
            _ptr: gdt_ptr,
        };

        // Install GDT
        install_gdt(
            &gdtr as *const super::CPUPointer as *const c_void,
            0x10,
            0x28,
            0x8,
        );
    }
}

pub fn set_interrupt_stack(stack_pointer: usize) {
    unsafe {
        base::critical::enter_local();

        TSS.set_stack(stack_pointer);
        CURRENT_KERNEL_STACK = stack_pointer;

        base::critical::leave_local();
    };
}

impl GlobalDescriptorTable {
    pub const fn null() -> Self {
        GlobalDescriptorTable([Entry::null(); 7])
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
            _limit_low: 0xFFFF,
            _base_low: 0,
            _base_mid: 0,
            _access: access,
            _flags_limit_high: flags | 0x0F,
            _base_high: 0,
        }
    }

    pub const fn null() -> Self {
        Entry {
            _limit_low: 0,
            _base_low: 0,
            _base_mid: 0,
            _access: 0,
            _flags_limit_high: 0,
            _base_high: 0,
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
