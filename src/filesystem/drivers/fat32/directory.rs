use super::{fat::FATBox, SECTOR_SIZE};
use crate::{
    error,
    filesystem::{self, File, FileMetadata},
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

pub struct Directory {
    first_cluster: u32,
    fat: FATBox,
}

#[repr(C)]
#[repr(packed(1))]
#[derive(Debug, Default, Clone, Copy)]
struct DirectoryEntry {
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
    _attr: u8,
    _class: u8,
    checksum: u8,
    name2: [u16; 6],
    _first_cluster_low: u16,
    name3: [u16; 2],
}

const ATTRIBUTE_READ_ONLY: u8 = 0x01;
const ATTRIBUTE_HIDDEN: u8 = 0x02;
const ATTRIBUTE_SYSTEM: u8 = 0x04;
const ATTRIBUTE_VOLUME_ID: u8 = 0x08;
const ATTRIBUTE_DIRECTORY: u8 = 0x10;
#[allow(dead_code)]
const ATTRIBUTE_ARCHIVE: u8 = 0x20;
const ATTRIBUTE_LONG_FILE_NAME: u8 =
    ATTRIBUTE_READ_ONLY | ATTRIBUTE_HIDDEN | ATTRIBUTE_SYSTEM | ATTRIBUTE_VOLUME_ID;

impl Directory {
    pub fn new(first_cluster: u32, fat: FATBox) -> Self {
        Directory {
            first_cluster: first_cluster,
            fat: fat,
        }
    }

    fn open(&self, name: &str) -> error::Result<DirectoryEntry> {
        // Lock the FAT
        let fat = self.fat.lock();

        // Get cluster chain
        let cluster_chain = fat.get_cluster_chain(self.first_cluster)?;

        // Read each cluster
        let mut buffer =
            [DirectoryEntry::default(); SECTOR_SIZE / core::mem::size_of::<DirectoryEntry>()];
        let u8_buffer =
            unsafe { core::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, SECTOR_SIZE) };
        'main_loop: for cluster in cluster_chain {
            fat.read_cluster(cluster, u8_buffer)?;

            for mut entry in buffer {
                // Look for end of entries
                if entry.filename[0] == 0 {
                    break 'main_loop;
                }

                // Look for blank entries
                if entry.filename[0] == 0xE5 {
                    continue;
                }

                // Correct first byte if required
                if entry.filename[0] == 0x05 {
                    entry.filename[0] = 0xE5;
                }

                // Ignore long file names
                // TODO: parse long file names
                if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                    continue;
                }

                for byte in &mut entry.filename {
                    if *byte >= b'A' && *byte <= b'Z' {
                        *byte = *byte - b'A' + b'a';
                    }
                }

                let filename = if entry.attributes & ATTRIBUTE_DIRECTORY != 0 {
                    String::from_utf8_lossy(&entry.filename).trim().to_string()
                } else {
                    let mut filename = String::from_utf8_lossy(&entry.filename[..8])
                        .trim()
                        .to_string();
                    if entry.filename[8] == ' ' as u8
                        && entry.filename[9] == ' ' as u8
                        && entry.filename[10] == ' ' as u8
                    {
                        filename
                    } else {
                        filename.push('.');
                        filename.push_str(String::from_utf8_lossy(&entry.filename[8..]).trim());
                        filename
                    }
                };

                if filename == name {
                    return Ok(entry);
                }
            }
        }

        Err(error::Status::NoEntry)
    }
}

impl filesystem::Directory for Directory {
    fn get_sub_files(&self) -> error::Result<Vec<(String, FileMetadata)>> {
        // Lock the FAT
        let fat = self.fat.lock();

        // Get cluster chain
        let cluster_chain = fat.get_cluster_chain(self.first_cluster)?;

        // Read each cluster
        let mut files = Vec::new();
        let mut buffer =
            [DirectoryEntry::default(); SECTOR_SIZE / core::mem::size_of::<DirectoryEntry>()];
        let u8_buffer =
            unsafe { core::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, SECTOR_SIZE) };
        let mut long_filename = [0u16; 255];
        let mut long_checksum = 0;
        let mut next_ord = 0;
        'main_loop: for cluster in cluster_chain {
            fat.read_cluster(cluster, u8_buffer)?;

