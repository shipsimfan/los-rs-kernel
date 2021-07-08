use super::{fat::FATBox, SECTOR_SIZE};
use crate::{
    error,
    filesystem::{self, File},
    logln,
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

    fn get_names(&self, directory: bool) -> Result<Vec<String>, error::Status> {
        // Lock the FAT
        let fat = self.fat.lock();

        // Get cluster chain
        let cluster_chain = fat.get_cluster_chain(self.first_cluster)?;

        // Read each cluster
        let mut names = Vec::new();
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

                // Correct first byte if required
                if entry.filename[0] == 0x05 {
                    entry.filename[0] = 0xE5;
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

                // Check if the entry is the right type of entry
                if (entry.attributes & ATTRIBUTE_DIRECTORY != 0) == directory {
                    if directory {
                        // Push the whole trimmed named if directory
                        names.push(String::from_utf8_lossy(&entry.filename).trim().to_string());
                    } else {
                        // Parse the filename otherwise
                        let mut filename = String::new();
                        for i in 0..8 {
                            filename.push(entry.filename[i] as char);
                        }
                        let mut filename = filename.trim().to_string();
                        if entry.filename[8] == ' ' as u8
                            && entry.filename[9] == ' ' as u8
                            && entry.filename[10] == ' ' as u8
                        {
                            names.push(filename);
                        } else {
                            filename.push('.');
                            for i in 8..11 {
                                filename.push(entry.filename[i] as char);
                            }

                            names.push(filename.trim().to_string());
                        }
                    }
                }
            }
        }

        Ok(names)
    }

    fn open(&self, name: &str) -> Result<DirectoryEntry, error::Status> {
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

        Err(error::Status::NotFound)
    }
}

impl filesystem::Directory for Directory {
    fn get_sub_files(&self) -> Result<Vec<String>, error::Status> {
        self.get_names(false)
    }

    fn get_sub_directories(&self) -> Result<Vec<String>, error::Status> {
        self.get_names(true)
    }

    fn open_file(&self, filename: &str) -> Result<Box<dyn File>, error::Status> {
        logln!("Opening {}", filename);
        Err(error::Status::NotSupported)
    }

    fn open_directory(
        &self,
        directory_name: &str,
    ) -> Result<Box<dyn filesystem::Directory>, error::Status> {
        let entry = self.open(directory_name)?;

        let first_cluster =
            (entry.first_cluster_low as u32) | ((entry.first_cluster_high as u32) << 16);
        Ok(Box::new(Directory::new(first_cluster, self.fat.clone())))
    }

    fn make_file(&self, _: &str) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn make_directory(&self, _: &str) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn rename_file(&self, _: &str, _: &str) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn rename_directory(&self, _: &str, _: &str) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn remove_file(&self, _: &str) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn remove_directory(&self, _: &str) -> error::Result {
        Err(error::Status::NotSupported)
    }
}
