use crate::{
    error,
    map::{Mappable, INVALID_ID},
};

use super::FileBox;
use alloc::sync::Arc;

pub struct FileDescriptor {
    id: isize,
    file: FileBox,
    current_offset: usize,
    read: bool,
    write: bool,
}

pub enum SeekFrom {
    Start,
    Current,
    End,
}

impl FileDescriptor {
    pub fn new(file: FileBox, read: bool, write: bool, starting_offset: usize) -> Self {
        file.lock().open();
        FileDescriptor {
            id: INVALID_ID,
            file: file,
            current_offset: starting_offset,
            read,
            write,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> error::Result<isize> {
        if !self.read {
            return Err(error::Status::WriteOnly);
        }

        let ret = self.file.lock().read(self.current_offset, buffer)?;

        if ret > 0 {
            self.current_offset += ret as usize;
        }

        Ok(ret)
    }

    pub fn write(&mut self, buffer: &[u8]) -> error::Result<isize> {
        if !self.write {
            return Err(error::Status::ReadOnly);
        }

        let ret = self.file.lock().write(self.current_offset, buffer)?;

        if ret > 0 {
            self.current_offset += ret as usize;
        }

        Ok(ret)
    }

    pub fn seek(&mut self, offset: usize, seek_from: SeekFrom) -> usize {
        match seek_from {
            SeekFrom::Start => self.current_offset = offset,
            SeekFrom::Current => self.current_offset += offset,
            SeekFrom::End => self.current_offset = self.file.lock().get_length() + offset,
        }

        self.current_offset
    }

    pub fn set_length(&self, new_length: usize) -> error::Result<()> {
        if !self.write {
            return Err(error::Status::ReadOnly);
        }

        self.file.lock().set_length(new_length)
    }
}

impl Clone for FileDescriptor {
    fn clone(&self) -> Self {
        FileDescriptor::new(
            self.file.clone(),
            self.read,
            self.write,
            self.current_offset,
        )
    }
}

impl Mappable for FileDescriptor {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        let ptr = Arc::as_ptr(&self.file);
        self.file.lock().close(ptr);
    }
}

impl From<usize> for SeekFrom {
    fn from(val: usize) -> Self {
        match val {
            1 => SeekFrom::Current,
            2 => SeekFrom::End,
            _ => SeekFrom::Start,
        }
    }
}
