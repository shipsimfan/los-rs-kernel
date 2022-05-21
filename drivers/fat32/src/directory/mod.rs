use crate::{
    cluster_chain::{Cluster, ClusterChain},
    fat::FAT,
    file::File,
};
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use base::{error::FAT32_FS_DRIVER_MODULE_NUMBER, multi_owner::Owner};
use filesystem::{DirectoryTrait, FileTrait, Metadata};
use iter::DirectoryIterator;
use process::{Mutex, ProcessTypes};

mod entry;
mod iter;

#[derive(Debug)]
enum DirectoryError {
    OpeningFileAsDirectory,
    OpeningDirectoryAsFile,
    UpdatingDirectoryMetadata,
    NotFound,
    NotImplemented,
}

pub struct Directory<T: ProcessTypes + 'static> {
    fat: Owner<FAT<T>, Mutex<FAT<T>, T>>,
    cluster_chain: ClusterChain,
}

impl<T: ProcessTypes + 'static> Directory<T> {
    pub fn new(
        first_cluster: Cluster,
        fat: &mut FAT<T>,
        fat_lock: Owner<FAT<T>, Mutex<FAT<T>, T>>,
    ) -> base::error::Result<Box<dyn DirectoryTrait>> {
        let cluster_chain = ClusterChain::new(first_cluster, fat)?;

        Ok(Box::new(Directory {
            cluster_chain,
            fat: fat_lock,
        }))
    }
}

