use crate::Increment;
use alloc::{sync::Arc, vec::Vec};

pub trait Mappable<ID: Increment + Eq> {
    fn id(&self) -> &ID;
}

pub trait MappableMut<ID: Increment + Eq>: Mappable<ID> {
    fn set_id(&mut self, id: &ID);
}

pub struct Map<ID: Increment + Eq, T: Mappable<ID>> {
    inner: Vec<T>,
    next_id: ID,
}

impl<ID: Increment + Eq, T: Mappable<ID>> Map<ID, T> {
    pub const fn new(first_id: ID) -> Self {
        Map {
            inner: Vec::new(),
            next_id: first_id,
        }
    }

    pub fn with_capacity(capacity: usize, first_id: ID) -> Self {
        Map {
            inner: Vec::with_capacity(capacity),
            next_id: first_id,
        }
    }

    pub fn get(&self, id: ID) -> Option<&T> {
        for value in &self.inner {
            if *value.id() == id {
                return Some(value);
            }
        }

        None
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        self.inner.iter()
    }

    pub fn get_mut(&mut self, id: &ID) -> Option<&mut T> {
        for value in &mut self.inner {
            if value.id() == id {
                return Some(value);
            }
        }

        None
    }

    pub fn insert_f<F>(&mut self, f: F)
    where
        F: FnOnce(&ID) -> T,
    {
        self.inner.push(f(&self.next_id));
        self.next_id.increment();
    }

    pub fn remove(&mut self, id: &ID) -> Option<T> {
        for i in 0..self.inner.len() {
            if self.inner[i].id() == id {
                return Some(self.inner.swap_remove(i));
            }
        }

        None
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&T) -> bool,
    {
        self.inner.retain(f)
    }

    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut T) -> bool,
    {
        self.inner.retain_mut(f)
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.inner.iter_mut()
    }
}

impl<ID: Increment + Eq, T: MappableMut<ID>> Map<ID, T> {
    pub fn insert(&mut self, mut value: T) {
        value.set_id(&self.next_id);
        self.inner.push(value);
        self.next_id.increment();
    }
}

impl<ID: Increment + Eq, T: Mappable<ID>> IntoIterator for Map<ID, T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, ID: Increment + Eq, T: Mappable<ID>> IntoIterator for &'a Map<ID, T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, ID: Increment + Eq, T: Mappable<ID>> IntoIterator for &'a mut Map<ID, T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<ID: Increment + Eq, T: Mappable<ID>> Mappable<ID> for Arc<T> {
    fn id(&self) -> &ID {
        self.as_ref().id()
    }
}

impl<ID: Increment + Eq, T> Mappable<ID> for (ID, T) {
    fn id(&self) -> &ID {
        &self.0
    }
}
