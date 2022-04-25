use alloc::boxed::Box;
use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

pub struct PinnedBox<T>(Pin<Box<T>>);

impl<T> PinnedBox<T> {
    pub fn new(data: T) -> Self {
        PinnedBox(Box::pin(data))
    }

    pub fn as_mut(&mut self) -> Pin<&mut T> {
        self.0.as_mut()
    }
}

impl<T: core::marker::Unpin> PinnedBox<T> {
    pub fn into_inner(self) -> Box<T> {
        Pin::into_inner(self.0)
    }
}

impl<T> Deref for PinnedBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T: core::marker::Unpin> DerefMut for PinnedBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut().get_mut()
    }
}
