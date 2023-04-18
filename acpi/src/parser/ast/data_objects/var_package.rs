use super::package::PackageElement;
use crate::parser::{ast::Argument, pkg_length, Context, Result, Stream};
use alloc::{boxed::Box, vec::Vec};

pub(crate) struct VarPackage<'a> {
    num_elements: Box<Argument<'a>>,
    package_element_list: Vec<PackageElement<'a>>,
}

impl<'a> VarPackage<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Variable Package")?;

        let num_elements = Box::new(Argument::parse(&mut stream, context)?);

        let mut package_element_list = Vec::new();
        while stream.remaining() > 0 {
            package_element_list.push(PackageElement::parse(&mut stream, context)?);
        }

        Ok(VarPackage {
            num_elements,
            package_element_list,
        })
    }
}

impl<'a> core::fmt::Display for VarPackage<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VarPackage ({}, [", self.num_elements)?;

        for i in 0..self.package_element_list.len() {
            write!(f, "{}", self.package_element_list[i])?;

            if i < self.package_element_list.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "])")
    }
}
