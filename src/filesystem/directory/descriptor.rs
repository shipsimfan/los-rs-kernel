use super::{DirectoryReference, Entry};
use crate::map::{Mappable, INVALID_ID};
use alloc::string::String;

#[derive(Clone)]
pub struct Descriptor {
    id: isize,
    directory: DirectoryReference,
    iter: usize,
}

impl Descriptor {
    pub fn new(directory: DirectoryReference) -> Self {
        directory.lock().open();
        Descriptor {
            id: INVALID_ID,
            directory,
            iter: 0,
        }
    }

    pub fn get_directory(&self) -> DirectoryReference {
        self.directory.clone()
    }

    pub fn get_full_path(&self) -> String {
        self.directory.lock().construct_path_name()
    }

    pub fn next(&mut self) -> Option<Entry> {
        let directory = self.directory.lock();

        let child = directory.get_child(self.iter);
        match child {
            Some((name, metadata)) => {
                self.iter += 1;
                Some(Entry::new(name, metadata))
            }
            None => None,
        }
    }
}

impl Mappable for Descriptor {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id;
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        let ptr = self.directory.as_ptr();
        self.directory.lock().close(ptr);
    }
}
