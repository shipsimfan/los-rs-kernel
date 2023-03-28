use super::{term_list, Result};
use crate::{
    namespace::{Namespace, Node},
    parser::{NameString, Prefix, Stream, TermList},
};
use alloc::{rc::Rc, vec::Vec};
use base::{log_info, Logger};
use core::cell::RefCell;

pub(crate) struct Interpreter<'namespace> {
    namespace: &'namespace Namespace,

    current_node: Rc<RefCell<dyn Node>>,
    node_context: Vec<Rc<RefCell<dyn Node>>>,

    logger: Logger,
}
fn perform_search(
    start_node: &Rc<RefCell<dyn Node>>,
    name: &NameString,
    include_final: bool,
) -> Option<Rc<RefCell<dyn Node>>> {
    let mut node = start_node.clone();

    for part in name.path() {
        let current_node_ref = node.borrow();
        let current_node = match current_node_ref.as_children() {
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
            match node.as_children() {
                Some(children) => children.get_child(name),
                None => None,
            }
        }
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

    pub(super) fn display_namespace(&self) {
        log_info!(self.logger, "{}", self.namespace)
    }

    pub(super) fn get_node(
        &self,
        name: &NameString,
        include_final: bool,
    ) -> Option<Rc<RefCell<dyn Node>>> {
        match name.prefix() {
            Prefix::None => {
                let mut node = self.current_node.clone();
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
            Prefix::Root => perform_search(self.namespace.root(), name, include_final),
            Prefix::Super(count) => {
                let mut node = self.current_node.clone();
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
