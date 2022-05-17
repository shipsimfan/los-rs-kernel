#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    size: usize,
    is_directory: bool,
}

impl Metadata {
    pub fn new(size: usize, is_directory: bool) -> Self {
        Metadata { size, is_directory }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub fn set_size(&mut self, new_size: usize) {
        self.size = new_size
    }
}
