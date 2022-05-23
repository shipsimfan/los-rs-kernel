use super::{reference::new_reference, Lock, Reference};
use crate::{critical::CriticalLock, map::Mappable};
use alloc::sync::Arc;

pub struct Owner<T, L: Lock<Data = T> = CriticalLock<T>>(Arc<L>);

#[inline(always)]
pub fn new_owner<T, L: Lock<Data = T>>(inner: Arc<L>) -> Owner<T, L> {
    Owner(inner)
}

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

    #[inline(always)]
    pub fn compare(&self, other: &Owner<T, L>) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    pub fn compare_ref(&self, other: &Reference<T, L>) -> bool {
        other.compare(&self.as_ref())
    }
}

impl<T: Mappable, L: Lock<Data = T>> Mappable for Owner<T, L> {
    fn id(&self) -> isize {
        self.0.lock(|inner| inner.id())
    }

    fn set_id(&mut self, id: isize) {
        self.0.lock(|inner| inner.set_id(id))
    }
}

impl<T, L: Lock<Data = T>> Clone for Owner<T, L> {
    fn clone(&self) -> Self {
        Owner(self.0.clone())
    }
}
