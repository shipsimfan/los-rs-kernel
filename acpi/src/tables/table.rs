use super::{Checksum, Error, Result, TableHeader};
use crate::namespace::Namespace;
use core::ptr::NonNull;

pub(super) trait Table: Sized {
    const REVISION: u8;
    const SIGNATURE: [u8; 4];

    fn load(ptr: NonNull<Self>, namespace: &mut Namespace) -> Result<()> {
        let this = unsafe { ptr.as_ref() };
        if this.verify() {
            this.do_load(namespace)
        } else {
            Err(Error::invalid_table(&Self::SIGNATURE))
        }
    }

    fn do_load(&self, namespace: &mut Namespace) -> Result<()>;

    fn header(&self) -> &TableHeader;

    fn verify(&self) -> bool {
        self.header().verify(Self::SIGNATURE, Self::REVISION) && self.verify_checksum()
    }
}

impl<T: Table> Checksum for T {
    fn length(&self) -> usize {
        self.header().length()
    }
}
