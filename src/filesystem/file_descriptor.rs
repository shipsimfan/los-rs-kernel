use super::FileBox;

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
}
