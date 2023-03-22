use crate::aml::{
    impl_core_display, term_objects::data_objects::DataRefObject, Display, NameString, Result,
    Stream,
};

pub(in crate::aml::term_objects) struct Name {
    offset: usize,
    name: NameString,
    data_ref_object: DataRefObject,
}

impl Name {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let name = NameString::parse(stream)?;
        let data_ref_object = DataRefObject::parse(stream)?;

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
