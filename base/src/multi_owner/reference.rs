use super::Lock;
use crate::{
    critical::CriticalLock,
    map::{Mappable, INVALID_ID},
};
use alloc::sync::Weak;

pub struct Reference<T, L: Lock<Data = T> = CriticalLock<T>>(Weak<L>);

#[inline(always)]
pub fn new_reference<T, L: Lock<Data = T>>(inner: Weak<L>) -> Reference<T, L> {
    Reference(inner)
}

impl<T, L: Lock<Data = T>> Reference<T, L> {
    #[inline(always)]
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.0.upgrade().map(|lock| lock.lock(f))
    }
}

impl<T: Mappable> Mappable for Reference<T> {
    fn id(&self) -> isize {
        self.0
            .upgrade()
            .map(|t| t.lock().id())
            .unwrap_or(INVALID_ID)
    }

    fn set_id(&mut self, id: isize) {
        self.0.upgrade().map(|t| t.lock().set_id(id));
    }
}

impl<T, L: Lock<Data = T>> Clone for Reference<T, L> {
    fn clone(&self) -> Self {
        Reference(self.0.clone())
    }
}
