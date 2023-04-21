use crate::{CriticalKey, LocalState};
use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct CriticalRefCell<T: ?Sized> {
    borrow: Cell<isize>,
    value: UnsafeCell<T>,
}

pub struct CriticalRef<'a, T: ?Sized + 'a> {
    key: Option<CriticalKey>,
    value: NonNull<T>,
    borrow: &'a Cell<isize>,
}

pub struct CriticalRefMut<'a, T: ?Sized + 'a> {
    key: Option<CriticalKey>,
    value: NonNull<T>,
    borrow: &'a Cell<isize>,
    phantom: PhantomData<&'a mut T>,
}

const UNUSED: isize = 0;

impl<T> CriticalRefCell<T> {
    pub const fn new(value: T) -> Self {
        CriticalRefCell {
            borrow: Cell::new(UNUSED),
            value: UnsafeCell::new(value),
        }
    }
}

impl<T: ?Sized> CriticalRefCell<T> {
    pub fn borrow(&self) -> CriticalRef<T> {
        self.try_borrow().expect("already mutably borrowed")
    }

    pub fn try_borrow(&self) -> Option<CriticalRef<T>> {
        let key = unsafe { LocalState::get().critical_state().enter() };
        let b = self.borrow.get().wrapping_add(1);
        if b > UNUSED {
            self.borrow.set(b);
            let value = unsafe { NonNull::new_unchecked(self.value.get()) };
            Some(CriticalRef {
                value,
                borrow: &self.borrow,
                key: Some(key),
            })
        } else {
            unsafe { LocalState::get().critical_state().leave(key) };
            None
        }
    }

    pub fn borrow_mut(&self) -> CriticalRefMut<T> {
        self.try_borrow_mut().expect("already borrowed")
    }

    pub fn try_borrow_mut(&self) -> Option<CriticalRefMut<T>> {
        let key = unsafe { LocalState::get().critical_state().enter() };
        if self.borrow.get() == UNUSED {
            self.borrow.set(UNUSED - 1);
            let value = unsafe { NonNull::new_unchecked(self.value.get()) };
            Some(CriticalRefMut {
                key: Some(key),
                value,
                borrow: &self.borrow,
                phantom: PhantomData,
            })
        } else {
            unsafe { LocalState::get().critical_state().leave(key) };
            None
        }
    }
}

impl<T: ?Sized> Deref for CriticalRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized + core::fmt::Display> core::fmt::Display for CriticalRef<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> Drop for CriticalRef<'_, T> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(borrow > UNUSED);
        self.borrow.set(borrow - 1);

        unsafe {
            LocalState::get()
                .critical_state()
                .leave(self.key.take().unwrap())
        };
    }
}

impl<T: ?Sized> Deref for CriticalRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for CriticalRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<T: ?Sized + core::fmt::Display> core::fmt::Display for CriticalRefMut<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> Drop for CriticalRefMut<'_, T> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        debug_assert!(borrow < UNUSED);
        self.borrow.set(borrow + 1);

        unsafe {
            LocalState::get()
                .critical_state()
                .leave(self.key.take().unwrap())
        }
    }
}
