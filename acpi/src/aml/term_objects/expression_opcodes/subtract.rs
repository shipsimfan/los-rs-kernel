use crate::aml::{
    impl_core_display, target::Target, term_objects::TermArg, Display, Result, Stream,
};
use alloc::boxed::Box;

pub(in crate::aml::term_objects) struct Subtract {
    operand1: Box<TermArg>,
    operand2: Box<TermArg>,
    target: Target,
}

impl Subtract {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let operand1 = Box::new(TermArg::parse(stream)?);
        let operand2 = Box::new(TermArg::parse(stream)?);
        let target = Target::parse(stream)?;

        Ok(Subtract {
            operand1,
            operand2,
            target,
        })
    }
}

impl Display for Subtract {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Subtract ({}, {}, {})",
            self.operand1, self.operand2, self.target
        )
    }
}

impl_core_display!(Subtract);
