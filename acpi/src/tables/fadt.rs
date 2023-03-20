use super::{Table, TableHeader, DSDT};
use base::{Logger, PhysicalAddress};
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
    pub(crate) fn get<'a>(fadt: NonNull<Self>, logger: &Logger) -> Option<&'a Self> {
        let fadt = unsafe { fadt.as_ref() };
        if fadt.verify() {
            Some(fadt)
        } else {
            logger.log(base::Level::Error, "Invalid FADT");
            None
        }
    }

    pub(crate) fn dsdt(&self) -> NonNull<DSDT> {
        NonNull::new(unsafe { PhysicalAddress::from_raw(self.dsdt as usize) }.into_virtual())
            .unwrap()
    }
}

impl Table for FADT {
    const SIGNATURE: [u8; 4] = *b"FACP";
    const REVISION: u8 = 1;

    fn header(&self) -> &TableHeader {
        &self.header
    }
}
