#![allow(non_camel_case_types)]

use crate::error;

pub type Elf64_Addr = u64;
pub type Elf64_Half = u16;
pub type Elf64_Off = u64;
pub type Elf64_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

#[repr(packed(1))]
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

#[repr(packed(1))]
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

impl Elf64_Ehdr {
    pub fn from_slice(slice: &[u8]) -> &Self {
        let (_, header, _) = unsafe { slice.align_to::<Elf64_Ehdr>() };
        &header[0]
    }

    pub fn verify(&self) -> Result<usize, error::Status> {
        // Verify MAG
        if self.e_ident[EI_MAG0] != ELFMAG0
            || self.e_ident[EI_MAG1] != ELFMAG1
            || self.e_ident[EI_MAG2] != ELFMAG2
            || self.e_ident[EI_MAG3] != ELFMAG3
        {
            return Err(error::Status::InvalidArgument);
        }

        // Verify class
        if self.e_ident[EI_CLASS] != ELFCLASS64 {
            return Err(error::Status::NotSupported);
        }

        // Verify data order
        if self.e_ident[EI_DATA] != ELFDATA2LSB {
            return Err(error::Status::NotSupported);
        }

        // Verify version
        if self.e_ident[EI_VERSION] < EV_CURRENT as u8 || self.e_version < EV_CURRENT {
            return Err(error::Status::InvalidArgument);
        }

        // Verify type
        if self.e_type != ET_EXEC {
            return Err(error::Status::InvalidArgument);
        }

        // Verify machine
        if self.e_macine != EM_AMD64 {
            return Err(error::Status::NotSupported);
        }

        Ok(self.e_entry as usize)
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
    pub fn from_slice(slice: &[u8]) -> &Self {
        let (_, header, _) = unsafe { slice.align_to::<Elf64_Phdr>() };
        &header[0]
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
}
