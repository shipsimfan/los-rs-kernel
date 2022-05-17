use crate::{File, FileTrait, Metadata, Volume};
use alloc::{borrow::ToOwned, boxed::Box, format, string::String, vec::Vec};
use base::{
    error::FILESYSTEM_MODULE_NUMBER,
    multi_owner::{Owner, Reference},
};
use process::{Mutex, ProcessTypes};

mod descriptor;
mod entry;

pub use descriptor::*;
pub use entry::*;

pub trait DirectoryTrait: Send {
    fn get_children(&self) -> base::error::Result<Vec<(String, Metadata)>>; // Used to get sub files on initialization
    fn open_file(&self, filename: &str) -> base::error::Result<Box<dyn FileTrait>>;
    fn open_directory(&self, directory_name: &str) -> base::error::Result<Box<dyn DirectoryTrait>>;
    fn make_file(&self, filename: &str) -> base::error::Result<()>;
    fn make_directory(&self, directory_name: &str) -> base::error::Result<()>;
    fn rename_file(&self, old_name: &str, new_name: &str) -> base::error::Result<()>;
    fn rename_directory(&self, old_name: &str, new_name: &str) -> base::error::Result<()>;
    fn remove(&self, name: &str) -> base::error::Result<()>;
    fn update_metadata(&self, name: &str, new_metadata: Metadata) -> base::error::Result<()>;
}

pub enum Parent<T: ProcessTypes + 'static> {
    None,
    Root(Reference<Volume<T>, Mutex<Volume<T>, T>>),
    Directory(Owner<Directory<T>, Mutex<Directory<T>, T>>),
}

pub enum Child<T: ProcessTypes + 'static> {
    File(Reference<File<T>, Mutex<File<T>, T>>),
    Directory(Reference<Directory<T>, Mutex<Directory<T>, T>>),
}

pub struct Directory<T: ProcessTypes + 'static> {
    parent: Parent<T>,
    inner: Box<dyn DirectoryTrait>,
    children: Vec<(String, Metadata, Option<Child<T>>)>,
    self_reference: Option<Reference<Directory<T>, Mutex<Directory<T>, T>>>,
}

#[derive(Debug)]
enum DirectoryError {
    NotFound,
    NotDirectory,
    NotFile,
    AlreadyExists,
    EntryInUse,
    DirectoryNotEmpty,
}

