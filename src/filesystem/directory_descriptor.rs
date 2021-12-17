use super::{DirectoryBox, DirectoryEntry};
use crate::map::{Mappable, INVALID_ID};
use alloc::{string::String, sync::Arc};

pub struct DirectoryDescriptor {
    id: isize,
    directory: DirectoryBox,
    iter: usize,
}

impl DirectoryDescriptor {
    pub fn new(directory: DirectoryBox) -> Self {
        directory.lock().open();
        DirectoryDescriptor {
            id: INVALID_ID,
            directory,
            iter: 0,
        }
    }

    pub fn get_directory(&self) -> DirectoryBox {
        self.directory.clone()
    }

    pub fn get_full_path(&self) -> String {
        self.directory.lock().construct_path_name()
    }

    pub fn next(&mut self) -> Option<DirectoryEntry> {
        let directory = self.directory.lock();

        let child = directory.get_child(self.iter);
        match child {
            Some((name, metadata)) => {
                self.iter += 1;
                Some(DirectoryEntry::new(name, metadata))
            }
            None => None,
        }
    }
}

impl Mappable for DirectoryDescriptor {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id;
    }
}

impl Drop for DirectoryDescriptor {
    fn drop(&mut self) {
        let ptr = Arc::as_ptr(&self.directory);
        self.directory.lock().close(ptr);
    }
}
