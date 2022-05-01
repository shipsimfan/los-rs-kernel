use super::{owner::new_owner, Lock, Owner};
use crate::{
    critical::CriticalLock,
    logging::{LogOutput, LogOutputMut},
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

    #[inline(always)]
    pub fn compare(&self, other: &Reference<T, L>) -> bool {
        self.0.ptr_eq(&other.0)
    }

    pub fn upgrade(self) -> Owner<T, L> {
        new_owner(self.0.upgrade().unwrap())
    }

    pub fn alive(&self) -> bool {
        self.0.strong_count() > 0
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

impl<T: LogOutputMut, L: Lock<Data = T>> LogOutput for Reference<T, L> {
    fn log(&self, event: crate::logging::LogEvent) {
        self.lock(|inner| inner.log(event));
    }
}
