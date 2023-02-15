use self::bitmap::Bitmap;
use super::PAGE_SIZE;

mod bitmap;

pub(super) struct PhysicalMemoryManager {
    page_bitmap: Bitmap<MAXIMUM_PHYSICAL_PAGES>,
}

const MAXIMUM_PHYSICAL_MEMORY: usize = 256 * (1024 * 1024 * 1024); // 256 Gigabytes
const MAXIMUM_PHYSICAL_PAGES: usize = MAXIMUM_PHYSICAL_MEMORY / PAGE_SIZE;

impl PhysicalMemoryManager {
    pub(super) const fn null() -> Self {
        PhysicalMemoryManager {
            page_bitmap: Bitmap::null(),
        }
    }
}
