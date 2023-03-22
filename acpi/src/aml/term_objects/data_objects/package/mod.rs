use crate::aml::{impl_core_display, next, pkg_length, Display, Result, Stream};
use alloc::vec::Vec;

mod package_element;

pub(super) use package_element::PackageElement;

pub(in crate::aml::term_objects) struct Package {
    offset: usize,

    num_elements: u8,
    elements: Vec<PackageElement>,
}

impl Package {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let num_elements = next!(stream);

        let mut elements = Vec::new();
        while stream.peek().is_some() {
            elements.push(PackageElement::parse(&mut stream)?);
        }

        Ok(Package {
            offset,
            num_elements,
            elements,
        })
    }
}

impl Display for Package {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Package ({}) @ {}:", self.num_elements, self.offset)?;

        for element in &self.elements {
            element.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl_core_display!(Package);
