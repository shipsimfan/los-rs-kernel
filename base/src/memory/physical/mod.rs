use super::{PAGE_MASK, PAGE_SIZE};
use bitmap::Bitmap;

mod address;
mod bitmap;
mod memory_map;

pub use address::PhysicalAddress;
pub use memory_map::{MemoryDescriptor, MemoryMap};

pub(super) struct PhysicalMemoryManager {
    page_bitmap: Bitmap<MAXIMUM_PHYSICAL_PAGES>,
    free_pages: usize,
    physical_top: PhysicalAddress,
    next_free_page: Option<usize>,
}

extern "C" {
    static __KERNEL_TOP: usize;
}

const MAXIMUM_PHYSICAL_MEMORY: usize = 256 * (1024 * 1024 * 1024); // 256 Gigabytes
const MAXIMUM_PHYSICAL_PAGES: usize = MAXIMUM_PHYSICAL_MEMORY / PAGE_SIZE;

impl PhysicalMemoryManager {
    pub(super) const fn null() -> Self {
        PhysicalMemoryManager {
            page_bitmap: Bitmap::null(),
            free_pages: 0,
            physical_top: PhysicalAddress::null(),
            next_free_page: None,
        }
    }

    pub(super) fn initialize<M: MemoryMap>(&mut self, memory_map: M) {
        let kernel_top = PhysicalAddress::from(unsafe { (&__KERNEL_TOP) as *const usize as usize });
        let kernel_top_page =
            kernel_top.to_page() + if *kernel_top & PAGE_MASK == 0 { 0 } else { 1 };

        for descriptor in memory_map {
            let top = *descriptor.address() + descriptor.num_pages() * PAGE_SIZE;
            if top > *self.physical_top {
                self.physical_top = top.into();
            }

            if descriptor.is_freeable() {
                if *descriptor.address() >= MAXIMUM_PHYSICAL_MEMORY {
                    panic!("Too much physical memory on this machine");
                }

                let mut start_page = descriptor.address().to_page();
                let end_page = start_page + descriptor.num_pages();

                // Make sure we do not free below the kernel's top
                if end_page <= kernel_top_page {
                    continue;
                }

                if start_page < kernel_top_page {
                    start_page = kernel_top_page;
                }

                self.free_pages += end_page - start_page;

                for page in start_page..end_page {
                    self.page_bitmap.clear(page);
                }
            }
        }

        self.next_free_page = self.page_bitmap.get_next(kernel_top_page, false);
    }

    pub(super) fn allocate(&mut self) -> PhysicalAddress {
        self.free_pages -= 1;

        assert!(self.next_free_page.is_some());
        let page = self.next_free_page.unwrap();

        assert!(!self.page_bitmap.get(page));
        self.page_bitmap.set(page);

        self.next_free_page = self.page_bitmap.get_next(page, false);
        match self.next_free_page {
            Some(next_free_page) => match next_free_page >= self.physical_top.to_page() {
                true => self.next_free_page = None,
                false => {}
            },
            None => {}
        }

        PhysicalAddress::from_page(page)
    }

    pub(super) fn free(&mut self, address: PhysicalAddress) {
        assert!(self.free_pages < self.physical_top.to_page());
        self.free_pages += 1;

        let index = address.to_page();

        assert!(index < self.physical_top.to_page());
        assert!(self.page_bitmap.get(index));
        self.page_bitmap.clear(index);
    }
}
