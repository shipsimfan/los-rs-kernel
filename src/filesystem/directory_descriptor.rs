use super::DirectoryBox;
use alloc::{string::String, sync::Arc};

pub struct DirectoryDescriptor {
    directory: DirectoryBox,
}

impl DirectoryDescriptor {
    pub fn new(directory: DirectoryBox) -> Self {
        directory.lock().open();
        DirectoryDescriptor { directory }
    }

    pub fn get_directory(&self) -> DirectoryBox {
        self.directory.clone()
    }

    pub fn get_full_path(&self) -> String {
        self.directory.lock().construct_path_name()
    }
}

impl Drop for DirectoryDescriptor {
    fn drop(&mut self) {
        let ptr = Arc::as_ptr(&self.directory);
        self.directory.lock().close(ptr);
    }
}
