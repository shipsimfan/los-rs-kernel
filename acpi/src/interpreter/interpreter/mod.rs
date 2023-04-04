use super::{term_list, Integer, Result};
use crate::{
    namespace::{Namespace, Node},
    parser::{NameString, Stream, TermList},
};
use alloc::{rc::Rc, vec::Vec};
use base::{log_info, Logger};
use core::cell::RefCell;

mod search;

pub(crate) struct Interpreter<'namespace> {
    namespace: &'namespace Namespace,

    current_node: Rc<RefCell<dyn Node>>,
    node_context: Vec<Rc<RefCell<dyn Node>>>,

    logger: Logger,

    wide_integers: bool,
}

impl<'namespace, 'name> Interpreter<'namespace> {
    pub(crate) fn new(
        namespace: &'namespace Namespace,
        logger: Logger,
        wide_integers: bool,
    ) -> Self {
        Interpreter {
            namespace,

            current_node: namespace.root().clone(),
            node_context: Vec::new(),

            logger,

            wide_integers,
        }
    }

    pub(super) fn logger(&self) -> &Logger {
        &self.logger
    }

    pub(super) fn display_namespace(&self) {
        log_info!(self.logger, "\n{}", self.namespace)
    }

    pub(super) fn get_node(
        &self,
        name: &NameString,
        include_final: bool,
    ) -> Option<Rc<RefCell<dyn Node>>> {
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
