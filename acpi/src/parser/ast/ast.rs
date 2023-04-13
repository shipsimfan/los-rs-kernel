use super::TermList;
use crate::parser::{Context, Result, Stream};
use base::log_info;

pub(crate) struct AST<'a> {
    term_list: TermList<'a>,
}

impl<'a> AST<'a> {
    pub(in crate::parser) fn parse(
        definition_block: &'a [u8],
        mut context: Context,
    ) -> Result<Self> {
        log_info!(context.logger(), "Parsing AML");

        let mut stream = Stream::new(definition_block, 0);

        let term_list = TermList::parse(&mut stream, &mut context)?;

        Ok(AST { term_list })
    }
}

impl<'a> core::fmt::Display for AST<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.term_list.fmt(f)
    }
}
