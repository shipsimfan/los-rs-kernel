use core::ptr::NonNull;

use super::Slab;

pub(super) struct SlabDescriptor {
    poison: u64,
    next: Option<NonNull<Slab>>,
    prev: Option<NonNull<Slab>>,
    free_list: Option<u16>,
    active_objects: u16,
}

const DESCRIPTOR_POISON: u64 = 0xAEDCD308A20AAE04;

impl SlabDescriptor {
    pub(super) fn initialize(&mut self) {
        *self = SlabDescriptor {
            poison: DESCRIPTOR_POISON,
            next: None,
            prev: None,
            free_list: Some(0),
            active_objects: 0,
        };
    }

    pub(super) fn active_objects(&self) -> usize {
        assert_eq!(self.poison, DESCRIPTOR_POISON);
        self.active_objects as usize
    }

    pub(super) fn take_first_free(&mut self) -> Option<u16> {
        assert_eq!(self.poison, DESCRIPTOR_POISON);
        self.active_objects += 1;
        self.free_list.take()
    }

    pub(super) fn set_first_free(&mut self, next: Option<u16>) {
        assert_eq!(self.poison, DESCRIPTOR_POISON);
        self.free_list = next;
    }

    pub(super) fn set_next(&mut self, next: Option<NonNull<Slab>>) {
        assert_eq!(self.poison, DESCRIPTOR_POISON);
        self.next = next;
    }
}
