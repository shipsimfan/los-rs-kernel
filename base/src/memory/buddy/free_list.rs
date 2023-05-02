use super::Page;
use crate::memory::buddy::{order_to_size, MAX_ORDER};
use core::{marker::PhantomData, ptr::NonNull};

pub(super) struct FreeList {
    head: Option<NonNull<Page>>,
    order: u8,
    count: usize,
}

pub(super) struct FreeListIter<'a> {
    current: Option<NonNull<Page>>,
    phantom: PhantomData<&'a ()>,
}

pub(super) struct FreeListIterMut<'a> {
    current: Option<NonNull<Page>>,
    phantom: PhantomData<&'a mut ()>,
}

impl FreeList {
    pub(super) const fn new(order: u8) -> Self {
        assert!(order < MAX_ORDER);

        FreeList {
            head: None,
            order,
            count: 0,
        }
    }

    pub(super) fn contains(&self, address: usize) -> bool {
        assert_eq!(address % order_to_size(self.order), 0);

        for node in self {
            if node as *const Page as usize == address {
                return true;
            }
        }

        false
    }

    pub(super) fn remove(&mut self, address: usize) -> Option<NonNull<Page>> {
        assert_eq!(address % order_to_size(self.order), 0);

        let (mut prev, mut current) = match self.head {
            Some(head) => match head.as_ptr() as usize == address {
                true => return self.pop(),
                false => (head, unsafe { head.as_ref() }.next()),
            },
            None => return None,
        };

        while let Some(current_node) = current {
            if current_node.as_ptr() as usize > address {
                return None;
            }

            if current_node.as_ptr() as usize == address {
                unsafe { prev.as_mut() }.set_next(unsafe { current_node.as_ref() }.next());
                self.count -= 1;
                return Some(current_node);
            }

            prev = current_node;
            current = unsafe { current_node.as_ref() }.next();
        }

        current
    }

    pub(super) fn pop(&mut self) -> Option<NonNull<Page>> {
        let ret = self.head;

        self.head = match self.head.map(|head| unsafe { head.as_ref() }) {
            Some(head) => head.next(),
            None => None,
        };

        if ret.is_some() {
            self.count -= 1;
        }

        ret
    }

    pub(super) fn insert(&mut self, mut new_node: NonNull<Page>) {
        assert_eq!(self.order, unsafe { new_node.as_ref() }.order());
        assert_eq!((new_node.as_ptr() as usize) % order_to_size(self.order), 0);

        self.count += 1;

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        for node in self {
            assert_ne!(node as *mut Page, new_node.as_ptr());

            if let Some(next) = node.next() {
                assert_ne!(next.as_ptr(), new_node.as_ptr());

                if next.as_ptr() < new_node.as_ptr() {
                    continue;
                }
            }

            unsafe { new_node.as_mut() }.set_next(node.next());
            node.set_next(Some(new_node));

            break;
        }
    }

    fn iter(&self) -> FreeListIter {
        FreeListIter {
            current: self.head,
            phantom: PhantomData,
        }
    }

    fn iter_mut(&mut self) -> FreeListIterMut {
        FreeListIterMut {
            current: self.head,
            phantom: PhantomData,
        }
    }
}

impl<'a> IntoIterator for &'a FreeList {
    type Item = &'a Page;
    type IntoIter = FreeListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut FreeList {
    type Item = &'a mut Page;
    type IntoIter = FreeListIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> Iterator for FreeListIter<'a> {
    type Item = &'a Page;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current.map(|node| unsafe { node.as_ref() });
        self.current = match ret {
            Some(node) => node.next(),
            None => None,
        };
        ret
    }
}

impl<'a> Iterator for FreeListIterMut<'a> {
    type Item = &'a mut Page;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current.map(|mut node| unsafe { node.as_mut() });
        self.current = match self.current.map(|node| unsafe { node.as_ref() }) {
            Some(node) => node.next(),
            None => None,
        };
        ret
    }
}
