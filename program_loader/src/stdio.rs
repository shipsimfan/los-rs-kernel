#![allow(unused)]

use alloc::boxed::Box;
use base::error::PROGRAM_LOADER_MODULE_NUMBER;
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

#[derive(Debug)]
struct InvalidStandardIOType;

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

    pub fn into_c(&self) -> CStandardIO {
        let (stdout_type, stdout_desc) = self.stdout.into_c();
        let (stderr_type, stderr_desc) = self.stderr.into_c();
        let (stdin_type, stdin_desc) = self.stdin.into_c();

        CStandardIO {
            stdout_type,
            stdout_desc,
            stderr_type,
            stderr_desc,
            stdin_type,
            stdin_desc,
        }
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

    pub fn into_c(&self) -> (usize, isize) {
        match self {
            StandardIOType::None => (STDIO_TYPE_NONE, 0),
            StandardIOType::Console => (STDIO_TYPE_CONSOLE, 0),
            StandardIOType::File(descriptor) => (STDIO_TYPE_FILE, *descriptor),
            StandardIOType::Device(descriptor) => (STDIO_TYPE_DEVICE, *descriptor),
        }
    }
}

impl CStandardIO {
    pub fn to_stdio(&self) -> base::error::Result<StandardIO> {
        Ok(StandardIO::new(
            CStandardIO::parse(self.stdout_type, self.stdout_desc)?,
            CStandardIO::parse(self.stderr_type, self.stderr_desc)?,
            CStandardIO::parse(self.stdin_type, self.stdin_desc)?,
        ))
    }

    fn parse(class: usize, desc: isize) -> base::error::Result<StandardIOType> {
        match class {
            STDIO_TYPE_NONE => Ok(StandardIOType::None),
            STDIO_TYPE_CONSOLE => Ok(StandardIOType::Console),
            STDIO_TYPE_FILE => Ok(StandardIOType::File(desc)),
            _ => Err(Box::new(InvalidStandardIOType)),
        }
    }
}

impl base::error::Error for InvalidStandardIOType {
    fn module_number(&self) -> i32 {
        PROGRAM_LOADER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::OutOfRange
    }
}

impl core::fmt::Display for InvalidStandardIOType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid standard I/O type")
    }
}
