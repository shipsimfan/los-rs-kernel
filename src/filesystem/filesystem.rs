use super::{Directory, DirectoryOwner, DirectoryReference, Parent};
use crate::{device::DeviceBox, error, map::*};
use alloc::{boxed::Box, string::String};

pub type DetectFilesystemFunction =
    fn(drive: DeviceBox, start: usize, size: usize) -> error::Result<Option<FilesystemStarter>>;

pub struct FilesystemStarter {
    volume_name: String,
    root_directory: Box<dyn Directory>,
}

pub struct Filesystem {
    number: isize,
    _volume_name: String,
    root_directory: DirectoryReference,
}

impl Filesystem {
    pub fn new(filesystem_starter: FilesystemStarter) -> error::Result<Self> {
        Ok(Filesystem {
            number: INVALID_ID,
            root_directory: DirectoryReference::new(DirectoryOwner::new(
                filesystem_starter.root_directory,
                Parent::Root(INVALID_ID),
            )?),
            _volume_name: filesystem_starter.volume_name,
        })
    }

    pub fn root_directory(&self) -> &DirectoryReference {
        &self.root_directory
    }
}

impl Mappable for Filesystem {
    fn id(&self) -> isize {
        self.number
    }

    fn set_id(&mut self, id: isize) {
        self.number = id;
        self.root_directory.lock().set_drive_number(id);
    }
}

impl FilesystemStarter {
    pub fn new(root_directory: Box<dyn Directory>, volume_name: String) -> Self {
        FilesystemStarter {
            root_directory: root_directory,
            volume_name: volume_name,
        }
    }
}
