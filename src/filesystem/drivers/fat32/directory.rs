use self::entry::DirectoryIterator;

use super::fat::FATBox;
use crate::{
    error,
    filesystem::{self, File, FileMetadata},
};
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};

mod entry;

pub struct Directory {
    first_cluster: u32,
    fat: FATBox,
}

impl Directory {
    pub fn new(first_cluster: u32, fat: FATBox) -> Self {
        Directory {
            first_cluster: first_cluster,
            fat: fat,
        }
    }

    fn create_iterator(&self) -> error::Result<DirectoryIterator> {
        DirectoryIterator::new(self.first_cluster, self.fat.clone())
    }

    fn remove(&self, name: &str, directory: bool) -> error::Result<()> {
        let mut iter = self.create_iterator()?;
        while let Some(entry) = iter.next()? {
            if entry.name() == name {
                if entry.is_directory() == directory {
                    // Free cluster chain
                    self.fat.lock().free_cluster_chain(entry.first_cluster())?;

                    // Remove entry
                    iter.remove()?;
                    iter.flush_buffer()?;

                    return Ok(());
                } else if directory {
                    return Err(error::Status::IsFile);
                } else {
                    return Err(error::Status::IsDirectory);
                }
            }
        }

        Err(error::Status::NoEntry)
    }
}

impl filesystem::Directory for Directory {
    fn get_sub_files(&self) -> error::Result<Vec<(String, FileMetadata)>> {
        let mut sub_files = Vec::new();
        let mut iter = self.create_iterator()?;
        while let Some(entry) = iter.next()? {
            if !entry.is_directory() {
                sub_files.push((
                    entry.name().to_owned(),
                    FileMetadata::new(entry.file_size()),
                ))
            }
        }

        Ok(sub_files)
    }

    fn get_sub_directories(&self) -> error::Result<Vec<String>> {
        let mut sub_directories = Vec::new();
        let mut iter = self.create_iterator()?;
        while let Some(entry) = iter.next()? {
            if entry.is_directory() {
                if entry.name() == "." || entry.name() == ".." {
                    continue;
                }
                sub_directories.push(entry.name().to_owned());
            }
        }

        Ok(sub_directories)
    }

    fn open_file(&self, filename: &str) -> error::Result<Box<dyn File>> {
        let mut iter = self.create_iterator()?;

        while let Some(entry) = iter.next()? {
            if entry.name() == filename {
                return if entry.is_directory() {
                    Err(error::Status::IsDirectory)
                } else {
                    Ok(Box::new(super::file::File::new(
                        entry.first_cluster(),
                        entry.file_size(),
                        self.fat.clone(),
                    )))
                };
            }
        }

        Err(error::Status::NoEntry)
    }

    fn open_directory(
        &self,
        directory_name: &str,
    ) -> error::Result<Box<dyn filesystem::Directory>> {
        let mut iter = self.create_iterator()?;

        while let Some(entry) = iter.next()? {
            if entry.name() == directory_name {
                return if !entry.is_directory() {
                    Err(error::Status::IsFile)
                } else {
                    Ok(Box::new(Directory::new(
                        entry.first_cluster(),
                        self.fat.clone(),
                    )))
                };
            }
        }

        Err(error::Status::NoEntry)
    }

    fn make_file(&self, filename: &str) -> error::Result<()> {
        // Allocate cluster
        let first_cluster = self.fat.lock().allocate_cluster()?;

        // Create the entry
        let entry = entry::DirectoryEntry::new(filename.to_owned(), false, first_cluster, 0);
        let mut iter = self.create_iterator()?;
        iter.create(entry)?;
        iter.flush_buffer()
    }

    fn make_directory(&self, directory_name: &str) -> error::Result<()> {
        // Allocate cluster
        let first_cluster = self.fat.lock().allocate_cluster()?;

        // Write empty directory
        let dot_entry = entry::DiskDirectoryEntry {
            filename: [
                b'.', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            ],
            attributes: entry::ATTRIBUTE_DIRECTORY,
            reserved: 0,
            creation_tenths: 0,
            creation_time: 0,
            creation_date: 0,
            last_accessed_date: 0,
            first_cluster_high: (first_cluster.wrapping_shr(16) & 0xFFFF) as u16,
            last_modification_time: 0,
            last_modification_date: 0,
            first_cluster_low: (first_cluster & 0xFFFF) as u16,
            file_size: 0,
        };

        let dot_dot_entry = entry::DiskDirectoryEntry {
            filename: [
                b'.', b'.', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            ],
            attributes: entry::ATTRIBUTE_DIRECTORY,
            reserved: 0,
            creation_tenths: 0,
            creation_time: 0,
            creation_date: 0,
            last_accessed_date: 0,
            first_cluster_high: (self.first_cluster.wrapping_shr(16) & 0xFFFF) as u16,
            last_modification_time: 0,
            last_modification_date: 0,
            first_cluster_low: (self.first_cluster & 0xFFFF) as u16,
            file_size: 0,
        };

        let bytes_per_cluster = self.fat.lock().bytes_per_cluster();
        let mut buffer = Vec::with_capacity(bytes_per_cluster);
        let dot_entry_slice = dot_entry.to_slice();
        for i in 0..dot_entry_slice.len() {
            buffer.push(dot_entry_slice[i]);
        }
        let dot_dot_entry_slice = dot_dot_entry.to_slice();
        for i in 0..dot_dot_entry_slice.len() {
            buffer.push(dot_dot_entry_slice[i]);
        }
        for _ in buffer.len()..buffer.capacity() {
            buffer.push(0);
        }

        self.fat
            .lock()
            .write_cluster(first_cluster, buffer.as_slice())?;

        // Create entry
        let entry = entry::DirectoryEntry::new(directory_name.to_owned(), true, first_cluster, 0);

        let mut iter = self.create_iterator()?;
        iter.create(entry)?;
        iter.flush_buffer()
    }

    fn rename_file(&self, _: &str, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn rename_directory(&self, _: &str, _: &str) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn remove_file(&self, filename: &str) -> error::Result<()> {
        self.remove(filename, false)
    }

    fn remove_directory(&self, directory_name: &str) -> error::Result<()> {
        self.remove(directory_name, true)
    }

    fn update_file_metadata(
        &self,
        filename: &str,
        new_metadata: FileMetadata,
    ) -> error::Result<()> {
        let mut iter = self.create_iterator()?;

        while let Some(mut entry) = iter.next()? {
            if entry.name() == filename {
                return if entry.is_directory() {
                    Err(error::Status::IsDirectory)
                } else {
                    entry.set_file_size(new_metadata.size());
                    iter.write_metadata(entry)?;
                    iter.flush_buffer()?;
                    Ok(())
                };
            }
        }

        Err(error::Status::NoEntry)
    }
}
