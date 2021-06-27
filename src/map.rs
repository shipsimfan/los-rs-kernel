use alloc::vec::Vec;

pub trait Mappable {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

pub struct Map<T: Mappable> {
    data: [Vec<T>; HASH_SIZE],
    count: usize,
    next_id: usize,
}

const HASH_SIZE: usize = 32;

pub const INVALID_ID: usize = usize::MAX;

impl<T: Mappable> Map<T> {
    pub fn new() -> Self {
        Map {
            data: Default::default(),
            count: 0,
            next_id: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        let id = self.next_id;
        let mut value = value;
        value.set_id(id);

        self.next_id += 1;
        self.count += 1;

        let hash = id % HASH_SIZE;
        self.data[hash].push(value);
        id
    }

    pub fn remove(&mut self, key: usize) {
        assert_ne!(key, INVALID_ID, "Invalid key while removing from map");

        let hash = key % HASH_SIZE;
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

    pub fn get(&self, key: usize) -> Option<&T> {
        assert_ne!(key, INVALID_ID, "Invalid key while getting from map");

        let hash = key % HASH_SIZE;
        for value in &self.data[hash] {
            if value.id() == key {
                return Some(value);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        assert_ne!(key, INVALID_ID, "Invalid key while getting from map");

        let hash = key % HASH_SIZE;
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