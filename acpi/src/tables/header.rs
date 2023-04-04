#[repr(packed)]
#[allow(unused)]
pub(super) struct TableHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    asl_compiler_id: [u8; 4],
    asl_compiler_revision: u32,
}

impl TableHeader {
    pub(super) fn verify(&self, signature: [u8; 4], revision: u8) -> bool {
        self.signature == signature && self.revision == revision
    }

    pub(super) fn length(&self) -> usize {
        self.length as usize
    }

    pub(super) fn signature(&self) -> &[u8; 4] {
        &self.signature
    }

    pub(super) fn revision(&self) -> u8 {
        self.revision
    }
}
