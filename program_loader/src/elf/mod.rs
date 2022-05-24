use alloc::boxed::Box;
use base::error::PROGRAM_LOADER_MODULE_NUMBER;
use filesystem::{FileDescriptor, SeekFrom};
use process_types::ProcessTypes;
use raw::*;

mod raw;

#[derive(Debug)]
struct MultipleTLSError;

pub fn verify_executable(file: &mut FileDescriptor<ProcessTypes>) -> base::error::Result<()> {
    Elf64_Ehdr::from_file(file)?.verify()
}

pub fn load_executable(
    file: &mut FileDescriptor<ProcessTypes>,
    tls_location: *mut u8,
) -> base::error::Result<(usize, usize, usize)> {
    // Get the ELF header
    let header = Elf64_Ehdr::from_file(file)?;
    let entry = header.e_entry();

    let mut tls_size = 0;
    let mut tls_align = 0;

    // Loop through each program header
    for i in 0..header.e_phnum() {
        // Get the program header
        let phdr = Elf64_Phdr::from_file(file, i, &header)?;

        if phdr.p_type() == PT_LOAD {
            // Load the segment
            let buffer = unsafe {
                core::slice::from_raw_parts_mut(phdr.p_vaddr() as *mut u8, phdr.p_filesz())
            };
            file.seek(phdr.p_offset(), SeekFrom::Start);
            file.read(buffer)?;

            // Write zeros if nescessary
            if phdr.p_memsz() > phdr.p_filesz() {
                unsafe {
                    core::ptr::write_bytes(
                        (phdr.p_vaddr() + phdr.p_filesz()) as *mut u8,
                        0,
                        phdr.p_memsz() - phdr.p_filesz(),
                    )
                };
            }
        } else if phdr.p_type() == PT_TLS {
            // Make sure a previous TLS isn't defined
            if tls_size > 0 {
                return Err(Box::new(MultipleTLSError));
            }

            // Load the segment
            let buffer = unsafe { core::slice::from_raw_parts_mut(tls_location, phdr.p_filesz()) };
            file.seek(phdr.p_offset(), SeekFrom::Start);
            file.read(buffer)?;

            // Write zeros if nescessary
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
    }

    Ok((entry, tls_size, tls_align))
}

impl base::error::Error for MultipleTLSError {
    fn module_number(&self) -> i32 {
        PROGRAM_LOADER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::InvalidExecutableFormat
    }
}

impl core::fmt::Display for MultipleTLSError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Multiple TLS segments defined")
    }
}
