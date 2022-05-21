use crate::cluster_chain::Cluster;
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use base::error::FAT32_FS_DRIVER_MODULE_NUMBER;

#[derive(Debug)]
struct InvalidArgument;

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
#[allow(unused)]
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
#[allow(unused)]
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

    #[allow(unaligned_references)]
    pub fn name1(&self) -> &[u16] {
        &self.name1
    }

    #[allow(unaligned_references)]
    pub fn name2(&self) -> &[u16] {
        &self.name2
    }

    #[allow(unaligned_references)]
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

    pub fn first_cluster(&self) -> Cluster {
        self.first_cluster
    }

    pub fn set_file_size(&mut self, new_size: usize) {
        self.file_size = new_size;
    }

    pub fn to_disk_entries(
        self,
    ) -> base::error::Result<(DiskDirectoryEntry, Vec<LongDirectoryEntry>)> {
        // Check to see if long file name is nescessary
        let is_long_filename = if self.directory {
            if self.name.len() > 11 {
                return Err(Box::new(InvalidArgument));
            } else {
                false
            }
        } else {
            let mut filename_length = 0;
            let mut extension_length = 0;
            let mut extension = false;
            for c in self.name.chars() {
                if c == ' ' {
                    filename_length = 9;
                    break;
                }

                if c == '.' {
                    if extension {
                        filename_length = 9;
                        break;
                    } else {
                        extension = true;
                    }
                } else {
                    if extension {
                        extension_length += 1;
                    } else {
                        filename_length += 1;
                    }
                }
            }
            filename_length > 8 || extension_length > 3
        };

        // Generate short name
        let short_name = if is_long_filename {
            // Generate basis name
            let basis_name = self
                .name
                .to_ascii_uppercase()
                .replace(' ', "")
                .trim_start_matches('.')
                .to_owned();

            let mut short_name = [b' '; 11];

            let mut i = 0;
            let mut iter = basis_name.chars();
            let mut extension = false;
            while let Some(c) = iter.next() {
                if c == '.' {
                    extension = true;
                    break;
                }

                short_name[i] = c as u8;
                i += 1;
                if i >= 8 {
                    break;
                }
            }

            if !extension {
                while let Some(c) = iter.next() {
                    if c == '.' {
                        break;
                    }
                }
            }

            i = 8;
            while let Some(c) = iter.next() {
                short_name[i] = c as u8;

                i += 1;
                if i >= 11 {
                    break;
                }
            }

            // TODO: Generate tail

            short_name
        } else {
            let mut short_name = [b' '; 11];
            let mut i = 0;
            for c in self.name.chars() {
                if c == '.' && !self.directory {
                    i = 8;
                    continue;
                }

                short_name[i] = c.to_ascii_uppercase() as u8;
                i += 1;
            }

            short_name
        };

        // Generate disk directory entry
        let disk_entry = DiskDirectoryEntry {
            filename: short_name,
            attributes: if self.directory {
                ATTRIBUTE_DIRECTORY
            } else {
                0
            },
            reserved: 0,
            creation_tenths: 0,
            creation_time: 0,
            creation_date: 0,
            last_accessed_date: 0,
            first_cluster_high: (self.first_cluster.wrapping_shr(16) & 0xFFFF) as u16,
            last_modification_time: 0,
            last_modification_date: 0,
            first_cluster_low: (self.first_cluster & 0xFFFF) as u16,
            file_size: self.file_size as u32,
        };

        // Generate long directory entries
        let mut long_entries = Vec::new();
        if is_long_filename {
            let checksum = {
                let mut sum: u8 = 0;
                for c in short_name {
                    sum = if sum & 1 == 0 { 0u8 } else { 0x80u8 }
                        .wrapping_add(sum.wrapping_shr(1))
                        .wrapping_add(c);
                }
                sum
            };

            let mut ord = 1;
            let mut offset = 0;
            let mut current = [0xFFFFu16; 13];

            for c in self.name.chars() {
                current[offset] = c as u16;

                offset += 1;
                if offset >= 13 {
                    long_entries.push(LongDirectoryEntry {
                        order: ord,
                        name1: [current[0], current[1], current[2], current[3], current[4]],
                        attr: ATTRIBUTE_LONG_FILE_NAME,
                        class: 0,
                        checksum,
                        name2: [
                            current[5],
                            current[6],
                            current[7],
                            current[8],
                            current[9],
                            current[10],
                        ],
                        first_cluster_low: 0,
                        name3: [current[11], current[12]],
                    });

                    ord += 1;
                    offset = 0;
                    for val in &mut current {
                        *val = 0xFFFF;
                    }
                }
            }

            if offset == 0 {
                let len = long_entries.len();
                long_entries[len - 1].order |= 0x40;
            } else {
                current[offset] = 0;
                long_entries.push(LongDirectoryEntry {
                    order: ord | 0x40,
                    name1: [current[0], current[1], current[2], current[3], current[4]],
                    attr: ATTRIBUTE_LONG_FILE_NAME,
                    class: 0,
                    checksum,
                    name2: [
                        current[5],
                        current[6],
                        current[7],
                        current[8],
                        current[9],
                        current[10],
                    ],
                    first_cluster_low: 0,
                    name3: [current[11], current[12]],
                });
            }
        }

        Ok((disk_entry, long_entries))
    }
}

impl base::error::Error for InvalidArgument {
    fn module_number(&self) -> i32 {
        FAT32_FS_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::InvalidArgument
    }
}

impl core::fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid argument")
    }
}
