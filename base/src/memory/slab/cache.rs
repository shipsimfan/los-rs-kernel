use super::{utilization::SlabUtilization, SlabInfo, SlabList};
use crate::memory::{
    slab::{Slab, SlabDescriptor},
    PAGE_SIZE,
};
use core::ptr::NonNull;

pub(in crate::memory) struct Cache {
    full: SlabList,
    partial: SlabList,
    empty: SlabList,
    active_objects: usize,
    info: SlabInfo,
}

const fn calculate_padding_size(object_size: usize, object_alignment: usize) -> usize {
    object_size.next_multiple_of(object_alignment) - object_size
}

impl Cache {
    pub(in crate::memory) const fn new(object_size: usize, object_alignment: usize) -> Self {
        assert!(object_alignment.is_power_of_two());
        assert!(object_size <= PAGE_SIZE - core::mem::size_of::<SlabDescriptor>());

        let padding_size = calculate_padding_size(object_size, object_alignment);
        let (order, num_objects, pre_descriptor_poison_size) =
            SlabUtilization::find_best(object_size, padding_size);
        let info = SlabInfo::new(
            order,
            object_size,
            padding_size,
            num_objects,
            pre_descriptor_poison_size,
        );

        Cache {
            full: SlabList::new(),
            partial: SlabList::new(),
            empty: SlabList::new(),
            active_objects: 0,
            info,
        }
    }

    pub(in crate::memory) fn allocate(&mut self) -> NonNull<u8> {
        self.active_objects += 1;
        self.allocate_from_partial()
            .unwrap_or_else(|| self.allocate_from_empty())
    }

    pub(in crate::memory) fn free(&mut self, address: NonNull<u8>) {
        // Get the slab associated with the address

        // Free the object in the slab

        // See if we need to move the slab from one list to another

        todo!("Cache Free")
    }

    fn allocate_from_partial(&mut self) -> Option<NonNull<u8>> {
        let partial_head = match self.partial.head_mut() {
            Some(slab) => slab,
            None => return None,
        };

        todo!("Allocate from partial")
    }

    fn allocate_from_empty(&mut self) -> NonNull<u8> {
        let mut slab = match self.empty.pop(&self.info) {
            Some(slab) => slab,
            None => Slab::new(&self.info),
        };

        let (object, active_objects) = unsafe { slab.as_mut() }.pop_free(&self.info).unwrap();
        assert_eq!(active_objects, 1);

        self.partial.push(&self.info, slab);

        object
    }
}
