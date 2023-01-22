use crate::{CriticalKey, CriticalState};
use core::{
    alloc::GlobalAlloc,
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub trait LocalState: 'static {
    fn try_critical_state<'a>() -> Option<&'a CriticalState>;
}

pub struct CriticalLock<T: Sized, L: LocalState> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
    phantom: PhantomData<L>,
}

pub struct CriticalLockGuard<'a, T: Sized + 'a, L: LocalState> {
    lock: &'a AtomicBool,
    data: &'a mut T,
    key: Option<CriticalKey>,
    phantom: PhantomData<L>,
}

impl<T: Sized, L: LocalState> CriticalLock<T, L> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        CriticalLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> CriticalLockGuard<T, L> {
        // Enter local critical
        let key = L::try_critical_state().map(|critical_state| unsafe { critical_state.enter() });

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
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
}

unsafe impl<T: Sized, L: LocalState> Sync for CriticalLock<T, L> {}
unsafe impl<T: Sized, L: LocalState> Send for CriticalLock<T, L> {}

impl<'a, T: Sized + 'a, L: LocalState> Deref for CriticalLockGuard<'a, T, L> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T: Sized + 'a, L: LocalState> DerefMut for CriticalLockGuard<'a, T, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T: Sized, L: LocalState> Drop for CriticalLockGuard<'a, T, L> {
    fn drop(&mut self) {
        unsafe {
            // Unlock globally
            self.lock.store(false, Ordering::SeqCst);

            // Leave local critical
            self.key
                .take()
                .map(|key| L::try_critical_state().unwrap().leave(key));
        }
    }
}

// Needed for locking the heap
unsafe impl<T: GlobalAlloc, L: LocalState> GlobalAlloc for CriticalLock<T, L> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.lock().dealloc(ptr, layout)
    }
}
