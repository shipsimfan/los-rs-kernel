use super::{File, FileBox, FileContainer, FileMetadata};
use crate::{error, locks::Mutex};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};

pub type DirectoryBox = Arc<Mutex<DirectoryContainer>>;

pub trait Directory: Send {
    fn get_sub_files(&self) -> Result<Vec<(String, FileMetadata)>, error::Status>; // Used to get sub files on initialization
    fn get_sub_directories(&self) -> Result<Vec<String>, error::Status>; // Used to get sub directories on initialization
    fn open_file(&self, filename: &str) -> Result<Box<dyn File>, error::Status>;
    fn open_directory(&self, directory_name: &str) -> Result<Box<dyn Directory>, error::Status>;
    fn make_file(&self, filename: &str) -> error::Result;
    fn make_directory(&self, directory_name: &str) -> error::Result;
    fn rename_file(&self, old_name: &str, new_name: &str) -> error::Result;
    fn rename_directory(&self, old_name: &str, new_name: &str) -> error::Result;
    fn remove_file(&self, filename: &str) -> error::Result;
    fn remove_directory(&self, directory_name: &str) -> error::Result;
}

pub struct DirectoryContainer {
    parent: Option<DirectoryBox>, // None for root directories
    directory: Box<dyn Directory>,
    sub_files: Vec<(String, FileMetadata, Option<FileBox>)>,
    sub_directories: Vec<(String, Option<DirectoryBox>)>,
    references: usize,
}

impl DirectoryContainer {
    pub fn new(
        directory: Box<dyn Directory>,
        parent_directory: Option<DirectoryBox>,
    ) -> Result<Self, error::Status> {
        let sub_file_names = directory.get_sub_files()?;
        let mut sub_files = Vec::with_capacity(sub_file_names.len());
        for (sub_file_name, sub_file_metadata) in sub_file_names {
            sub_files.push((sub_file_name, sub_file_metadata, None));
        }

        let sub_directory_names = directory.get_sub_directories()?;
        let mut sub_directories = Vec::with_capacity(sub_directory_names.len());
        for sub_directory_name in sub_directory_names {
            sub_directories.push((sub_directory_name, None));
        }

        Ok(DirectoryContainer {
            parent: parent_directory,
            directory: directory,
            sub_files: sub_files,
            sub_directories: sub_directories,
            references: 0,
        })
    }

    pub fn get_file_metadata(&self, name: &str) -> Result<FileMetadata, error::Status> {
        for (sub_name, sub_metadata, _) in &self.sub_files {
            if sub_name == name {
                return Ok(sub_metadata.clone());
            }
        }

        Err(error::Status::NotFound)
    }

    pub fn open_directory(
        &mut self,
        name: &str,
        self_box: &DirectoryBox,
    ) -> Result<DirectoryBox, error::Status> {
        for (sub_name, sub_dir) in &mut self.sub_directories {
            if sub_name != name {
                continue;
            }

            match sub_dir {
                Some(sub_dir) => return Ok(sub_dir.clone()),
                None => {
                    let new_directory = self.directory.open_directory(name)?;
                    let new_directory = Arc::new(Mutex::new(DirectoryContainer::new(
                        new_directory,
                        Some(self_box.clone()),
                    )?));
                    *sub_dir = Some(new_directory.clone());
                    self.references += 1;
                    return Ok(new_directory);
                }
            }
        }

        Err(error::Status::NotFound)
    }

    pub fn open_file(
        &mut self,
        name: &str,
        self_box: &DirectoryBox,
    ) -> Result<FileBox, error::Status> {
        for (sub_name, _, sub_file) in &mut self.sub_files {
            if sub_name != name {
                continue;
            }

            match sub_file {
                Some(sub_file) => return Ok(sub_file.clone()),
                None => {
                    let new_file = self.directory.open_file(name)?;
                    let new_file =
                        Arc::new(Mutex::new(FileContainer::new(new_file, self_box.clone())));
                    *sub_file = Some(new_file.clone());
                    self.references += 1;
                    return Ok(new_file);
                }
            }
        }

        Err(error::Status::NotFound)
    }

    pub fn close_directory(
        &mut self,
        directory_arc_ptr: *const Mutex<DirectoryContainer>,
        arc_ptr: *const Mutex<DirectoryContainer>,
    ) {
        for (_, sub_directory) in &mut self.sub_directories {
            let test_ptr = match sub_directory {
                None => continue,
                Some(directory) => Arc::as_ptr(directory),
            };

            if test_ptr == directory_arc_ptr {
                *sub_directory = None;
                self.references -= 1;
                if self.references == 0 {
                    match &self.parent {
                        None => {}
                        Some(parent) => {
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
        for (_, _, sub_file) in &mut self.sub_files {
            let test_ptr = match sub_file {
                None => continue,
                Some(file) => Arc::as_ptr(file),
            };

            if test_ptr == file_arc_ptr {
                *sub_file = None;
                self.references -= 1;
                if self.references == 0 {
                    match &self.parent {
                        None => {}
                        Some(parent) => {
                            let ptr = Arc::as_ptr(parent);
                            parent.lock().close_directory(arc_ptr, ptr);
                        }
                    }
                }

                break;
            }
        }
    }
}
