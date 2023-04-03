use crate::parser::{DataObject, NameString, Result, Stream};

pub(crate) enum PackageElement<'a> {
    DataObject(DataObject<'a>),
    NameString(NameString),
}

impl<'a> PackageElement<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        match DataObject::parse_opt(stream)? {
            Some(data_object) => Ok(PackageElement::DataObject(data_object)),
            None => {
                NameString::parse(stream).map(|name_string| PackageElement::NameString(name_string))
            }
        }
    }
}

impl<'a> core::fmt::Display for PackageElement<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PackageElement::DataObject(data_object) => data_object.fmt(f),
            PackageElement::NameString(name_string) => name_string.fmt(f),
        }
    }
}
