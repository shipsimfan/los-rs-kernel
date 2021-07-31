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
                sub_directories.push(entry.name().to_owned());
            }
        }

        Ok(sub_directories)
    }

    fn open_file(&self, _: &str) -> error::Result<Box<dyn File>> {
        Err(error::Status::NotImplemented)
    }

    fn open_directory(&self, _: &str) -> error::Result<Box<dyn filesystem::Directory>> {
        Err(error::Status::NotImplemented)
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

    fn update_file_metadata(&self, _: &str, _: FileMetadata) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }
}
