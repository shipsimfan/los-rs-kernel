use super::Result;
use crate::{
    namespace::Namespace,
    parser::{Stream, TermList},
};

pub(crate) struct Interpreter<'name> {
    namespace: &'name mut Namespace,
}

impl<'namespace> Interpreter<'namespace> {
    pub(crate) fn new(namespace: &'namespace mut Namespace) -> Self {
        Interpreter { namespace }
    }

    pub(crate) fn load_definition_block(&mut self, definition_block: &[u8]) -> Result<()> {
        for term in TermList::parse(Stream::from(definition_block)) {
            let term = term?;

            panic!("{}", term)
        }

        Ok(())
    }
}
