use super::{File, FileBox, FileContainer, Metadata};
use crate::{error, locks::Mutex};
use alloc::{borrow::ToOwned, boxed::Box, format, string::String, sync::Arc, vec::Vec};
use core::ffi::c_void;

pub type DirectoryBox = Arc<Mutex<DirectoryContainer>>;

pub trait Directory: Send {
    fn get_children(&self) -> error::Result<Vec<(String, Metadata)>>; // Used to get sub files on initialization
    fn open_file(&self, filename: &str) -> error::Result<Box<dyn File>>;
    fn open_directory(&self, directory_name: &str) -> error::Result<Box<dyn Directory>>;
    fn make_file(&self, filename: &str) -> error::Result<()>;
    fn make_directory(&self, directory_name: &str) -> error::Result<()>;
    fn rename_file(&self, old_name: &str, new_name: &str) -> error::Result<()>;
    fn rename_directory(&self, old_name: &str, new_name: &str) -> error::Result<()>;
    fn remove(&self, name: &str) -> error::Result<()>;
    fn update_metadata(&self, name: &str, new_metadata: Metadata) -> error::Result<()>;
}

pub enum Parent {
    Root(isize),
    Other(DirectoryBox),
}

pub enum Child {
    File(FileBox),
    Directory(DirectoryBox),
}

pub struct DirectoryContainer {
    parent: Parent,
    directory: Box<dyn Directory>,
    children: Vec<(String, Metadata, Option<Child>)>,
    references: usize,
}

impl DirectoryContainer {
    pub fn new(directory: Box<dyn Directory>, parent: Parent) -> error::Result<Self> {
        let driver_children = directory.get_children()?;
        let mut children = Vec::with_capacity(driver_children.len());
        for (name, metadata) in driver_children {
            children.push((name, metadata, None));
        }

        Ok(DirectoryContainer {
            parent,
            directory,
            children,
            references: 0,
        })
    }

    pub fn get_parent(&self) -> Option<DirectoryBox> {
        match &self.parent {
            Parent::Root(_) => None,
            Parent::Other(parent) => Some(parent.clone()),
        }
    }

    pub fn get_metadata(&self, name: &str) -> error::Result<Metadata> {
        for (sub_name, metadata, _) in &self.children {
            if sub_name == name {
                return Ok(metadata.clone());
            }
        }

        Err(error::Status::NoEntry)
    }

    pub fn get_metadata_ptr(&self, ptr: *const c_void) -> error::Result<Metadata> {
        for (_, metadata, child) in &self.children {
            match child {
                None => {}
                Some(child) => match child {
                    Child::File(file) => {
                        if file.matching_data(ptr as *const _) {
                            return Ok(metadata.clone());
                        }
                    }
                    Child::Directory(dir) => {
                        if dir.matching_data(ptr as *const _) {
                            return Ok(metadata.clone());
                        }
                    }
                },
            }
        }

        Err(error::Status::NoEntry)
    }

    pub fn update_metadata(
        &mut self,
        ptr: *const FileContainer,
        new_metadata: Metadata,
    ) -> error::Result<()> {
        for (name, metadata, child) in &mut self.children {
            match child {
                None => continue,
                Some(child) => match child {
                    Child::File(file) => {
                        if file.matching_data(ptr as *const _) {
                            self.directory.update_metadata(name, new_metadata.clone())?;
                            *metadata = new_metadata;
                            return Ok(());
                        }
                    }
                    Child::Directory(dir) => {
                        if dir.matching_data(ptr as *const _) {
                            self.directory.update_metadata(name, new_metadata.clone())?;
                            *metadata = new_metadata;
                            return Ok(());
                        }
                    }
                },
            };
        }

        Err(error::Status::NoEntry)
    }

    pub fn set_drive_number(&mut self, number: isize) {
        self.parent = Parent::Root(number);
    }

    pub fn get_name(&self, ptr: *const c_void) -> &str {
        for (name, _, child) in &self.children {
            match child {
                None => {}
                Some(child) => match child {
                    Child::File(file) => {
                        if file.matching_data(ptr as *const _) {
                            return name;
                        }
                    }
                    Child::Directory(dir) => {
                        if dir.matching_data(ptr as *const _) {
                            return name;
                        }
                    }
                },
            };
        }

        "DIRECTORY_NAME_ERROR"
    }

    pub fn construct_path_name(&self) -> String {
        match &self.parent {
            Parent::Root(fs_number) => format!(":{}", fs_number),
            Parent::Other(parent_lock) => {
                let parent = parent_lock.lock();
                format!(
                    "{}/{}",
                    parent.construct_path_name(),
                    parent.get_name(self as *const _ as *const _)
                )
            }
        }
    }

    pub fn open_directory(
        &mut self,
        name: &str,
        self_box: &DirectoryBox,
    ) -> error::Result<DirectoryBox> {
        for (sub_name, metadata, child) in &mut self.children {
            if sub_name != name {
                continue;
            }

            if !metadata.is_directory() {
                return Err(error::Status::NotDirectory);
            }

            match child {
                Some(child) => match child {
                    Child::Directory(dir) => return Ok(dir.clone()),
                    Child::File(_) => panic!("Entry is file depsite metadata saying directory"),
                },
                None => {
                    let new_directory = self.directory.open_directory(name)?;
                    let new_directory = Arc::new(Mutex::new(DirectoryContainer::new(
                        new_directory,
                        Parent::Other(self_box.clone()),
                    )?));
                    *child = Some(Child::Directory(new_directory.clone()));
                    self.references += 1;
                    return Ok(new_directory);
                }
            }
        }

        Err(error::Status::NoEntry)
    }

