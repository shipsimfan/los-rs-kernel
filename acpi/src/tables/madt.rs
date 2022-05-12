use crate::{Header, Table};

#[repr(packed(1))]
pub struct MADT {
    pub header: Header,
    pub apic_address: u32,
    pub flags: u32,
}

impl Table for MADT {
    const SIGNATURE: &'static str = "APIC";

    fn verify(&self) -> bool {
        self.header.calculate_checksum() == 0
    }
}
