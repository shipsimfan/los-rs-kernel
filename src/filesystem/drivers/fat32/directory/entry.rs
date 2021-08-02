#![allow(dead_code)]
use super::super::fat::{Cluster, FATBox};
use crate::error;
use alloc::{borrow::ToOwned, string::String, vec::Vec};

#[repr(C)]
#[repr(packed(1))]
#[derive(Debug, Default, Clone, Copy)]
struct DiskDirectoryEntry {
    filename: [u8; 11],
    attributes: u8,
    reserved: u8,
    creation_tenths: u8,
    creation_time: u16,
    creation_date: u16,
    last_accessed_date: u16,
    first_cluster_high: u16,
    last_modification_time: u16,
    last_modification_date: u16,
    first_cluster_low: u16,
    file_size: u32,
}

#[repr(packed(1))]
struct LongDirectoryEntry {
    order: u8,
    name1: [u16; 5],
    attr: u8,
    class: u8,
    checksum: u8,
    name2: [u16; 6],
    first_cluster_low: u16,
    name3: [u16; 2],
}

struct Buffer {
    buffer: Vec<u8>,
    current_cluster_index: usize,
    cluster_chain: Vec<u32>,
    modified: bool,
    fat: FATBox,
}

pub struct DirectoryIterator {
    buffer: Buffer,
    current_index: usize,
    cluster_top: usize,
}

pub struct DirectoryEntry {
    name: String,
    directory: bool,
    first_cluster: Cluster,
    file_size: usize,
}

const ATTRIBUTE_READ_ONLY: u8 = 0x01;
const ATTRIBUTE_HIDDEN: u8 = 0x02;
const ATTRIBUTE_SYSTEM: u8 = 0x04;
const ATTRIBUTE_VOLUME_ID: u8 = 0x08;
const ATTRIBUTE_DIRECTORY: u8 = 0x10;
const ATTRIBUTE_ARCHIVE: u8 = 0x20;
const ATTRIBUTE_LONG_FILE_NAME: u8 =
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
}

impl Buffer {
    pub fn new(first_cluster: u32, fat_lock: FATBox) -> error::Result<Self> {
        let mut fat = fat_lock.lock();

        // Create the buffer
        let mut buffer = Vec::with_capacity(fat.bytes_per_cluster());
        unsafe { buffer.set_len(buffer.capacity()) };

        // Get the cluster chain
        let cluster_chain = fat.get_cluster_chain(first_cluster)?;

        // Get first cluster
        fat.read_cluster(cluster_chain[0], buffer.as_mut_slice())?;

        drop(fat);

        Ok(Buffer {
            buffer,
            current_cluster_index: 0,
            cluster_chain,
            modified: false,
            fat: fat_lock,
        })
    }

    pub fn set_current_cluster_index(&mut self, new_cluster_index: usize) -> error::Result<()> {
        if new_cluster_index == self.current_cluster_index {
            return Ok(());
        }

        if new_cluster_index >= self.cluster_chain.len() {
            return Err(error::Status::OutOfRange);
        }

        self.flush_buffer()?;

        self.current_cluster_index = new_cluster_index;
        self.fat.lock().read_cluster(
            self.cluster_chain[self.current_cluster_index],
            self.buffer.as_mut_slice(),
        )
    }

    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> error::Result<()> {
        if offset + buffer.len() > self.buffer.len() {
            return Err(error::Status::OutOfRange);
        }

        for i in 0..buffer.len() {
            buffer[i] = self.buffer[i + offset];
        }

        Ok(())
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result<()> {
        if offset + buffer.len() > self.buffer.len() {
            return Err(error::Status::OutOfRange);
        }

        self.modified = true;
        for i in 0..buffer.len() {
            self.buffer[i + offset] = buffer[i];
        }

        Ok(())
    }

    pub fn flush_buffer(&mut self) -> error::Result<()> {
        if self.modified {
            self.fat.lock().write_cluster(
                self.cluster_chain[self.current_cluster_index],
                self.buffer.as_slice(),
            )?;
            self.modified = false;
        }

        Ok(())
    }
}

impl DirectoryIterator {
    pub fn new(first_cluster: u32, fat: FATBox) -> error::Result<Self> {
        Ok(DirectoryIterator {
            buffer: Buffer::new(first_cluster, fat.clone())?,
            current_index: 1,
            cluster_top: fat.lock().bytes_per_cluster(),
        })
    }

    pub fn flush_buffer(&mut self) -> error::Result<()> {
        self.buffer.flush_buffer()
    }

