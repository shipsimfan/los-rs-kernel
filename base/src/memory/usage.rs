#[repr(C)]
#[derive(Clone)]
pub struct MemoryUsage {
    available_pages: usize,
    free_pages: usize,

    userspace_pages: usize,
}

impl MemoryUsage {
    pub(super) const fn null() -> Self {
        MemoryUsage {
            available_pages: 0,
            free_pages: 0,

            userspace_pages: 0,
        }
    }

    pub(super) fn set_free_and_available(&mut self, free_pages: usize, available_pages: usize) {
        self.free_pages = free_pages;
        self.available_pages = available_pages;
    }

    pub(super) fn free_page(&mut self) {
        self.free_pages += 1;
    }

    pub(super) fn free_userspace_page(&mut self) {
        self.userspace_pages -= 1;
        self.free_page();
    }
}
