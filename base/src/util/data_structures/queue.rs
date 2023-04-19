use alloc::collections::VecDeque;

pub struct Queue<T> {
    inner: VecDeque<T>,
}

impl<T> Queue<T> {
    pub const fn new() -> Self {
        Queue {
            inner: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            inner: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push_back(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn pop_if<F>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce(&T) -> bool,
    {
        match self.inner.front() {
            Some(front) => match f(front) {
                true => self.inner.pop_front(),
                false => None,
            },
            None => None,
        }
    }

    pub fn iter(&self) -> alloc::collections::vec_deque::Iter<T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> alloc::collections::vec_deque::IterMut<T> {
        self.inner.iter_mut()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = alloc::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Queue<T> {
    type Item = &'a mut T;
    type IntoIter = alloc::collections::vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
