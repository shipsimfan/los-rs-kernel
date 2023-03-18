use super::{mask, PAGE_SIZE};
use crate::{CriticalLock, MemoryManager};
use core::{alloc::Allocator, ptr::NonNull};
use slab::SlabNode;

mod node;
mod slab;

pub struct SlabAllocator {
    partial_list: CriticalLock<Option<NonNull<SlabNode>>>,
    metadata: SlabMetadata,
}

struct SlabMetadata {
    pages_per_slab: usize,
    slab_mask: usize,
    object_size: usize,
    object_alignment: usize,
    object_offset: usize,
    objects_per_slab: usize,
}

const TARGET_UNUSED_RATIO: usize = 8;

impl SlabAllocator {
    pub const fn new(object_size: usize, object_alignment: usize) -> Self {
        SlabAllocator {
            partial_list: CriticalLock::new(None),
            metadata: SlabMetadata::new(object_size, object_alignment),
        }
    }

    pub fn new_for<T: Sized>() -> Self {
        Self::new(core::mem::size_of::<T>(), core::mem::align_of::<T>())
    }
}

unsafe impl Allocator for SlabAllocator {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        // Verify the requested allocation is compatible with the slab
        assert!(layout.size() <= self.metadata.object_size);
        assert!(layout.align() <= self.metadata.object_alignment);

        let mut partial_list = self.partial_list.lock();

        // Get the slab to allocate from
        let slab = unsafe {
            match *partial_list {
                Some(slab) => slab,
                None => {
                    let new_slab = SlabNode::new(&self.metadata);
                    *partial_list = Some(new_slab);
                    new_slab
                }
            }
            .as_mut()
        };

        // Allocate the object from the slab
        let (ret, allocated_objects) = slab.allocate();

        // If the slab is fully used, remove it from the partial list
        if allocated_objects == self.metadata.objects_per_slab {
            *partial_list = slab.remove();
        }

        Ok(NonNull::slice_from_raw_parts(ret, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: core::alloc::Layout) {
        // Verify the requested deallocation is compatible with the slab
        assert!(layout.size() <= self.metadata.object_size);
        assert!(layout.align() <= self.metadata.object_alignment);

        // Get the slab the allocation is from
        let mut slab =
            NonNull::new((ptr.as_ptr() as usize & self.metadata.slab_mask) as *mut SlabNode)
                .unwrap();

        // Free the object in the slab
        let allocated_objects = unsafe { slab.as_ref() }.free(ptr);

        // If the slab was fully used, insert it into the partial list
        if allocated_objects == self.metadata.objects_per_slab {
            let mut partial_list = self.partial_list.lock();
            unsafe { slab.as_mut() }.set_next(*partial_list);
            *partial_list = Some(slab);
        } else if allocated_objects == 1 {
            // If the slab had only one allocation left, free the whole slab
            let mut partial_list = self.partial_list.lock();

            let next = unsafe { slab.as_mut() }.remove();
            if slab == partial_list.unwrap() {
                *partial_list = next;
            }

            drop(partial_list);

            MemoryManager::get().free_pages(slab.cast(), self.metadata.pages_per_slab);
        }
    }
}

impl SlabMetadata {
    pub(self) const fn new(object_size: usize, object_alignment: usize) -> Self {
        let object_offset = core::mem::size_of::<SlabNode>().next_multiple_of(object_alignment);
        let object_size = object_size.next_multiple_of(object_alignment);

        let mut pages_per_slab = 1;
        let target_avaible_space = object_offset * TARGET_UNUSED_RATIO;
        loop {
            if pages_per_slab * PAGE_SIZE >= target_avaible_space {
                break;
            }

            pages_per_slab *= 2;
        }

        let objects_per_slab = (pages_per_slab * PAGE_SIZE - object_offset) / object_size;

        let slab_mask = mask!(pages_per_slab * PAGE_SIZE);

        Self {
            pages_per_slab,
            slab_mask,
            object_size,
            object_alignment,
            object_offset,
            objects_per_slab,
        }
    }
}
