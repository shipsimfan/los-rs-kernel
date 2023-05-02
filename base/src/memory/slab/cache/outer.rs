use super::CacheInner;
use core::{
    alloc::{AllocError, Allocator, Layout},
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct Cache<T> {
    inner: NonNull<(AtomicUsize, CacheInner)>,
    phantom: PhantomData<T>,
}

impl<T> Cache<T> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<T> Clone for Cache<T> {
    fn clone(&self) -> Self {
        unsafe { self.inner.as_ref() }
            .0
            .fetch_add(1, Ordering::Acquire);
        Cache {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for Cache<T> {
    fn drop(&mut self) {
        let count = unsafe { self.inner.as_ref() }
            .0
            .fetch_sub(1, Ordering::Acquire);

        if count == 1 {
            unsafe { core::ptr::drop_in_place(self.inner.as_ptr()) }
        }
    }
}

unsafe impl<T> Allocator for Cache<T> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        assert_eq!(layout, Layout::new::<T>());

        todo!()
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }

    unsafe fn grow(
        &self,
        _: NonNull<u8>,
        _: Layout,
        _: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        panic!("Cannot grow an item in a cache");
    }

    unsafe fn shrink(
        &self,
        _: NonNull<u8>,
        _: Layout,
        _: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        panic!("Cannot shrink an item in a cache");
    }
}
