use crate::{Address, Header, Table};

#[repr(packed(1))]
pub struct HPET {
    pub header: Header,
    pub hardware_revision_id: u8,
    pub compartor_info: u8,
    pub pci_vendor_id: u16,
    pub address: Address,
    pub number: u8,
    pub minimum_tick: u16,
    pub page_protection: u8,
}

impl Table for HPET {
    const SIGNATURE: &'static str = "HPET";

    fn verify(&self) -> bool {
        self.header.calculate_checksum() == 0
    }
}
