use super::CriticalKey;
use crate::{local::LocalState, BootVideo, LogOutput};
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct CriticalLock<T: Sized + 'static> {
    ticket: AtomicUsize,
    now_serving: AtomicUsize,
    data: UnsafeCell<T>,
}

pub struct CriticalLockGuard<'a, T: Sized + 'static> {
    queue: &'a CriticalLock<T>,
    data: &'a mut T,
    key: Option<CriticalKey>,
}

impl<T: Sized + 'static> CriticalLock<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        CriticalLock {
            ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock<'a>(&'a self) -> CriticalLockGuard<'a, T> {
        let (data, key) = unsafe { self.lock_no_guard(false) };

        CriticalLockGuard {
            queue: self,
            data,
            key,
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn lock_no_guard<'a, 'b>(
        &'a self,
        assert: bool,
    ) -> (&'b mut T, Option<CriticalKey>) {
        // Enter local critical
        let key = LocalState::try_get().map(|local_state| unsafe {
            if assert {
                local_state.critical_state().enter_assert()
            } else {
                local_state.critical_state().enter()
            }
        });

        // Get a ticket number
        let ticket = self.ticket.fetch_add(1, Ordering::AcqRel);

        // Lock globally
        while self.now_serving.load(Ordering::Acquire) != ticket {
            core::hint::spin_loop();
        }

        (&mut *self.data.get(), key)
    }

    #[inline(always)]
    pub(crate) unsafe fn unlock(&self, key: Option<CriticalKey>) {
        // Unlock globally
        self.now_serving.fetch_add(1, Ordering::Release);

        // Leave local critical
        key.map(|key| LocalState::get().critical_state().leave(key));
    }
}

unsafe impl<T: Sized> Sync for CriticalLock<T> {}
unsafe impl<T: Sized> Send for CriticalLock<T> {}

impl<T: BootVideo> LogOutput for CriticalLock<T> {
    fn write_str(&self, s: &str) {
        self.lock().write_str(s).ok();
    }

    fn write_fmt(&self, args: core::fmt::Arguments<'_>) {
        self.lock().write_fmt(args).ok();
    }
}

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
            self.queue.unlock(self.key.take());
        }
    }
}
