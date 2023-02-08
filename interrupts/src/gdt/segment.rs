use super::TSS;

#[repr(u8)]
#[allow(unused)]
pub(super) enum PrivilegeLevel {
    Ring0,
    Ring1,
    Ring2,
    Ring3,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum SegmentType {
    Data,
    Code,
}

#[repr(packed)]
#[allow(unused)]
pub(super) struct Descriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,           // P | DPL | S | TYPE
    flags_limit_high: u8, // G | D/B | L | AVL | LIMIT_HIGH
    base_high: u8,
}

const WRITE: u8 = 0x02;
const NORMAL: u8 = 0x10;
const PRESENT: u8 = 0x80;
const TSS_TYPE: u8 = 0x09;

const LONG: u8 = 0x20;
const _32_BIT_SEGMENT: u8 = 0x40;
const GRANULARITY: u8 = 0x80;

impl Descriptor {
    pub(super) const fn new_null() -> Self {
        Descriptor {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            flags_limit_high: 0,
            base_high: 0,
        }
    }

    pub(super) const fn new_normal(
        privilege_level: PrivilegeLevel,
        segment_type: SegmentType,
    ) -> Self {
        Descriptor {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access: PRESENT
                | ((privilege_level as u8) << 5)
                | NORMAL
                | ((segment_type as u8) << 3)
                | WRITE,
            flags_limit_high: segment_type.generate_flags() | 0x0F,
            base_high: 0,
        }
    }

    pub(super) const fn new_tss() -> (Self, Self) {
        const LIMIT: usize = core::mem::size_of::<TSS>();
        (
            Descriptor {
                limit_low: (LIMIT & 0xFFFF) as u16,
                base_low: 0,
                base_mid: 0,
                access: PRESENT | TSS_TYPE,
                flags_limit_high: ((LIMIT >> 16) & 0xF) as u8,
                base_high: 0,
            },
            Descriptor::new_null(),
        )
    }

    pub(super) fn update_tss_low(&mut self, tss_low: u32) {
        self.base_low = (tss_low & 0xFFFF) as u16;
        self.base_mid = ((tss_low >> 16) & 0xFF) as u8;
        self.base_high = (tss_low >> 24) as u8;
    }

    pub(super) fn update_tss_high(&mut self, tss_high: u32) {
        self.limit_low = (tss_high & 0xFFFF) as u16;
        self.base_low = (tss_high >> 16) as u16;
    }
}

impl SegmentType {
    pub(self) const fn generate_flags(self) -> u8 {
        match self {
            SegmentType::Code => LONG | GRANULARITY,
            SegmentType::Data => _32_BIT_SEGMENT | GRANULARITY,
        }
    }
}
