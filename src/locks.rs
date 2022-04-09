use crate::process::{self, ThreadQueue};
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Spinlock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct Mutex<T: ?Sized> {
    lock: AtomicBool,
    queue: ThreadQueue,
    data: UnsafeCell<T>,
}

pub struct SpinlockGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
    data: &'a mut T,
}

impl<T> Mutex<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        Mutex {
            lock: AtomicBool::new(false),
            queue: ThreadQueue::new(),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> MutexGuard<T> {
        unsafe { crate::critical::enter_local() };
        match process::get_current_thread_option() {
            None => unsafe { crate::critical::leave_local() },
            Some(_) => {
                if self
                    .lock
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_err()
                {
                    unsafe { crate::critical::leave_local() };
                    process::yield_thread(Some(self.queue.into_current_queue()));
                } else {
                    unsafe { crate::critical::leave_local() };
                }
            }
        }

        MutexGuard {
            lock: &self,
            data: unsafe { &mut *self.data.get() },
        }
    }

    #[inline(always)]
    pub fn _try_lock(&self) -> Option<MutexGuard<T>> {
        match process::get_current_thread_option() {
            None => None,
            Some(_) => {
                if self
                    .lock
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    Some(MutexGuard {
                        lock: &self,
                        data: unsafe { &mut *self.data.get() },
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn matching_data(&self, other: *const T) -> bool {
        self.data.get() as *const _ == other
    }

    pub unsafe fn as_ptr(&self) -> *mut T {
        self.data.get()
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        unsafe {
            crate::critical::enter_local();
            let mutex = &mut *(self.lock as *const _ as *mut Mutex<T>);

            match mutex.queue.pop() {
                None => mutex.lock.store(false, Ordering::Relaxed),
                Some(next_thread) => {
                    mutex.lock.store(true, Ordering::Relaxed);
                    process::queue_thread(next_thread);
                }
            }
            crate::critical::leave_local();
        }
    }
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

#[allow(dead_code)]
impl<T> Spinlock<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> SpinlockGuard<T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.is_locked() {
                core::hint::spin_loop();
            }
        }

        SpinlockGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }

    #[inline(always)]
    pub fn try_lock(&self) -> Option<SpinlockGuard<T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinlockGuard {
                lock: &self.lock,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    pub fn matching_data(&self, other: *const T) -> bool {
        self.data.get() == other as *mut _
    }
}

impl<T: ?Sized> PartialEq for Spinlock<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.get() == other.data.get()
    }
}

impl<'a, T: ?Sized> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: ?Sized> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T: ?Sized> Drop for SpinlockGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

unsafe impl<T: ?Sized + Send> Sync for Spinlock<T> {}
unsafe impl<T: ?Sized + Send> Send for Spinlock<T> {}
