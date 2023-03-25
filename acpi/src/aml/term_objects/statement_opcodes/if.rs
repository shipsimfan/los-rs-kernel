use crate::aml::{
    impl_core_display, pkg_length,
    term_objects::{TermArg, TermList},
    Display, Result, Stream,
};

use super::Else;

pub(in crate::aml::term_objects) struct If {
    predicate: TermArg,
    term_list: TermList,
    r#else: Option<Else>,
}

const ELSE_OP: u8 = 0xA1;

impl If {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let predicate = TermArg::parse(&mut stream)?;
        let term_list = TermList::parse(&mut stream)?;

        todo!("Implement parsing else");
    }
}

impl Display for If {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "If ({}) ", self.predicate)?;

        self.term_list
            .display(f, depth, last && self.r#else.is_none())?;

        match self.r#else.as_ref() {
            Some(r#else) => r#else.display(f, depth, last),
            None => Ok(()),
        }
    }
}

impl_core_display!(If);
