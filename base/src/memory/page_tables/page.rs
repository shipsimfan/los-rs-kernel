use crate::memory::PAGE_SIZE;

#[repr(packed(4096))]
#[allow(unused)]
pub(in crate::memory) struct Page([u8; PAGE_SIZE]);
