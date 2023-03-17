use crate::memory::PAGE_SIZE;

#[repr(packed(4096))]
pub(in crate::memory) struct Page([u8; PAGE_SIZE]);

impl Page {
    pub(in crate::memory) const fn null() -> Self {
        Page([0; PAGE_SIZE])
    }
}
