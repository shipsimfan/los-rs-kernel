use crate::parser::{next, pkg_length, Result, Stream};
use alloc::vec::Vec;

mod element;

pub(crate) use element::PackageElement;

pub(crate) struct Package<'a> {
    elements: Vec<PackageElement<'a>>,
}

impl<'a> Package<'a> {
    pub(in crate::parser) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let num_elements = next!(stream) as usize;
        let mut elements = Vec::with_capacity(num_elements);
        for _ in 0..num_elements {
            elements.push(PackageElement::parse(&mut stream)?);
        }

        Ok(Package { elements })
    }

    pub(crate) fn elements(&self) -> &[PackageElement] {
        &self.elements
    }
}

impl<'a> core::fmt::Display for Package<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Package  {{")?;

        for i in 0..self.elements.len() {
            self.elements[i].fmt(f)?;

            if i < self.elements.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, " }}")
    }
}
