use crate::aml::{impl_core_display, next, pkg_length, Display, Result, Stream};
use alloc::vec::Vec;

mod package_element;

pub(super) use package_element::PackageElement;

pub(in crate::aml::term_objects) struct Package {
    num_elements: u8,
    elements: Vec<PackageElement>,
}

impl Package {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let num_elements = next!(stream);

        let mut elements = Vec::new();
        while stream.peek().is_some() {
            elements.push(PackageElement::parse(&mut stream)?);
        }

        Ok(Package {
            num_elements,
            elements,
        })
    }
}

impl Display for Package {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Package ({}) {{", self.num_elements)?;

        if self.elements.len() == 0 {
            return writeln!(f, "}}");
        }

        for i in 0..self.elements.len() {
            self.elements[i].display(f, depth + 1, last)?;

            if i < self.elements.len() - 1 {
                write!(f, ", ")?;
            }
        }

        self.display_prefix(f, depth)?;
        write!(f, " }}")
    }
}

impl_core_display!(Package);
