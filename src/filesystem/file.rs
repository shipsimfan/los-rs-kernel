use super::DirectoryBox;
use crate::{error, locks::Mutex};
use alloc::{boxed::Box, sync::Arc};

pub type FileBox = Arc<Mutex<FileContainer>>;

pub trait File: Send {
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> error::Result<isize>;
    fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result<isize>;
    fn set_length(&mut self, new_length: usize) -> error::Result<()>;
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

    pub fn read(&mut self, offset: usize, buffer: &mut [u8]) -> error::Result<isize> {
        self.file.read(offset, buffer)
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result<isize> {
        let file_length = self.file.get_length();
        if offset + buffer.len() > file_length {
            self.set_length(offset + buffer.len())?;
        }

        self.file.write(offset, buffer)
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

    pub fn set_length(&mut self, new_length: usize) -> error::Result<()> {
        // Update file
        self.file.set_length(new_length)?;

        // Update directory
        let mut directory = self.parent.lock();
        let mut metadata = directory.get_file_metadata_ptr(self)?;
        metadata.set_size(new_length);
        directory.update_file_metadata(self, metadata)?;

        Ok(())
    }
}

impl FileMetadata {
    pub fn new(size: usize) -> Self {
        FileMetadata { size: size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_size(&mut self, new_size: usize) {
        self.size = new_size
    }
}
