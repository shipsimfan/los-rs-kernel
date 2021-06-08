use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::mem::size_of;

use crate::memory::KERNEL_VMA;

pub trait Table {
    fn verify(&self) -> Result<(), String>;
    fn get_signature() -> &'static str;
}

pub enum RootTable {
    RSDT(&'static RSDT),
    XSDT(&'static XSDT),
}

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

#[repr(packed(1))]
pub struct Header {
    signature: [u8; 4],
    pub length: u32,
    _revision: u8,
    _checksum: u8,
    _oem_id: [u8; 6],
    _oem_table_id: [u8; 8],
    _oem_revision: u32,
    _creator_id: u32,
    _creator_revision: u32,
}

#[repr(packed(1))]
pub struct RSDT {
    header: Header,
    _tables: u32,
}

#[repr(packed(1))]
pub struct XSDT {
    header: Header,
    _tables: u64,
}

#[repr(packed(1))]
pub struct MADT {
    pub header: Header,
    pub apic_address: u32,
    pub flags: u32,
}

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

#[repr(packed(1))]
pub struct Address {
    pub address_space_id: u8,
    pub register_bit_width: u8,
    pub register_bit_offset: u8,
    pub reserved: u8,
    pub address: u64,
}

pub struct TablePointer(*const Header);

pub fn from_ptr<T: Table>(ptr: usize) -> Result<&'static T, String> {
    let ptr = if ptr < KERNEL_VMA {
        ptr + KERNEL_VMA
    } else {
        ptr
    };

    let ret = unsafe { &*(ptr as *mut T) };
    ret.verify()?;
    Ok(ret)
}

impl Table for RSDP {
    fn get_signature() -> &'static str {
        "N/A"
    }

    fn verify(&self) -> Result<(), String> {
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
            return Err(format!("Invalid RSDP 1.0 checksum: {}", checksum));
        }

        // Verify version
        if self.revision != 2 {
            return Err(format!("Invalid ACPI version: {}", self.revision));
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

        if checksum != 0 {
            return Err(format!("Invalid RSDP 2.0 checksum: {}", checksum));
        }

        Ok(())
    }
}

impl RSDP {
    pub fn get_root_table(&self) -> Result<RootTable, String> {
        if self.xrsdt_address == 0 {
            Ok(RootTable::RSDT(from_ptr::<RSDT>(
                self.rsdt_address as usize,
            )?))
        } else {
            Ok(RootTable::XSDT(from_ptr::<XSDT>(
                self.xrsdt_address as usize,
            )?))
        }
    }
}

impl Header {
    pub fn check_signature(&self, signature: &str) -> bool {
        let mut iter = signature.chars().into_iter();
        for c1 in self.signature {
            match iter.next() {
                None => return false,
                Some(c2) => {
                    if c1 != c2 as u8 {
                        return false;
                    }
                }
            }
        }

        match iter.next() {
            None => true,
            Some(_) => false,
        }
    }

    pub fn calculate_checksum(&self) -> u8 {
        let mut checksum: u8 = 0;
        let length = self.length as isize;
        let ptr = self as *const _ as *const u8;
        let mut i = 0;
        while i < length {
            checksum = checksum.wrapping_add(unsafe { *ptr.offset(i) });
            i += 1;
        }

        checksum
    }
}

impl RSDT {
    fn get_tables(&self) -> Vec<TablePointer> {
        let mut ret = Vec::new();

        let length =
            ((self.header.length as usize - size_of::<Header>()) / size_of::<u32>()) as isize;
        let ptr = self as *const _ as *const u8;
        let ptr = unsafe { ptr.offset(size_of::<Header>() as isize) } as *const u32;
        let mut i = 0;
        while i < length {
            let mut table = unsafe { *ptr.offset(i) } as usize;
            if table < KERNEL_VMA {
                table += KERNEL_VMA;
            }
            ret.push(TablePointer(table as *const Header));
            i += 1;
        }

        ret
    }
}

impl Table for RSDT {
    fn get_signature() -> &'static str {
        "RSDT"
    }

    fn verify(&self) -> Result<(), String> {
        if self.header.calculate_checksum() != 0 {
            Err("Invalid RSDT checksum".to_string())
        } else {
            Ok(())
        }
    }
}

impl XSDT {
    pub fn get_tables(&self) -> Vec<TablePointer> {
        let mut ret = Vec::new();

        let length =
            ((self.header.length as usize - size_of::<Header>()) / size_of::<u64>()) as isize;
        let ptr = self as *const _ as *const u8;
        let ptr = unsafe { ptr.offset(size_of::<Header>() as isize) } as *const u64;
        let mut i = 0;
        while i < length {
            let mut table = unsafe { *ptr.offset(i) } as usize;
            if table < KERNEL_VMA {
                table += KERNEL_VMA;
            }
            ret.push(TablePointer(table as *const Header));
            i += 1;
        }

        ret
    }
}

impl Table for XSDT {
    fn get_signature() -> &'static str {
        "XSDT"
    }

    fn verify(&self) -> Result<(), String> {
        if self.header.calculate_checksum() != 0 {
            Err("Invalid XSDT checksum".to_string())
        } else {
            Ok(())
        }
    }
}

impl TablePointer {
    pub fn get(&self) -> *const Header {
        self.0
    }
}

unsafe impl Send for TablePointer {}

impl RootTable {
    pub fn get_tables(&self) -> Vec<TablePointer> {
        match self {
            RootTable::RSDT(rsdt) => rsdt.get_tables(),
            RootTable::XSDT(xsdt) => xsdt.get_tables(),
        }
    }
}

impl Table for MADT {
    fn get_signature() -> &'static str {
        "APIC"
    }

    fn verify(&self) -> Result<(), String> {
        if self.header.calculate_checksum() != 0 {
            Err("Invalid MADT checksum".to_string())
        } else {
            Ok(())
        }
    }
}

impl Table for HPET {
    fn get_signature() -> &'static str {
        "HPET"
    }

    fn verify(&self) -> Result<(), String> {
        if self.header.calculate_checksum() != 0 {
            Err("Invalid HPET checksum".to_string())
        } else {
            Ok(())
        }
    }
}