    pub fn open_file(&mut self, name: &str, self_box: &DirectoryBox) -> error::Result<FileBox> {
        for (sub_name, metadata, child) in &mut self.children {
            if sub_name != name {
                continue;
            }

            if metadata.is_directory() {
                return Err(crate::error::Status::IsDirectory);
            }

            match child {
                Some(child) => match child {
                    Child::File(file) => return Ok(file.clone()),
                    Child::Directory(_) => {
                        panic!("Entry is directory despite metadata saying file")
                    }
                },
                None => {
                    let new_file = self.directory.open_file(name)?;
                    let new_file =
                        Arc::new(Mutex::new(FileContainer::new(new_file, self_box.clone())));
                    *child = Some(Child::File(new_file.clone()));
                    self.references += 1;
                    return Ok(new_file);
                }
            }
        }

        Err(error::Status::NoEntry)
    }

    pub fn open(&mut self) {
        self.references += 1;
    }

    pub fn close_directory(
        &mut self,
        directory_arc_ptr: *const Mutex<DirectoryContainer>,
        arc_ptr: *const Mutex<DirectoryContainer>,
    ) {
        for (_, metadata, child) in &mut self.children {
            if !metadata.is_directory() {
                continue;
            }

            let test_ptr = match child {
                None => continue,
                Some(child) => Arc::as_ptr(match child {
                    Child::Directory(dir) => dir,
                    Child::File(_) => panic!("Entry is file despite metadata saying directory"),
                }),
            };

            if test_ptr == directory_arc_ptr {
                *child = None;
                self.references -= 1;
                if self.references == 0 {
                    match &self.parent {
                        Parent::Root(_) => {}
                        Parent::Other(parent) => {
                            let ptr = Arc::as_ptr(parent);
                            parent.lock().close_directory(arc_ptr, ptr)
                        }
                    }
                }
            }
        }
    }

    pub fn close_file(
        &mut self,
        file_arc_ptr: *const Mutex<FileContainer>,
        arc_ptr: *const Mutex<DirectoryContainer>,
    ) {
        for (_, metadata, child) in &mut self.children {
            if metadata.is_directory() {
                continue;
            }

            let test_ptr = match child {
                None => continue,
                Some(child) => Arc::as_ptr(match child {
                    Child::File(file) => file,
                    Child::Directory(_) => {
                        panic!("Entry is directory despite metadata saying file")
                    }
                }),
            };

            if test_ptr == file_arc_ptr {
                *child = None;
                self.references -= 1;
                if self.references == 0 {
                    match &self.parent {
                        Parent::Root(_) => {}
                        Parent::Other(parent) => {
                            let ptr = Arc::as_ptr(parent);
                            parent.lock().close_directory(arc_ptr, ptr);
                        }
                    }
                }

                break;
            }
        }
    }

    pub fn close(&mut self, arc_ptr: *const Mutex<DirectoryContainer>) {
        self.references -= 1;
        if self.references == 0 {
            match &self.parent {
                Parent::Root(_) => {}
                Parent::Other(parent) => {
                    let ptr = Arc::as_ptr(parent);
                    parent.lock().close_directory(arc_ptr, ptr);
                }
            }
        }
    }

    pub fn get_child(&self, idx: usize) -> Option<(&str, &Metadata)> {
        match self.children.get(idx) {
            Some((name, metadata, _)) => Some((name, metadata)),
            None => None,
        }
    }

    pub fn remove(&mut self, name: &str) -> error::Result<()> {
        let mut status = error::Status::NoEntry;
        let directory = &self.directory;
        self.children.retain(|(sub_name, metadata, child)| -> bool {
            if sub_name == name {
                return match child {
                    Some(_) => {
                        status = error::Status::InUse;
                        true
                    }
                    None => {
                        if metadata.is_directory() {
                            // Verify sub-directory has zero children
                            let target_directory = match directory.open_directory(name) {
                                Ok(directory) => directory,
                                Err(ret_status) => {
                                    status = ret_status;
                                    return true;
                                }
                            };

                            if match target_directory.get_children() {
                                Ok(children) => children,
                                Err(ret_status) => {
                                    status = ret_status;
                                    return true;
                                }
                            }
                            .len()
                                != 0
                            {
                                status = error::Status::NotEmpty;
                                return true;
                            }
                        }

                        // Remove on disk
                        status = match directory.remove(name) {
                            Ok(()) => error::Status::Success,
                            Err(status) => status,
                        };
                        false
                    }
                };
            }

            true
        });

        if status == error::Status::Success {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn create_file(&mut self, filename: &str) -> error::Result<()> {
        // Verify file does not exist
        for (sub_name, _, _) in &self.children {
            if sub_name == filename {
                return Err(error::Status::Exists);
            }
        }

        // Create the file
        self.directory.make_file(filename)?;
        self.children
            .push((filename.to_owned(), Metadata::new(0, false), None));
        Ok(())
    }

    pub fn create_directory(&mut self, directory_name: &str) -> error::Result<()> {
        // Verify directory does not exist
        for (sub_name, _, _) in &self.children {
            if sub_name == directory_name {
                return Err(error::Status::Exists);
            }
        }

        // Create the directory
        self.directory.make_directory(directory_name)?;
        self.children
            .push((directory_name.to_owned(), Metadata::new(0, true), None));
        Ok(())
    }
}
