use alloc::boxed::Box;

struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T,
}

struct SortedNode<I, T> {
    next: Option<Box<SortedNode<I, T>>>,
    data: T,
    value: I,
}

pub struct Queue<T: PartialEq> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

pub struct SortedQueue<I: PartialOrd, T: PartialEq> {
    head: Option<Box<SortedNode<I, T>>>,
    length: usize,
}

impl<T: PartialEq> Queue<T> {
    pub const fn new() -> Self {
        Queue::<T> {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, new_item: T) {
        let new_node = Box::new(Node::<T> {
            next: None,
            data: new_item,
        });

        let mut current_node = &mut self.head;
        while match current_node {
            None => false,
            Some(node) => {
                current_node = &mut node.next;
                true
            }
        } {}

        *current_node = Some(new_node);

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let head = self.head.take().unwrap();
            self.head = head.next;
            self.length -= 1;
            Some(head.data)
        }
    }

    pub unsafe fn remove(&mut self, value: T) -> Option<T> {
        let mut none: Option<Box<Node<T>>> = None;

        let mut current_node: *mut Option<Box<Node<T>>> = &mut self.head;
        let mut previous_node: *mut Option<Box<Node<T>>> = &mut none;
        while let Some(node) = &mut *current_node {
            if node.data == value {
                let node = match &mut *previous_node {
                    Some(previous_node) => {
                        let mut node = previous_node.next.take().unwrap();
                        previous_node.next = node.next.take();
                        node
                    }
                    None => {
                        let mut node = self.head.take().unwrap();
                        self.head = node.next.take();
                        node
                    }
                };

                self.length -= 1;

                return Some(node.data);
            }

            previous_node = current_node;
            current_node = &mut node.next;
        }

        None
    }

    pub fn is_front(&self) -> bool {
        self.head.is_some()
    }

    pub fn _len(&self) -> usize {
        self.length
    }
}

unsafe impl<T: PartialEq> Send for Queue<T> {}

impl<I: PartialOrd, T: PartialEq> SortedQueue<I, T> {
    pub const fn new() -> Self {
        SortedQueue {
            head: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, new_item: T, value: I) {
        let mut new_node = Box::new(SortedNode {
            next: None,
            data: new_item,
            value,
        });

        let mut none: Option<Box<SortedNode<I, T>>> = None;

        let mut current_node: *mut Option<Box<SortedNode<I, T>>> = &mut self.head;
        let mut previous_node: *mut Option<Box<SortedNode<I, T>>> = &mut none;

        unsafe {
            while let Some(node) = &mut *current_node {
                if node.value > new_node.value {
                    match &mut *previous_node {
                        Some(previous_node) => {
                            new_node.next = previous_node.next.take();
                            previous_node.next = Some(new_node)
                        }
                        None => {
                            new_node.next = self.head.take();
                            self.head = Some(new_node)
                        }
                    }

                    self.length += 1;

                    return;
                }

                previous_node = current_node;
                current_node = &mut node.next;
            }

            match &mut *previous_node {
                Some(previous_node) => previous_node.next = Some(new_node),
                None => self.head = Some(new_node),
            }
        }

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let head = self.head.take().unwrap();
            self.head = head.next;
            self.length -= 1;
            Some(head.data)
        }
    }

    pub fn pop_le(&mut self, value: I) -> Option<T> {
        if self.head.is_none() {
            None
        } else if self.head.as_ref().unwrap().value <= value {
            let head = self.head.take().unwrap();
            self.head = head.next;
            self.length -= 1;
            Some(head.data)
        } else {
            None
        }
    }

    pub unsafe fn remove(&mut self, value: T) -> Option<T> {
        let mut none: Option<Box<SortedNode<I, T>>> = None;

        let mut current_node: *mut Option<Box<SortedNode<I, T>>> = &mut self.head;
        let mut previous_node: *mut Option<Box<SortedNode<I, T>>> = &mut none;
        while let Some(node) = &mut *current_node {
            if node.data == value {
                match &mut *previous_node {
                    Some(previous_node) => previous_node.next = node.next.take(),
                    None => self.head = node.next.take(),
                }

                self.length -= 1;

                return Some((*current_node).take().unwrap().data);
            }

            previous_node = current_node;
            current_node = &mut node.next;
        }

        None
    }
}

unsafe impl<I: PartialOrd + Copy, T: PartialEq> Send for SortedQueue<I, T> {}
