use core::{
    ops::{Index, IndexMut},
    ptr::null_mut,
};

use crate::{
    bootloader::{self, MemoryDescriptor},
    interrupts::exceptions::{install_exception_handler, ExceptionInfo, Registers},
    process,
};

use super::{physical, PhysicalAddress, VirtualAddress, KERNEL_VMA, PAGE_SIZE};

trait PhysicalDrop {
    unsafe fn physical_drop(&mut self);
}

#[repr(C)]
pub struct AddressSpace(PhysicalAddress);

struct Table<T: PhysicalDrop> {
    entries: [usize; 512],
    _phantom: [T; 0],
}

struct PageIndex {
    pub pml4_index: usize,
    pub pdpt_index: usize,
    pub page_directory_index: usize,
    pub page_table_index: usize,
    pub _offset: usize,
}

type PML4 = Table<PDPT>;
type PDPT = Table<PageDirectory>;
type PageDirectory = Table<PageTable>;
type PageTable = Table<Page>;
type Page = [u8; PAGE_SIZE];

const PAGE_FLAG_PRESENT: usize = 1 << 0;
const PAGE_FLAG_WRITABLE: usize = 1 << 1;
const PAGE_FLAG_USER: usize = 1 << 2;

static mut KERNEL_ADDRESS_SPACE: AddressSpace = AddressSpace(0);

extern "C" {
    fn set_current_pml4(pml4: PhysicalAddress);
    fn get_cr2() -> usize;
    fn get_current_address_space() -> AddressSpace;
}

pub unsafe fn initialize(
    mmap: *const bootloader::MemoryMap,
    gmode: *const bootloader::GraphicsMode,
) {
    // Allocate the PDPTs
    let mut kernel_space = AddressSpace::new();
    let kernel_pml4 = (kernel_space.get() + KERNEL_VMA) as *mut PML4;
    let mut i = 256;
    while i < 512 {
        let pdpt = PDPT::new();
        (*kernel_pml4).set_entry(i, pdpt, true);

        i += 1;
    }

    // Allocate pages from memory map
    let mut desc = (*mmap).address;
    let top = desc as usize + (*mmap).size;
    let desc_size = (*mmap).desc_size;

    let mut ptr = desc as usize;
    while ptr < top {
        let mut paddr = (*desc).physical_address;
        let num_pages = (*desc).num_pages;

        let mut i = 0;
        while i < num_pages {
            kernel_space.allocate(paddr + KERNEL_VMA, paddr);

            i += 1;
            paddr += PAGE_SIZE;
        }

        ptr += desc_size;
        desc = ptr as *const MemoryDescriptor;
    }

    // Allocate framebuffer
    let mut paddr = ((*gmode).framebuffer as usize) & !(PAGE_SIZE - 1);
    let top = paddr + (*gmode).framebuffer_size;

    while paddr < top {
        kernel_space.allocate(paddr + KERNEL_VMA, paddr);
        paddr += PAGE_SIZE;
    }

    // Set current address space
    KERNEL_ADDRESS_SPACE = kernel_space;
    KERNEL_ADDRESS_SPACE.set_as_current();

    // Set the page fault handler
    install_exception_handler(0xE, page_fault_handler);
}

pub fn allocate(virtual_address: VirtualAddress, physical_address: PhysicalAddress) {
    let mut current_address_space = unsafe { get_current_address_space() };
    current_address_space.allocate(virtual_address, physical_address)
}

unsafe fn page_fault_handler(_registers: Registers, info: ExceptionInfo) {
    let cr2 = get_cr2();

    if (info.error_code & 1) == 0 {
        if cr2 < PAGE_SIZE {
            match process::get_current_thread_option_cli() {
                Some(_) => process::exit_process(129 + 32),
                None => {
                    let rip = info.rip;
                    panic!("Null pointer exception at {:#X}", rip);
                }
            }
        } else {
            let mut current_address_space = get_current_address_space();
            current_address_space.allocate(cr2, physical::allocate());
        }
    } else {
        process::exit_process(129 + 33);
    }
}

impl AddressSpace {
    pub fn new() -> Self {
        let phys = unsafe { PML4::new() };
        let new_pml4 = (phys + KERNEL_VMA) as *mut PML4;
        let kernel_pml4 = (unsafe { KERNEL_ADDRESS_SPACE.0 } + KERNEL_VMA) as *mut PML4;
        let mut i = 256;
        while i < 512 {
            unsafe {
                (*new_pml4).entries[i] = (*kernel_pml4).entries[i];
            }

            i += 1;
        }

        AddressSpace(phys)
    }

    pub fn get(&self) -> &PhysicalAddress {
        &self.0
    }

