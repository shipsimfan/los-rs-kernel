use super::DataObject;
use crate::{
    parser::{name_string, next, pkg_length, Context, Result, Stream},
    Path,
};
use alloc::vec::Vec;

pub(crate) enum PackageElement<'a> {
    DataObject(DataObject<'a>),
    Path(Path),
}

pub(crate) struct Package<'a> {
    num_elements: u8,
    package_element_list: Vec<PackageElement<'a>>,
}

impl<'a> Package<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Package")?;

        let num_elements = next!(stream, "Package");

        let mut package_element_list = Vec::with_capacity(num_elements as usize);
        while stream.remaining() > 0 {
            package_element_list.push(PackageElement::parse(&mut stream, context)?);
        }

        Ok(Package {
            num_elements,
            package_element_list,
        })
    }
}

impl<'a> core::fmt::Display for Package<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Package ({}, [", self.num_elements)?;

        for i in 0..self.package_element_list.len() {
            write!(f, "{}", self.package_element_list[i])?;

            if i < self.package_element_list.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "])")
    }
}

impl<'a> PackageElement<'a> {
    pub(self) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        if let Some(data_object) = DataObject::parse_opt(stream, context)? {
            return Ok(PackageElement::DataObject(data_object));
        }

        Ok(PackageElement::Path(name_string::parse(
            stream,
            "Package Element",
        )?))
    }
}

impl<'a> core::fmt::Display for PackageElement<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PackageElement::DataObject(data_object) => data_object.fmt(f),
            PackageElement::Path(path) => path.fmt(f),
        }
    }
}
