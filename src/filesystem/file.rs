use super::DirectoryBox;
use crate::{error, locks::Mutex};
use alloc::{boxed::Box, sync::Arc};

pub type FileBox = Arc<Mutex<FileContainer>>;

pub trait File: Send {
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> Result<usize, error::Status>;
    fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result;
    fn set_length(&mut self, new_length: usize) -> error::Result;
    fn get_length(&self) -> usize;
}

pub struct FileContainer {
    parent: DirectoryBox,
    file: Box<dyn File>,
    references: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct FileMetadata {
    size: usize,
}

impl FileContainer {
    pub fn new(file: Box<dyn File>, parent: DirectoryBox) -> Self {
        FileContainer {
            file: file,
            parent: parent,
            references: 0,
        }
    }

    pub fn open(&mut self) {
        self.references += 1;
    }

    pub fn read(&mut self, offset: usize, buffer: &mut [u8]) -> Result<usize, error::Status> {
        self.file.read(offset, buffer)
    }

    pub fn close(&mut self, arc_ptr: *const Mutex<FileContainer>) {
        self.references -= 1;
        if self.references == 0 {
            let ptr = Arc::as_ptr(&self.parent);
            self.parent.lock().close_file(arc_ptr, ptr);
        }
    }

    pub fn get_length(&self) -> usize {
        self.file.get_length()
    }
}

impl FileMetadata {
    pub fn new(size: usize) -> Self {
        FileMetadata { size: size }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
