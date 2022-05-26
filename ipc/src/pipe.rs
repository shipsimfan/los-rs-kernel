use alloc::collections::VecDeque;
use base::multi_owner::Owner;
use process::{Mutex, ProcessTypes};

pub struct Pipe {
    buffer: VecDeque<u8>,
}

pub struct PipeReader<T: ProcessTypes + 'static> {
    pipe: Owner<Pipe, Mutex<Pipe, T>>,
}

pub struct PipeWriter<T: ProcessTypes + 'static> {
    pipe: Owner<Pipe, Mutex<Pipe, T>>,
}

impl Pipe {
    pub fn new<T: ProcessTypes>() -> (PipeReader<T>, PipeWriter<T>) {
        let pipe = Owner::new(Pipe {
            buffer: VecDeque::new(),
        });

        (PipeReader { pipe: pipe.clone() }, PipeWriter { pipe })
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

impl<T: ProcessTypes> PipeReader<T> {
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        self.pipe.lock(|pipe| pipe.read(buffer))
    }
}

impl<T: ProcessTypes> Clone for PipeReader<T> {
    fn clone(&self) -> Self {
        PipeReader {
            pipe: self.pipe.clone(),
        }
    }
}

impl<T: ProcessTypes> PipeWriter<T> {
    pub fn write(&self, buffer: &[u8]) {
        self.pipe.lock(|pipe| pipe.write(buffer))
    }
}

impl<T: ProcessTypes> Clone for PipeWriter<T> {
    fn clone(&self) -> Self {
        PipeWriter {
            pipe: self.pipe.clone(),
        }
    }
}
