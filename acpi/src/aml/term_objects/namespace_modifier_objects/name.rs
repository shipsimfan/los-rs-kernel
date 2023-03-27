use crate::aml::{
    impl_core_display, term_objects::data_objects::DataRefObject, Display, Error, NameString,
    Result, Stream,
};

pub(in crate::aml::term_objects) struct Name {
    name: NameString,
    data_ref_object: DataRefObject,
}

impl Name {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let data_ref_object = DataRefObject::parse(stream)?
            .ok_or_else(|| Error::unexpected_byte(stream.next().unwrap(), stream.offset() - 1))
            .unwrap();

        Ok(Name {
            name,
            data_ref_object,
        })
    }
}

impl Display for Name {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Name ({}, {})", self.name, self.data_ref_object)
    }
}

impl_core_display!(Name);
