use super::MemoryDescriptor;
use crate::raw;
use core::ptr::NonNull;

pub struct MemoryMap {
    current: NonNull<raw::MemoryDescriptor>,
    descriptor_size: usize,

    remaining_descriptors: usize,
}

impl From<NonNull<raw::MemoryMap>> for MemoryMap {
    fn from(raw: NonNull<raw::MemoryMap>) -> Self {
        unsafe {
            let raw = raw.as_ref();
            MemoryMap {
                current: NonNull::new(raw.address().into_virtual()).unwrap(),
                descriptor_size: raw.descriptor_size(),
                remaining_descriptors: raw.size() / raw.descriptor_size(),
            }
        }
    }
}

impl base::MemoryMap for MemoryMap {
    type Descriptor = MemoryDescriptor;

    fn bottom_and_top(&self) -> (usize, usize) {
        let bottom = self.current.as_ptr() as usize;
        (
            bottom,
            bottom + self.descriptor_size * self.remaining_descriptors,
        )
    }
}

impl Iterator for MemoryMap {
    type Item = MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_descriptors == 0 {
            return None;
        }

        self.remaining_descriptors -= 1;

        let ptr = self.current;
        self.current =
            NonNull::new((self.current.as_ptr() as usize + self.descriptor_size) as *mut _)
                .unwrap();
        Some(ptr.into())
    }
}
