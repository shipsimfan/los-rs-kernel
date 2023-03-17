pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub const KERNEL_VMA: usize = 0xFFFF800000000000;

pub(super) const TABLE_ENTRIES: usize = 512;
