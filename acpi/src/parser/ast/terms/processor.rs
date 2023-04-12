use super::TermList;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{name_string, next, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct Processor<'a> {
    path: Path,
    id: u8,
    address: u32,
    length: u8,
    term_list: TermList<'a>,
}

impl<'a> Processor<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Processor")?;

        let path = name_string::parse(&mut stream, "Processor")?;
        let id = next!(stream, "Processor");
        let address = u32::from_le_bytes([
            next!(stream, "Processor"),
            next!(stream, "Processor"),
            next!(stream, "Processor"),
            next!(stream, "Processor"),
        ]);
        let length = next!(stream, "Processor");

        context.push_path(&path);
        let term_list = TermList::parse(&mut stream, context)?;
        context.pop_path();

        Ok(Processor {
            path,
            id,
            address,
            length,
            term_list,
        })
    }

    pub(super) fn parse_methods(&mut self, context: &mut Context) -> Result<()> {
        self.term_list.parse_methods(context)
    }
}

impl<'a> Display for Processor<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(
            f,
            "Processor ({}, {:#04X}, {:#010X}, {:#04X}) ",
            self.path, self.id, self.address, self.length
        )?;
        self.term_list.display(f, depth, last)
    }
}

impl_core_display_lifetime!(Processor);
