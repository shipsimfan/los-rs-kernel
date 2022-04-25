use super::{reference::new_reference, Lock, Reference};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct Owner<T, L: Lock<Data = T>>(Arc<L>);

impl<T, L: Lock<Data = T>> Owner<T, L> {
    #[inline(always)]
    pub fn new(data: T) -> Self {
        Owner(Arc::new(L::new(data)))
    }

    #[inline(always)]
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.0.lock(f)
    }

    #[inline(always)]
    pub fn as_ref(&self) -> Reference<T, L> {
        new_reference(Arc::downgrade(&self.0))
    }
}
