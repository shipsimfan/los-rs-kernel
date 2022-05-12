use memory::KERNEL_VMA;

mod fadt;
mod hpet;
mod madt;
mod root;
mod rsdp;

pub use fadt::*;
pub use hpet::*;
pub use madt::*;
pub use root::*;
pub use rsdp::*;

pub trait Table {
    const SIGNATURE: &'static str;

    fn verify(&self) -> bool;
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
pub struct Address {
    pub address_space_id: u8,
    pub register_bit_width: u8,
    pub register_bit_offset: u8,
    pub reserved: u8,
    pub address: u64,
}

pub fn from_ptr<T: Table>(ptr: usize) -> Option<&'static T> {
    let ptr = if ptr < KERNEL_VMA {
        ptr + KERNEL_VMA
    } else {
        ptr
    };

    let ret = unsafe { &*(ptr as *mut T) };
    if ret.verify() {
        Some(ret)
    } else {
        None
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
