use super::{xsdt::XSDT, Checksum, Table};
use crate::loader;
use base::PhysicalAddress;
use core::ptr::NonNull;

#[repr(packed)]
#[allow(unused)]
pub struct RSDP {
    acpi_1_rsdp: ACPI1RSDP,
    length: u32,
    xsdt_address: PhysicalAddress,
    extended_checksum: u8,
    reserved: [u8; 3],
}

#[repr(packed)]
#[allow(unused)]
struct ACPI1RSDP {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

const SIGNATURE: [u8; 8] = *b"RSD PTR ";
const REVISION: u8 = 2;

impl RSDP {
    pub(crate) fn get<'a>(rsdp: NonNull<Self>) -> loader::Result<&'a Self> {
        let rsdp = unsafe { rsdp.as_ref() };
        if rsdp.verify() {
            Ok(rsdp)
        } else {
            Err(loader::Error::invalid_table(&SIGNATURE))
        }
    }

    pub(crate) fn xsdt(&self) -> loader::Result<&XSDT> {
        let xsdt: &XSDT = unsafe {
            NonNull::new(self.xsdt_address.into_virtual())
                .ok_or(loader::Error::missing_table(&XSDT::SIGNATURE))?
                .as_ref()
        };
        if xsdt.verify() {
            Ok(xsdt)
        } else {
            Err(loader::Error::invalid_table(&XSDT::SIGNATURE))
        }
    }

    fn verify(&self) -> bool {
        self.acpi_1_rsdp.verify()
            && self.length as usize >= core::mem::size_of::<Self>()
            && self.verify_checksum()
    }
}

impl Checksum for RSDP {
    fn length(&self) -> usize {
        self.length as usize
    }
}

impl ACPI1RSDP {
    pub(self) fn verify(&self) -> bool {
        self.signature == SIGNATURE && self.verify_checksum() && self.revision == REVISION
    }
}

impl Checksum for ACPI1RSDP {
    fn length(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
