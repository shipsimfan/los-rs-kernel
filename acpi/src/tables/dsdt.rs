use super::{Error, Table, TableHeader};
use crate::interpreter::Interpreter;

#[repr(packed)]
pub(crate) struct DSDT {
    header: TableHeader,
    definition_block: u8,
}

impl Table for DSDT {
    const SIGNATURE: [u8; 4] = *b"DSDT";
    const REVISION: u8 = 1;

    fn do_load(&self, namespace: &mut crate::namespace::Namespace) -> super::Result<()> {
        Interpreter::new(namespace)
            .load_definition_block(unsafe {
                core::slice::from_raw_parts(
                    &self.definition_block,
                    self.header.length() - core::mem::size_of::<TableHeader>(),
                )
            })
            .map_err(|error| Error::interpreter_error(&Self::SIGNATURE, error))
    }

    fn header(&self) -> &TableHeader {
        &self.header
    }
}
