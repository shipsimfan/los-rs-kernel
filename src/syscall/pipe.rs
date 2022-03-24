

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
            //close pipe reader identified by prd
        }
        CLOSE_PIPE_WRITE_SYSCALL => {
            // close pipe writer identified by pwd
        }
        CREATE_PIPE_SYSCALL => {
            //create pipe and store refs in prd and pwd args    
        }
        READ_PIPE_SYSCALL => {
            //read upto buffer_len from prd into buffer
        }
        WRITE_PIPE_SYSCALL => {
            //write upto buffer_len from buffer into pwd
        }
    }
}