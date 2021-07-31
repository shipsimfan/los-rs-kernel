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
    current_cluster_index: usize,
    current_offset: usize,
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
    pub fn from_slice(slice: &[u8]) -> &DiskDirectoryEntry {
        unsafe { &*(slice.as_ptr() as *const DiskDirectoryEntry) }
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

        let fat = self.fat.lock();
        if self.modified {
            fat.write_cluster(
                self.cluster_chain[self.current_cluster_index],
                self.buffer.as_slice(),
            )?;
            self.modified = false;
        }

        self.current_cluster_index = new_cluster_index;
        fat.read_cluster(
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
}

impl DirectoryIterator {
    pub fn new(first_cluster: u32, fat: FATBox) -> error::Result<Self> {
        Ok(DirectoryIterator {
            buffer: Buffer::new(first_cluster, fat.clone())?,
            current_offset: 2 * core::mem::size_of::<DiskDirectoryEntry>(),
            current_cluster_index: 0,
            cluster_top: fat.lock().bytes_per_cluster(),
        })
    }

    pub fn next(&mut self) -> error::Result<Option<DirectoryEntry>> {
        let mut entry_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let mut long_filename = [0u16; 256];
        let mut long_checksum = 0;
        let mut next_ord = 0;
        loop {
            self.buffer
                .set_current_cluster_index(self.current_cluster_index)?;
            self.buffer.read(self.current_offset, &mut entry_buffer)?;
            let entry = DiskDirectoryEntry::from_slice(&entry_buffer);

            if entry.filename[0] == 0 {
                return Ok(None);
            }

            if entry.filename[0] == 0xE5 {
                long_filename[0] = 0;
                next_ord = 0;
            } else {
                if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                    let long_entry = LongDirectoryEntry::from_entry(entry);

                    if long_entry.order & 0x40 != 0 {
                        next_ord = long_entry.order & !0x40;
                        long_checksum = long_entry.checksum;
                        if (next_ord as usize) * 13 < 256 {
                            long_filename[next_ord as usize * 13] = 0;
                        }
                    }

                    if next_ord == 0 {
                        long_filename[0] = 0;
                    } else {
                        if long_entry.order & !0x40 != next_ord
                            || long_entry.checksum != long_checksum
                        {
                            long_filename[0] = 0;
                            next_ord = 0;
                        } else {
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
                        }
                    }
                } else {
                    if entry.attributes & ATTRIBUTE_VOLUME_ID != 0 {
                        long_filename[0] = 0;
                        next_ord = 0;
                    } else {
                        let mut filename;
                        if long_filename[0] == 0 {
                            if entry.attributes & ATTRIBUTE_DIRECTORY == 0 {
                                filename = String::new();
                                for i in 0..8 {
                                    if i == 0 && entry.filename[0] == 0x05 {
                                        filename.push(0xE5 as char);
                                    } else {
                                        filename.push(entry.filename[i] as char);
                                    }
                                }
                                filename = filename.trim().to_owned();
                                if entry.filename[8] != b' '
                                    || entry.filename[9] != b' '
                                    || entry.filename[10] != b' '
                                {
                                    filename.push('.');
                                    for i in 8..11 {
                                        filename.push(entry.filename[i] as char);
                                    }

                                    filename = filename.trim().to_owned();
                                }
                            } else {
                                filename =
                                    String::from_utf8_lossy(&entry.filename).trim().to_owned();
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

                        self.current_offset += core::mem::size_of::<DiskDirectoryEntry>();
                        if self.current_offset >= self.cluster_top {
                            self.current_offset -= self.cluster_top;
                            self.current_cluster_index += 1;
                        }

                        return Ok(Some(DirectoryEntry::new(
                            filename,
                            entry.attributes & ATTRIBUTE_DIRECTORY != 0,
                            (entry.first_cluster_low as u32)
                                | ((entry.first_cluster_high as u32) << 16),
                            entry.file_size as usize,
                        )));
                    }
                }
            }

            self.current_offset += core::mem::size_of::<DiskDirectoryEntry>();
            if self.current_offset >= self.cluster_top {
                self.current_offset -= self.cluster_top;
                self.current_cluster_index += 1;
            }
        }
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
