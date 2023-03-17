use super::{IDENTITY_MAP_SIZE, PAGE_SIZE};
use core::ptr::NonNull;
use free_list::{FreeList, FreeResult};
use node::Node;

mod free_list;
mod node;

pub(super) struct BuddyAllocator {
    free_lists: [FreeList; MAX_ORDER],
}

const MAX_ORDER: usize = (IDENTITY_MAP_SIZE / PAGE_SIZE).checked_ilog2().unwrap() as usize;

fn order(num_pages: usize) -> usize {
    num_pages.next_power_of_two().checked_ilog2().unwrap() as usize
}

impl BuddyAllocator {
    pub(super) const fn new() -> Self {
        BuddyAllocator {
            free_lists: [FreeList::new(); MAX_ORDER],
        }
    }

    pub(super) unsafe fn initialize_orders(&mut self) {
        let mut pages_per_node = 1;
        for free_list in &mut self.free_lists {
            free_list.set_pages_per_node(pages_per_node);
            pages_per_node *= 2;
        }
    }

    #[allow(unused)]
    pub(super) fn allocate(&mut self, num_pages: usize, alignment: usize) -> usize {
        assert!(num_pages >= alignment / PAGE_SIZE);

        // Find the list which is the best fit for num pages

        // Remove and return the first free chunk of the list

        // If there is none, search larger lists until a free chunk is found
        //  If none can be found, we ran out of memory, panic!

        // Split the chunk found in the larger list as many times as needed

        todo!()
    }

    pub(super) unsafe fn free_at(&mut self, address: usize, num_pages: usize) {
        assert!(address % (num_pages * PAGE_SIZE) == 0);
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
        let new_node = match self.free(address, num_pages) {
            Some(new_node) => unsafe { new_node.as_ref() },
            None => return,
        };

        // Search for any smaller chunks which this allocation contains
        for order in (0..order).rev() {
            let mut iter = self.free_lists[order].iter_mut();
            while let Some(node) = iter.next() {
                if new_node.contains_node(node) {
                    // If found, remove them to be merged with this allocation
                    iter.remove_node(node);
                } else if node.is_above(address) {
                    break;
                }
            }
        }
    }

    pub(super) fn free(&mut self, address: usize, num_pages: usize) -> Option<NonNull<Node>> {
        assert!(address % (num_pages * PAGE_SIZE) == 0);

        // Find the list which is the best fit for num pages
        let order = order(num_pages);

        match self.free_lists[order].free(address) {
            FreeResult::EquivalentFound => return None,
            FreeResult::NewNode { new_node } => return Some(new_node),
            FreeResult::BuddyMerge { address, num_pages } => self.free(address, num_pages),
        }
    }
}
