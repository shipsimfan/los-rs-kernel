use crate::{Directory, DirectoryTrait, Parent};
use alloc::{boxed::Box, string::String};
use base::{
    map::{Mappable, INVALID_ID},
    multi_owner::{Lock, Owner},
};
use process::{Mutex, ProcessTypes};

pub trait VolumeTrait: Send {
    fn set_name(&mut self, new_name: String) -> base::error::Result<()>;
}

pub struct Volume<T: ProcessTypes + 'static> {
    name: String,
    id: u64,
    root_directory: Owner<Directory<T>, Mutex<Directory<T>, T>>,
    inner: Box<dyn VolumeTrait>,
    drive_index: isize,
    mount_point: Option<usize>,
}

impl<T: ProcessTypes + 'static> Volume<T> {
    pub fn new(
        name: String,
        id: u64,
        root_directory: Box<dyn DirectoryTrait>,
        inner: Box<dyn VolumeTrait>,
    ) -> base::error::Result<Owner<Self, Mutex<Self, T>>> {
        let volume_owner: Owner<Self, Mutex<Self, T>> = Owner::new(Volume {
            name,
            id,
            root_directory: Directory::new(root_directory)?,
            inner,
            drive_index: INVALID_ID,
            mount_point: None,
        });

        unsafe {
            volume_owner.lock(|volume| {
                volume
                    .root_directory
                    .lock(|directory| directory.set_parent(Parent::Root(volume_owner.as_ref())));
            })
        };

        Ok(volume_owner)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn mount_point(&self) -> Option<usize> {
        self.mount_point
    }

    pub fn root_directory(&self) -> &Owner<Directory<T>, Mutex<Directory<T>, T>> {
        &self.root_directory
    }

    pub fn set_name(&mut self, new_name: String) -> base::error::Result<()> {
        self.inner.set_name(new_name.clone())?;

        self.name = new_name;

        Ok(())
    }

    pub unsafe fn set_mount_point(&mut self, mount_point: Option<usize>) {
        self.mount_point = mount_point;
    }
}

impl<T: ProcessTypes + 'static> Mappable for Volume<T> {
    fn id(&self) -> isize {
        self.drive_index
    }

    fn set_id(&mut self, id: isize) {
        self.drive_index = id
    }
}

impl<T: VolumeTrait, L: Lock<Data = T>> VolumeTrait for Owner<T, L> {
    fn set_name(&mut self, new_name: String) -> base::error::Result<()> {
        self.lock(|inner| inner.set_name(new_name))
    }
}
