use crate::memory::buddy::order_to_size;
use core::ptr::NonNull;

pub(super) struct SlabDescriptor {
    poison: u64,
    next: Option<NonNull<SlabDescriptor>>,
    prev: Option<NonNull<SlabDescriptor>>,
    free_list: Option<u16>,
    active_objects: u16,
}

const DESCRIPTOR_POISON: u64 = 0xAEDCD308A20AAE04;
const OBJECT_POISON: u8 = 0xF1;
const PADDING_POISON: u8 = 0x4A;

fn slab_to_descriptor(address: NonNull<u8>, order: u8) -> NonNull<SlabDescriptor> {
    NonNull::new(
        (address.as_ptr() as usize + order_to_size(order) - core::mem::size_of::<SlabDescriptor>())
            as *mut SlabDescriptor,
    )
    .unwrap()
}

fn set_and_increment(address: &mut NonNull<u8>, value: u8) {
    *unsafe { address.as_mut() } = value;
    *address = unsafe { NonNull::new_unchecked(address.as_ptr().add(1)) };
}

pub(super) fn initialize_slab(
    mut address: NonNull<u8>,
    order: u8,
    object_size: usize,
    padding_size: usize,
    num_objects: usize,
    next: Option<NonNull<SlabDescriptor>>,
    prev: Option<NonNull<SlabDescriptor>>,
) -> NonNull<SlabDescriptor> {
    assert_eq!(address.as_ptr() as usize % order_to_size(order), 0);
    assert!(num_objects > 0);
    assert!(object_size >= 2);
    assert!(
        order_to_size(order)
            >= core::mem::size_of::<SlabDescriptor>()
                + object_size * num_objects
                + padding_size * (num_objects - 1)
    );

    // Initialize the descriptor
    let mut descriptor = slab_to_descriptor(address, order);
    *unsafe { descriptor.as_mut() } = SlabDescriptor {
        poison: DESCRIPTOR_POISON,
        next,
        prev,
        free_list: Some(0),
        active_objects: 0,
    };

    // Initialize each object and padding
    for i in 0..num_objects {
        for _ in 0..object_size {
            set_and_increment(&mut address, OBJECT_POISON);
        }

        if i == num_objects - 1 {
            continue;
        }

        for _ in 0..padding_size {
            set_and_increment(&mut address, PADDING_POISON);
        }
    }

    while (address.as_ptr() as usize)
        < order_to_size(order) - core::mem::size_of::<SlabDescriptor>()
    {
        set_and_increment(&mut address, PADDING_POISON);
    }

    descriptor
}
