use super::{node::Node, SlabMetadata};
use crate::{CriticalLock, MemoryManager};
use core::ptr::NonNull;

pub(super) struct SlabNode {
    next: Option<NonNull<SlabNode>>,
    prev: Option<NonNull<SlabNode>>,
    slab: CriticalLock<Slab>,
}

struct Slab {
    free_list: Option<NonNull<Node>>,
    allocated_objects: usize,
}

impl SlabNode {
    pub(super) fn new(metadata: &SlabMetadata) -> NonNull<Self> {
        // Allocate the slab
        let mut new_slab = MemoryManager::get()
            .allocate_pages(metadata.pages_per_slab)
            .cast::<Self>();

        // Initialize the metadata
        let mut slab = unsafe { new_slab.as_mut() };
        slab.next = None;
        slab.prev = None;

        let mut ptr = unsafe { (new_slab.as_ptr() as *mut Node).byte_add(metadata.object_offset) };
        slab.slab = CriticalLock::new(Slab::new(unsafe { NonNull::new_unchecked(ptr) }));

        // Initialize the free list
        unsafe {
            for i in 0..metadata.objects_per_slab {
                let next = if i == metadata.objects_per_slab - 1 {
                    None
                } else {
                    Some(NonNull::new_unchecked(ptr.byte_add(metadata.object_size)))
                };

                *ptr = Node::new(next);

                ptr = ptr.byte_add(metadata.object_size);
            }
        }

        new_slab
    }

    pub(super) fn remove(&mut self) -> Option<NonNull<SlabNode>> {
        let ret = self.next;

        self.next
            .map(|mut next| unsafe { next.as_mut().prev = self.prev });
        self.prev
            .map(|mut prev| unsafe { prev.as_mut().next = self.next });

        self.next = None;
        self.prev = None;

        ret
    }

    pub(super) fn allocate<T>(&self) -> (NonNull<T>, usize) {
        self.slab.lock().allocate()
    }
}

impl Slab {
    pub(self) fn new(free_list: NonNull<Node>) -> Self {
        Slab {
            free_list: Some(free_list),
            allocated_objects: 0,
        }
    }

    pub(self) fn allocate<T>(&mut self) -> (NonNull<T>, usize) {
        let mut ret = self.free_list.unwrap();
        self.free_list = unsafe { ret.as_mut().take_next() };
        self.allocated_objects += 1;
        (ret.cast(), self.allocated_objects)
    }
}
