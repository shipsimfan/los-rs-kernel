use crate::{critical::CriticalLock, map::Mappable};

use super::{reference::new_reference, Lock, Reference};
use alloc::sync::Arc;

pub struct Owner<T, L: Lock<Data = T> = CriticalLock<T>>(Arc<L>);

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

impl<T: Mappable> Mappable for Owner<T> {
    fn id(&self) -> isize {
        self.0.lock().id()
    }

    fn set_id(&mut self, id: isize) {
        self.0.lock().set_id(id)
    }
}

impl<T, L: Lock<Data = T>> Clone for Owner<T, L> {
    fn clone(&self) -> Self {
        Owner(self.0.clone())
    }
}
