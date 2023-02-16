use super::PhysicalAddress;

pub trait MemoryMap: Iterator<Item = Self::Descriptor> {
    type Descriptor: MemoryDescriptor;
}

pub trait MemoryDescriptor {
    fn address(&self) -> PhysicalAddress;
    fn num_pages(&self) -> usize;
    fn is_freeable(&self) -> bool;
}
