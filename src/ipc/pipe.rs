use alloc::collections::VecDeque;
use alloc::sync::Arc;
use crate::locks::Mutex;





pub struct Pipe {
    buffer: VecDeque<u8>,
}

pub struct PipeReader {
    pipe: Arc<Mutex<Pipe>>,
}

pub struct PipeWriter {
    pipe: Arc<Mutex<Pipe>>,
}

impl Pipe {
    pub fn new() -> (PipeReader, PipeWriter) {
        let pipe = Arc::new(
            Mutex::new(Pipe {
                buffer: VecDeque::new(),
            }));
    
        (
            PipeReader {
                pipe: pipe.clone(),
            },
            PipeWriter {
                pipe,
            }
        )
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        for i in 0..buffer.len() {
            match self.buffer.pop_front() {
                Some(val) => buffer[i] = val,
                None => return i,
            }
        }
    
        buffer.len()
    } 
    
    pub fn write(&mut self, buffer: &[u8]) {
        for i in 0..buffer.len() {
            self.buffer.push_back(buffer[i])
        }
    }
    
} 
    

impl PipeReader {
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        self.pipe.lock().read(buffer)
    }
}

impl PipeWriter {
    pub fn write(&self, buffer: &mut [u8]) {
        self.pipe.lock().write(buffer)

    }
}
