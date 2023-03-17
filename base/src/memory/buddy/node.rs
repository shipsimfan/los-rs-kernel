use core::ptr::NonNull;

use crate::memory::PAGE_SIZE;

#[repr(align(4096))]
pub(super) struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
    num_pages: usize,
}

impl Node {
    pub(super) fn contains(&self, address: usize, num_pages: usize) -> bool {
        if self.is_above(address) {
            return false;
        }

        if address + num_pages > self as *const _ as usize + num_pages * PAGE_SIZE {
            return false;
        }

        true
    }

    pub(super) fn is_above(&self, address: usize) -> bool {
        address < self as *const _ as usize
    }

    pub(super) fn next(&self) -> Option<NonNull<Node>> {
        self.next
    }
}
