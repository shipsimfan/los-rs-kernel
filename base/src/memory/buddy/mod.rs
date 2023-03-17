use super::{IDENTITY_MAP_SIZE, PAGE_SIZE};
use free_list::FreeList;

mod free_list;
mod node;

pub(super) struct BuddyAllocator {
    free_lists: [FreeList; MAX_ORDER],
}

const MAX_ORDER: usize = (IDENTITY_MAP_SIZE / PAGE_SIZE).checked_ilog2().unwrap() as usize;

fn order(num_pages: usize) -> usize {
    (num_pages.next_power_of_two() / 2)
        .checked_ilog2()
        .unwrap_or(0) as usize
}

impl BuddyAllocator {
    pub(super) const fn new() -> Self {
        BuddyAllocator {
            free_lists: [FreeList::new(); MAX_ORDER],
        }
    }

    pub(super) fn allocate(&mut self, num_pages: usize) -> usize {
        // Find the list which is the best fit for num pages

        // Remove and return the first free chunk of the list

        // If there is none, search larger lists until a free chunk is found
        //  If none can be found, we ran out of memory, panic!

        // Split the chunk found in the larger list as many times as needed

        todo!()
    }

    pub(super) unsafe fn allocate_at(&mut self, address: usize, num_pages: usize) {
        assert!((address / PAGE_SIZE) % num_pages == 0);
        // Find the list which is the best fit for num pages
        let order = order(num_pages);

        // Search for any larger chunk which contains this allocation
        for order in (order + 1..MAX_ORDER).rev() {
            for node in &self.free_lists[order] {
                if node.contains(address, num_pages) {
                    return;
                } else if node.is_above(address) {
                    break;
                }
            }
        }

        // Search for any chunks equivalent to this allocation
        //  If found, ignore this allocation

        // Search for any smaller chunks which this allocation contains
        //  If found, remove them to be merged with this allocation

        // Insert the new node into the linked list for this allocation
    }

    pub(super) fn free(&mut self, address: usize, num_pages: usize) {
        assert!((address / PAGE_SIZE) % num_pages == 0);

        // Find the list which is the best fit for num pages

        // Search the list for this allocations buddy
        //  If found, remove buddy, and repeat free for larger chunk

        // Insert the new node into the linked list for this allocation

        todo!()
    }
}
