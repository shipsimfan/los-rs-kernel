use super::{Directory, DirectoryBox, DirectoryContainer, ParentDirectory};
use crate::{device::DeviceBox, error, locks::Mutex, map::*};
use alloc::{boxed::Box, string::String, sync::Arc};

pub type DetectFilesystemFunction = fn(
    drive: DeviceBox,
    start: usize,
    size: usize,
) -> Result<Option<FilesystemStarter>, error::Status>;

pub struct FilesystemStarter {
    volume_name: String,
    root_directory: Box<dyn Directory>,
}

pub struct Filesystem {
    number: usize,
    _volume_name: String,
    root_directory: DirectoryBox,
}

impl Filesystem {
    pub fn new(filesystem_starter: FilesystemStarter) -> Result<Self, error::Status> {
        Ok(Filesystem {
            number: INVALID_ID,
            root_directory: Arc::new(Mutex::new(DirectoryContainer::new(
                filesystem_starter.root_directory,
                ParentDirectory::Root(INVALID_ID),
            )?)),
            _volume_name: filesystem_starter.volume_name,
        })
    }

    pub fn root_directory(&self) -> &DirectoryBox {
        &self.root_directory
    }
}

impl Mappable for Filesystem {
    fn id(&self) -> usize {
        self.number
    }

    fn set_id(&mut self, id: usize) {
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
