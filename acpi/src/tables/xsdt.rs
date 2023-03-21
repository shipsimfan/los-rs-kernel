use super::{Table, TableHeader};
use crate::loader;
use base::PhysicalAddress;
use core::ptr::NonNull;

#[repr(packed)]
pub(crate) struct XSDT {
    header: TableHeader,
    entries: u8,
}

struct Iter {
    current: *const PhysicalAddress,
    remaining: usize,
}

impl XSDT {
    pub(crate) fn get_table<T: Table>(&self) -> loader::Result<&T> {
        for table in self.iter() {
            let table = table.into_virtual::<[u8; 4]>();

            if unsafe { *table } == T::SIGNATURE {
                let table = unsafe {
                    NonNull::new(table as *mut T)
                        .ok_or(loader::Error::invalid_table(&T::SIGNATURE))?
                        .as_ref()
                };

                return if table.verify() {
                    Ok(table)
                } else {
                    Err(loader::Error::invalid_table(&T::SIGNATURE))
                };
            }
        }

        Err(loader::Error::missing_table(&T::SIGNATURE))
    }

    fn iter(&self) -> Iter {
        Iter {
            current: &self.entries as *const _ as *const _,
            remaining: (self.header.length() - core::mem::size_of::<TableHeader>())
                / core::mem::size_of::<PhysicalAddress>(),
        }
    }
}

impl Table for XSDT {
    const SIGNATURE: [u8; 4] = *b"XSDT";
    const REVISION: u8 = 1;

    fn header(&self) -> &TableHeader {
        &self.header
    }
}

impl Iterator for Iter {
    type Item = PhysicalAddress;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;

        let ret = unsafe { *self.current };
        unsafe { self.current.add(1) };
        Some(ret)
    }
}
