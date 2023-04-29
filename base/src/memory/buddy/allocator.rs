use super::{FreeList, Page, MAX_ORDER};
use crate::memory::{buddy::order_to_size, PAGE_SIZE};
use core::ptr::NonNull;

pub(in crate::memory) struct BuddyAllocator {
    free_lists: [FreeList; MAX_ORDER as usize],
}

fn calculate_buddy(address: usize, order: u8) -> NonNull<Page> {
    let adjust = order_to_size(order);

    NonNull::new(if (address >> order) & PAGE_SIZE == 0 {
        address + adjust
    } else {
        address - adjust
    } as *mut Page)
    .unwrap()
}

impl BuddyAllocator {
    pub(in crate::memory) const fn new() -> Self {
        BuddyAllocator {
            free_lists: [
                FreeList::new(0),
                FreeList::new(1),
                FreeList::new(2),
                FreeList::new(3),
                FreeList::new(4),
                FreeList::new(5),
                FreeList::new(6),
                FreeList::new(7),
                FreeList::new(8),
                FreeList::new(9),
                FreeList::new(10),
                FreeList::new(11),
                FreeList::new(12),
                FreeList::new(13),
                FreeList::new(14),
                FreeList::new(15),
            ],
        }
    }

    pub(in crate::memory) fn allocate(&mut self, order: u8) -> NonNull<u8> {
        assert!(order < MAX_ORDER);

        if let Some(page) = self.free_lists[order as usize].pop() {
            return page.cast();
        }

        for t_order in order + 1..MAX_ORDER {
            if let Some(mut page) = self.free_lists[t_order as usize].pop() {
                self.break_page(unsafe { page.as_mut() }, order);
                return page.cast();
            }
        }

        panic!("No page can be found for an order {} allocation", order);
    }

    pub(in crate::memory) fn free(&mut self, address: NonNull<u8>, order: u8) {
        assert!(order < MAX_ORDER);

        if order == MAX_ORDER - 1 {
            return self.free_lists[order as usize].insert(Page::new(address, order));
        }

        let buddy = calculate_buddy(address.as_ptr() as usize, order);

        match self.free_lists[order as usize].remove(buddy.as_ptr() as usize) {
            Some(buddy) => {
                let main = if (address.as_ptr() as usize) < buddy.as_ptr() as usize {
                    address
                } else {
                    buddy.cast()
                };

                self.free(main, order + 1);
            }
            None => self.free_lists[order as usize].insert(Page::new(address, order)),
        }
    }

    pub(super) fn init_free(&mut self, address: usize) {
        assert_eq!(address % PAGE_SIZE, 0);

        // Check for existence in all free lists
        for order in 0..MAX_ORDER {
            if self.free_lists[order as usize].contains(address & !(order_to_size(order) - 1)) {
                return;
            }
        }

        // If not already freed, free it
        self.free(NonNull::new(address as *mut u8).unwrap(), 0);
    }

    fn break_page(&mut self, page: &mut Page, target_order: u8) {
        for order in (target_order..page.order()).rev() {
            let buddy = page.split();
            self.free_lists[order as usize].insert(buddy);
        }

        assert!(page.order() == target_order);
    }
}
