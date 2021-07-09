use crate::error;

use super::FileBox;
use alloc::sync::Arc;

pub struct FileDescriptor {
    file: FileBox,
    current_offset: usize,
}

impl FileDescriptor {
    pub fn new(file: FileBox) -> Self {
        file.lock().open();
        FileDescriptor {
            file: file,
            current_offset: 0,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, error::Status> {
        let ret = self.file.lock().read(self.current_offset, buffer)?;

        self.current_offset += ret;

        Ok(ret)
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        let ptr = Arc::as_ptr(&self.file);
        self.file.lock().close(ptr);
    }
}
