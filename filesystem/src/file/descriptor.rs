use crate::File;
use alloc::boxed::Box;
use base::{error::FILESYSTEM_MODULE_NUMBER, multi_owner::Owner};
use process::{Mutex, ProcessTypes};

pub struct FileDescriptor<T: ProcessTypes + 'static> {
    inner: Owner<File<T>, Mutex<File<T>, T>>,
    current_offset: usize,
    read: bool,
    write: bool,
}

pub enum SeekFrom {
    Start,
    Current,
    End,
}

#[derive(Debug)]
enum FileDescriptorError {
    ReadingWriteOnly,
    WritingReadOnly,
}

impl<T: ProcessTypes + 'static> FileDescriptor<T> {
    pub fn new(
        inner: Owner<File<T>, Mutex<File<T>, T>>,
        read: bool,
        write: bool,
        starting_offset: usize,
    ) -> Self {
        FileDescriptor {
            inner,
            current_offset: starting_offset,
            read,
            write,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> base::error::Result<isize> {
        if !self.read {
            return Err(Box::new(FileDescriptorError::ReadingWriteOnly));
        }

        let ret = self
            .inner
            .lock(|file| file.read(self.current_offset, buffer))?;

        if ret > 0 {
            self.current_offset += ret as usize;
        }

        Ok(ret)
    }

    pub fn write(&mut self, buffer: &[u8]) -> base::error::Result<isize> {
        if !self.write {
            return Err(Box::new(FileDescriptorError::WritingReadOnly));
        }

        let ret = self
            .inner
            .lock(|file| file.write(self.current_offset, buffer))?;

        if ret > 0 {
            self.current_offset += ret as usize;
        }

        Ok(ret)
    }

    pub fn seek(&mut self, offset: usize, seek_from: SeekFrom) -> usize {
        match seek_from {
            SeekFrom::Start => self.current_offset = offset,
            SeekFrom::Current => self.current_offset += offset,
            SeekFrom::End => {
                self.current_offset = self.inner.lock(|file| file.get_length() + offset)
            }
        }

        self.current_offset
    }

    pub fn set_length(&self, new_length: usize) -> base::error::Result<()> {
        if !self.write {
            return Err(Box::new(FileDescriptorError::WritingReadOnly));
        }

        self.inner.lock(|file| file.set_length(new_length))
    }

    pub fn tell(&self) -> usize {
        self.current_offset
    }
}

impl<T: ProcessTypes + 'static> Clone for FileDescriptor<T> {
    fn clone(&self) -> Self {
        FileDescriptor {
            inner: self.inner.clone(),
            current_offset: 0,
            read: self.read,
            write: self.write,
        }
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

impl base::error::Error for FileDescriptorError {
    fn module_number(&self) -> i32 {
        FILESYSTEM_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            FileDescriptorError::WritingReadOnly => base::error::Status::ReadOnly,
            FileDescriptorError::ReadingWriteOnly => base::error::Status::WriteOnly,
        }
    }
}

impl core::fmt::Display for FileDescriptorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FileDescriptorError::WritingReadOnly => {
                write!(f, "Writing to a read-only file descriptor")
            }
            FileDescriptorError::ReadingWriteOnly => {
                write!(f, "Reading to a write-only file descriptor")
            }
        }
    }
}
