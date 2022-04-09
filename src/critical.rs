use core::{
    arch::asm,
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct CriticalLock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct CriticalLockGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

// Does not require lock or atomic due to being independent between cores
extern "C" {
    static mut LOCAL_CRITICAL_COUNT: usize;
}

#[inline(always)]
pub unsafe fn enter_local() {
    assert!(LOCAL_CRITICAL_COUNT <= 1000);

    asm!("cli");
    LOCAL_CRITICAL_COUNT += 1;
}

#[inline(always)]
pub unsafe fn leave_local() {
    LOCAL_CRITICAL_COUNT -= 1;
    if LOCAL_CRITICAL_COUNT == 0 {
        asm!("sti");
    }
}

pub unsafe fn leave_local_without_sti() {
    LOCAL_CRITICAL_COUNT -= 1;
}

impl<T> CriticalLock<T> {
    pub const fn new(data: T) -> Self {
        CriticalLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> CriticalLockGuard<T> {
        // Lock locally
        unsafe { enter_local() };

        // Lock globally
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
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

    pub fn data_eq(&self, other: *const T) -> bool {
        self.data.get() == other as *mut _
    }
}

impl<T: ?Sized> PartialEq for CriticalLock<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.get() == other.data.get()
    }
}

impl<'a, T> Deref for CriticalLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T> DerefMut for CriticalLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T: ?Sized> Drop for CriticalLockGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            // Drop local globally first
            self.lock.store(false, Ordering::Release);

            // Then drop local critical
            leave_local();
        }
    }
}

unsafe impl<T: ?Sized + Send> Sync for CriticalLock<T> {}
unsafe impl<T: ?Sized + Send> Send for CriticalLock<T> {}
