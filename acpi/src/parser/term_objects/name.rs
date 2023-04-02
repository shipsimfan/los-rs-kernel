use crate::parser::{DataObject, NameString, Result, Stream};

pub(crate) struct Name<'a> {
    name: NameString,
    data_object: DataObject<'a>,
}

impl<'a> Name<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let data_object = DataObject::parse(stream)?;

        Ok(Name { name, data_object })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn data_object(&self) -> &DataObject {
        &self.data_object
    }
}
