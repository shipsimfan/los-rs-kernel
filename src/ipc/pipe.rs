use crate::locks::Mutex;
use crate::error;
use crate::logln;
use alloc::collections::VecDeque;
use alloc::sync::Arc;

pub struct Pipe {
    buffer: VecDeque<u8>,
    reader_count: usize,
    writer_count: usize,
}

pub struct PipeReader {
    pipe: Arc<Mutex<Pipe>>,
}

pub struct PipeWriter {
    pipe: Arc<Mutex<Pipe>>,
}

impl Pipe {
    pub fn new() -> (PipeReader, PipeWriter) {
        let pipe = Arc::new(Mutex::new(Pipe {
            buffer: VecDeque::new(),
            reader_count: 1,
            writer_count: 1,
        
        }));
        

        (PipeReader { pipe: pipe.clone() }, PipeWriter { pipe })
    }

    //    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
    pub fn read(&mut self, buffer: &mut [u8]) -> error::Result<usize> {
            
        if self.writer_count<1{
            return  Err(error::Status::NoWriters);
        }
        for i in 0..buffer.len() {
            match self.buffer.pop_front() {
                Some(val) => buffer[i] = val,
                None => return Ok(i),
            }
        }

        Ok(buffer.len())
    }

    pub fn write(&mut self, buffer: &[u8]) -> error::Result<()> {

        if self.reader_count<1{
            return  Err(error::Status::NoReaders);
        }
        for i in 0..buffer.len() {
            self.buffer.push_back(buffer[i])
        }
        Ok(())
    }

    pub fn increment_write(&mut self){
        self.writer_count += 1;
    }
    pub fn increment_read(&mut self){
        self.reader_count += 1;

    }
    pub fn decrement_write(&mut self){
        self.writer_count -= 1;

    }
    pub fn decrement_read(&mut self){
        self.reader_count -= 1;

    }
    
}

impl PipeReader {
    pub fn read(&self, buffer: &mut [u8]) -> error::Result<usize> {
        self.pipe.lock().read(buffer)
    }
}
impl Clone for PipeReader{
    fn clone(&self) -> Self{
        self.pipe.lock().increment_read();
        
        PipeReader{
            pipe: self.pipe.clone(),
        }
    }
}

impl Drop for PipeReader {
    fn drop(&mut self) {
        self.pipe.lock().decrement_read();

    }
}

impl PipeWriter {
    pub fn write(&self, buffer: &mut [u8]) -> error::Result<()>{
        self.pipe.lock().write(buffer)
    }
}

impl Clone for PipeWriter{
    fn clone(&self) -> Self{
        self.pipe.lock().increment_write();
        
        PipeWriter{
            pipe: self.pipe.clone(),
        }
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        self.pipe.lock().decrement_write();

    }
}
