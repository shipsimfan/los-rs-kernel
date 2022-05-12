use crate::{RootTable, Table, RSDT, XSDT};

#[repr(packed(1))]
pub struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xrsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

impl RSDP {
    pub fn get_root_table(&self) -> Option<RootTable> {
        if self.xrsdt_address == 0 {
            Some(RootTable::RSDT(crate::from_ptr::<RSDT>(
                self.rsdt_address as usize,
            )?))
        } else {
            Some(RootTable::XSDT(crate::from_ptr::<XSDT>(
                self.xrsdt_address as usize,
            )?))
        }
    }
}

impl Table for RSDP {
    const SIGNATURE: &'static str = "N/A";

    fn verify(&self) -> bool {
        // Verify ACPI 1.0 checksum
        let mut checksum: u8 = 0;
        for c in self.signature {
            checksum = checksum.wrapping_add(c);
        }
        checksum = checksum.wrapping_add(self.checksum);
        for c in self.oem_id {
            checksum = checksum.wrapping_add(c);
        }
        checksum = checksum.wrapping_add(self.revision);
        let mut i = 0;
        while i < 32 {
            checksum = checksum.wrapping_add(((self.rsdt_address.wrapping_shr(i)) & 0xFF) as u8);
            i += 8;
        }
        if checksum != 0 {
            return false;
        }

        // Verify version
        if self.revision != 2 {
            return false;
        }

        // Verify ACPI 2.0+ checksum
        i = 0;
        while i < 32 {
            checksum = checksum.wrapping_add(((self.length.wrapping_shr(i)) & 0xFF) as u8);
            i += 8;
        }

        i = 0;
        while i < 64 {
            checksum = checksum.wrapping_add(((self.xrsdt_address.wrapping_shr(i)) & 0xFF) as u8);
            i += 8;
        }
        checksum = checksum.wrapping_add(self.extended_checksum);
        for c in self.reserved {
            checksum = checksum.wrapping_add(c);
        }

        checksum == 0
    }
}
