use super::DeviceReference;
use crate::error;
use alloc::{borrow::ToOwned, string::String, vec::Vec};

struct Container {
    pub name: String,
    pub device: DeviceReference,
    pub children: Vec<Container>,
}

pub struct Tree {
    root_devices: Vec<Container>,
}

fn parse_path(path: &str) -> error::Result<(Vec<String>, String)> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut first = true;
    for c in path.chars() {
        if first {
            if c != '/' && c != '\\' {
                return Err(error::Status::InvalidArgument);
            }

            first = false;
            continue;
        }

        match c {
            '/' | '\\' => {
                parts.push(current_part);
                current_part = String::new();
            }
            _ => current_part.push(c),
        }
    }

    Ok((parts, current_part))
}

impl Tree {
    pub const fn new() -> Self {
        Tree {
            root_devices: Vec::new(),
        }
    }

    pub fn register_device(&mut self, path: &str, device: DeviceReference) -> error::Result<()> {
        match parse_path(path) {
            Err(err) => Err(err),
            Ok((path_parts, name)) => {
                let mut current_device = &mut self.root_devices;
                'main: for part in path_parts {
                    for device in current_device {
                        if part == device.name {
                            current_device = &mut device.children;
                            continue 'main;
                        }
                    }

                    return Err(error::Status::NotFound);
                }

                let not_mut_vec = current_device as &Vec<Container>;

                for device in not_mut_vec {
                    if device.name == name {
                        return Err(error::Status::Exists);
                    }
                }

                current_device.push(Container {
                    name: name,
                    device: device,
                    children: Vec::new(),
                });

                Ok(())
            }
        }
    }

    pub fn _remove_device(&mut self, path: &str) {
        match parse_path(path) {
            Err(_) => {}
            Ok((path_parts, name)) => {
                let mut current_device = &mut self.root_devices;
                'main: for part in path_parts {
                    for device in current_device {
                        if part == device.name {
                            current_device = &mut device.children;
                            continue 'main;
                        }
                    }

                    return;
                }

                current_device.retain(|device| device.name != name);
            }
        }
    }

    pub fn get_device(&mut self, path: &str) -> error::Result<DeviceReference> {
        self.get_container(path)
            .map(|container| container.device.clone())
    }

    pub fn get_children(&self, path: &str) -> error::Result<Vec<String>> {
        if path == "" {
            Ok(self.get_root_devices())
        } else {
            let device = self.get_container(path)?;

            let mut children = Vec::with_capacity(device.children.len());

            for child in &device.children {
                children.push(child.name.to_owned())
            }

            Ok(children)
        }
    }

    fn get_container(&self, path: &str) -> error::Result<&Container> {
        let (path_parts, name) = parse_path(path)?;

        let mut current_device = &self.root_devices;
        'main: for part in path_parts {
            for device in current_device {
                if part == device.name {
                    current_device = &device.children;
                    continue 'main;
                }
            }

            return Err(error::Status::NoDevice);
        }

        for device in current_device {
            if device.name == name {
                return Ok(&device);
            }
        }

        Err(error::Status::NoDevice)
    }

    fn get_root_devices(&self) -> Vec<String> {
        let mut ret = Vec::with_capacity(self.root_devices.len());
        for device in &self.root_devices {
            ret.push(device.name.clone())
        }

        ret
    }
}
