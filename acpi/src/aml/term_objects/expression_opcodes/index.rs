use crate::aml::{
    impl_core_display, target::Target, term_objects::TermArg, Display, Result, Stream,
};
use alloc::boxed::Box;

pub(in crate::aml) struct Index {
    buff_pkg_str_obj: TermArg,
    index_value: TermArg,
    target: Box<Target>,
}

impl Index {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let buff_pkg_str_obj = TermArg::parse(stream)?;
        let index_value = TermArg::parse(stream)?;
        let target = Box::new(Target::parse(stream)?);

        Ok(Index {
            buff_pkg_str_obj,
            index_value,
            target,
        })
    }
}

impl Display for Index {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Index ({}, {}, {})",
            self.buff_pkg_str_obj, self.index_value, self.target
        )
    }
}

impl_core_display!(Index);
