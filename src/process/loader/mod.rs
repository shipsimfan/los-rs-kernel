use crate::error;

mod elf;

pub fn verify_executable(buffer: &[u8]) -> error::Result<usize> {
    let header = elf::Elf64_Ehdr::from_slice(buffer);
    header.verify()
}

pub fn load_executable(buffer: &[u8], tls_location: *mut u8) -> error::Result<(usize, usize)> {
    // Get the header
    let header = elf::Elf64_Ehdr::from_slice(buffer);

    let start = buffer.as_ptr() as usize;

    let mut tls_size = 0;
    let mut tls_align = 0;

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
        } else if phdr.p_type() == elf::PT_TLS {
            if tls_size > 0 {
                return Err(error::Status::InvalidExecutableFormat);
            }

            unsafe {
                core::ptr::copy_nonoverlapping(
                    (start + phdr.p_offset()) as *const u8,
                    tls_location,
                    phdr.p_filesz(),
                );
            }

            if phdr.p_memsz() > phdr.p_filesz() {
                unsafe {
                    core::ptr::write_bytes(
                        (tls_location as usize + phdr.p_filesz()) as *mut u8,
                        0,
                        phdr.p_memsz() - phdr.p_filesz(),
                    )
                };
            }

            tls_size = phdr.p_memsz();
            tls_align = phdr.p_align();
        }

        idx += header.e_phentsize();
    }

    Ok((tls_size, tls_align))
}
