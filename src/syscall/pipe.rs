

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
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
        
            process.lock().close_pipe_reader((arg1 & 0x7FFFFFFFFFFF) as isize)
            
        }
        CLOSE_PIPE_WRITE_SYSCALL => {
            // close pipe writer identified by pwd (arg1)
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
        
            process.lock().close_pipe_writer((arg1 & 0x7FFFFFFFFFFF) as isize)
            
        }
        
        CREATE_PIPE_SYSCALL => {
            //create pipe and store refs in prd and pwd args (arg1,arg2)    
            let process = process::get_current_thread()
            .process()
            .unwrap()
            .upgrade()
            .unwrap();
    
            let prw = process.lock().create_pipe();
            //point arg1 to pr, point arg2 to pw
            *arg1 = prw.0;
            *arg2 = prw.1;
        
    }
        READ_PIPE_SYSCALL => {
            //read upto buffer_len from prd into buffer (arg1 prd, arg2 buffer, arg3 buffer_len)
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
        
            let pr = process.lock().get_pipe_reader((arg1 & 0x7FFFFFFFFFFF) as isize);
            pr.read(arg2)
            
        }
        
        WRITE_PIPE_SYSCALL => {
            //write upto buffer_len from buffer into pwd (arg1 pwd, arg2 buffer, arg3 buffer_len)
            let process = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
        
            let pw = process.lock().get_pipe_writer((arg1 & 0x7FFFFFFFFFFF) as isize);
            pw.write(arg2)
        }
    }
}