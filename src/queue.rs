use alloc::boxed::Box;

struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T,
}

pub struct Queue<T: PartialEq> {
    head: Option<Box<Node<T>>>,
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

    pub fn remove(&mut self, value: T) {
        let mut none: Option<Box<Node<T>>> = None;

        let mut current_node: *mut Option<Box<Node<T>>> = &mut self.head;
        let mut previous_node: *mut Option<Box<Node<T>>> = &mut none;
        while let Some(node) = unsafe { &mut *current_node } {
            if node.data == value {
                match unsafe { &mut *previous_node } {
                    Some(previous_node) => previous_node.next = node.next.take(),
                    None => self.head = node.next.take(),
                }
            }

            previous_node = current_node;
            current_node = &mut node.next;
        }
    }

    pub fn is_front(&self) -> bool {
        self.head.is_some()
    }

    pub fn _len(&self) -> usize {
        self.length
    }
}

unsafe impl<T: PartialEq> Send for Queue<T> {}