use crate::{device, error, logln, process};

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
) -> isize {
    match code {
        OPEN_DEVICE_SYSCALL => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };

            match process::get_current_thread()
                .process()
                .unwrap()
                .open_device(path)
            {
                Ok(dd) => dd,
                Err(status) => status.to_return_code(),
            }
        }
        CLOSE_DEVICE_SYSCALL => {
            process::get_current_thread()
                .process()
                .unwrap()
                .close_device((arg1 & 0x7FFFFFFFFFFF) as isize);
            0
        }
        READ_DEVICE_SYSCALL => {
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();

            let device = match process.lock().get_device((arg1 & 0x7FFFFFFFFFFF) as isize) {
                Ok(device) => device,
                Err(status) => return status.to_return_code(),
            };

            let buffer = match super::to_slice_mut(arg3, arg4) {
                Ok(slice) => slice,
                Err(status) => return status.to_return_code(),
            };

            let device = device.lock();
            match device.read(arg2, buffer) {
                Ok(()) => 0,
                Err(status) => status.to_return_code(),
            }
        }
        WRITE_DEVICE_SYSCALL => {
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
            let device = match process.lock().get_device((arg1 & 0x7FFFFFFFFFFF) as isize) {
                Ok(device) => device,
                Err(status) => return status.to_return_code(),
            };

            let buffer = match super::to_slice_mut(arg3, arg4) {
                Ok(slice) => slice,
                Err(status) => return status.to_return_code(),
            };

            let mut device = device.lock();
            match device.write(arg2, buffer) {
                Ok(()) => 0,
                Err(status) => status.to_return_code(),
            }
        }
        IOCTRL_DEVICE_SYSCALL => {
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
            let device = match process.lock().get_device((arg1 & 0x7FFFFFFFFFFF) as isize) {
                Ok(device) => device,
                Err(status) => return status.to_return_code(),
            };

            let mut device = device.lock();
            match device.ioctrl(arg2, arg3) {
                Ok(val) => val as isize,
                Err(status) => return status.to_return_code(),
            }
        }
        LIST_DEVICE_CHILDREN_SYSCALL => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };

            let devices = match device::get_children(path) {
                Ok(devices) => devices,
                Err(status) => return status.to_return_code(),
            };

            // Is the caller just getting a count?
            if arg2 == 0 {
                // Caller wants count
                devices.len() as isize
            } else {
                // Caller wants names
                let buffer = match super::to_slice_mut(arg2, arg3) {
                    Ok(buffer) => buffer,
                    Err(status) => return status.to_return_code(),
                };

                let mut i = 0;
                for device in devices {
                    if i >= arg3 {
                        break;
                    }

                    let str_buffer = match super::to_slice_mut(buffer[i], arg4) {
                        Ok(buffer) => buffer,
                        Err(status) => return status.to_return_code(),
                    };

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

                i as isize
            }
        }
        _ => {
            logln!("Invalid filesystem system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
