use crate::Device;
use alloc::{boxed::Box, string::String, vec::Vec};
use base::multi_owner::{Owner, Reference};
use process::{Mutex, ProcessTypes};

pub struct Node<T: ProcessTypes + 'static> {
    name: String,
    device: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    children: Children<T>,
}

pub struct Children<T: ProcessTypes + 'static> {
    children: Vec<Node<T>>,
}

impl<T: ProcessTypes> Node<T> {
    pub fn new(name: String, device: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>) -> Self {
        Node {
            name,
            device,
            children: Children::new(),
        }
    }

    pub fn children(&self) -> Box<[String]> {
        self.children.children()
    }

    pub fn device(&self) -> Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>> {
        self.device.as_ref()
    }

    pub fn child(&self, path: &[&str]) -> Option<&Node<T>> {
        self.children.child(path)
    }

    pub fn child_mut(&mut self, path: &[&str]) -> Option<&mut Node<T>> {
        self.children.child_mut(path)
    }

    pub fn insert(
        &mut self,
        name: String,
        device: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    ) -> bool {
        self.children.insert(name, device)
    }

    pub fn remove_device(&mut self, path: &[&str]) {
        self.children.remove_device(path)
    }
}

impl<T: ProcessTypes> Children<T> {
    pub const fn new() -> Self {
        Children {
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> Box<[String]> {
        let mut ret = Vec::with_capacity(self.children.len());
        for child in &self.children {
            ret.push(child.name.clone());
        }

        ret.into_boxed_slice()
    }

    pub fn child(&self, path: &[&str]) -> Option<&Node<T>> {
        if path.len() == 0 {
            return None;
        }

        let part = path[0];

        let mut device_found = None;
        for device in &self.children {
            if device.name == part {
                device_found = Some(device);
                break;
            }
        }

        match device_found {
            Some(device) => match path.len() {
                1 => Some(device),
                _ => device.child(&path[1..]),
            },
            None => None,
        }
    }

    pub fn child_mut(&mut self, path: &[&str]) -> Option<&mut Node<T>> {
        if path.len() == 0 {
            return None;
        }

        let part = path[0];

        let mut device_found = None;
        for device in &mut self.children {
            if device.name == part {
                device_found = Some(device);
                break;
            }
        }

        match device_found {
            Some(device) => match path.len() {
                1 => Some(device),
                _ => device.child_mut(&path[1..]),
            },
            None => None,
        }
    }

    pub fn insert(
        &mut self,
        name: String,
        device: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    ) -> bool {
        for child in &self.children {
            if child.name == name {
                return false;
            }
        }

        self.children.push(Node::new(name, device));

        true
    }

    pub fn remove_device(&mut self, path: &[&str]) {
        if path.len() > 1 {
            for child in &mut self.children {
                if child.name == path[0] {
                    child.remove_device(&path[1..]);
                    return;
                }
            }
        } else if path.len() == 1 {
            self.children.retain(|child| child.name != path[0]);
        }
    }
}
