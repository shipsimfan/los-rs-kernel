pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub const KERNEL_VMA: usize = 0xFFFF800000000000;

pub(super) const TABLE_ENTRIES: usize = PAGE_SIZE / core::mem::size_of::<usize>();

pub(super) const IDENTITY_MAP_PAGE_SIZE: usize = 1024 * 1024 * 1024; // 1 GB
pub(super) const IDENTITY_MAP_NUM_PDPTS: usize = 128;
pub(super) const IDENTITY_MAP_NUM_PAGES: usize = IDENTITY_MAP_NUM_PDPTS * TABLE_ENTRIES;

pub(super) const IDENTITY_MAP_SIZE: usize = IDENTITY_MAP_PAGE_SIZE * IDENTITY_MAP_NUM_PAGES; // 64 TB
pub(super) const IDENTITY_MAP_BOTTOM: usize = KERNEL_VMA;
pub(super) const IDENTITY_MAP_TOP: usize = KERNEL_VMA + IDENTITY_MAP_SIZE;
