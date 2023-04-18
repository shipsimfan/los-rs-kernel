use crate::parser::{ast::Argument, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct LoadTable<'a> {
    signature: Box<Argument<'a>>,
    oem_id: Box<Argument<'a>>,
    oem_table_id: Box<Argument<'a>>,
    root_path: Box<Argument<'a>>,
    parameter_path: Box<Argument<'a>>,
    parameter_data: Box<Argument<'a>>,
}

impl<'a> LoadTable<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let signature = Box::new(Argument::parse(stream, context)?);
        let oem_id = Box::new(Argument::parse(stream, context)?);
        let oem_table_id = Box::new(Argument::parse(stream, context)?);
        let root_path = Box::new(Argument::parse(stream, context)?);
        let parameter_path = Box::new(Argument::parse(stream, context)?);
        let parameter_data = Box::new(Argument::parse(stream, context)?);

        Ok(LoadTable {
            signature,
            oem_id,
            oem_table_id,
            root_path,
            parameter_path,
            parameter_data,
        })
    }
}

impl<'a> core::fmt::Display for LoadTable<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "LoadTable ({}, {}, {}, {}, {}, {})",
            self.signature,
            self.oem_id,
            self.oem_table_id,
            self.root_path,
            self.parameter_path,
            self.parameter_data
        )
    }
}
