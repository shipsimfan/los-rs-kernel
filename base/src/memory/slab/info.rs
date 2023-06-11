use super::SlabDescriptor;
use crate::memory::buddy::order_to_size;

pub(super) struct SlabInfo {
    order: u8,
    object_size: usize,
    total_object_size: usize,
    num_objects: usize,
    pre_descriptor_poison_size: usize,
}

impl SlabInfo {
    pub(super) const fn new(
        order: u8,
        object_size: usize,
        padding_size: usize,
        num_objects: usize,
        pre_descriptor_poison_size: usize,
    ) -> Self {
        assert!(num_objects > 1);
        assert!(num_objects < u16::MAX as usize);
        assert!(object_size >= 2);
        assert!(
            order_to_size(order)
                == core::mem::size_of::<SlabDescriptor>()
                    + object_size * num_objects
                    + padding_size * (num_objects - 1)
                    + pre_descriptor_poison_size
        );

        SlabInfo {
            order,
            object_size,
            total_object_size: object_size + padding_size,
            num_objects,
            pre_descriptor_poison_size,
        }
    }

    pub(super) fn order(&self) -> u8 {
        self.order
    }

    pub(super) fn object_size(&self) -> usize {
        self.object_size
    }

    pub(super) fn total_object_size(&self) -> usize {
        self.total_object_size
    }

    pub(super) fn num_objects(&self) -> usize {
        self.num_objects
    }

    pub(super) fn pre_descriptor_poison_size(&self) -> usize {
        self.pre_descriptor_poison_size
    }
}
