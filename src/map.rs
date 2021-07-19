use alloc::vec::Vec;

use crate::logln;

pub trait Mappable {
    fn id(&self) -> isize;
    fn set_id(&mut self, id: isize);
}

pub struct Map<T: Mappable> {
    data: [Vec<T>; HASH_SIZE as usize],
    count: usize,
    next_id: isize,
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
            if key != value.id() {
                true
            } else {
                decrement += 1;
                false
            }
        });

        self.count -= decrement;
    }

    pub fn _get(&self, key: isize) -> Option<&T> {
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

    pub fn count(&self) -> usize {
        self.count
    }
}
