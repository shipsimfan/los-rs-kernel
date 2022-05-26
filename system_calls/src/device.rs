use crate::SystemCallError;
use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use process_types::ProcessTypes;

#[derive(Debug)]
enum DeviceError {
    NotFound,
}

const OPEN_DEVICE_SYSCALL: usize = 0x6000;
const CLOSE_DEVICE_SYSCALL: usize = 0x6001;
const READ_DEVICE_SYSCALL: usize = 0x6002;
const WRITE_DEVICE_SYSCALL: usize = 0x6003;
const IOCTRL_DEVICE_SYSCALL: usize = 0x6004;
const LIST_DEVICE_CHILDREN_SYSCALL: usize = 0x6005;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        OPEN_DEVICE_SYSCALL => {
            let path = super::to_str(arg1)?;
            let device = device::get_device::<ProcessTypes>(path)?;
            Ok(process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().insert_device(device))
            }))
        }
        CLOSE_DEVICE_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().remove_device(arg1 as isize))
            });
            Ok(0)
        }
        READ_DEVICE_SYSCALL => {
            let device = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .device(arg1 as isize)
                        .map(|device| device.clone())
                })
            }) {
                Some(device) => device,
                None => return Err(Box::new(DeviceError::NotFound)),
            };

            let buffer = super::to_slice_mut(arg3, arg4)?;

            match device.lock(|device| device.read(arg2, buffer)) {
                Some(result) => result.map(|bytes_read| bytes_read as isize),
                None => return Err(Box::new(DeviceError::NotFound)),
            }
        }
        WRITE_DEVICE_SYSCALL => {
            let device = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .device(arg1 as isize)
                        .map(|device| device.clone())
                })
            }) {
                Some(device) => device,
                None => return Err(Box::new(DeviceError::NotFound)),
            };

            let buffer = super::to_slice(arg3, arg4)?;

            match device.lock(|device| device.write(arg2, buffer)) {
                Some(result) => result.map(|bytes_written| bytes_written as isize),
                None => return Err(Box::new(DeviceError::NotFound)),
            }
        }
        IOCTRL_DEVICE_SYSCALL => {
            let device = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .device(arg1 as isize)
                        .map(|device| device.clone())
                })
            }) {
                Some(device) => device,
                None => return Err(Box::new(DeviceError::NotFound)),
            };

            match device.lock(|device| device.ioctrl(arg2, arg3)) {
                Some(result) => result.map(|result| result as isize),
                None => return Err(Box::new(DeviceError::NotFound)),
            }
        }
        LIST_DEVICE_CHILDREN_SYSCALL => {
            let path = super::to_str(arg1)?;
            let devices = device::get_children::<ProcessTypes>(path)?;

            // Is the caller just getting a count?
            if arg2 == 0 {
                // Caller wants count
                Ok(devices.len() as isize)
            } else {
                // Caller wants names
                let buffer = super::to_slice_mut(arg2, arg3)?;

                let mut i = 0;
                for device in devices.iter() {
                    if i >= arg3 {
                        break;
                    }

                    let str_buffer = super::to_slice_mut(buffer[i], arg4)?;

                    let mut j = 0;
                    for c in device.bytes() {
                        if j >= arg4 - 1 {
                            break;
                        }

                        str_buffer[j] = c;

                        j += 1;
                    }

                    str_buffer[j] = 0;

                    i += 1;
                }

                Ok(i as isize)
            }
        }
        _ => {
            log_error!("Invalid device system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for DeviceError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            DeviceError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeviceError::NotFound => write!(f, "Device not found"),
        }
    }
}
