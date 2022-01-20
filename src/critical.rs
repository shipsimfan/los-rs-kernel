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
    local_critical_state: bool,
}

// Does not require lock or atomic due to being independent between cores
extern "C" {
    static mut LOCAL_CRITICAL_STATE: bool;
}

#[inline(always)]
pub unsafe fn enter_local() -> bool {
    asm!("cli");
    let old_state = LOCAL_CRITICAL_STATE;
    LOCAL_CRITICAL_STATE = true;
    old_state
}

#[inline(always)]
pub unsafe fn leave_local(old_state: bool) {
    LOCAL_CRITICAL_STATE = old_state;
    if !old_state {
        asm!("sti");
    }
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
        let local_critical_state = unsafe { enter_local() };

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
            local_critical_state,
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
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
            leave_local(self.local_critical_state);
        }
    }
}

unsafe impl<T: ?Sized + Send> Sync for CriticalLock<T> {}
unsafe impl<T: ?Sized + Send> Send for CriticalLock<T> {}
