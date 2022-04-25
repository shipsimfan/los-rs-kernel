use super::{PhysicalAddress, PAGE_SIZE};
use base::critical::CriticalLock;

struct Bitmap {
    map: [usize; BITMAP_SIZE as usize],
    size: usize,
    free_pages: usize,
    total_pages: usize,
    next_free_page: PhysicalAddress,
}

const MAXIMUM_MEMORY: usize = 128 * (1024 * 1024 * 1024); // 128 Gigabytes
const BITMAP_SIZE: usize = MAXIMUM_MEMORY / (super::PAGE_SIZE * 64);

static mut BITMAP_INITIALIZED: bool = false;

static BITMAP: CriticalLock<Bitmap> = CriticalLock::new(Bitmap {
    map: [0xFFFFFFFFFFFFFFFF; BITMAP_SIZE as usize],
    size: 0,
    free_pages: 0,
    total_pages: 0,
    next_free_page: 0xFFFFFFFFFFFFFFFF,
});

extern "C" {
    static __KERNEL_TOP: usize;
    static __KERNEL_BOTTOM: usize;
    static __KERNEL_LMA: usize;
}

pub unsafe fn initialize(mmap: &base::bootloader::MemoryMap) {
    assert!(!BITMAP_INITIALIZED);
    BITMAP_INITIALIZED = true;

    // Scan through descriptors
    let mut desc = mmap.address;
    let mut ptr = desc as usize;
    let top = ptr + mmap.size;

    let mut bitmap = BITMAP.lock();

    while ptr < top {
        let class = (*desc).class;
        let num_pages = (*desc).num_pages;
        let mut physical_address = (*desc).physical_address;

        bitmap.total_pages += num_pages;

        match class {
            base::bootloader::MemoryClass::LoaderCode
            | base::bootloader::MemoryClass::LoaderData
            | base::bootloader::MemoryClass::BootSerivesCode
            | base::bootloader::MemoryClass::BootServicesData
            | base::bootloader::MemoryClass::Conventional
            | base::bootloader::MemoryClass::Persistent => {
                if physical_address >= MAXIMUM_MEMORY as usize {
                    panic!("Physical memory found too high!");
                }

                if ((physical_address / PAGE_SIZE) + num_pages) / 64 >= bitmap.size {
                    bitmap.size = ((physical_address / PAGE_SIZE) + num_pages) / 64;
                }

                let mut i = 0;
                while i < num_pages {
                    bitmap.free(physical_address);

                    i += 1;
                    physical_address += PAGE_SIZE;
                }
            }
            _ => {}
        }

        ptr += mmap.desc_size;
        desc = ptr as *const base::bootloader::MemoryDescriptor;
    }

    // Calculate the kernel size
    let kernel_top = (&__KERNEL_TOP) as *const usize as usize;
    let kernel_bottom = (&__KERNEL_BOTTOM) as *const usize as usize;
    let kernel_lma = (&__KERNEL_LMA) as *const usize as usize;

    let kernel_size = kernel_top - kernel_bottom;

    // Allocate the kernel
    let mut addr = kernel_lma;
    while addr < kernel_lma + kernel_size {
        bitmap.allocate_page(addr);
        addr += PAGE_SIZE;
    }

    // Find first free page
    let mut i = 0;
    while i < bitmap.size * 64 * PAGE_SIZE {
        if bitmap.is_page_free(i) {
            bitmap.next_free_page = i;
            break;
        }

        i += PAGE_SIZE;
    }

    super::MEMORY_USAGE.lock().available_pages = bitmap.total_pages;
    super::MEMORY_USAGE.lock().free_pages = bitmap.free_pages;
}

pub unsafe fn allocate() -> PhysicalAddress {
    super::MEMORY_USAGE.lock().free_pages -= 1;
    BITMAP.lock().allocate()
}

pub unsafe fn free(address: PhysicalAddress) {
    super::MEMORY_USAGE.lock().free_pages += 1;
    BITMAP.lock().free(address);
}

impl Bitmap {
    pub fn is_page_free(&self, address: PhysicalAddress) -> bool {
        let i = address / (PAGE_SIZE * 64);
        let b = 63 - ((address / PAGE_SIZE) % 64) as u32;

        if i >= self.size {
            false
        } else {
            (self.map[i as usize].wrapping_shr(b) & 1) == 0
        }
    }

    pub fn allocate(&mut self) -> PhysicalAddress {
        let ret = self.next_free_page;
        self.allocate_page(ret);
        ret
    }

    pub fn allocate_page(&mut self, address: PhysicalAddress) {
        let i = address / (PAGE_SIZE * 64);
        let b = 63 - ((address / PAGE_SIZE) % 64) as u32;

        if i >= self.size {
            return;
        }

        if (self.map[i as usize].wrapping_shr(b) & 1) > 0 {
            return;
        }

        self.free_pages -= 1;

        self.map[i as usize] |= (1 as usize) << b;

        if address == self.next_free_page {
            let mut i = self.next_free_page;
            while i < self.size * 64 * PAGE_SIZE {
                if self.is_page_free(i) {
                    self.next_free_page = i;
                    break;
                }

                i += PAGE_SIZE;
            }
        }
    }

    pub fn free(&mut self, address: PhysicalAddress) {
        let i = address / (PAGE_SIZE * 64);
        let b = 63 - ((address / PAGE_SIZE) % 64) as u32;

        if i >= self.size {
            return;
        }

        if (self.map[i as usize].wrapping_shr(b) & 1) == 0 {
            return;
        }

        if address < self.next_free_page {
            self.next_free_page = address;
        }

        self.free_pages += 1;
        self.map[i as usize] &= !((1 as usize) << b);
    }
}
