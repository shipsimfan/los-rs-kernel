use crate::error;

mod elf;

pub fn verify_executable(buffer: &[u8]) -> Result<usize, error::Status> {
    let header = elf::Elf64_Ehdr::from_slice(buffer);
    header.verify()
}

pub fn load_executable(buffer: &[u8]) -> Result<(), error::Status> {
    // Get the header
    let header = elf::Elf64_Ehdr::from_slice(buffer);

    // Loop through each program header
    let mut idx = header.e_phoff();
    for _ in 0..header.e_phnum() {
        let phdr = elf::Elf64_Phdr::from_slice(&buffer[idx..]);
        if phdr.p_type() == elf::PT_LOAD {
            let dest = unsafe {
                core::slice::from_raw_parts_mut(phdr.p_vaddr() as *mut u8, phdr.p_filesz())
            };

            let offset = phdr.p_offset();
            let filesz = phdr.p_filesz();
            let memsz = phdr.p_memsz();

            for i in 0..filesz {
                dest[i] = buffer[offset + i];
            }

            if memsz > filesz {
                for i in filesz..memsz {
                    dest[i] = buffer[offset + i];
                }
            }
        }

        idx += header.e_phentsize();
    }

    Ok(())
}
