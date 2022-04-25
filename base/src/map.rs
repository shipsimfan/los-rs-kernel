use alloc::{boxed::Box, vec::Vec};
use core::ptr::NonNull;

pub trait Mappable {
    fn id(&self) -> isize;
    fn set_id(&mut self, id: isize);
}

pub struct Map<T: Mappable> {
    data: Box<[Vec<T>]>,
    length: usize,
    next_id: isize,
}

pub struct Iter<'a, T: Mappable> {
    map: &'a Map<T>,
    index: usize,
    iter: core::slice::Iter<'a, T>,
}

pub struct IterMut<'a, T: Mappable> {
    map: NonNull<Map<T>>,
    index: usize,
    iter: core::slice::IterMut<'a, T>,
}

const MAP_SIZE: usize = 256;

impl<T: Mappable> Map<T> {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(MAP_SIZE);
        for _ in 0..MAP_SIZE {
            data.push(Vec::new());
        }

        Map {
            data: data.into_boxed_slice(),
            length: 0,
            next_id: 0,
        }
    }

    pub fn insert(&mut self, mut value: T) -> isize {
        let id = self.next_id;
        self.next_id += 1;

        value.set_id(id);

        self.data[(id as usize) & 0xFF].push(value);
        self.length += 1;

        id
    }

    pub fn remove(&mut self, id: isize) -> Option<T> {
        // Get the vector
        let vec = &mut self.data[(id as usize) & 0xFF];

        // Remove previous instance
        for i in 0..vec.len() {
            if vec[i].id() == id {
                self.length -= 1;
                return Some(vec.remove(i));
            }
        }

        None
    }

    pub fn get_mut(&mut self, id: isize) -> Option<&mut T> {
        // Get the vector
        let vec = &mut self.data[(id as usize) & 0xFF];

        // Locate the item
        for v in vec {
            if v.id() == id {
                return Some(v);
            }
        }

        None
    }

    pub fn get(&self, id: isize) -> Option<&T> {
        // Get the vector
        let vec = &self.data[(id as usize) & 0xFF];

        // Locate the item
        for v in vec {
            if v.id() == id {
                return Some(v);
            }
        }

        None
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            map: self,
            index: 0,
            iter: self.data[0].iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            map: NonNull::new(self).unwrap(),
            index: 0,
            iter: self.data[0].iter_mut(),
        }
    }
}

impl<'a, T: Mappable> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(value) => return Some(&value),
                None => {}
            }

            if self.index < MAP_SIZE - 1 {
                self.index += 1;
                self.iter = self.map.data[self.index].iter()
            } else {
                return None;
            }
        }
    }
}

impl<'a, T: Mappable> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(value) => return Some(value),
                None => {}
            }

            if self.index < MAP_SIZE - 1 {
                self.index += 1;
                self.iter = unsafe { self.map.as_mut() }.data[self.index].iter_mut();
            } else {
                return None;
            }
        }
    }
}
