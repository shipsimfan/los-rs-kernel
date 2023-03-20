use super::PhysicalAddress;

pub trait MemoryMap: Iterator<Item = Self::Descriptor> {
    type Descriptor: MemoryDescriptor;

    fn bottom_and_top(&self) -> (usize, usize);
}

pub trait MemoryDescriptor {
    fn address(&self) -> PhysicalAddress;
    fn num_pages(&self) -> usize;
    fn is_usable(&self) -> bool;
}
