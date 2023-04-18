use crate::parser::{ast::Argument, next, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct Match<'a> {
    search_package: Box<Argument<'a>>,
    opcode1: u8,
    operand1: Box<Argument<'a>>,
    opcode2: u8,
    operand2: Box<Argument<'a>>,
    start_index: Box<Argument<'a>>,
}

impl<'a> Match<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let search_package = Box::new(Argument::parse(stream, context)?);
        let opcode1 = next!(stream, "Match");
        let operand1 = Box::new(Argument::parse(stream, context)?);
        let opcode2 = next!(stream, "Match");
        let operand2 = Box::new(Argument::parse(stream, context)?);
        let start_index = Box::new(Argument::parse(stream, context)?);

        Ok(Match {
            search_package,
            opcode1,
            operand1,
            opcode2,
            operand2,
            start_index,
        })
    }
}

impl<'a> core::fmt::Display for Match<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Match ({}, {}, {}, {}, {}, {})",
            self.search_package,
            self.opcode1,
            self.operand1,
            self.opcode2,
            self.operand2,
            self.start_index
        )
    }
}
