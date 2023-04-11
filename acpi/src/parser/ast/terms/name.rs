use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::DataObject, name_string, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct Name<'a> {
    path: Path,
    data_object: DataObject<'a>,
}

impl<'a> Name<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let path = name_string::parse(stream, "Name")?;
        let data_object = DataObject::parse(stream, context)?;

        Ok(Name { path, data_object })
    }
}

impl<'a> Display for Name<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(f, "Name ({}, {})", self.path, self.data_object)
    }
}

impl_core_display_lifetime!(Name);
