use super::{Object, SlabDescriptor, SlabInfo};
use crate::{memory::buddy::order_to_size, MemoryManager};
use core::ptr::NonNull;

pub(super) struct Slab;

const PRE_DESCRIPTOR_POISON: u8 = 0xC9;

fn calculate_descriptor_ptr(ptr: usize, order: u8) -> usize {
    ptr + order_to_size(order) - core::mem::size_of::<SlabDescriptor>()
}

fn calculate_descriptor_poison_ptr(ptr: usize, order: u8, posion_size: usize) -> usize {
    calculate_descriptor_ptr(ptr, order) - posion_size
}

fn calculate_index(ptr: usize, index: u16, total_object_size: usize) -> usize {
    ptr + index as usize * total_object_size
}

impl Slab {
    pub(super) fn new(info: &SlabInfo) -> NonNull<Self> {
        let mut ptr = MemoryManager::get()
            .buddy_allocator
            .lock()
            .allocate(info.order())
            .cast::<Self>();

        unsafe { ptr.as_mut() }.initialize(info);

        ptr
    }

    pub(super) fn next(&self) -> Option<NonNull<Slab>> {
        todo!("Slab next")
    }

    pub(super) fn pop_free(&mut self, info: &SlabInfo) -> Option<(NonNull<u8>, usize)> {
        let mut descriptor = self.descriptor(info, true);
        let object_index = match unsafe { descriptor.as_mut() }.take_first_free() {
            Some(index) => index,
            None => return None,
        };

        let object = self.get(object_index, info.total_object_size());

        let next = unsafe { object.as_ref() }.next(info);

        unsafe { descriptor.as_mut() }.set_first_free(next);

        Some((
            object.cast(),
            unsafe { descriptor.as_ref() }.active_objects(),
        ))
    }

    pub(super) fn set_next(&mut self, info: &SlabInfo, next: Option<NonNull<Slab>>) {
        unsafe { self.descriptor(info, true).as_mut() }.set_next(next);
    }

    pub(super) fn set_prev(&mut self, info: &SlabInfo, prev: Option<NonNull<Slab>>) {
        todo!("Slab set prev");
    }

    fn get(&self, index: u16, total_object_size: usize) -> NonNull<Object> {
        NonNull::new(
            calculate_index(self as *const _ as usize, index, total_object_size) as *mut Object,
        )
        .unwrap()
    }

    fn descriptor(&self, info: &SlabInfo, verify_poison: bool) -> NonNull<SlabDescriptor> {
        if verify_poison {
            self.verify_descriptor_poison(info);
        }

        NonNull::new(calculate_descriptor_ptr(self as *const _ as usize, info.order()) as *mut _)
            .unwrap()
    }

    fn verify_descriptor_poison(&self, info: &SlabInfo) {
        let ptr: &u8 = self.map_ptr(|ptr| {
            calculate_descriptor_poison_ptr(ptr, info.order(), info.pre_descriptor_poison_size())
        });

        let slice = unsafe { core::slice::from_raw_parts(ptr, info.pre_descriptor_poison_size()) };

        for byte in slice {
            assert_eq!(*byte, PRE_DESCRIPTOR_POISON);
        }
    }

    fn map_ptr<F, T>(&self, f: F) -> &T
    where
        F: Fn(usize) -> usize,
    {
        unsafe { &*(f(self as *const _ as usize) as *const T) }
    }

    fn map_ptr_mut<F, T>(&mut self, f: F) -> &mut T
    where
        F: Fn(usize) -> usize,
    {
        unsafe { &mut *(f(self as *mut _ as usize) as *mut T) }
    }

    fn initialize(&mut self, info: &SlabInfo) {
        assert_eq!(self as *const _ as usize % order_to_size(info.order()), 0);

        unsafe { self.descriptor(info, false).as_mut() }.initialize();

        for i in 0..info.num_objects() {
            unsafe { self.get(i as u16, info.total_object_size()).as_mut() }.initialize(
                info,
                if i < info.num_objects() - 1 {
                    Some((i + 1) as u16)
                } else {
                    None
                },
            );
        }

        let ptr: &mut u8 = self.map_ptr_mut(|ptr| {
            calculate_descriptor_poison_ptr(ptr, info.order(), info.pre_descriptor_poison_size())
        });
        let slice =
            unsafe { core::slice::from_raw_parts_mut(ptr, info.pre_descriptor_poison_size()) };
        for byte in slice {
            *byte = PRE_DESCRIPTOR_POISON;
        }
    }
}
