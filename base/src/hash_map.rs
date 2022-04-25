use alloc::{boxed::Box, string::String, vec::Vec};
use core::ptr::NonNull;

pub trait Hash: PartialEq + Eq {
    fn hash(&self) -> u8;
}

pub struct HashMap<K: Hash, V> {
    data: Box<[Vec<(K, V)>]>,
    length: usize,
}

pub struct Iter<'a, K: Hash, V> {
    hash_map: &'a HashMap<K, V>,
    index: usize,
    iter: core::slice::Iter<'a, (K, V)>,
}

pub struct IterMut<'a, K: Hash, V> {
    hash_map: NonNull<HashMap<K, V>>,
    index: usize,
    iter: core::slice::IterMut<'a, (K, V)>,
}

const HASH_SIZE: usize = u8::MAX as usize + 1;

impl<K: Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(HASH_SIZE);
        for _ in 0..HASH_SIZE {
            data.push(Vec::new());
        }

        HashMap {
            data: data.into_boxed_slice(),
            length: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let ret = self.remove(&key);

        // Insert new value
        self.data[key.hash() as usize].push((key, value));
        self.length += 1;

        ret
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        // Get the vector
        let vec = &mut self.data[key.hash() as usize];

        // Remove previous instance
        for i in 0..vec.len() {
            if &vec[i].0 == key {
                self.length -= 1;
                return Some(vec.remove(i).1);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        // Get the vector
        let vec = &mut self.data[key.hash() as usize];

        // Locate the item
        for (k, v) in vec {
            if k == key {
                return Some(v);
            }
        }

        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        // Get the vector
        let vec = &self.data[key.hash() as usize];

        // Locate the item
        for (k, v) in vec {
            if k == key {
                return Some(v);
            }
        }

        None
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            hash_map: self,
            index: 0,
            iter: self.data[0].iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            hash_map: NonNull::new(self).unwrap(),
            index: 0,
            iter: self.data[0].iter_mut(),
        }
    }
}

impl<'a, K: Hash, V> Iterator for Iter<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(value) => return Some(&value.1),
                None => {}
            }

            if self.index < HASH_SIZE - 1 {
                self.index += 1;
                self.iter = self.hash_map.data[self.index].iter()
            } else {
                return None;
            }
        }
    }
}

impl<'a, K: Hash, V> Iterator for IterMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(value) => return Some(&mut value.1),
                None => {}
            }

            if self.index < HASH_SIZE - 1 {
                self.index += 1;
                self.iter = unsafe { self.hash_map.as_mut() }.data[self.index].iter_mut();
            } else {
                return None;
            }
        }
    }
}

impl Hash for usize {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for isize {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for u128 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for i128 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for u64 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for i64 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for u32 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for i32 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for u16 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for i16 {
    fn hash(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

impl Hash for u8 {
    fn hash(&self) -> u8 {
        *self
    }
}

impl Hash for i8 {
    fn hash(&self) -> u8 {
        *self as u8
    }
}

impl Hash for String {
    fn hash(&self) -> u8 {
        const P: usize = 251;
        let mut hash_value = 0;
        let mut p_pow = 1;
        for c in self.bytes() {
            hash_value = (hash_value + c as usize * p_pow) & 0xFF;
            p_pow = (p_pow * P) & 0xFF;
        }

        hash_value as u8
    }
}
