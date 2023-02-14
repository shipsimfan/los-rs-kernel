use core::cell::RefCell;

#[repr(packed, C)]
#[derive(Clone, Copy)]
#[allow(unused)]
struct Reversedu64 {
    upper: u32,
    lower: u32,
}

#[repr(packed, C)]
#[allow(unused)]
pub struct TSS {
    reserved: u32,
    rsp: [Reversedu64; 3],
    reserved2: u64,
    ist: [Reversedu64; 7],
    reserved3: u64,
    reserved4: u16,
    io_map_base: u16,
}

impl TSS {
    pub fn new() -> RefCell<Self> {
        RefCell::new(TSS {
            reserved: 0,
            rsp: [Reversedu64::default(); 3],
            reserved2: 0,
            ist: [Reversedu64::default(); 7],
            reserved3: 0,
            reserved4: 0,
            io_map_base: core::mem::size_of::<TSS>() as u16,
        })
    }

    pub(super) fn set_interrupt_stack(&mut self, stack: u64) {
        self.rsp[0] = stack.into();
    }
}

impl Default for Reversedu64 {
    fn default() -> Self {
        Reversedu64 { upper: 0, lower: 0 }
    }
}

impl From<u64> for Reversedu64 {
    fn from(value: u64) -> Self {
        Reversedu64 {
            upper: (value >> 32) as u32,
            lower: (value & 0xFFFFFFFF) as u32,
        }
    }
}
