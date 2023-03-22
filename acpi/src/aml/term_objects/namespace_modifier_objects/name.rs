use crate::aml::{
    impl_core_display, term_objects::data_objects::DataRefObject, Display, Error, NameString,
    Result, Stream,
};

pub(in crate::aml::term_objects) struct Name {
    offset: usize,
    name: NameString,
    data_ref_object: DataRefObject,
}

impl Name {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let name = NameString::parse(stream)?;
        let data_ref_object = DataRefObject::parse(stream)?
            .ok_or_else(|| Error::unexpected_byte(stream.next().unwrap(), stream.offset() - 1))?;

        Ok(Name {
            offset,
            name,
            data_ref_object,
        })
    }
}

impl Display for Name {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Name {} @ {}:", self.name, self.offset)?;
        self.data_ref_object.display(f, depth + 1)
    }
}

impl_core_display!(Name);
