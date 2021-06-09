use alloc::boxed::Box;

struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Queue<T> {
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

    pub fn len(&self) -> usize {
        self.length
    }
}
