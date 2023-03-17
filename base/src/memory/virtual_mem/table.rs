use super::{page::Page, physical_to_virtual, ptr_to_physical};
use crate::{
    memory::{physical::PhysicalMemoryManager, MemoryUsage, PAGE_MASK, TABLE_ENTRIES},
    MemoryManager,
};
use core::{marker::PhantomData, ptr::NonNull};

pub(super) type PML4 = Table<PDPT>;
pub(super) type PDPT = Table<PD>;
pub(super) type PD = Table<PT>;
pub(super) type PT = Table<Page>;

pub(super) trait PhysicalDrop {
    fn physical_drop(
        &mut self,
        physical_manager: &mut PhysicalMemoryManager,
        usage: &mut MemoryUsage,
    );
}

#[repr(packed)]
pub(super) struct Table<T: PhysicalDrop> {
    entries: [u64; TABLE_ENTRIES],
    phantom: PhantomData<T>,
}

const PAGE_FLAG_PRESENT: u64 = 1 << 0;
const PAGE_FLAG_WRITABLE: u64 = 1 << 1;
const PAGE_FLAG_USER: u64 = 1 << 2;

impl<T: PhysicalDrop> Table<T> {
    pub(super) fn new(physical_manager: &mut PhysicalMemoryManager) -> NonNull<Self> {
        let physical = physical_manager.allocate();
        let mut ptr = physical_to_virtual::<Self>(physical).unwrap();

        for i in 0..512 {
            unsafe { ptr.as_mut().clear_entry(i) };
        }

        ptr
    }

    pub(super) fn clone_entry(&mut self, index: usize, other: &Self) {
        self.entries[index] = other.entries[index];
    }

    pub(super) fn set_entry(&mut self, index: usize, ptr: *mut T, user: bool, write: bool) {
        let mut entry = *ptr_to_physical(ptr) as u64;
        entry |= PAGE_FLAG_PRESENT;

        if write {
            entry |= PAGE_FLAG_WRITABLE;
        }

        if user {
            entry |= PAGE_FLAG_USER;
        }

        self.entries[index] = entry;
    }

    fn get_entry(&self, index: usize) -> Option<NonNull<T>> {
        physical_to_virtual((self.entries[index] as usize & PAGE_MASK).into())
    }

    fn clear_entry(&mut self, index: usize) {
        self.entries[index] = 0;
    }
}

impl<T: PhysicalDrop> PhysicalDrop for Table<T> {
    fn physical_drop(
        &mut self,
        physical_manager: &mut PhysicalMemoryManager,
        usage: &mut MemoryUsage,
    ) {
        for i in 0..512 {
            self.get_entry(i)
                .map(|mut entry| unsafe { entry.as_mut().physical_drop(physical_manager, usage) });
            self.clear_entry(i);
        }
    }
}

impl<T: PhysicalDrop> Drop for Table<T> {
    fn drop(&mut self) {
        let memory_manager = MemoryManager::get();

        let mut physical_manager = memory_manager.virtual_manager().physical_manager().lock();
        let mut usage = memory_manager.usage().lock();

        self.physical_drop(&mut physical_manager, &mut usage);
    }
}
