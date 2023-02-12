use super::CriticalKey;
use crate::LocalState;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct CriticalLock<T: Sized + 'static> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct CriticalLockGuard<'a, T: Sized + 'static> {
    lock: &'a AtomicBool,
    data: &'a mut T,
    key: Option<CriticalKey>,
}

impl<T: Sized + 'static> CriticalLock<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        CriticalLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock<'a>(&'a self) -> CriticalLockGuard<'a, T> {
        // Enter local critical
        let key = LocalState::try_get()
            .map(|local_state| unsafe { local_state.critical_state().enter() });

        // Lock globally
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::Relaxed)
            .is_err()
        {
            while self.is_locked() {
                core::hint::spin_loop();
            }
        }

        CriticalLockGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
            key,
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
}

unsafe impl<T: Sized> Sync for CriticalLock<T> {}
unsafe impl<T: Sized> Send for CriticalLock<T> {}

impl<'a, T: Sized + 'static> Deref for CriticalLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T: Sized + 'static> DerefMut for CriticalLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T: Sized + 'static> Drop for CriticalLockGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            // Unlock globally
            self.lock.store(false, Ordering::SeqCst);

            // Leave local critical
            self.key
                .take()
                .map(|key| LocalState::get().critical_state().leave(key));
        }
    }
}
