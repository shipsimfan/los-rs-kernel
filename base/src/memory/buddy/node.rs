use core::ptr::NonNull;

use crate::memory::PAGE_SIZE;

#[repr(align(4096))]
pub(in crate::memory) struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
    num_pages: usize,
}

impl Node {
    pub(super) fn new(
        address: usize,
        num_pages: usize,
        next: Option<NonNull<Node>>,
        prev: Option<NonNull<Node>>,
    ) -> NonNull<Node> {
        assert!((address / PAGE_SIZE) % num_pages == 0);

        let mut ptr = address as *mut Node;
        unsafe {
            *ptr = Node {
                next,
                prev,
                num_pages,
            }
        };
        NonNull::new(ptr).unwrap()
    }

    pub(super) fn contains(&self, address: usize, num_pages: usize) -> bool {
        if self.is_above(address) {
            return false;
        }

        if address + num_pages > self as *const _ as usize + num_pages * PAGE_SIZE {
            return false;
        }

        true
    }

    pub(super) fn contains_node(&self, node: &Node) -> bool {
        self.contains(node as *const _ as usize, node.num_pages)
    }

    pub(super) fn equal(&self, address: usize) -> bool {
        address == self as *const _ as usize
    }

    pub(super) fn is_above(&self, address: usize) -> bool {
        address < self as *const _ as usize
    }

    pub(super) fn next(&self) -> Option<NonNull<Node>> {
        self.next
    }

    pub(super) fn remove(&mut self) {
        self.next
            .map(|mut next| unsafe { next.as_mut().set_prev(self.prev) });
        self.prev
            .map(|mut prev| unsafe { prev.as_mut().set_next(self.next) });
    }

    pub(super) fn set_next(&mut self, next: Option<NonNull<Node>>) {
        self.next = next;
    }

    pub(super) fn set_prev(&mut self, prev: Option<NonNull<Node>>) {
        self.prev = prev;
    }
}