            // Read each entry
            for mut entry in buffer {
                // Look for end of entries
                if entry.filename[0] == 0 {
                    break 'main_loop;
                }

                // Look for blank entries
                if entry.filename[0] == 0xE5 {
                    long_filename[0] = 0;
                    next_ord = 0;
                    continue;
                }

                // Ignore long file names
                if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                    let long_entry = LongDirectoryEntry::from_entry(&entry);

                    if long_entry.order & 0x40 != 0 {
                        next_ord = long_entry.order & !0x40;
                        long_checksum = long_entry.checksum;
                        if next_ord * 13 < 255 {
                            long_filename[next_ord as usize * 13] = 0;
                        }
                    }

                    if next_ord == 0 {
                        long_filename[0] = 0;
                        continue;
                    }

                    if long_entry.order & !0x40 != next_ord {
                        long_filename[0] = 0;
                        next_ord = 0;
                    }

                    if long_entry.checksum != long_checksum {
                        long_filename[0] = 0;
                        next_ord = 0;
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

                // Ignore volume ID entry
                if entry.attributes & ATTRIBUTE_VOLUME_ID != 0 {
                    long_filename[0] = 0;
                    next_ord = 0;
                    continue;
                }

                // Correct first byte if required
                if entry.filename[0] == 0x05 {
                    entry.filename[0] = 0xE5;
                }

                // Change filename to lower case
                for byte in &mut entry.filename {
                    if *byte >= b'A' && *byte <= b'Z' {
                        *byte = *byte - b'A' + b'a';
                    }
                }

                // Check if the entry is the right type of entry
                if entry.attributes & ATTRIBUTE_DIRECTORY == 0 {
                    let mut filename = String::new();
                    if long_filename[0] == 0 {
                        for i in 0..8 {
                            filename.push(entry.filename[i] as char);
                        }
                        filename = filename.trim().to_string();
                        if entry.filename[8] != ' ' as u8
                            || entry.filename[9] != ' ' as u8
                            || entry.filename[10] != ' ' as u8
                        {
                            filename.push('.');
                            for i in 8..11 {
                                filename.push(entry.filename[i] as char);
                            }
                        }
                    } else {
                        for c in long_filename {
                            if c == 0 {
                                break;
                            }

                            filename.push(unsafe { char::from_u32_unchecked(c as u32) });
                        }
                    }

                    // Get metadata
                    let metadata = FileMetadata::new(entry.file_size as usize);

                    files.push((filename.trim().to_string(), metadata));
                }

                long_filename[0] = 0;
                next_ord = 0;
            }
        }

