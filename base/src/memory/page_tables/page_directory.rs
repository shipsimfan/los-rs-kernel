use crate::memory::TABLE_ENTRIES;

#[repr(packed(4096))]
pub(in crate::memory) struct PageDirectory([PageDirectoryEntry; TABLE_ENTRIES]);

#[repr(packed(4096))]
pub(in crate::memory) struct PageDirectoryEntry(usize);
