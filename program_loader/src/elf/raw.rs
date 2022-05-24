#![allow(non_camel_case_types, unused)]

use alloc::boxed::Box;
use base::error::PROGRAM_LOADER_MODULE_NUMBER;
use filesystem::{FileDescriptor, SeekFrom};
use process_types::ProcessTypes;

pub type Elf64_Addr = u64;
pub type Elf64_Half = u16;
pub type Elf64_Off = u64;
pub type Elf64_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

#[derive(Debug)]
enum ElfHeaderError {
    InvalidExecutable,
    NotSupported,
}

pub struct Elf64_Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: Elf64_Half,
    e_macine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half,
}

pub struct Elf64_Phdr {
    p_type: Elf64_Word,
    p_flags: Elf64_Word,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: Elf64_Xword,
    p_memsz: Elf64_Xword,
    p_align: Elf64_Xword,
}

pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_ABIVERSION: usize = 8;
pub const EI_NIDENT: usize = 16;

pub const ELFMAG0: u8 = 0x7F;
pub const ELFMAG1: u8 = 'E' as u8;
pub const ELFMAG2: u8 = 'L' as u8;
pub const ELFMAG3: u8 = 'F' as u8;

pub const ELFCLASS64: u8 = 2;

pub const ELFDATA2LSB: u8 = 1;

pub const ET_EXEC: Elf64_Half = 2;

pub const EM_AMD64: Elf64_Half = 62;

pub const EV_CURRENT: Elf64_Word = 1;

pub const PT_LOAD: Elf64_Word = 1;
pub const PT_TLS: Elf64_Word = 7;

impl Elf64_Ehdr {
    pub fn from_file(file: &mut FileDescriptor<ProcessTypes>) -> base::error::Result<Self> {
        let mut slice = [0; core::mem::size_of::<Elf64_Ehdr>()];
        file.seek(0, SeekFrom::Start);
        file.read(slice.as_mut_slice())?;

        Ok(Elf64_Ehdr {
            e_ident: [
                slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
                slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14],
                slice[15],
            ],
            e_type: (slice[16] as Elf64_Half) | ((slice[17] as Elf64_Half) << 8),
            e_macine: (slice[18] as Elf64_Half) | ((slice[19] as Elf64_Half) << 8),
            e_version: (slice[20] as Elf64_Word)
                | ((slice[21] as Elf64_Word) << 8)
                | ((slice[22] as Elf64_Word) << 16)
                | ((slice[23] as Elf64_Word) << 24),
            e_entry: (slice[24] as Elf64_Addr)
                | ((slice[25] as Elf64_Addr) << 8)
                | ((slice[26] as Elf64_Addr) << 16)
                | ((slice[27] as Elf64_Addr) << 24)
                | ((slice[28] as Elf64_Addr) << 32)
                | ((slice[29] as Elf64_Addr) << 40)
                | ((slice[30] as Elf64_Addr) << 48)
                | ((slice[31] as Elf64_Addr) << 56),
            e_phoff: (slice[32] as Elf64_Off)
                | ((slice[33] as Elf64_Off) << 8)
                | ((slice[34] as Elf64_Off) << 16)
                | ((slice[35] as Elf64_Off) << 24)
                | ((slice[36] as Elf64_Off) << 32)
                | ((slice[37] as Elf64_Off) << 40)
                | ((slice[38] as Elf64_Off) << 48)
                | ((slice[39] as Elf64_Off) << 56),
            e_shoff: (slice[40] as Elf64_Off)
                | ((slice[41] as Elf64_Off) << 8)
                | ((slice[42] as Elf64_Off) << 16)
                | ((slice[43] as Elf64_Off) << 24)
                | ((slice[44] as Elf64_Off) << 32)
                | ((slice[45] as Elf64_Off) << 40)
                | ((slice[46] as Elf64_Off) << 48)
                | ((slice[47] as Elf64_Off) << 56),
            e_flags: (slice[48] as Elf64_Word)
                | ((slice[49] as Elf64_Word) << 8)
                | ((slice[50] as Elf64_Word) << 16)
                | ((slice[51] as Elf64_Word) << 24),
            e_ehsize: (slice[52] as Elf64_Half) | ((slice[53] as Elf64_Half) << 8),
            e_phentsize: (slice[54] as Elf64_Half) | ((slice[55] as Elf64_Half) << 8),
            e_phnum: (slice[56] as Elf64_Half) | ((slice[57] as Elf64_Half) << 8),
            e_shentsize: (slice[58] as Elf64_Half) | ((slice[59] as Elf64_Half) << 8),
            e_shnum: (slice[60] as Elf64_Half) | ((slice[61] as Elf64_Half) << 8),
            e_shstrndx: (slice[62] as Elf64_Half) | ((slice[63] as Elf64_Half) << 8),
        })
    }

    pub fn verify(&self) -> base::error::Result<()> {
        // Verify MAG
        if self.e_ident[EI_MAG0] != ELFMAG0
            || self.e_ident[EI_MAG1] != ELFMAG1
            || self.e_ident[EI_MAG2] != ELFMAG2
            || self.e_ident[EI_MAG3] != ELFMAG3
        {
            return Err(Box::new(ElfHeaderError::InvalidExecutable));
        }

        // Verify version
        if self.e_ident[EI_VERSION] < EV_CURRENT as u8 || self.e_version < EV_CURRENT {
            return Err(Box::new(ElfHeaderError::InvalidExecutable));
        }

        // Verify type
        if self.e_type != ET_EXEC {
            return Err(Box::new(ElfHeaderError::InvalidExecutable));
        }

        // Verify class
        if self.e_ident[EI_CLASS] != ELFCLASS64 {
            return Err(Box::new(ElfHeaderError::NotSupported));
        }

        // Verify data order
        if self.e_ident[EI_DATA] != ELFDATA2LSB {
            return Err(Box::new(ElfHeaderError::NotSupported));
        }

        // Verify machine
        if self.e_macine != EM_AMD64 {
            return Err(Box::new(ElfHeaderError::NotSupported));
        }

        Ok(())
    }

    pub fn e_entry(&self) -> usize {
        self.e_entry as usize
    }

    pub fn e_phoff(&self) -> usize {
        self.e_phoff as usize
    }

    pub fn e_phentsize(&self) -> usize {
        self.e_phentsize as usize
    }

    pub fn e_phnum(&self) -> usize {
        self.e_phnum as usize
    }
}

