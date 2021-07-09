use super::{Directory, DirectoryBox};
use crate::{error, locks::Mutex};
use alloc::{boxed::Box, sync::Arc};

pub type FileBox = Arc<Mutex<FileContainer>>;

pub trait File: Send {
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> error::Result;
    fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result;
    fn set_length(&mut self, new_length: usize) -> error::Result;
}

pub struct FileContainer {
    parent: DirectoryBox,
    file: Box<dyn File>,
    references: usize,
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
}
