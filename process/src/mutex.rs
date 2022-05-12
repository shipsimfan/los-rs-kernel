use crate::{
    current_thread_option, queue_thread, thread_queue::ThreadQueue, yield_thread, ProcessTypes,
};
use base::multi_owner::Lock;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T, PT: ProcessTypes + 'static> {
    lock: AtomicBool,
    queue: ThreadQueue<PT>,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: 'a, PT: ProcessTypes + 'static> {
    lock: &'a Mutex<T, PT>,
    data: &'a mut T,
}

impl<T, PT: ProcessTypes + 'static> Mutex<T, PT> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        Mutex {
            lock: AtomicBool::new(false),
            queue: ThreadQueue::new(),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> MutexGuard<T, PT> {
        unsafe { base::critical::enter_local_assert() };
        match current_thread_option::<PT>() {
            None => unsafe { base::critical::leave_local() },
            Some(_) => {
                if self
                    .lock
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_err()
                {
                    unsafe { base::critical::leave_local() };
                    yield_thread(Some(self.queue.current_queue()));
                } else {
                    unsafe { base::critical::leave_local() };
                }
            }
        }

        MutexGuard {
            lock: &self,
            data: unsafe { &mut *self.data.get() },
        }
    }

    #[inline(always)]
    pub fn try_lock(&self) -> Option<MutexGuard<T, PT>> {
        match current_thread_option::<PT>() {
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

impl<T: Send, PT: ProcessTypes + 'static> Lock for Mutex<T, PT> {
    type Data = T;

    fn new(data: Self::Data) -> Self {
        Self::new(data)
    }

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let mut guard = Self::lock(&self);
        f(&mut *guard)
    }
}

unsafe impl<T: Send, PT: ProcessTypes> Sync for Mutex<T, PT> {}
unsafe impl<T: Send, PT: ProcessTypes> Send for Mutex<T, PT> {}

impl<'a, T, PT: ProcessTypes> Deref for MutexGuard<'a, T, PT> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T, PT: ProcessTypes> DerefMut for MutexGuard<'a, T, PT> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T, PT: ProcessTypes> Drop for MutexGuard<'a, T, PT> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        unsafe {
            base::critical::enter_local();
            let mutex = &mut *(self.lock as *const _ as *mut Mutex<T, PT>);

            match mutex.queue.pop() {
                None => mutex.lock.store(false, Ordering::Relaxed),
                Some(next_thread) => {
                    mutex.lock.store(true, Ordering::Relaxed);
                    queue_thread(next_thread);
                }
            }
            base::critical::leave_local();
        }
    }
}
