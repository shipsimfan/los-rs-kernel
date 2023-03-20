use super::{Checksum, TableHeader};

pub(crate) trait Table: Sized {
    const REVISION: u8;
    const SIGNATURE: [u8; 4];

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
