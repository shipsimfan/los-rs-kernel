use super::{term_list, Integer, Result};
use crate::{
    namespace::{Namespace, Node},
    parser::{NameString, Stream, TermList},
};
use alloc::{rc::Rc, vec::Vec};
use base::{log_info, Logger};
use core::cell::RefCell;

mod search;

pub(crate) struct Interpreter<'a, 'b> {
    namespace: &'b Namespace<'a>,

    current_node: Rc<RefCell<Node<'a>>>,
    node_context: Vec<Rc<RefCell<Node<'a>>>>,

    logger: Logger,

    wide_integers: bool,
    executing_method: bool,
}

impl<'a, 'b> Interpreter<'a, 'b> {
    pub(crate) fn new(
        namespace: &'b Namespace<'a>,
        logger: Logger,
        wide_integers: bool,
        executing_method: bool,
    ) -> Self {
        Interpreter {
            namespace,

            current_node: namespace.root().clone(),
            node_context: Vec::new(),

            logger,

            wide_integers,
            executing_method,
        }
    }

    pub(super) fn logger(&self) -> &Logger {
        &self.logger
    }

    pub(super) fn display_namespace(&self) {
        log_info!(self.logger, "\n{}", self.namespace)
    }

    pub(super) fn wide_integers(&self) -> bool {
        self.wide_integers
    }

    pub(super) fn executing_method(&self) -> bool {
        self.executing_method
    }

    pub(super) fn get_node(
        &self,
        name: &NameString,
        include_final: bool,
    ) -> Option<Rc<RefCell<Node<'a>>>> {
        search::get_node(
            &self.current_node,
            self.namespace.root(),
            name,
            include_final,
        )
    }

    pub(super) fn create_integer(&self, value: u64) -> Integer {
        if self.wide_integers {
            Integer::ACPI2(value)
        } else {
            Integer::ACPI1(value as u32)
        }
    }

    pub(crate) fn load_definition_block(&mut self, definition_block: &'a [u8]) -> Result<()> {
        term_list::execute(self, &mut TermList::parse(Stream::from(definition_block))).map(|_| ())
    }

    pub(super) fn push_current_node(&mut self, mut new_node: Rc<RefCell<Node<'a>>>) {
        core::mem::swap(&mut new_node, &mut self.current_node);
        self.node_context.push(new_node);
    }

    pub(super) fn pop_current_node(&mut self) {
        self.current_node = self.node_context.pop().unwrap();
    }
}
