use crate::error;

mod elf;

pub fn verify_executable(buffer: &[u8]) -> Result<usize, error::Status> {
    let header = elf::Elf64_Ehdr::from_slice(buffer);
    header.verify()
}

pub fn load_executable(buffer: &[u8]) -> Result<(), error::Status> {
    // Get the header
    let header = elf::Elf64_Ehdr::from_slice(buffer);

    let start = buffer.as_ptr() as usize;

    // Loop through each program header
    let mut idx = header.e_phoff();
    for _ in 0..header.e_phnum() {
        let phdr = elf::Elf64_Phdr::from_slice(&buffer[idx..]);
        if phdr.p_type() == elf::PT_LOAD {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (start + phdr.p_offset()) as *const u8,
                    phdr.p_vaddr() as *mut u8,
                    phdr.p_filesz(),
                )
            };

            if phdr.p_memsz() > phdr.p_filesz() {
                unsafe {
                    core::ptr::write_bytes(
                        (phdr.p_vaddr() + phdr.p_filesz()) as *mut u8,
                        0,
                        phdr.p_memsz() - phdr.p_filesz(),
                    )
                };
            }
        }

        idx += header.e_phentsize();
    }

    Ok(())
}
