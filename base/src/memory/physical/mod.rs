use super::{PAGE_MASK, PAGE_SIZE};
use bitmap::Bitmap;

mod address;
mod bitmap;

pub(super) use address::PhysicalAddress;

pub(super) struct PhysicalMemoryManager {
    page_bitmap: Bitmap<MAXIMUM_PHYSICAL_PAGES>,
    free_pages: usize,
    total_pages: usize,
    next_free_page: Option<usize>,
}

const MAXIMUM_PHYSICAL_MEMORY: usize = 256 * (1024 * 1024 * 1024); // 256 Gigabytes
const MAXIMUM_PHYSICAL_PAGES: usize = MAXIMUM_PHYSICAL_MEMORY / PAGE_SIZE;

fn page_to_address(page: usize) -> PhysicalAddress {
    (page * PAGE_SIZE).into()
}

fn address_to_page(address: PhysicalAddress) -> usize {
    (*address & PAGE_MASK) / PAGE_SIZE
}

impl PhysicalMemoryManager {
    pub(super) const fn null() -> Self {
        PhysicalMemoryManager {
            page_bitmap: Bitmap::null(),
            free_pages: 0,
            total_pages: 0,
            next_free_page: None,
        }
    }

    pub(super) fn initialize(&mut self, memory_map: *const uefi::memory::raw::MemoryMap) {}

    pub(super) fn allocate(&mut self) -> PhysicalAddress {
        assert!(self.free_pages > 0);
        self.free_pages -= 1;

        assert!(self.next_free_page.is_some());
        let page = self.next_free_page.unwrap();

        assert!(!self.page_bitmap.get(page));
        self.page_bitmap.set(page);

        self.next_free_page = self.page_bitmap.get_next(page, false);
        match self.next_free_page {
            Some(next_free_page) => match next_free_page >= self.total_pages {
                true => self.next_free_page = None,
                false => {}
            },
            None => {}
        }

        page_to_address(page)
    }

    pub(super) fn free(&mut self, address: PhysicalAddress) {
        assert!(self.free_pages < self.total_pages);
        self.free_pages += 1;

        let index = address_to_page(address);

        assert!(index < self.total_pages);
        assert!(self.page_bitmap.get(index));
        self.page_bitmap.clear(index);
    }
}
