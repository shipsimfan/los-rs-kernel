use crate::multi_owner::Lock;

use super::{enter_local, leave_local};
use core::{
    alloc::GlobalAlloc,
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct CriticalLock<T: Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct CriticalLockGuard<'a, T: Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

impl<T: Sized> CriticalLock<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        CriticalLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> CriticalLockGuard<T> {
        // Enter local critical
        unsafe { enter_local() };

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
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
}

impl<T: Sized> Lock for CriticalLock<T> {
    type Data = T;

    #[inline(always)]
    fn new(data: Self::Data) -> Self {
        Self::new(data)
    }

    #[inline(always)]
    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let mut guard = Self::lock(&self);
        f(&mut *guard)
    }
}

unsafe impl<T: Sized> Sync for CriticalLock<T> {}
unsafe impl<T: Sized> Send for CriticalLock<T> {}

impl<'a, T: Sized> Deref for CriticalLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T: Sized> DerefMut for CriticalLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T: Sized> Drop for CriticalLockGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            // Unlock globally
            self.lock.store(false, Ordering::SeqCst);

            // Leave local critical
            leave_local();
        }
    }
}

// Needed for locking the heap
unsafe impl<T: GlobalAlloc> GlobalAlloc for CriticalLock<T> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.lock().dealloc(ptr, layout)
    }
}