impl<T: ProcessTypes + 'static> Directory<T> {
    pub fn new(
        inner: Box<dyn DirectoryTrait>,
    ) -> base::error::Result<Owner<Directory<T>, Mutex<Directory<T>, T>>> {
        let driver_children = inner.get_children()?;
        let mut children = Vec::with_capacity(driver_children.len());
        for (name, metadata) in driver_children {
            children.push((name, metadata, None));
        }

        let new_directory: Owner<Directory<_>, Mutex<Directory<T>, T>> = Owner::new(Directory {
            parent: Parent::None,
            inner,
            children,
            self_reference: None,
        });

        new_directory.lock(|directory| directory.set_self_reference(new_directory.as_ref()));

        Ok(new_directory)
    }

    pub fn parent(&self) -> &Parent<T> {
        &self.parent
    }

    pub fn construct_path_name(&self) -> String {
        match &self.parent {
            Parent::Root(volume) => {
                match volume.lock(|volume| volume.mount_point()).unwrap_or(None) {
                    Some(mount_point) => format!(":{}", mount_point),
                    None => format!(":?"),
                }
            }
            Parent::Directory(parent_lock) => parent_lock.lock(|parent| {
                format!(
                    "{}/{}",
                    parent.construct_path_name(),
                    parent.get_name(self.self_reference.as_ref().unwrap()),
                )
            }),
            _ => panic!("Directory shouldn't have null parent"),
        }
    }

    pub fn get_child(&self, idx: usize) -> Option<(&str, &Metadata)> {
        match self.children.get(idx) {
            Some((name, metadata, _)) => Some((name, metadata)),
            None => None,
        }
    }

    pub fn get_metadata(&self, name: &str) -> base::error::Result<Metadata> {
        for (sub_name, metadata, _) in &self.children {
            if sub_name == name {
                return Ok(metadata.clone());
            }
        }

        Err(Box::new(DirectoryError::NotFound))
    }

    pub fn get_file_metadata_ref(
        &self,
        reference: &Reference<File<T>, Mutex<File<T>, T>>,
    ) -> base::error::Result<Metadata> {
        for (_, metadata, child) in &self.children {
            match child {
                None => {}
                Some(child) => match child {
                    Child::File(file) => {
                        if file.compare(reference) {
                            return Ok(metadata.clone());
                        }
                    }
                    _ => {}
                },
            }
        }

        Err(Box::new(DirectoryError::NotFound))
    }

    pub fn update_file_metadata_ref(
        &mut self,
        reference: &Reference<File<T>, Mutex<File<T>, T>>,
        new_metadata: Metadata,
    ) -> base::error::Result<()> {
        for (name, metadata, child) in &mut self.children {
            match child {
                None => {}
                Some(child) => match child {
                    Child::File(file) => {
                        if file.compare(reference) {
                            self.inner.update_metadata(name, new_metadata)?;
                            return Ok(*metadata = new_metadata);
                        }
                    }
                    _ => {}
                },
            }
        }

        Err(Box::new(DirectoryError::NotFound))
    }

    pub fn open_directory(
        &mut self,
        name: &str,
    ) -> base::error::Result<Owner<Directory<T>, Mutex<Directory<T>, T>>> {
        for (child_name, metadata, child) in &mut self.children {
            if child_name != name {
                continue;
            }

            if !metadata.is_directory() {
                return Err(Box::new(DirectoryError::NotDirectory));
            }

            match child {
                Some(child) => match child {
                    Child::Directory(directory) => return Ok(directory.clone().upgrade()),
                    _ => panic!("Entry is file despite metadata holding directory"),
                },
                None => {
                    let new_directory = Directory::<T>::new(self.inner.open_directory(name)?)?;
                    new_directory.lock(|directory| unsafe {
                        directory.set_parent(Parent::Directory(
                            self.self_reference.clone().unwrap().upgrade(),
                        ))
                    });
                    *child = Some(Child::Directory(new_directory.as_ref()));
                    return Ok(new_directory);
                }
            }
        }

        Err(Box::new(DirectoryError::NotFound))
    }

    pub fn open_file(
        &mut self,
        name: &str,
    ) -> base::error::Result<Owner<File<T>, Mutex<File<T>, T>>> {
        for (child_name, metadata, child) in &mut self.children {
            if child_name != name {
                continue;
            }

            if metadata.is_directory() {
                return Err(Box::new(DirectoryError::NotFile));
            }

            match child {
                Some(child) => match child {
                    Child::File(file) => return Ok(file.clone().upgrade()),
                    _ => panic!("Entry is directory despite metadata holding file"),
                },
                None => {
                    let new_file = File::<T>::new(
                        self.inner.open_file(name)?,
                        self.self_reference.clone().unwrap().upgrade(),
                    );
                    *child = Some(Child::File(new_file.as_ref()));
                    return Ok(new_file);
                }
            }
        }

        Err(Box::new(DirectoryError::NotFound))
    }

    pub fn create_file(&mut self, filename: &str) -> base::error::Result<()> {
        // Verify file does not exist
        for (sub_name, _, _) in &self.children {
            if sub_name == filename {
                return Err(Box::new(DirectoryError::AlreadyExists));
            }
        }

        // Create the file
        self.inner.make_file(filename)?;
        self.children
            .push((filename.to_owned(), Metadata::new(0, false), None));
        Ok(())
    }

    pub fn create_directory(&mut self, directory_name: &str) -> base::error::Result<()> {
        // Verify directory does not exist
        for (sub_name, _, _) in &self.children {
            if sub_name == directory_name {
                return Err(Box::new(DirectoryError::AlreadyExists));
            }
        }

        // Create the directory
        self.inner.make_directory(directory_name)?;
        self.children
            .push((directory_name.to_owned(), Metadata::new(0, true), None));
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> base::error::Result<()> {
        let mut status: Option<Box<dyn base::error::Error>> =
            Some(Box::new(DirectoryError::NotFound));
        let directory = &self.inner;
        self.children.retain(|(sub_name, metadata, child)| -> bool {
            if sub_name == name {
                return match child {
                    Some(_) => {
                        status = Some(Box::new(DirectoryError::EntryInUse));
                        true
                    }
                    None => {
                        if metadata.is_directory() {
                            // Verify sub-directory has zero children
                            let target_directory = match directory.open_directory(name) {
                                Ok(directory) => directory,
                                Err(ret_status) => {
                                    status = Some(ret_status);
                                    return true;
                                }
                            };

                            if match target_directory.get_children() {
                                Ok(children) => children,
                                Err(ret_status) => {
                                    status = Some(ret_status);
                                    return true;
                                }
                            }
                            .len()
                                != 0
                            {
                                status = Some(Box::new(DirectoryError::DirectoryNotEmpty));
                                return true;
                            }
                        }

                        // Remove on disk
                        status = match directory.remove(name) {
                            Ok(()) => None,
                            Err(status) => Some(status),
                        };
                        false
                    }
                };
            }

            true
        });

        match status {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    pub unsafe fn set_parent(&mut self, new_parent: Parent<T>) {
        self.parent = new_parent;
    }

    fn get_name(&self, reference: &Reference<Directory<T>, Mutex<Directory<T>, T>>) -> &str {
        for (name, _, child) in &self.children {
            match child {
                None => {}
                Some(child) => match child {
                    Child::Directory(directory) => {
                        if directory.compare(reference) {
                            return name;
                        }
                    }
                    _ => {}
                },
            };
        }

        "DIRECTORY_NAME_ERROR"
    }

    fn set_self_reference(
        &mut self,
        self_reference: Reference<Directory<T>, Mutex<Directory<T>, T>>,
    ) {
        self.self_reference = Some(self_reference);
    }
}

impl base::error::Error for DirectoryError {
    fn module_number(&self) -> i32 {
        FILESYSTEM_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            DirectoryError::NotFound => base::error::Status::NotFound,
            DirectoryError::NotDirectory => base::error::Status::NotDirectory,
            DirectoryError::NotFile => base::error::Status::IsDirectory,
            DirectoryError::AlreadyExists => base::error::Status::Exists,
            DirectoryError::EntryInUse => base::error::Status::InUse,
            DirectoryError::DirectoryNotEmpty => base::error::Status::NotEmpty,
        }
    }
}

impl core::fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DirectoryError::NotFound => write!(f, "Not found"),
            DirectoryError::NotDirectory => write!(f, "Not a directory"),
            DirectoryError::NotFile => write!(f, "Not a file"),
            DirectoryError::AlreadyExists => write!(f, "Already exists"),
            DirectoryError::EntryInUse => write!(f, "Cannot remove an entry in use"),
            DirectoryError::DirectoryNotEmpty => {
                write!(f, "Cannot remove a directory that is not empty")
            }
        }
    }
}
