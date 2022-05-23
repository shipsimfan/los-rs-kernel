use filesystem::{FileDescriptor, SeekFrom};
use process::ProcessTypes;
use raw::*;

mod raw;

pub fn verify_executable<T: ProcessTypes + 'static>(
    file: &mut FileDescriptor<T>,
) -> base::error::Result<()> {
    let mut buffer = [0; core::mem::size_of::<Elf64_Ehdr>()];
    file.seek(0, SeekFrom::Start);
    file.read(buffer.as_mut_slice())?;

    let header = Elf64_Ehdr::from_array(buffer);
    header.verify()
}
