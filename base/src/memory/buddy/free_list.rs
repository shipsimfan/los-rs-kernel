use super::node::Node;
use core::{marker::PhantomData, ptr::NonNull};

pub(super) struct FreeList {
    head: Option<NonNull<Node>>,
    pages_per_node: usize,
}

pub(super) struct Iter<'a> {
    current: Option<NonNull<Node>>,
    phantom: PhantomData<&'a ()>,
}

impl FreeList {
    pub(super) const fn new() -> Self {
        FreeList {
            head: None,
            pages_per_node: 1,
        }
    }
}

impl<'a> IntoIterator for &'a FreeList {
    type IntoIter = Iter<'a>;
    type Item = &'a Node;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            current: self.head,
            phantom: PhantomData,
        }
    }
}

impl Clone for FreeList {
    fn clone(&self) -> Self {
        FreeList {
            head: self.head,
            pages_per_node: self.pages_per_node * 2,
        }
    }
}

impl Copy for FreeList {}

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
