use super::{term_list, Result};
use crate::{
    namespace::Namespace,
    parser::{Stream, TermList},
};
use base::Logger;

pub(crate) struct Interpreter<'namespace> {
    namespace: &'namespace Namespace,
    logger: Logger,
}

impl<'namespace, 'name> Interpreter<'namespace> {
    pub(crate) fn new(namespace: &'namespace Namespace, logger: Logger) -> Self {
        Interpreter { namespace, logger }
    }

    pub(super) fn logger(&self) -> &Logger {
        &self.logger
    }

    pub(crate) fn load_definition_block(&mut self, definition_block: &[u8]) -> Result<()> {
        term_list::execute(self, &mut TermList::parse(Stream::from(definition_block)))
    }
}
