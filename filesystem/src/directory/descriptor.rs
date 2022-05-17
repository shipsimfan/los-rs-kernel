use crate::{Directory, DirectoryEntry};
use alloc::{borrow::ToOwned, string::String};
use base::{
    map::{Mappable, INVALID_ID},
    multi_owner::Owner,
};
use process::{Mutex, ProcessTypes};

pub struct DirectoryDescriptor<T: ProcessTypes + 'static> {
    id: isize,
    directory: Owner<Directory<T>, Mutex<Directory<T>, T>>,
    iter: usize,
}

impl<T: ProcessTypes + 'static> DirectoryDescriptor<T> {
    pub fn new(directory: Owner<Directory<T>, Mutex<Directory<T>, T>>) -> Self {
        DirectoryDescriptor {
            id: INVALID_ID,
            directory,
            iter: 0,
        }
    }

    pub fn get_directory(&self) -> &Owner<Directory<T>, Mutex<Directory<T>, T>> {
        &self.directory
    }

    pub fn get_full_path(&self) -> String {
        self.directory
            .lock(|directory| directory.construct_path_name())
    }

    pub fn next(&mut self) -> Option<DirectoryEntry> {
        let child = self.directory.lock(|directory| {
            directory
                .get_child(self.iter)
                .map(|(name, metadata)| (name.to_owned(), metadata.clone()))
        });

        match child {
            Some((name, metadata)) => {
                self.iter += 1;
                Some(DirectoryEntry::new(name, metadata))
            }
            None => None,
        }
    }
}

impl<T: ProcessTypes + 'static> Mappable for DirectoryDescriptor<T> {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id;
    }
}
