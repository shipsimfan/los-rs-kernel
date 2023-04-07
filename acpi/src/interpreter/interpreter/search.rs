use crate::{
    namespace::Node,
    parser::{NameString, Prefix},
};
use alloc::rc::Rc;
use core::cell::RefCell;

pub(super) fn get_node<'a>(
    start_node: &Rc<RefCell<Node<'a>>>,
    namespace_root: &Rc<RefCell<Node<'a>>>,
    name: &NameString,
    include_final: bool,
) -> Option<Rc<RefCell<Node<'a>>>> {
    match name.prefix() {
        Prefix::None => {
            let mut node = start_node.clone();
            loop {
                let parent = match perform_search(&node, name, include_final) {
                    Some(node) => return Some(node),
                    None => match node.borrow().parent() {
                        Some(parent) => parent,
                        None => return None,
                    },
                };

                node = parent;
            }
        }
        Prefix::Root => perform_search(namespace_root, name, include_final),
        Prefix::Super(count) => {
            let mut node = start_node.clone();
            for _ in 0..count {
                let parent = match node.borrow().parent() {
                    Some(node) => node,
                    None => return None,
                };
                node = parent;
            }
            perform_search(&node, name, include_final)
        }
    }
}

fn perform_search<'a>(
    start_node: &Rc<RefCell<Node<'a>>>,
    name: &NameString,
    include_final: bool,
) -> Option<Rc<RefCell<Node<'a>>>> {
    let mut node = start_node.clone();

    for part in name.path() {
        let current_node_ref = node.borrow();
        let current_node = match current_node_ref.children() {
            Some(children) => children,
            None => return None,
        };

        match current_node.get_child(*part) {
            Some(new_node) => {
                drop(current_node_ref);
                node = new_node
            }
            None => return None,
        };
    }

    if !include_final {
        return Some(node);
    }

    match name.name() {
        Some(name) => {
            let node = node.borrow();
            match node.children() {
                Some(children) => children.get_child(name),
                None => None,
            }
        }
        None => Some(node),
    }
}
