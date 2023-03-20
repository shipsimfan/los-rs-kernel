use super::{Table, TableHeader};
use base::Logger;
use core::ptr::NonNull;

#[repr(packed)]
pub(crate) struct DSDT {
    header: TableHeader,
    definition_block: u8,
}

impl DSDT {
    pub(crate) fn get<'a>(dsdt: NonNull<Self>, logger: &Logger) -> Option<&'a Self> {
        let dsdt = unsafe { dsdt.as_ref() };
        if dsdt.verify() {
            Some(dsdt)
        } else {
            logger.log(base::Level::Error, "Invalid DSDT");
            None
        }
    }

    pub(crate) fn definition_block(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                &self.definition_block,
                self.header.length() - core::mem::size_of::<TableHeader>(),
            )
        }
    }
}

impl Table for DSDT {
    const SIGNATURE: [u8; 4] = *b"DSDT";
    const REVISION: u8 = 1;

    fn header(&self) -> &TableHeader {
        &self.header
    }
}