    pub fn allocate(&mut self, virtual_address: VirtualAddress, physical_address: PhysicalAddress) {
        let index = PageIndex::new(virtual_address);

        let user = virtual_address < KERNEL_VMA;

        if self.0 >= KERNEL_VMA {
            panic!("Address space should be a physical address!");
        }

        unsafe {
            // Check PDPT
            let pml4 = (self.0 + KERNEL_VMA) as *mut PML4;
            if (*pml4).entries[index.pml4_index] & PAGE_FLAG_PRESENT == 0 {
                let new_pdpt = PDPT::new();
                (*pml4).set_entry(index.pml4_index, new_pdpt, user);
            }

            // Check page directory
            let pdpt = (*pml4).get_entry(index.pml4_index);
            if (*pdpt).entries[index.pdpt_index] & PAGE_FLAG_PRESENT == 0 {
                let new_page_directory = PageDirectory::new();
                (*pdpt).set_entry(index.pdpt_index, new_page_directory, user);
            }

            // Check page table
            let page_directory = (*pdpt).get_entry(index.pdpt_index);
            if (*page_directory).entries[index.page_directory_index] & PAGE_FLAG_PRESENT == 0 {
                let new_page_table = PageTable::new();
                (*page_directory).set_entry(index.page_directory_index, new_page_table, user);
            }

            // Check page
            let page_table = (*page_directory).get_entry(index.page_directory_index);
            if (*page_table).entries[index.page_table_index] & PAGE_FLAG_PRESENT == 0 {
                (*page_table).set_entry(index.page_table_index, physical_address, user);
            }
        }
    }

    pub fn set_as_current(&self) {
        unsafe {
            set_current_pml4(self.0);
        }
    }

    pub unsafe fn free(&mut self) {
        let pml4 = (self.0 + KERNEL_VMA) as *mut PML4;

        let mut i = 0;
        while i < 256 {
            let entry = (*pml4).get_entry(i);
            if entry != null_mut() {
                (*entry).physical_drop();
            }

            i += 1;
        }

        physical::free(self.0);
    }
}

impl<T: PhysicalDrop> Table<T> {
    pub unsafe fn new() -> PhysicalAddress {
        let phys = physical::allocate();
        let new_table = (phys + KERNEL_VMA) as *mut Table<T>;

        let mut i = 0;
        while i < 512 {
            (*new_table).clear_entry(i);

            i += 1;
        }

        phys
    }

    pub fn get_entry(&self, index: usize) -> *mut T {
        if self.entries[index] == 0 {
            null_mut()
        } else {
            ((self.entries[index] & !(PAGE_SIZE - 1)) + KERNEL_VMA) as *mut T
        }
    }

    pub fn set_entry(&mut self, index: usize, address: PhysicalAddress, user: bool) {
        let mut entry = address & !(PAGE_SIZE - 1);
        entry |= PAGE_FLAG_PRESENT | PAGE_FLAG_WRITABLE;

        if user {
            entry |= PAGE_FLAG_USER;
        }

        self.entries[index] = entry;
    }

    pub fn clear_entry(&mut self, index: usize) {
        self.entries[index] = 0;
    }
}

impl<T: PhysicalDrop> Index<usize> for Table<T> {
    type Output = usize;
    fn index<'a>(&'a self, index: usize) -> &'a usize {
        &self.entries[index]
    }
}

impl<T: PhysicalDrop> IndexMut<usize> for Table<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T: PhysicalDrop> PhysicalDrop for Table<T> {
    unsafe fn physical_drop(&mut self) {
        let mut i = 0;
        while i < 512 {
            if self.entries[i] != 0 {
                let entry = self.get_entry(i);
                (*entry).physical_drop();

                self.clear_entry(i);
            }

            i += 1;
        }

        physical::free(self as *const _ as usize - KERNEL_VMA);
    }
}

impl PhysicalDrop for Page {
    unsafe fn physical_drop(&mut self) {
        let addr = self as *const _ as usize - KERNEL_VMA;
        for i in self {
            *i = 0;
        }

        physical::free(addr);
    }
}

impl PageIndex {
    pub fn new(address: VirtualAddress) -> Self {
        let correct = address & 0x0000FFFFFFFFFFFF;

        PageIndex {
            _offset: (correct & 0xFFF) as usize,
            page_table_index: ((correct.wrapping_shr(12)) & 0x1FF) as usize,
            page_directory_index: ((correct.wrapping_shr(21)) & 0x1FF) as usize,
            pdpt_index: ((correct.wrapping_shr(30)) & 0x1FF) as usize,
            pml4_index: ((correct.wrapping_shr(39)) & 0x1FF) as usize,
        }
    }
}
