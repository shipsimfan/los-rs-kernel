use crate::gdt;

#[repr(packed, C)]
#[derive(Clone, Copy)]
#[allow(unused)]
pub(super) struct Vector {
    offset_low: u16,
    segment: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

const FLAGS: u8 = 0b11101110;

impl Vector {
    pub(super) const fn null() -> Self {
        Vector {
            offset_low: 0,
            segment: 0,
            ist: 0,
            flags: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub(super) fn new(offset: u64) -> Self {
        Vector {
            offset_low: (offset & 0xFFFF) as u16,
            segment: gdt::KERNEL_CODE_SEGMENT_OFFSET as u16,
            ist: 0,
            flags: FLAGS,
            offset_mid: ((offset >> 16) & 0xFFFF) as u16,
            offset_high: (offset >> 32) as u32,
            reserved: 0,
        }
    }
}
