use crate::aml::{
    impl_core_display, target::Target, term_objects::TermArg, Display, Result, Stream,
};
use alloc::boxed::Box;

pub(in crate::aml::term_objects) struct ToBuffer {
    operand: Box<TermArg>,
    target: Target,
}

impl ToBuffer {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let operand = Box::new(TermArg::parse(stream)?);
        let target = Target::parse(stream)?;

        Ok(ToBuffer { operand, target })
    }
}

impl Display for ToBuffer {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "ToBuffer ({}, {})", self.operand, self.target)
    }
}

impl_core_display!(ToBuffer);
