use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct DataRegion {
    offset: usize,
    name: NameString,
    term1: TermArg,
    term2: TermArg,
    term3: TermArg,
}

impl DataRegion {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let name = NameString::parse(stream)?;
        let term1 = TermArg::parse(stream)?;
        let term2 = TermArg::parse(stream)?;
        let term3 = TermArg::parse(stream)?;

        Ok(DataRegion {
            offset,
            name,
            term1,
            term2,
            term3,
        })
    }
}

impl Display for DataRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Data Region {} @ {}", self.name, self.offset)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Argument 1:")?;
        self.term1.display(f, depth + 2)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Argument 2:")?;
        self.term2.display(f, depth + 2)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Argument 3:")?;
        self.term3.display(f, depth + 2)
    }
}

impl_core_display!(DataRegion);
