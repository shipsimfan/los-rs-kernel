use crate::aml::{
    impl_core_display, next, pkg_length, term_objects::TermList, Display, NameString, Result,
    Stream,
};

pub(in crate::aml::term_objects) struct Processor {
    name: NameString,
    proc_id: u8,
    pblk_addr: u32,
    pblk_len: u8,
    term_list: TermList,
}

impl Processor {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let proc_id = next!(stream);
        let pblk_addr =
            u32::from_le_bytes([next!(stream), next!(stream), next!(stream), next!(stream)]);
        let pblk_len = next!(stream);
        let term_list = TermList::parse(&mut stream)?;

        Ok(Processor {
            name,
            proc_id,
            pblk_addr,
            pblk_len,
            term_list,
        })
    }
}

impl Display for Processor {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Processor ({}, {:#02X}, {:#08X}, {:#02X}) ",
            self.name, self.proc_id, self.pblk_addr, self.pblk_len
        )?;

        self.term_list.display(f, depth, last)
    }
}

impl_core_display!(Processor);
