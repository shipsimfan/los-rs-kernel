use crate::memory::TABLE_ENTRIES;

#[repr(packed(4096))]
pub(in crate::memory) struct PageTable([PageTableEntry; TABLE_ENTRIES]);

#[repr(packed(4096))]
pub(in crate::memory) struct PageTableEntry(usize);
