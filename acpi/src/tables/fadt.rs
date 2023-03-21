use super::{Table, TableHeader, DSDT};
use crate::loader;
use base::PhysicalAddress;
use core::ptr::NonNull;

#[repr(packed)]
#[allow(unused)]
pub(crate) struct FADT {
    header: TableHeader,
    firmware_control: u32,
    dsdt: u32,
    // TODO: Add remaining fields
}

impl FADT {
    pub(crate) fn dsdt(&self) -> loader::Result<&DSDT> {
        let dsdt: &DSDT = unsafe {
            NonNull::new(PhysicalAddress::from_raw(self.dsdt as usize).into_virtual())
                .ok_or(loader::Error::missing_table(&DSDT::SIGNATURE))?
                .as_ref()
        };

        if dsdt.verify() {
            Ok(dsdt)
        } else {
            Err(loader::Error::invalid_table(&DSDT::SIGNATURE))
        }
    }
}

impl Table for FADT {
    const SIGNATURE: [u8; 4] = *b"FACP";
    const REVISION: u8 = 1;

    fn header(&self) -> &TableHeader {
        &self.header
    }
}