impl Elf64_Phdr {
    pub fn from_file(
        file: &mut FileDescriptor<ProcessTypes>,
        idx: usize,
        header: &Elf64_Ehdr,
    ) -> base::error::Result<Self> {
        let mut slice = [0; core::mem::size_of::<Elf64_Phdr>()];
        file.seek(
            header.e_phoff() + idx * header.e_phentsize(),
            SeekFrom::Start,
        );
        file.read(&mut slice)?;

        Ok(Elf64_Phdr {
            p_type: (slice[0] as Elf64_Word)
                | ((slice[1] as Elf64_Word) << 8)
                | ((slice[2] as Elf64_Word) << 16)
                | ((slice[3] as Elf64_Word) << 24),
            p_flags: (slice[4] as Elf64_Word)
                | ((slice[5] as Elf64_Word) << 8)
                | ((slice[6] as Elf64_Word) << 16)
                | ((slice[7] as Elf64_Word) << 24),
            p_offset: (slice[8] as Elf64_Off)
                | ((slice[9] as Elf64_Off) << 8)
                | ((slice[10] as Elf64_Off) << 16)
                | ((slice[11] as Elf64_Off) << 24)
                | ((slice[12] as Elf64_Off) << 32)
                | ((slice[13] as Elf64_Off) << 40)
                | ((slice[14] as Elf64_Off) << 48)
                | ((slice[15] as Elf64_Off) << 56),
            p_vaddr: (slice[16] as Elf64_Addr)
                | ((slice[17] as Elf64_Addr) << 8)
                | ((slice[18] as Elf64_Addr) << 16)
                | ((slice[19] as Elf64_Addr) << 24)
                | ((slice[20] as Elf64_Addr) << 32)
                | ((slice[21] as Elf64_Addr) << 40)
                | ((slice[22] as Elf64_Addr) << 48)
                | ((slice[23] as Elf64_Addr) << 56),
            p_paddr: (slice[24] as Elf64_Addr)
                | ((slice[25] as Elf64_Addr) << 8)
                | ((slice[26] as Elf64_Addr) << 16)
                | ((slice[27] as Elf64_Addr) << 24)
                | ((slice[28] as Elf64_Addr) << 32)
                | ((slice[29] as Elf64_Addr) << 40)
                | ((slice[30] as Elf64_Addr) << 48)
                | ((slice[31] as Elf64_Addr) << 56),
            p_filesz: (slice[32] as Elf64_Xword)
                | ((slice[33] as Elf64_Xword) << 8)
                | ((slice[34] as Elf64_Xword) << 16)
                | ((slice[35] as Elf64_Xword) << 24)
                | ((slice[36] as Elf64_Xword) << 32)
                | ((slice[37] as Elf64_Xword) << 40)
                | ((slice[38] as Elf64_Xword) << 48)
                | ((slice[39] as Elf64_Xword) << 56),
            p_memsz: (slice[40] as Elf64_Xword)
                | ((slice[41] as Elf64_Xword) << 8)
                | ((slice[42] as Elf64_Xword) << 16)
                | ((slice[43] as Elf64_Xword) << 24)
                | ((slice[44] as Elf64_Xword) << 32)
                | ((slice[45] as Elf64_Xword) << 40)
                | ((slice[46] as Elf64_Xword) << 48)
                | ((slice[47] as Elf64_Xword) << 56),
            p_align: (slice[48] as Elf64_Xword)
                | ((slice[49] as Elf64_Xword) << 8)
                | ((slice[50] as Elf64_Xword) << 16)
                | ((slice[51] as Elf64_Xword) << 24)
                | ((slice[52] as Elf64_Xword) << 32)
                | ((slice[53] as Elf64_Xword) << 40)
                | ((slice[54] as Elf64_Xword) << 48)
                | ((slice[55] as Elf64_Xword) << 56),
        })
    }

    pub fn p_type(&self) -> Elf64_Word {
        self.p_type
    }

    pub fn p_offset(&self) -> usize {
        self.p_offset as usize
    }

    pub fn p_vaddr(&self) -> usize {
        self.p_vaddr as usize
    }

    pub fn p_filesz(&self) -> usize {
        self.p_filesz as usize
    }

    pub fn p_memsz(&self) -> usize {
        self.p_memsz as usize
    }

    pub fn p_align(&self) -> usize {
        self.p_align as usize
    }
}

impl base::error::Error for ElfHeaderError {
    fn module_number(&self) -> i32 {
        PROGRAM_LOADER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ElfHeaderError::InvalidExecutable => base::error::Status::InvalidExecutableFormat,
            ElfHeaderError::NotSupported => base::error::Status::NotSupported,
        }
    }
}

impl core::fmt::Display for ElfHeaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ElfHeaderError::InvalidExecutable => write!(f, "The file is an invalid ELF executable"),
            ElfHeaderError::NotSupported => {
                write!(f, "This executable is not supported on this machine")
            }
        }
    }
}
