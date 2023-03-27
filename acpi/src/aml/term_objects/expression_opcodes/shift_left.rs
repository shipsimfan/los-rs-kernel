use crate::aml::{
    impl_core_display, target::Target, term_objects::TermArg, Display, Result, Stream,
};
use alloc::boxed::Box;

pub(in crate::aml::term_objects) struct ShiftLeft {
    operand: Box<TermArg>,
    shift_count: Box<TermArg>,
    target: Target,
}

impl ShiftLeft {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let operand = Box::new(TermArg::parse(stream)?);
        let shift_count = Box::new(TermArg::parse(stream)?);
        let target = Target::parse(stream)?;

        Ok(ShiftLeft {
            operand,
            shift_count,
            target,
        })
    }
}

impl Display for ShiftLeft {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "ShiftLeft ({}, {}, {})",
            self.operand, self.shift_count, self.target
        )
    }
}

impl_core_display!(ShiftLeft);
