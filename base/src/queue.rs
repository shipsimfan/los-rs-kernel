use alloc::collections::{BTreeMap, VecDeque};

pub struct Queue<T> {
    inner: VecDeque<T>,
}

pub struct SortedQueue<K: Ord, T> {
    inner: BTreeMap<K, T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            inner: VecDeque::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        self.inner.push_back(data);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T: PartialEq> Queue<T> {
    pub fn remove(&mut self, value: T) -> Option<T> {
        let mut index = None;
        for i in 0..self.inner.len() {
            if self.inner[i] == value {
                index = Some(i);
                break;
            }
        }

        let index = match index {
            Some(index) => index,
            None => return None,
        };

        self.inner.remove(index)
    }
}

impl<K: Ord, T> SortedQueue<K, T> {
    pub fn new() -> Self {
        SortedQueue {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, value: T, key: K) {
        self.inner.insert(key, value);
    }

    pub fn pop_le(&mut self, key: K) -> Option<T> {
        match self.inner.first_key_value() {
            Some((inner_key, _)) => {
                if inner_key > &key {
                    return None;
                }
            }
            None => return None,
        }

        self.inner.pop_first().map(|(_, value)| value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_first().map(|(_, value)| value)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K: Ord + Clone, T: PartialEq> SortedQueue<K, T> {
    pub fn remove(&mut self, value: T) -> Option<T> {
        let mut key = None;
        for (inner_key, inner_value) in &self.inner {
            if *inner_value == value {
                key = Some(inner_key.clone());
                break;
            }
        }

        let key = match key {
            Some(key) => key,
            None => return None,
        };

        self.inner.remove(&key)
    }
}
