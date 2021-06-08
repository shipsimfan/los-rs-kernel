#![allow(dead_code)]

use crate::process::{self, Thread, ThreadQueue};
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicPtr, Ordering},
};

pub struct Spinlock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct Mutex<T: ?Sized> {
    lock: AtomicPtr<Thread>,
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

extern "C" {
    fn get_rflags() -> usize;
}

impl<T> Mutex<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        Mutex {
            lock: AtomicPtr::new(null_mut()),
            queue: ThreadQueue::new(),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn lock(&self) -> MutexGuard<T> {
        match process::get_current_thread_option() {
            None => {}
            Some(current_thread) => {
                let return_interrupts = unsafe { get_rflags() } & (1 << 9) == 0;
                unsafe { asm!("cli") };
                if self
                    .lock
                    .compare_exchange(
                        null_mut(),
                        current_thread.data.get(),
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    )
                    .is_err()
                {
                    unsafe {
                        (*(&self.queue as *const _ as *mut ThreadQueue)).push(current_thread);
                    }
                    process::yield_thread();
                }

                if return_interrupts {
                    unsafe { asm!("sti") };
                }
            }
        }

        MutexGuard {
            lock: &self,
            data: unsafe { &mut *self.data.get() },
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        match process::get_current_thread_option() {
            None => None,
            Some(current_thread) => {
                let return_interrupts = unsafe { get_rflags() } & (1 << 9) == 0;
                unsafe { asm!("cli") };
                let ret = if self
                    .lock
                    .compare_exchange(
                        null_mut(),
                        current_thread.data.get(),
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    Some(MutexGuard {
                        lock: &self,
                        data: unsafe { &mut *self.data.get() },
                    })
                } else {
                    None
                };

                if return_interrupts {
                    unsafe { asm!("sti") };
                }

                ret
            }
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        let return_interrupts = unsafe { get_rflags() } & (1 << 9) == 0;
        unsafe { asm!("cli") };
        let ret = self.lock.load(Ordering::Relaxed) != null_mut();
        if return_interrupts {
            unsafe { asm!("sti") };
        };

        ret
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
        let return_interrupts = unsafe { get_rflags() } & (1 << 9) == 0;
        unsafe { asm!("cli") };
        let mutex = unsafe { &mut *(self.lock as *const _ as *mut Mutex<T>) };

        match mutex.queue.pop() {
            None => mutex.lock.store(null_mut(), Ordering::Relaxed),
            Some(next_thread) => {
                mutex.lock.store(next_thread.data.get(), Ordering::Relaxed);
                process::queue_thread(next_thread);
            }
        }

        if return_interrupts {
            unsafe {
                asm!("sti");
            }
        }
    }
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

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
    #[allow(dead_code)]
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
