use super::PAGE_SIZE;
use crate::{MemoryDescriptor, MemoryManager, MemoryMap};

mod allocator;
mod free_list;
mod page;

pub(self) use free_list::FreeList;
pub(self) use page::Page;

pub(super) use allocator::BuddyAllocator;

const MAX_ORDER: u8 = 16;

pub const fn order_to_size(order: u8) -> usize {
    if order >= MAX_ORDER {
        panic!("Max order");
    }

    PAGE_SIZE << order as usize
}

pub(super) const fn num_pages_to_order(num_pages: usize) -> u8 {
    num_pages.next_power_of_two().ilog2() as u8
}

pub(super) fn initialize<M: MemoryMap>(memory_map: M, in_use_regions: &[(usize, usize)]) {
    let mut buddy_allocator = MemoryManager::get().buddy_allocator.lock();

    for descriptor in memory_map {
        if !descriptor.is_usable() {
            continue;
        }

        let mut address = descriptor.address().into_virtual::<u8>() as usize;

        for _ in 0..descriptor.num_pages() {
            if !is_in_in_use_regions(address, in_use_regions) {
                buddy_allocator.init_free(address);

                address += PAGE_SIZE;
            }
        }
    }
}

fn is_in_in_use_regions(address: usize, in_use_regions: &[(usize, usize)]) -> bool {
    for (lower, upper) in in_use_regions {
        if address >= *lower && address + PAGE_SIZE < *upper {
            return true;
        }
    }

    false
}
