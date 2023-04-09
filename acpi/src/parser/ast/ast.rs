use super::TermList;
use crate::parser::{Context, Result, Stream};
use base::log_info;

pub(crate) struct AST {
    term_list: TermList,
}

impl AST {
    pub(in crate::parser) fn parse(definition_block: &[u8], mut context: Context) -> Result<Self> {
        log_info!(context.logger(), "Parsing AML");

        let mut stream = Stream::new(definition_block, 0);

        let term_list = TermList::parse(&mut stream, &mut context)?;

        Ok(AST { term_list })
    }
}

impl core::fmt::Display for AST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.term_list.fmt(f)
    }
}
