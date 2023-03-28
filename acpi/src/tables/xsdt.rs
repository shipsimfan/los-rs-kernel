use super::{Checksum, Error, Result, TableHeader};
use base::PhysicalAddress;
use core::ptr::NonNull;

#[repr(packed)]
pub(super) struct XSDT {
    header: TableHeader,
    entries: u8,
}

pub(super) struct Iter {
    current: *const PhysicalAddress,
    remaining: usize,
}

const REVISION: u8 = 1;

pub(super) const SIGNATURE: [u8; 4] = *b"XSDT";

impl XSDT {
    pub(super) fn load<'a>(ptr: NonNull<Self>) -> Result<&'a Self> {
        let this = unsafe { ptr.as_ref() };
        if this.verify() {
            Ok(this)
        } else {
            Err(Error::invalid_table(&SIGNATURE))
        }
    }

    pub(super) fn iter(&self) -> Iter {
        Iter {
            current: &self.entries as *const _ as *const _,
            remaining: (self.header.length() - core::mem::size_of::<TableHeader>())
                / core::mem::size_of::<PhysicalAddress>(),
        }
    }

    fn verify(&self) -> bool {
        self.header.verify(SIGNATURE, REVISION) && self.verify_checksum()
    }
}

impl IntoIterator for XSDT {
    type IntoIter = Iter;
    type Item = NonNull<TableHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Checksum for XSDT {
    fn length(&self) -> usize {
        self.header.length()
    }
}

impl Iterator for Iter {
    type Item = NonNull<TableHeader>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = None;

        while ret.is_none() {
            if self.remaining == 0 {
                return None;
            }
            self.remaining -= 1;

            let ptr = unsafe { *self.current };
            self.current = unsafe { self.current.add(1) };

            ret = NonNull::new(ptr.into_virtual())
        }

        ret
    }
}
