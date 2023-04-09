use super::{Error, Table, TableHeader};
use crate::parser;

#[repr(packed)]
pub(crate) struct DSDT {
    header: TableHeader,
    definition_block: u8,
}

impl Table for DSDT {
    const SIGNATURE: [u8; 4] = *b"DSDT";
    const REVISION: u8 = 1;

    fn do_load(&self, _: &mut crate::namespace::Namespace) -> super::Result<()> {
        let mut logger: base::Logger = "DSDT Interpreter".into();

        #[cfg(not(feature = "dsdt_logging"))]
        logger.set_minimum_level(base::Level::Info);

        let ast = parser::parse_definition_block(
            unsafe {
                core::slice::from_raw_parts(
                    &self.definition_block,
                    self.header.length() - core::mem::size_of::<TableHeader>(),
                )
            },
            logger,
            self.header.revision() >= 2,
        )
        .map_err(|error| Error::parser_error(&Self::SIGNATURE, error))?;

        panic!("AST: {}", ast)
    }

    fn header(&self) -> &TableHeader {
        &self.header
    }
}
