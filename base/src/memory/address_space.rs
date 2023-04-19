use super::{page_tables::PML4, KERNEL_PML4};
use crate::CriticalLock;
#[repr(C)]
pub struct AddressSpace(CriticalLock<PML4>);

impl AddressSpace {
    pub fn new() -> Self {
        let mut new_pml4 = PML4::null();
        let kernel_pml4 = KERNEL_PML4.lock();

        for i in 256..512 {
            new_pml4.set_entry(i, kernel_pml4[i]);
        }

        AddressSpace(CriticalLock::new(new_pml4))
    }
}
