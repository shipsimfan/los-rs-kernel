use super::fat::FATBox;

pub struct File {
    first_cluster: u32,
    file_size: usize,
    fat: FATBox,
}

impl File {
    pub fn new(first_cluster: u32, file_size: usize, fat: FATBox) -> Self {
        File {
            first_cluster: first_cluster,
            file_size: file_size,
            fat: fat,
        }
    }
}

impl crate::filesystem::File for File {
    fn write(&mut self, _: usize, _: &[u8]) -> crate::error::Result {
        Err(crate::error::Status::NotSupported)
    }

    fn read(&mut self, _: usize, _: &mut [u8]) -> Result<usize, crate::error::Status> {
        Err(crate::error::Status::NotSupported)
    }

    fn set_length(&mut self, _: usize) -> crate::error::Result {
        Err(crate::error::Status::NotSupported)
    }
}
