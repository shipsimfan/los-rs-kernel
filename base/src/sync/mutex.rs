use crate::CriticalLock;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

pub struct Mutex<T: 'static> {
    inner: CriticalLock<MutexInner<T>>,
}

struct MutexInner<T: 'static> {
    lock: bool,
    // TODO: Add a thread queue
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: 'static> {
    lock: &'a Mutex<T>,
    data: &'a mut T,
}

impl<T: 'static> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            inner: CriticalLock::new(MutexInner {
                lock: false,
                data: UnsafeCell::new(data),
            }),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        let mut inner = self.inner.lock();

        if inner.lock {
            panic!("Locking an already locked mutex!");
            // TODO: Check if a local state exists,
            //  If so, place the current thread into a queue instead of panicing
        }

        inner.lock = true;

        MutexGuard {
            lock: self,
            data: unsafe { &mut *inner.data.get() },
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let mut inner = self.inner.lock();

        if inner.lock {
            None
        } else {
            inner.lock = true;
            Some(MutexGuard {
                lock: self,
                data: unsafe { &mut *inner.data.get() },
            })
        }
    }

    pub(self) fn unlock(&self) {
        let mut inner = self.inner.lock();
        // TODO: Wake a thread from the queue instead of unlocking if one is waiting
        inner.lock = false;
    }
}

unsafe impl<T: 'static> Send for Mutex<T> {}
unsafe impl<T: 'static> Sync for Mutex<T> {}

impl<'a, T: 'static> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T: 'static> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T: 'static> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}
