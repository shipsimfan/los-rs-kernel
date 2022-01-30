use super::File;
use crate::{filesystem::directory::DirectoryReference, locks::Mutex};
use alloc::boxed::Box;

pub struct FileOwner {
    parent: DirectoryReference,
    file: Box<dyn File>,
    references: usize,
}

impl FileOwner {
    pub fn new(file: Box<dyn File>, parent: DirectoryReference) -> Self {
        FileOwner {
            file: file,
            parent: parent,
            references: 0,
        }
    }

    pub fn open(&mut self) {
        self.references += 1;
    }

    pub fn read(&mut self, offset: usize, buffer: &mut [u8]) -> crate::error::Result<isize> {
        self.file.read(offset, buffer)
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) -> crate::error::Result<isize> {
        let file_length = self.file.get_length();
        if offset + buffer.len() > file_length {
            self.set_length(offset + buffer.len())?;
        }

        self.file.write(offset, buffer)
    }

    pub fn close(&mut self, arc_ptr: *const Mutex<FileOwner>) {
        self.references -= 1;
        if self.references == 0 {
            let ptr = self.parent.as_ptr();
            self.parent.lock().close_file(arc_ptr, ptr);
        }
    }

    pub fn get_length(&self) -> usize {
        self.file.get_length()
    }

    pub fn set_length(&mut self, new_length: usize) -> crate::error::Result<()> {
        // Update file
        self.file.set_length(new_length)?;

        // Update directory
        let mut directory = self.parent.lock();
        let mut metadata = directory.get_metadata_ptr(self as *const _ as *const _)?;
        metadata.set_size(new_length);
        directory.update_metadata(self, metadata)?;

        Ok(())
    }
}
