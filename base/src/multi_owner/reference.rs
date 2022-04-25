use super::Lock;
use alloc::sync::Weak;

pub struct Reference<T, L: Lock<Data = T>>(Weak<L>);

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
