use super::node::Node;
use crate::memory::PAGE_SIZE;
use core::{marker::PhantomData, ptr::NonNull};

pub(super) enum FreeResult {
    EquivalentFound,
    NewNode { new_node: NonNull<Node> },
    BuddyMerge { address: usize, num_pages: usize },
}

#[derive(Clone, Copy)]
pub(super) struct FreeList {
    head: Option<NonNull<Node>>,
    pages_per_node: usize,
}

pub(super) struct Iter<'a> {
    current: Option<NonNull<Node>>,
    phantom: PhantomData<&'a ()>,
}

pub(super) struct IterMut<'a> {
    list: &'a mut FreeList,
    current: Option<NonNull<Node>>,
}

fn calculate_buddy(address: usize, num_pages: usize) -> (usize, bool) {
    if (address / (num_pages * PAGE_SIZE)) & 1 == 1 {
        (address - (num_pages * PAGE_SIZE), true)
    } else {
        (address + (num_pages * PAGE_SIZE), false)
    }
}

impl FreeList {
    pub(super) const fn new() -> Self {
        FreeList {
            head: None,
            pages_per_node: 1,
        }
    }

    pub(super) fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            current: self.head,
            phantom: PhantomData,
        }
    }

    pub(super) unsafe fn set_pages_per_node(&mut self, pages_per_node: usize) {
        assert!(self.head.is_none());
        self.pages_per_node = pages_per_node;
    }

    pub(super) fn iter_mut<'a>(&'a mut self) -> IterMut<'a> {
        IterMut {
            current: self.head,
            list: self,
        }
    }

    pub(self) fn remove_node(&mut self, node: &mut Node) {
        node.remove(&mut self.head)
    }

    pub(super) fn free(&mut self, address: usize) -> FreeResult {
        assert!((address / PAGE_SIZE) % self.pages_per_node == 0);

        if self.head.is_none() {
            let new_node = Node::new(address, self.pages_per_node, None, None);
            self.head = Some(new_node);
            return FreeResult::NewNode { new_node };
        }

        let (buddy, down) = calculate_buddy(address, self.pages_per_node);
        let mut current_ptr = self.head.unwrap();
        let mut current = unsafe { current_ptr.as_mut() };
        loop {
            if current.equal(address) {
                return FreeResult::EquivalentFound;
            }

            if down && current.equal(buddy) {
                current.remove(&mut self.head);
                return FreeResult::BuddyMerge {
                    address: buddy,
                    num_pages: self.pages_per_node * 2,
                };
            }

            match current.next() {
                Some(mut next_ptr) => {
                    let next = unsafe { next_ptr.as_mut() };
                    if !down && next.equal(buddy) {
                        next.remove(&mut self.head);
                        return FreeResult::BuddyMerge {
                            address,
                            num_pages: self.pages_per_node * 2,
                        };
                    } else if next.is_above(address) {
                        let new_node = Node::new(
                            address,
                            self.pages_per_node,
                            Some(next_ptr),
                            Some(current_ptr),
                        );

                        current.set_next(Some(new_node));
                        next.set_prev(Some(new_node));

                        return FreeResult::NewNode { new_node };
                    }

                    current_ptr = next_ptr;
                    current = next;
                }
                None => {
                    let new_node = Node::new(address, self.pages_per_node, None, Some(current_ptr));
                    current.set_next(Some(new_node));
                    return FreeResult::NewNode { new_node };
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a FreeList {
    type IntoIter = Iter<'a>;
    type Item = &'a Node;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut FreeList {
    type IntoIter = IterMut<'a>;
    type Item = &'a mut Node;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                let node = unsafe { node.as_ref() };
                self.current = node.next();
                Some(node)
            }
            None => None,
        }
    }
}

impl<'a> IterMut<'a> {
    pub(super) fn remove_node(&mut self, node: &mut Node) {
        self.list.remove_node(node);
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(mut node) => {
                let node = unsafe { node.as_mut() };
                self.current = node.next();
                Some(node)
            }
            None => None,
        }
    }
}
