use crate::Directory;
use alloc::boxed::Box;
use base::multi_owner::{Owner, Reference};
use process::{Mutex, ProcessTypes};

mod descriptor;

pub use descriptor::*;

pub trait FileTrait: Send {
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> base::error::Result<isize>;
    fn write(&mut self, offset: usize, buffer: &[u8]) -> base::error::Result<isize>;
    fn set_length(&mut self, new_length: usize) -> base::error::Result<()>;
    fn get_length(&self) -> usize;
}

pub struct File<T: ProcessTypes + 'static> {
    parent: Owner<Directory<T>, Mutex<Directory<T>, T>>,
    inner: Box<dyn FileTrait>,
    self_reference: Option<Reference<File<T>, Mutex<File<T>, T>>>,
}

impl<T: ProcessTypes + 'static> File<T> {
    pub fn new(
        inner: Box<dyn FileTrait>,
        parent: Owner<Directory<T>, Mutex<Directory<T>, T>>,
    ) -> Owner<Self, Mutex<Self, T>> {
        let file_owner: Owner<Self, Mutex<Self, T>> = Owner::new(File {
            parent,
            inner,
            self_reference: None,
        });

        file_owner.lock(|file| file.set_self_reference(file_owner.as_ref()));

        file_owner
    }

    pub fn read(&mut self, offset: usize, buffer: &mut [u8]) -> base::error::Result<isize> {
        self.inner.read(offset, buffer)
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) -> base::error::Result<isize> {
        let file_length = self.inner.get_length();
        if offset + buffer.len() > file_length {
            self.set_length(offset + buffer.len())?;
        }

        self.inner.write(offset, buffer)
    }

    pub fn get_length(&self) -> usize {
        self.inner.get_length()
    }

    pub fn set_length(&mut self, new_length: usize) -> base::error::Result<()> {
        // Update file
        self.inner.set_length(new_length)?;

        // Update directory
        self.parent.lock(|directory| {
            match directory.get_file_metadata_ref(self.self_reference.as_ref().unwrap()) {
                Ok(mut metadata) => {
                    metadata.set_size(new_length);
                    directory
                        .update_file_metadata_ref(self.self_reference.as_ref().unwrap(), metadata)
                }
                Err(error) => Err(error),
            }
        })
    }

    fn set_self_reference(&mut self, self_reference: Reference<File<T>, Mutex<File<T>, T>>) {
        self.self_reference = Some(self_reference)
    }
}

impl<T: ProcessTypes + 'static> Drop for File<T> {
    fn drop(&mut self) {
        self.parent
            .lock(|parent| unsafe { parent.close_file(self.self_reference.as_ref().unwrap()) });
    }
}