    pub fn next(&mut self) -> error::Result<Option<DirectoryEntry>> {
        let mut entry_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let mut long_filename = [0u16; 256];
        let mut long_checksum = 0;
        let mut next_ord = 0;
        loop {
            self.current_index += 1;

            let (cluster_index, offset) = self.get_cluster_index_and_offset();

            self.buffer.set_current_cluster_index(cluster_index)?;
            self.buffer.read(offset, &mut entry_buffer)?;
            let entry = DiskDirectoryEntry::from_slice(&entry_buffer);

            if entry.filename[0] == 0 {
                return Ok(None);
            }

            if entry.filename[0] == 0xE5 {
                long_filename[0] = 0;
                next_ord = 0;
                continue;
            }

            if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                let long_entry = LongDirectoryEntry::from_entry(&entry);

                if long_entry.order & 0x40 != 0 {
                    next_ord = long_entry.order & !0x40;
                    long_checksum = long_entry.checksum;
                    if (next_ord as usize) * 13 < 256 {
                        long_filename[next_ord as usize * 13] = 0;
                    }
                }

                if next_ord == 0 {
                    long_filename[0] = 0;
                    continue;
                }
                if long_entry.order & !0x40 != next_ord || long_entry.checksum != long_checksum {
                    long_filename[0] = 0;
                    next_ord = 0;
                    continue;
                }

                next_ord -= 1;
                let offset = next_ord as usize * 13;
                for i in 0..5 {
                    long_filename[offset + i] = long_entry.name1[i];
                }

                for i in 0..6 {
                    long_filename[offset + i + 5] = long_entry.name2[i];
                }

                for i in 0..2 {
                    long_filename[offset + i + 11] = long_entry.name3[i];
                }

                continue;
            }

            if entry.attributes & ATTRIBUTE_VOLUME_ID != 0 {
                long_filename[0] = 0;
                next_ord = 0;
                continue;
            }

            let mut filename;
            if long_filename[0] == 0 {
                if entry.attributes & ATTRIBUTE_DIRECTORY == 0 {
                    filename = String::new();
                    for i in 0..8 {
                        if i == 0 && entry.filename[0] == 0x05 {
                            filename.push((0xE5 as char).to_ascii_lowercase());
                        } else {
                            filename.push((entry.filename[i] as char).to_ascii_lowercase());
                        }
                    }
                    filename = filename.trim().to_owned();
                    if entry.filename[8] != b' '
                        || entry.filename[9] != b' '
                        || entry.filename[10] != b' '
                    {
                        filename.push('.');
                        for i in 8..11 {
                            filename.push((entry.filename[i] as char).to_ascii_lowercase());
                        }

                        filename = filename.trim().to_owned();
                    }
                } else {
                    filename = String::from_utf8_lossy(&entry.filename)
                        .trim()
                        .to_ascii_lowercase();
                }
            } else {
                filename = String::new();
                for c in long_filename {
                    if c == 0 {
                        break;
                    }

                    filename.push(unsafe { char::from_u32_unchecked(c as u32) });
                }
            }

            return Ok(Some(DirectoryEntry::new(
                filename,
                entry.attributes & ATTRIBUTE_DIRECTORY != 0,
                (entry.first_cluster_low as u32) | ((entry.first_cluster_high as u32) << 16),
                entry.file_size as usize,
            )));
        }
    }

    pub fn write_metadata(&mut self, new_entry: DirectoryEntry) -> error::Result<()> {
        let mut buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let (cluster_index, offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(cluster_index)?;
        self.buffer.read(offset, &mut buffer)?;
        let mut entry = DiskDirectoryEntry::from_slice(&buffer);

        entry.file_size = new_entry.file_size as u32;
        entry.first_cluster_low = (new_entry.first_cluster & 0xFFFF) as u16;
        entry.first_cluster_high = (new_entry.first_cluster.wrapping_shr(16) & 0xFFFF) as u16;

        let buffer = entry.to_slice();
        self.buffer.write(offset, &buffer)?;

        Ok(())
    }

    pub fn remove(&mut self) -> error::Result<()> {
        let mut write_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let mut read_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];

        // Check next entry
        self.current_index += 1;
        let (next_cluster_index, next_offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(next_cluster_index)?;
        self.buffer.read(next_offset, &mut read_buffer)?;

        // Initialize the write buffer
        if read_buffer[0] != 0 {
            write_buffer[0] = 0xE5;
        }

        // Clear current entry
        self.current_index -= 1;
        let (cluster_index, offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(cluster_index)?;
        self.buffer.write(offset, &write_buffer)?;

        // Remove long directory entries
        let mut next_ord = 1;
        loop {
            // Read entry
            self.current_index -= 1;
            let (cluster_index, offset) = self.get_cluster_index_and_offset();
            self.buffer.set_current_cluster_index(cluster_index)?;
            self.buffer.read(offset, &mut read_buffer)?;
            let entry = DiskDirectoryEntry::from_slice(&read_buffer);

            // Check to see if valid
            if entry.attributes & ATTRIBUTE_LONG_FILE_NAME != ATTRIBUTE_LONG_FILE_NAME {
                break;
            }

            if entry.filename[0] & !0x40 != next_ord {
                break;
            }

            // Remove if valid
            self.buffer.write(offset, &write_buffer)?;

            if entry.filename[0] & 0x40 != 0 {
                break;
            }

            next_ord += 1;
        }

        Ok(())
    }

    fn get_cluster_index_and_offset(&self) -> (usize, usize) {
        let byte_index = self.current_index * core::mem::size_of::<DiskDirectoryEntry>();
        (byte_index / self.cluster_top, byte_index % self.cluster_top)
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

    pub fn set_file_size(&mut self, new_size: usize) {
        self.file_size = new_size;
    }

    pub fn first_cluster(&self) -> Cluster {
        self.first_cluster
    }
}
