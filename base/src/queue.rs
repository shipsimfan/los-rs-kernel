use alloc::rc::Rc;
use core::cell::RefCell;

struct Node<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
    data: Option<T>,
}

struct SortedNode<K: PartialOrd, T> {
    next: Option<Rc<RefCell<SortedNode<K, T>>>>,
    prev: Option<Rc<RefCell<SortedNode<K, T>>>>,
    data: Option<T>,
    key: K,
}

pub struct Queue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

pub struct SortedQueue<K: PartialOrd, T> {
    head: Option<Rc<RefCell<SortedNode<K, T>>>>,
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
        let new_node = Rc::new(RefCell::new(Node {
            next: None,
            prev: self.tail.clone(),
            data: Some(data),
        }));

        match &self.tail {
            Some(tail) => tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                let mut head = head.borrow_mut();

                match head.next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                    }
                    None => self.tail = None,
                }

                self.length -= 1;
                Some(head.data.take().unwrap())
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
        let mut current_node_opt = match self.head.clone() {
            Some(head) => head,
            None => return None,
        };

        loop {
            let mut current_node = current_node_opt.borrow_mut();

            if current_node.data.as_ref().unwrap() == &value {
                let data = match current_node.prev.take() {
                    Some(prev) => {
                        current_node
                            .next
                            .as_ref()
                            .map(|node| node.borrow_mut().prev = Some(prev.clone()));

                        prev.borrow_mut().next = current_node.next.take();

                        current_node.data.take().unwrap()
                    }
                    None => {
                        current_node
                            .next
                            .as_ref()
                            .map(|node| node.borrow_mut().prev = None);

                        self.head = current_node.next.take();
                        current_node.data.take().unwrap()
                    }
                };

                self.length -= 1;
                return Some(data);
            }

            current_node_opt = match current_node.next.clone() {
                Some(next) => {
                    drop(current_node);
                    next
                }
                None => return None,
            }
        }
    }
}

impl<K: PartialOrd, T> SortedQueue<K, T> {
    pub const fn new() -> Self {
        SortedQueue {
            head: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, data: T, key: K) {
        self.length += 1;

        // Create new node
        let mut new_node = SortedNode {
            next: None,
            prev: None,
            key,
            data: Some(data),
        };

        // Check to see if we have a head
        let mut current_node_opt = match self.head.clone() {
            Some(head) => Some(head),
            None => return self.head = Some(Rc::new(RefCell::new(new_node))),
        };

        // Find where to insert the node
        let mut previous_node = None;
        loop {
            let current_node = match &current_node_opt {
                Some(current_node) => current_node.borrow(),
                None => break,
            };

            if current_node.key > new_node.key {
                break;
            }

            let temp = current_node.next.clone();

            drop(current_node);

            previous_node = current_node_opt.take();
            current_node_opt = temp;
        }

        // Insert the node after previous_node and before current_node
        new_node.next = current_node_opt
            .as_ref()
            .map(|current_node| current_node.clone());
        new_node.prev = previous_node
            .as_ref()
            .map(|previous_node| previous_node.clone());
        let new_node = Rc::new(RefCell::new(new_node));

        match previous_node {
            Some(previous_node) => previous_node.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        match &current_node_opt {
            Some(current_node) => current_node.borrow_mut().prev = Some(new_node),
            None => {}
        }
    }

    pub fn pop_le(&mut self, value: K) -> Option<T> {
        match self.head.as_ref() {
            Some(head) => {
                if head.borrow().key <= value {
                    self.pop()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let ret_node = self.head.take();

        match ret_node {
            Some(node) => {
                self.length -= 1;
                self.head = node.borrow_mut().next.take();
                self.head
                    .as_ref()
                    .map(|next_node| next_node.borrow_mut().prev = None);
                Some(node.borrow_mut().data.take().unwrap())
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
        // Locate the value
        let mut current_node_opt = self.head.clone();

        loop {
            let current_node = match &current_node_opt {
                Some(current_node) => current_node,
                None => return None,
            };

            let current_node = current_node.borrow();

            if current_node.data.as_ref().unwrap() == &value {
                break;
            }

            let next_node = current_node.next.clone();

            drop(current_node);

            current_node_opt = next_node;
        }

        if current_node_opt.is_none() {
            return None;
        }

        let current_node_ref = current_node_opt.unwrap();

        // Remove it
        let mut current_node = current_node_ref.borrow_mut();
        let previous_node = current_node.prev.take();
        let next_node = current_node.next.take();

        match &previous_node {
            Some(previous_node) => previous_node.borrow_mut().next = next_node.clone(),
            None => self.head = next_node.clone(),
        }

        match &next_node {
            Some(next_node) => next_node.borrow_mut().prev = previous_node,
            None => {}
        }

        let ret = current_node.data.take().unwrap();
        drop(current_node);

        self.length -= 1;

        Some(ret)
    }
}
