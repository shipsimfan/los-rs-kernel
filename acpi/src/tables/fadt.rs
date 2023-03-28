use core::ptr::NonNull;

use base::PhysicalAddress;

use super::{Error, Table, TableHeader, DSDT};

#[repr(packed)]
#[allow(unused)]
pub(crate) struct FADT {
    header: TableHeader,
    firmware_control: u32,
    dsdt: u32,
    // TODO: Add remaining fields
}

impl Table for FADT {
    const SIGNATURE: [u8; 4] = *b"FACP";
    const REVISION: u8 = 1;

    fn do_load(&self, namespace: &mut crate::namespace::Namespace) -> super::Result<()> {
        DSDT::load(
            match NonNull::new(
                unsafe { PhysicalAddress::from_raw(self.dsdt as usize) }.into_virtual(),
            ) {
                Some(ptr) => ptr,
                None => return Err(Error::missing_table(&DSDT::SIGNATURE)),
            },
            namespace,
        )
    }

    fn header(&self) -> &TableHeader {
        &self.header
    }
}