        Ok(files)
    }

    fn get_sub_directories(&self) -> error::Result<Vec<String>> {
        // Lock the FAT
        let fat = self.fat.lock();

        // Get cluster chain
        let cluster_chain = fat.get_cluster_chain(self.first_cluster)?;

        // Read each cluster
        let mut directories = Vec::new();
        let mut buffer =
            [DirectoryEntry::default(); SECTOR_SIZE / core::mem::size_of::<DirectoryEntry>()];
        let u8_buffer =
            unsafe { core::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, SECTOR_SIZE) };
        'main_loop: for cluster in cluster_chain {
            fat.read_cluster(cluster, u8_buffer)?;

            // Read each entry
            for mut entry in buffer {
                // Look for end of entries
                if entry.filename[0] == 0 {
                    break 'main_loop;
                }

                // Look for blank entries
                if entry.filename[0] == 0xE5 {
                    continue;
                }

                // Ignore long file names
                // TODO: parse long file names
                if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                    continue;
                }

                // Ignore volume ID entry
                if entry.attributes & ATTRIBUTE_VOLUME_ID != 0 {
                    continue;
                }

                // Correct first byte if required
                if entry.filename[0] == 0x05 {
                    entry.filename[0] = 0xE5;
                }

                // Change filename to lowercase
                for byte in &mut entry.filename {
                    if *byte >= b'A' && *byte <= b'Z' {
                        *byte = *byte - b'A' + b'a';
                    }
                }

                // Check if the entry is the right type of entry
                if entry.attributes & ATTRIBUTE_DIRECTORY != 0 {
                    directories.push(String::from_utf8_lossy(&entry.filename).trim().to_string());
                }
            }
        }

        Ok(directories)
    }

    fn open_file(&self, filename: &str) -> error::Result<Box<dyn File>> {
        let entry = self.open(filename)?;

        let first_cluster =
            (entry.first_cluster_low as u32) | ((entry.first_cluster_high as u32) << 16);
        Ok(Box::new(super::file::File::new(
            first_cluster,
            entry.file_size as usize,
            self.fat.clone(),
        )))
    }

    fn open_directory(
        &self,
        directory_name: &str,
    ) -> error::Result<Box<dyn filesystem::Directory>> {
        let entry = self.open(directory_name)?;

        let first_cluster =
            (entry.first_cluster_low as u32) | ((entry.first_cluster_high as u32) << 16);
        Ok(Box::new(Directory::new(first_cluster, self.fat.clone())))
    }

    fn make_file(&self, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn make_directory(&self, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn rename_file(&self, _: &str, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn rename_directory(&self, _: &str, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn remove_file(&self, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn remove_directory(&self, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn update_file_metadata(
        &self,
        target_filename: &str,
        new_metadata: FileMetadata,
    ) -> error::Result<()> {
        // Lock the FAT
        let fat = self.fat.lock();

        // Get cluster chain
        let cluster_chain = fat.get_cluster_chain(self.first_cluster)?;

        // Read each cluster
        let mut buffer =
            [DirectoryEntry::default(); SECTOR_SIZE / core::mem::size_of::<DirectoryEntry>()];
        let u8_buffer =
            unsafe { core::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, SECTOR_SIZE) };
        let mut long_filename = [0u16; 255];
        let mut long_checksum = 0;
        let mut next_ord = 0;
        'main_loop: for cluster in cluster_chain {
            fat.read_cluster(cluster, u8_buffer)?;

            // Read each entry
            for mut entry in &mut buffer {
                // Look for end of entries
                if entry.filename[0] == 0 {
                    break 'main_loop;
                }

                // Look for blank entries
                if entry.filename[0] == 0xE5 {
                    long_filename[0] = 0;
                    next_ord = 0;
                    continue;
                }

                // Ignore long file names
                if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                    let long_entry = LongDirectoryEntry::from_entry(&entry);

                    if long_entry.order & 0x40 != 0 {
                        next_ord = long_entry.order & !0x40;
                        long_checksum = long_entry.checksum;
                        if next_ord * 13 < 255 {
                            long_filename[next_ord as usize * 13] = 0;
                        }
                    }

                    if next_ord == 0 {
                        long_filename[0] = 0;
                        continue;
                    }

                    if long_entry.order & !0x40 != next_ord {
                        long_filename[0] = 0;
                        next_ord = 0;
                    }

                    if long_entry.checksum != long_checksum {
                        long_filename[0] = 0;
                        next_ord = 0;
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

                // Ignore volume ID entry
                if entry.attributes & ATTRIBUTE_VOLUME_ID != 0 {
                    long_filename[0] = 0;
                    next_ord = 0;
                    continue;
                }

                // Check if the entry is the right type of entry
                if entry.attributes & ATTRIBUTE_DIRECTORY == 0 {
                    // Correct first byte if required
                    if entry.filename[0] == 0x05 {
                        entry.filename[0] = 0xE5;
                    }

                    let mut filename = String::new();
                    if long_filename[0] == 0 {
                        for i in 0..8 {
                            filename.push(entry.filename[i] as char);
                        }
                        filename = filename.trim().to_string();
                        if entry.filename[8] != ' ' as u8
                            || entry.filename[9] != ' ' as u8
                            || entry.filename[10] != ' ' as u8
                        {
                            filename.push('.');
                            for i in 8..11 {
                                filename.push(entry.filename[i] as char);
                            }
                        }

                        filename = filename.to_ascii_lowercase();
                    } else {
                        for c in long_filename {
                            if c == 0 {
                                break;
                            }

                            filename.push(unsafe { char::from_u32_unchecked(c as u32) });
                        }
                    }

                    // Undo first byte if required
                    if entry.filename[0] == 0xE5 {
                        entry.filename[0] = 0x05;
                    }

                    // Update metadata
                    if filename.trim() == target_filename {
                        entry.file_size = new_metadata.size() as u32;
                        fat.write_cluster(cluster, u8_buffer)?;
                        return Ok(());
                    }
                }

                long_filename[0] = 0;
                next_ord = 0;
            }
        }

        Err(error::Status::NoEntry)
    }
}

impl LongDirectoryEntry {
    pub fn from_entry(entry: &DirectoryEntry) -> &LongDirectoryEntry {
        unsafe { core::mem::transmute(entry) }
    }
}
