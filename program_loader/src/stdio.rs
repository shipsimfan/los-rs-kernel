#![allow(unused)]

use filesystem::DirectoryDescriptor;
use process::Process;
use process_types::{Descriptors, ProcessTypes};

pub enum StandardIOType {
    None,
    Console,
    File(isize),
    Device(isize),
}

pub struct StandardIO {
    stdout: StandardIOType,
    stderr: StandardIOType,
    stdin: StandardIOType,
}

#[repr(packed(1))]
pub struct CStandardIO {
    stdout_type: usize,
    stdout_desc: isize,
    stderr_type: usize,
    stderr_desc: isize,
    stdin_type: usize,
    stdin_desc: isize,
}

const STDIO_TYPE_NONE: usize = 0;
const STDIO_TYPE_CONSOLE: usize = 1;
const STDIO_TYPE_FILE: usize = 2;
const STDIO_TYPE_DEVICE: usize = 3;

impl StandardIO {
    pub fn new(stdout: StandardIOType, stderr: StandardIOType, stdin: StandardIOType) -> Self {
        StandardIO {
            stdout,
            stderr,
            stdin,
        }
    }

    pub fn build_descriptors(
        &self,
        working_directory: DirectoryDescriptor<ProcessTypes>,
    ) -> Descriptors<ProcessTypes> {
        let mut new_descriptors = Descriptors::new(Some(working_directory));

        process::current_thread::<ProcessTypes>().lock(|thread| {
            thread.process().lock(|process| {
                self.stdout.copy_descriptor(process, &mut new_descriptors);
                self.stderr.copy_descriptor(process, &mut new_descriptors);
                self.stdin.copy_descriptor(process, &mut new_descriptors);
            })
        });

        new_descriptors
    }
}

impl StandardIOType {
    pub fn copy_descriptor(
        &self,
        process: &Process<ProcessTypes>,
        new_descriptors: &mut Descriptors<ProcessTypes>,
    ) {
        match self {
            StandardIOType::None | StandardIOType::Console => {}
            StandardIOType::File(id) => {
                match process.descriptors().file(*id) {
                    Some(descriptor) => {
                        descriptor
                            .lock(|descriptor| new_descriptors.insert_file(descriptor.clone()));
                    }
                    None => {}
                };
            }
            StandardIOType::Device(id) => {
                let descriptor = match process.descriptors().device(*id) {
                    Some(descriptor) => descriptor.clone(),
                    None => return,
                };
                new_descriptors.insert_device(descriptor);
            }
        }
    }
}
