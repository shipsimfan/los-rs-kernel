use crate::logln;
use alloc::vec::Vec;
use core::{
    ops::{Deref, DerefMut},
    slice::Iter,
};

pub trait Mappable {
    fn id(&self) -> isize;
    fn set_id(&mut self, id: isize);
}

pub struct Map<T: Mappable> {
    data: [Vec<T>; HASH_SIZE as usize],
    count: usize,
    next_id: isize,
}

pub struct MappedItem<T> {
    data: T,
    id: isize,
}

pub struct MapIterator<'a, T: Mappable> {
    map: &'a Map<T>,
    iter: Iter<'a, T>,
    index: usize,
}

const HASH_SIZE: isize = 32;

pub const INVALID_ID: isize = isize::MIN;

impl<T: Mappable> Map<T> {
    pub const fn new() -> Self {
        Map::with_starting_index(0)
    }

    pub const fn with_starting_index(starting_index: isize) -> Self {
        Map {
            data: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            count: 0,
            next_id: starting_index,
        }
    }

    pub fn insert(&mut self, value: T) -> isize {
        let id = self.next_id;
        let mut value = value;
        value.set_id(id);

        self.next_id += 1;
        self.count += 1;

        let hash = (id % HASH_SIZE) as usize;
        self.data[hash].push(value);
        id
    }

    pub fn remove(&mut self, key: isize) {
        if key == INVALID_ID {
            logln!("Invalid key while removing from map!");
            return;
        }

        let hash = (key % HASH_SIZE) as usize;
        let mut decrement = 0;
        self.data[hash].retain(|value| -> bool {
            let id = value.id();
            if key != id && id != INVALID_ID {
                true
            } else {
                decrement += 1;
                false
            }
        });

        self.count -= decrement;
    }

    pub fn remove_all_except(&mut self, key: isize) {
        let mut new_count = 0;
        for hash in 0..HASH_SIZE as usize {
            self.data[hash].retain(|val| -> bool { val.id() == key });
            new_count += self.data[hash].len();
        }

        self.count = new_count;
    }

    pub fn get_mut(&mut self, key: isize) -> Option<&mut T> {
        if key == INVALID_ID {
            logln!("Invalid key while getting from map!");
            return None;
        }

        let hash = (key % HASH_SIZE) as usize;
        for value in &mut self.data[hash] {
            if value.id() == key {
                return Some(value);
            }
        }

        None
    }

    pub fn get(&self, key: isize) -> Option<&T> {
        if key == INVALID_ID {
            logln!("Invalid key while getting from map!");
            return None;
        }

        let hash = (key % HASH_SIZE) as usize;
        for value in &self.data[hash] {
            if value.id() == key {
                return Some(value);
            }
        }

        None
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn iter(&self) -> MapIterator<T> {
        MapIterator {
            map: self,
            iter: self.data[0].iter(),
            index: 0,
        }
    }

    pub fn ids(&self) -> Vec<isize> {
        let mut ids = Vec::with_capacity(self.count);
        for item in self {
            ids.push(item.id());
        }

        ids
    }
}

impl<T> MappedItem<T> {
    pub fn new(data: T) -> Self {
        MappedItem {
            data,
            id: INVALID_ID,
        }
    }
}

impl<T> Deref for MappedItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for MappedItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Mappable for MappedItem<T> {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl<'a, T: Mappable> Iterator for MapIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < HASH_SIZE as usize {
            match self.iter.next() {
                Some(value) => return Some(value),
                None => {
                    self.index += 1;
                    if self.index < HASH_SIZE as usize {
                        self.iter = self.map.data[self.index].iter()
                    }
                }
            }
        }

        None
    }
}

impl<'a, T: Mappable> IntoIterator for &'a Map<T> {
    type IntoIter = MapIterator<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator {
            map: self,
            iter: self.data[0].iter(),
            index: 0,
        }
    }
}
