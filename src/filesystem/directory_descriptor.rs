use crate::map::{Mappable, INVALID_ID};

use super::{DirectoryBox, DirectoryEntry};
use alloc::{string::String, sync::Arc};

enum IterStage {
    File,
    Directory,
}

pub struct DirectoryDescriptor {
    id: usize,
    directory: DirectoryBox,
    stage: IterStage,
    iter: usize,
}

impl DirectoryDescriptor {
    pub fn new(directory: DirectoryBox) -> Self {
        directory.lock().open();
        DirectoryDescriptor {
            id: INVALID_ID,
            directory,
            stage: IterStage::Directory,
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
        match self.stage {
            IterStage::Directory => {
                let sub_directories = directory.get_sub_directories();
                if self.iter >= sub_directories.len() {
                    self.stage = IterStage::File;
                    self.iter = 0;
                    drop(sub_directories);
                    drop(directory);
                    self.next()
                } else {
                    let (dir_name, _) = &sub_directories[self.iter];
                    self.iter += 1;
                    Some(DirectoryEntry::from_directory(dir_name))
                }
            }
            IterStage::File => {
                let sub_files = directory.get_sub_files();
                if self.iter >= sub_files.len() {
                    None
                } else {
                    let (file_name, metadata, _) = &sub_files[self.iter];
                    self.iter += 1;
                    Some(DirectoryEntry::from_file(file_name, metadata))
                }
            }
        }
    }
}

impl Mappable for DirectoryDescriptor {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Drop for DirectoryDescriptor {
    fn drop(&mut self) {
        let ptr = Arc::as_ptr(&self.directory);
        self.directory.lock().close(ptr);
    }
}
