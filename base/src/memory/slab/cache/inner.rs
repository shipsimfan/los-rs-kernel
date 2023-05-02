pub(in crate::memory) struct CacheInner {}

impl CacheInner {
    pub(in crate::memory) const fn new(size: usize, alignment: usize) -> Self {
        // Calculate the alignment padding

        // Find the ideal slab order

        CacheInner {}
    }

    pub(in crate::memory) const fn new_type<T>() -> Self {
        CacheInner::new(core::mem::size_of::<T>(), core::mem::align_of::<T>())
    }
}
