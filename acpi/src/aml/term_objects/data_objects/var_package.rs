use super::package::PackageElement;
use crate::aml::{impl_core_display, pkg_length, term_objects::TermArg, Display, Result, Stream};
use alloc::{boxed::Box, vec::Vec};

pub(in crate::aml::term_objects) struct VarPackage {
    offset: usize,

    num_elements: Box<TermArg>,
    elements: Vec<PackageElement>,
}

impl VarPackage {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let num_elements = Box::new(TermArg::parse(&mut stream)?);

        let mut elements = Vec::new();
        while stream.peek().is_some() {
            elements.push(PackageElement::parse(&mut stream)?);
        }

        Ok(VarPackage {
            offset,
            num_elements,
            elements,
        })
    }
}

impl Display for VarPackage {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "VarPackage @ {}:", self.offset)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Num Elements:")?;
        self.num_elements.display(f, depth)?;

        for element in &self.elements {
            element.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl_core_display!(VarPackage);
