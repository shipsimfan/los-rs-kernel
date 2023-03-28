use super::{term_list, Result};
use crate::{
    namespace::{Namespace, Node},
    parser::{NameString, Prefix, Stream, TermList},
};
use alloc::{rc::Rc, vec::Vec};
use base::Logger;
use core::cell::RefCell;

pub(crate) struct Interpreter<'namespace> {
    namespace: &'namespace Namespace,

    current_node: Rc<RefCell<dyn Node>>,
    node_context: Vec<Rc<RefCell<dyn Node>>>,

    logger: Logger,
}

fn search_node(node: &Rc<RefCell<dyn Node>>, name: [u8; 4]) -> Option<Rc<RefCell<dyn Node>>> {
    let node = node.borrow();
    let children = match node.as_children() {
        Some(children) => children,
        None => return None,
    };

    let mut result = None;
    for child in children.children() {
        if let Some(child_name) = child.borrow().name() {
            if child_name == name {
                result = Some(child.clone());
                break;
            }
        }
    }

    result
}

fn perform_search(
    start_node: &Rc<RefCell<dyn Node>>,
    name: &NameString,
) -> Option<Rc<RefCell<dyn Node>>> {
    let mut node = start_node.clone();

    for part in name.path() {
        node = match search_node(&node, *part) {
            Some(node) => node,
            None => return None,
        };
    }

    match name.name() {
        Some(name) => search_node(&node, name),
        None => Some(node),
    }
}

impl<'namespace, 'name> Interpreter<'namespace> {
    pub(crate) fn new(namespace: &'namespace Namespace, logger: Logger) -> Self {
        Interpreter {
            namespace,

            current_node: namespace.root().clone(),
            node_context: Vec::new(),

            logger,
        }
    }

    pub(super) fn logger(&self) -> &Logger {
        &self.logger
    }

    pub(super) fn get_node(&self, name: &NameString) -> Option<Rc<RefCell<dyn Node>>> {
        match name.prefix() {
            Prefix::None => {
                let mut node = self.current_node.clone();
                loop {
                    let parent = match perform_search(&node, name) {
                        Some(node) => return Some(node),
                        None => match node.borrow().parent() {
                            Some(parent) => parent,
                            None => return None,
                        },
                    };

                    node = parent;
                }
            }
            Prefix::Root => perform_search(self.namespace.root(), name),
            Prefix::Super(count) => {
                let mut node = self.current_node.clone();
                for _ in 0..count {
                    let parent = match node.borrow().parent() {
                        Some(node) => node,
                        None => return None,
                    };
                    node = parent;
                }
                perform_search(&node, name)
            }
        }
    }

    pub(crate) fn load_definition_block(&mut self, definition_block: &[u8]) -> Result<()> {
        term_list::execute(self, &mut TermList::parse(Stream::from(definition_block)))
    }

    pub(super) fn push_current_node(&mut self, mut new_node: Rc<RefCell<dyn Node>>) {
        core::mem::swap(&mut new_node, &mut self.current_node);
        self.node_context.push(new_node);
    }

    pub(super) fn pop_current_node(&mut self) {
        self.current_node = self.node_context.pop().unwrap();
    }
}
