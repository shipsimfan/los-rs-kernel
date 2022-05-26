use crate::Device;

use self::node::Children;
use alloc::{borrow::ToOwned, boxed::Box, collections::VecDeque, string::String};
use base::{
    error::DEVICE_MODULE_NUMBER,
    multi_owner::{Owner, Reference},
};
use process::{Mutex, ProcessTypes};

mod node;

pub struct Tree<T: ProcessTypes + 'static> {
    root_devices: Children<T>,
}

#[derive(Debug, Clone, Copy)]
enum DeviceError {
    InvalidPath,
    NotFound,
    AlreadyExists,
}

fn parse_path(path: &str) -> base::error::Result<VecDeque<&str>> {
    // Verify first character
    match path.chars().next() {
        Some(c) => match c {
            '\\' | '/' => {}
            _ => return Err(DeviceError::invalid_path()),
        },
        None => return Err(DeviceError::invalid_path()),
    }

    // Collect the parts
    let mut parts: VecDeque<&str> = VecDeque::new();
    for part in path.split(&['\\', '/']) {
        let part = part.trim();
        if part.len() > 0 {
            parts.push_back(part);
        }
    }

    Ok(parts)
}

impl<T: ProcessTypes> Tree<T> {
    pub const fn new() -> Mutex<Self, T> {
        Mutex::new(Tree {
            root_devices: Children::new(),
        })
    }

    pub fn get_device(
        &self,
        path: &str,
    ) -> base::error::Result<Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>> {
        let path = parse_path(path)?;
        match self.root_devices.child(path.as_slices().0) {
            Some(device) => Ok(device.device()),
            None => Err(DeviceError::device_not_found()),
        }
    }

    pub fn get_children(&self, path: &str) -> base::error::Result<Box<[String]>> {
        let path = parse_path(path)?;
        match self.root_devices.child(path.as_slices().0) {
            Some(device) => Ok(device.children()),
            None => Err(DeviceError::device_not_found()),
        }
    }

    pub fn register_device(
        &mut self,
        path: &str,
        device: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    ) -> base::error::Result<()> {
        let mut path = parse_path(path)?;
        let name = match path.pop_back() {
            Some(name) => name,
            None => return Err(DeviceError::invalid_path()),
        };

        if path.len() > 0 {
            let parent_device = match self.root_devices.child_mut(path.as_slices().0) {
                Some(device) => device,
                None => return Err(DeviceError::device_not_found()),
            };

            if parent_device.insert(name.to_owned(), device) {
                Ok(())
            } else {
                Err(DeviceError::already_exists())
            }
        } else if self.root_devices.insert(name.to_owned(), device) {
            Ok(())
        } else {
            Err(DeviceError::already_exists())
        }
    }

    pub fn remove_device(&mut self, path: &str) {
        let path = match parse_path(path) {
            Ok(path) => path,
            Err(_) => return,
        };
        self.root_devices.remove_device(path.as_slices().0);
    }
}

impl DeviceError {
    pub fn invalid_path() -> Box<dyn base::error::Error> {
        Box::new(DeviceError::InvalidPath)
    }

    pub fn device_not_found() -> Box<dyn base::error::Error> {
        Box::new(DeviceError::NotFound)
    }

    pub fn already_exists() -> Box<dyn base::error::Error> {
        Box::new(DeviceError::AlreadyExists)
    }
}

impl base::error::Error for DeviceError {
    fn module_number(&self) -> i32 {
        DEVICE_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            DeviceError::AlreadyExists => base::error::Status::Exists,
            DeviceError::InvalidPath => base::error::Status::InvalidPath,
            DeviceError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DeviceError::InvalidPath => "Invalid path",
                DeviceError::NotFound => "Device not found",
                DeviceError::AlreadyExists => "Device already exists",
            }
        )
    }
}
