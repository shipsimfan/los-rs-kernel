use super::MemoryDescriptor;
use crate::raw;

pub struct MemoryMap {
    current: *const raw::MemoryDescriptor,
    descriptor_size: usize,

    remaining_descriptors: usize,
}

impl From<*const raw::MemoryMap> for MemoryMap {
    fn from(raw: *const raw::MemoryMap) -> Self {
        unsafe {
            MemoryMap {
                current: (*raw).address(),
                descriptor_size: (*raw).descriptor_size(),
                remaining_descriptors: (*raw).size() / (*raw).descriptor_size(),
            }
        }
    }
}

impl base::MemoryMap for MemoryMap {
    type Descriptor = MemoryDescriptor;
}

impl Iterator for MemoryMap {
    type Item = MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_descriptors == 0 {
            return None;
        }

        let ptr = self.current;
        self.current = unsafe { self.current.byte_add(self.descriptor_size) };
        Some(ptr.into())
    }
}