impl<T: ProcessTypes + 'static> DirectoryTrait for Directory<T> {
    fn get_children(&mut self) -> base::error::Result<Vec<(String, Metadata)>> {
        let mut children = Vec::new();

        self.fat
            .lock(|fat| -> base::error::Result<Vec<(String, Metadata)>> {
                let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;
                while let Some(entry) = iter.next()? {
                    children.push((
                        entry.name().to_owned(),
                        Metadata::new(entry.file_size(), entry.is_directory()),
                    ))
                }

                Ok(children)
            })
    }

    fn open_directory(
        &mut self,
        directory_name: &str,
    ) -> base::error::Result<Box<dyn DirectoryTrait>> {
        self.fat
            .lock(|fat| -> base::error::Result<Box<dyn DirectoryTrait>> {
                let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;

                while let Some(entry) = iter.next()? {
                    if entry.name() == directory_name {
                        return if !entry.is_directory() {
                            Err(Box::new(DirectoryError::OpeningFileAsDirectory))
                        } else {
                            Directory::new(entry.first_cluster(), fat, self.fat.clone())
                        };
                    }
                }

                Err(Box::new(DirectoryError::NotFound))
            })
    }

    fn open_file(&mut self, filename: &str) -> base::error::Result<Box<dyn FileTrait>> {
        self.fat
            .lock(|fat| -> base::error::Result<Box<dyn FileTrait>> {
                let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;

                while let Some(entry) = iter.next()? {
                    if entry.name() == filename {
                        return if entry.is_directory() {
                            Err(Box::new(DirectoryError::OpeningDirectoryAsFile))
                        } else {
                            Ok(Box::new(File::new(
                                entry.first_cluster(),
                                entry.file_size(),
                                fat,
                                self.fat.clone(),
                            )?))
                        };
                    }
                }

                Err(Box::new(DirectoryError::NotFound))
            })
    }

    fn make_directory(&mut self, directory_name: &str) -> base::error::Result<()> {
        // Allocate cluster
        let (child_first_cluster, bytes_per_cluster) = self
            .fat
            .lock(|fat| (fat.allocate_cluster(), fat.bytes_per_cluster()));
        let child_first_cluster = child_first_cluster?;
        let self_first_cluster = self.cluster_chain.first();

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
            first_cluster_high: (child_first_cluster.wrapping_shr(16) & 0xFFFF) as u16,
            last_modification_time: 0,
            last_modification_date: 0,
            first_cluster_low: (child_first_cluster & 0xFFFF) as u16,
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
            first_cluster_high: (self_first_cluster.wrapping_shr(16) & 0xFFFF) as u16,
            last_modification_time: 0,
            last_modification_date: 0,
            first_cluster_low: (self_first_cluster & 0xFFFF) as u16,
            file_size: 0,
        };

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

        // Create entry
        let entry =
            entry::DirectoryEntry::new(directory_name.to_owned(), true, child_first_cluster, 0);

        // Write it
        self.fat.lock(|fat| {
            fat.write_cluster(child_first_cluster, buffer.as_slice())?;

            let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;
            iter.create(entry)?;
            iter.flush_buffer()
        })
    }

    fn make_file(&mut self, filename: &str) -> base::error::Result<()> {
        self.fat.lock(|fat| -> base::error::Result<()> {
            // Allocate cluster
            let first_cluster = fat.allocate_cluster()?;

            // Create the entry
            let entry = entry::DirectoryEntry::new(filename.to_owned(), false, first_cluster, 0);
            let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;
            iter.create(entry)?;
            iter.flush_buffer()
        })
    }

    fn rename_file(&mut self, _: &str, _: &str) -> base::error::Result<()> {
        Err(Box::new(DirectoryError::NotImplemented))
    }

    fn rename_directory(&mut self, _: &str, _: &str) -> base::error::Result<()> {
        Err(Box::new(DirectoryError::NotImplemented))
    }

    fn remove(&mut self, name: &str) -> base::error::Result<()> {
        self.fat.lock(|fat| -> base::error::Result<()> {
            let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;
            let mut cluster_chain_to_free = None;
            while let Some(entry) = iter.next()? {
                if entry.name() == name {
                    // Free cluster chain
                    cluster_chain_to_free = Some(entry.first_cluster());

                    // Remove entry
                    iter.remove()?;
                    iter.flush_buffer()?;

                    break;
                }
            }

            drop(iter);

            match cluster_chain_to_free {
                Some(first_cluster) => fat.free_cluster_chain(first_cluster),
                None => Err(Box::new(DirectoryError::NotFound)),
            }
        })
    }

    fn update_metadata(&mut self, name: &str, new_metadata: Metadata) -> base::error::Result<()> {
        self.fat.lock(|fat| -> base::error::Result<()> {
            let mut iter = DirectoryIterator::new(&mut self.cluster_chain, fat)?;

            while let Some(mut entry) = iter.next()? {
                if entry.name() == name {
                    return if entry.is_directory() {
                        Err(Box::new(DirectoryError::UpdatingDirectoryMetadata))
                    } else {
                        entry.set_file_size(new_metadata.size());
                        iter.write_metadata(entry)?;
                        iter.flush_buffer()?;
                        Ok(())
                    };
                }
            }

            Err(Box::new(DirectoryError::NotFound))
        })
    }
}

impl base::error::Error for DirectoryError {
    fn module_number(&self) -> i32 {
        FAT32_FS_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            DirectoryError::NotFound => base::error::Status::NotFound,
            DirectoryError::OpeningFileAsDirectory => base::error::Status::IsFile,
            DirectoryError::OpeningDirectoryAsFile => base::error::Status::IsDirectory,
            DirectoryError::NotImplemented => base::error::Status::NotImplemented,
            DirectoryError::UpdatingDirectoryMetadata => base::error::Status::IsDirectory,
        }
    }
}

impl core::fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DirectoryError::NotFound => write!(f, "Not found"),
            DirectoryError::OpeningFileAsDirectory => write!(f, "Opening file as directory"),
            DirectoryError::OpeningDirectoryAsFile => write!(f, "Opening directory as file"),
            DirectoryError::NotImplemented => write!(f, "Not implemented on FAT32"),
            DirectoryError::UpdatingDirectoryMetadata => {
                write!(f, "Cannot update directory metadata")
            }
        }
    }
}
