use crate::{display_prefix, impl_core_display, Display};

pub(crate) enum Term {}

impl Display for Term {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);

        todo!()
    }
}

impl_core_display!(Term);
