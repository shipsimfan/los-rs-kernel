use crate::aml::{term_objects::data_objects::DataRefObject, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects::data_objects) enum PackageElement {
    DataRefObject(DataRefObject),
    NameString(NameString),
}

impl PackageElement {
    pub(in crate::aml::term_objects::data_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match DataRefObject::parse(stream)? {
            Some(data_ref_object) => Ok(PackageElement::DataRefObject(data_ref_object)),
            None => {
                NameString::parse(stream).map(|name_string| PackageElement::NameString(name_string))
            }
        }
    }
}

impl Display for PackageElement {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            PackageElement::DataRefObject(data_ref_object) => {
                data_ref_object.display(f, depth, last)
            }
            PackageElement::NameString(name_string) => {
                self.display_prefix(f, depth)?;
                writeln!(f, "{}", name_string)
            }
        }
    }
}
