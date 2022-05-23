use base::map::{Mappable, INVALID_ID};
use core::ops::Deref;

pub struct Descriptor<T>(isize, T);

impl<T> Descriptor<T> {
    pub fn new(inner: T) -> Self {
        Descriptor(INVALID_ID, inner)
    }
}

impl<T> Mappable for Descriptor<T> {
    fn set_id(&mut self, id: isize) {
        self.0 = id
    }

    fn id(&self) -> isize {
        self.0
    }
}

impl<T> Deref for Descriptor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
