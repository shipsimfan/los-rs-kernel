use base::multi_owner::Lock;

use crate::{
    current_thread_option, queue_thread, thread_queue::ThreadQueue, yield_thread, ProcessOwner,
    Signals,
};
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    lock: AtomicBool,
    queue: ThreadQueue<O, D, S>,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T: 'a, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>
{
    lock: &'a Mutex<T, O, D, S>,
    data: &'a mut T,
}

impl<T, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Mutex<T, O, D, S> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        Mutex {
            lock: AtomicBool::new(false),
            queue: ThreadQueue::new(),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> MutexGuard<T, O, D, S> {
        unsafe { base::critical::enter_local() };
        match current_thread_option::<O, D, S>() {
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
    pub fn try_lock(&self) -> Option<MutexGuard<T, O, D, S>> {
        match current_thread_option::<O, D, S>() {
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

impl<T: Send, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Lock
    for Mutex<T, O, D, S>
{
    type Data = T;

    fn new(data: Self::Data) -> Self {
        Self::new(data)
    }

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let mut guard = Self::lock(&self);
        f(&mut *guard)
    }
}

unsafe impl<T: Send, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Sync
    for Mutex<T, O, D, S>
{
}
unsafe impl<T: Send, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Send
    for Mutex<T, O, D, S>
{
}

impl<'a, T, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Deref
    for MutexGuard<'a, T, O, D, S>
{
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> DerefMut
    for MutexGuard<'a, T, O, D, S>
{
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T, O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Drop
    for MutexGuard<'a, T, O, D, S>
{
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        unsafe {
            base::critical::enter_local();
            let mutex = &mut *(self.lock as *const _ as *mut Mutex<T, O, D, S>);

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
