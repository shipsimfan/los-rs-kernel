use crate::aml::{impl_core_display, next, term_objects::TermArg, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Fatal {
    r#type: u8,
    code: u32,
    arg: TermArg,
}

impl Fatal {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let r#type = next!(stream);
        let code = u32::from_le_bytes([next!(stream), next!(stream), next!(stream), next!(stream)]);
        let arg = TermArg::parse(stream)?;

        Ok(Fatal { r#type, code, arg })
    }
}

impl Display for Fatal {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Fatal ({}, {}, {})", self.r#type, self.code, self.arg)
    }
}

impl_core_display!(Fatal);
