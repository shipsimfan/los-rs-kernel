use crate::Device;
use alloc::{boxed::Box, string::String, vec::Vec};
use base::multi_owner::{Owner, Reference};
use process::{Mutex, ProcessOwner, Signals};

pub struct Node<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    name: String,
    device: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, O, D, S>>,
    children: Children<O, D, S>,
}

pub struct Children<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    children: Vec<Node<O, D, S>>,
}

impl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Node<O, D, S> {
    pub fn new(name: String, device: Box<dyn Device>) -> Self {
        Node {
            name,
            device: Owner::new(device),
            children: Children::new(),
        }
    }

    pub fn children(&self) -> Box<[String]> {
        self.children.children()
    }

    pub fn device(&self) -> Reference<Box<dyn Device>, Mutex<Box<dyn Device>, O, D, S>> {
        self.device.as_ref()
    }

    pub fn child(&self, path: &[&str]) -> Option<&Node<O, D, S>> {
        self.children.child(path)
    }

    pub fn child_mut(&mut self, path: &[&str]) -> Option<&mut Node<O, D, S>> {
        self.children.child_mut(path)
    }

    pub fn insert(&mut self, name: String, device: Box<dyn Device>) -> bool {
        self.children.insert(name, device)
    }

    pub fn remove_device(&mut self, path: &[&str]) {
        self.children.remove_device(path)
    }
}

impl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Children<O, D, S> {
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

    pub fn child(&self, path: &[&str]) -> Option<&Node<O, D, S>> {
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

    pub fn child_mut(&mut self, path: &[&str]) -> Option<&mut Node<O, D, S>> {
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

    pub fn insert(&mut self, name: String, device: Box<dyn Device>) -> bool {
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
