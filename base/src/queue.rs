use crate::pinned_box::PinnedBox;
use core::ptr::NonNull;

struct Node<T> {
    next: Option<PinnedBox<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    data: T,
}

struct SortedNode<K: PartialOrd, T> {
    next: Option<PinnedBox<SortedNode<K, T>>>,
    prev: Option<NonNull<SortedNode<K, T>>>,
    data: T,
    key: K,
}

pub struct Queue<T> {
    head: Option<PinnedBox<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}

pub struct SortedQueue<K: PartialOrd, T> {
    head: Option<PinnedBox<SortedNode<K, T>>>,
    tail: Option<NonNull<SortedNode<K, T>>>,
    length: usize,
}

impl<T> Queue<T> {
    pub const fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let mut new_node = PinnedBox::new(Node {
            next: None,
            prev: self.tail.clone(),
            data,
        });

        let new_tail = NonNull::new(&mut *new_node);

        match &mut self.tail {
            Some(tail) => unsafe { tail.as_mut() }.next = Some(new_node),
            None => self.head = Some(new_node),
        }

        self.tail = new_tail;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut head) => {
                match head.next.take() {
                    Some(mut new_head) => {
                        new_head.prev = None;
                        self.head = Some(new_head);
                    }
                    None => self.tail = None,
                }

                self.length -= 1;
                Some(head.into_inner().data)
            }
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T: PartialEq> Queue<T> {
    pub fn remove(&mut self, value: T) -> Option<T> {
        let mut current_node = match &mut self.head {
            Some(head) => head,
            None => return None,
        };

        loop {
            if current_node.data == value {
                let data = match current_node.prev {
                    Some(mut prev) => {
                        current_node
                            .next
                            .as_mut()
                            .map(|node| node.prev = Some(prev));

                        let prev = unsafe { prev.as_mut() };
                        let mut node = prev.next.take().unwrap();
                        prev.next = node.next.take();

                        node.into_inner().data
                    }
                    None => {
                        current_node.next.as_mut().map(|node| node.prev = None);

                        let mut node = self.head.take().unwrap();
                        self.head = node.next.take();
                        node.into_inner().data
                    }
                };
                self.length -= 1;
                return Some(data);
            }

            current_node = match &mut current_node.next {
                Some(next) => next,
                None => return None,
            }
        }
    }
}

impl<K: PartialOrd, T> SortedQueue<K, T> {
    pub const fn new() -> Self {
        SortedQueue {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, data: T, key: K) {
        let mut new_node = PinnedBox::new(SortedNode {
            next: None,
            prev: None,
            data,
            key,
        });

        let mut current_node = match &mut self.tail {
            Some(tail) => tail.clone(),
            None => {
                self.tail = NonNull::new(&mut *new_node);
                self.head = Some(new_node);

                return;
            }
        };

        loop {
            let c_node = unsafe { current_node.as_mut() };

            if c_node.key <= new_node.key {
                new_node.next = c_node.next.take();
                let ptr = NonNull::new(&mut *new_node).unwrap();
                match &mut new_node.next {
                    Some(next) => next.prev = Some(ptr),
                    None => {
                        self.tail = Some(ptr);
                    }
                }

                new_node.prev = Some(current_node);
                c_node.next = Some(new_node);

                self.length += 1;

                return;
            }

            current_node = match &mut c_node.prev {
                Some(prev) => prev.clone(),
                None => break,
            }
        }

        match self.head.take() {
            Some(mut head) => {
                head.prev = Some(NonNull::new(&mut *new_node).unwrap());
                new_node.next = Some(head);
            }
            None => {}
        }

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut head) => {
                match head.next.take() {
                    Some(mut new_head) => {
                        new_head.prev = None;
                        self.head = Some(new_head);
                    }
                    None => self.tail = None,
                }

                self.length -= 1;
                Some(head.into_inner().data)
            }
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<K: PartialOrd, T: PartialEq> SortedQueue<K, T> {
    pub fn remove(&mut self, value: T) -> Option<T> {
        let mut current_node = match &mut self.head {
            Some(head) => head,
            None => return None,
        };

        loop {
            if current_node.data == value {
                let data = match current_node.prev {
                    Some(mut prev) => {
                        current_node
                            .next
                            .as_mut()
                            .map(|node| node.prev = Some(prev));

                        let prev = unsafe { prev.as_mut() };
                        let mut node = prev.next.take().unwrap();
                        prev.next = node.next.take();

                        node.into_inner().data
                    }
                    None => {
                        current_node.next.as_mut().map(|node| node.prev = None);

                        let mut node = self.head.take().unwrap();
                        self.head = node.next.take();
                        node.into_inner().data
                    }
                };
                self.length -= 1;
                return Some(data);
            }

            current_node = match &mut current_node.next {
                Some(next) => next,
                None => return None,
            }
        }
    }
}

impl<T> Unpin for Node<T> {}

impl<K: PartialOrd, T> Unpin for SortedNode<K, T> {}
