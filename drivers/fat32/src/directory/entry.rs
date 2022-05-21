use crate::cluster_chain::Cluster;
use alloc::string::String;

pub struct DirectoryEntry {
    name: String,
    directory: bool,
    first_cluster: Cluster,
    file_size: usize,
}

#[repr(C)]
#[repr(packed(1))]
#[derive(Debug, Default, Clone, Copy)]
pub struct DiskDirectoryEntry {
    pub filename: [u8; 11],
    pub attributes: u8,
    pub reserved: u8,
    pub creation_tenths: u8,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_accessed_date: u16,
    pub first_cluster_high: u16,
    pub last_modification_time: u16,
    pub last_modification_date: u16,
    pub first_cluster_low: u16,
    pub file_size: u32,
}

#[repr(packed(1))]
pub struct LongDirectoryEntry {
    order: u8,
    name1: [u16; 5],
    attr: u8,
    class: u8,
    checksum: u8,
    name2: [u16; 6],
    first_cluster_low: u16,
    name3: [u16; 2],
}

const ATTRIBUTE_READ_ONLY: u8 = 0x01;
const ATTRIBUTE_HIDDEN: u8 = 0x02;
const ATTRIBUTE_SYSTEM: u8 = 0x04;
pub const ATTRIBUTE_VOLUME_ID: u8 = 0x08;
pub const ATTRIBUTE_DIRECTORY: u8 = 0x10;
const ATTRIBUTE_ARCHIVE: u8 = 0x20;
pub const ATTRIBUTE_LONG_FILE_NAME: u8 =
    ATTRIBUTE_READ_ONLY | ATTRIBUTE_HIDDEN | ATTRIBUTE_SYSTEM | ATTRIBUTE_VOLUME_ID;

impl DiskDirectoryEntry {
    pub fn from_slice(slice: &[u8]) -> DiskDirectoryEntry {
        DiskDirectoryEntry {
            filename: [
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
                slice[8], slice[9], slice[10],
            ],
            attributes: slice[11],
            reserved: slice[12],
            creation_tenths: slice[13],
            creation_time: (slice[14] as u16) | ((slice[15] as u16) << 8),
            creation_date: (slice[16] as u16) | ((slice[17] as u16) << 8),
            last_accessed_date: (slice[18] as u16) | ((slice[19] as u16) << 8),
            first_cluster_high: (slice[20] as u16) | ((slice[21] as u16) << 8),
            last_modification_time: (slice[22] as u16) | ((slice[23] as u16) << 8),
            last_modification_date: (slice[24] as u16) | ((slice[25] as u16) << 8),
            first_cluster_low: (slice[26] as u16) | ((slice[27] as u16) << 8),
            file_size: (slice[28] as u32)
                | ((slice[29] as u32) << 8)
                | ((slice[30] as u32) << 16)
                | ((slice[31] as u32) << 24),
        }
    }

    pub fn to_slice(self) -> [u8; 32] {
        [
            self.filename[0],
            self.filename[1],
            self.filename[2],
            self.filename[3],
            self.filename[4],
            self.filename[5],
            self.filename[6],
            self.filename[7],
            self.filename[8],
            self.filename[9],
            self.filename[10],
            self.attributes,
            self.reserved,
            self.creation_tenths,
            (self.creation_time & 0xFF) as u8,
            (self.creation_time.wrapping_shr(8) & 0xFF) as u8,
            (self.creation_date & 0xFF) as u8,
            (self.creation_date.wrapping_shr(8) & 0xFF) as u8,
            (self.last_accessed_date & 0xFF) as u8,
            (self.last_accessed_date.wrapping_shr(8) & 0xFF) as u8,
            (self.first_cluster_high & 0xFF) as u8,
            (self.first_cluster_high.wrapping_shr(8) & 0xFF) as u8,
            (self.last_modification_time & 0xFF) as u8,
            (self.last_modification_time.wrapping_shr(8) & 0xFF) as u8,
            (self.last_modification_date & 0xFF) as u8,
            (self.last_modification_date.wrapping_shr(8) & 0xFF) as u8,
            (self.first_cluster_low & 0xFF) as u8,
            (self.first_cluster_low.wrapping_shr(8) & 0xFF) as u8,
            (self.file_size & 0xFF) as u8,
            (self.file_size.wrapping_shr(8) & 0xFF) as u8,
            (self.file_size.wrapping_shr(16) & 0xFF) as u8,
            (self.file_size.wrapping_shr(24) & 0xFF) as u8,
        ]
    }
}

impl LongDirectoryEntry {
    pub fn from_entry(entry: &DiskDirectoryEntry) -> &LongDirectoryEntry {
        unsafe { core::mem::transmute(entry) }
    }

    pub fn order(&self) -> u8 {
        self.order
    }

    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    pub fn name1(&self) -> &[u16] {
        &self.name1
    }

    pub fn name2(&self) -> &[u16] {
        &self.name2
    }

    pub fn name3(&self) -> &[u16] {
        &self.name3
    }

    pub fn to_slice(self) -> [u8; 32] {
        unsafe { core::mem::transmute::<Self, DiskDirectoryEntry>(self) }.to_slice()
    }
}

impl DirectoryEntry {
    pub fn new(name: String, directory: bool, first_cluster: u32, file_size: usize) -> Self {
        DirectoryEntry {
            name,
            directory,
            first_cluster,
            file_size,
        }
    }

    pub fn is_directory(&self) -> bool {
        self.directory
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_size(&self) -> usize {
        self.file_size
    }
}
