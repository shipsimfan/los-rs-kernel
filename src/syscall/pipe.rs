use crate::{error, logln, process};

const CLOSE_PIPE_READ_SYSCALL: usize = 0xA000;
const CLOSE_PIPE_WRITE_SYSCALL: usize = 0xA001;
const CREATE_PIPE_SYSCALL: usize = 0xA002;
const READ_PIPE_SYSCALL: usize = 0xA003;
const WRITE_PIPE_SYSCALL: usize = 0xA004;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        CLOSE_PIPE_READ_SYSCALL => {
            //close pipe reader identified by prd (arg1)
            let process = process::get_current_thread().process().unwrap();

            process.close_pipe_reader((arg1 & 0x7FFFFFFFFFFF) as isize);

            0
        }
        CLOSE_PIPE_WRITE_SYSCALL => {
            // close pipe writer identified by pwd (arg1)
            let process = process::get_current_thread().process().unwrap();

            process.close_pipe_writer((arg1 & 0x7FFFFFFFFFFF) as isize);

            0
        }

        CREATE_PIPE_SYSCALL => {
            let ptr1 = match super::to_ptr_mut(arg1) {
                Ok(ptr) => ptr,
                Err(error) => return error.to_return_code(),
            };

            let ptr2 = match super::to_ptr_mut(arg2) {
                Ok(ptr) => ptr,
                Err(error) => return error.to_return_code(),
            };

            //create pipe and store refs in prd and pwd args (arg1,arg2)
            let mut process = process::get_current_thread().process().unwrap();

            match process.create_pipe() {
                Some((pr, pw)) => {
                    //point arg1 to pr, point arg2 to pw
                    unsafe {
                        *ptr1 = pr;
                        *ptr2 = pw;
                        0
                    }
                }
                None => error::Status::NoProcess.to_return_code(),
            }
        }
        READ_PIPE_SYSCALL => {
            let buffer = match super::to_slice_mut(arg2, arg3) {
                Ok(buffer) => buffer,
                Err(error) => return error.to_return_code(),
            };

            //read upto buffer_len from prd into buffer (arg1 prd, arg2 buffer, arg3 buffer_len)
            let process = process::get_current_thread().process().unwrap();

            let pr = match process.get_pipe_reader((arg1 & 0x7FFFFFFFFFFF) as isize) {
                Ok(pr) => pr,
                Err(error) => return error.to_return_code(),
            };

            let ret = pr.lock().read(buffer);
            
            return match ret{
                Ok(ret) => ret as isize,
                Err(error) => error.to_return_code()
            };
        }

        WRITE_PIPE_SYSCALL => {
            let buffer = match super::to_slice_mut(arg2, arg3) {
                Ok(buffer) => buffer,
                Err(error) => return error.to_return_code(),
            };

            //write upto buffer_len from buffer into pwd (arg1 pwd, arg2 buffer, arg3 buffer_len)
            let process = process::get_current_thread().process().unwrap();

            let pw = match process.get_pipe_writer((arg1 & 0x7FFFFFFFFFFF) as isize) {
                Ok(pw) => pw,
                Err(error) => return error.to_return_code(),
            };

            return match pw.lock().write(buffer){
                Ok(_ret) => buffer.len() as isize,
                Err(error) => error.to_return_code()
            };


        }
        _ => {
            logln!("Invalid pipe system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
