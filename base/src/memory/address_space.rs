use super::{
    page_tables::{PDPT, PML4},
    KERNEL_ADDRESS_SPACE,
};
use crate::{CriticalLock, PhysicalAddress};
#[repr(C)]
pub struct AddressSpace(CriticalLock<PML4>);

extern "C" {
    fn set_cr3(cr3: usize);
}

impl AddressSpace {
    pub(super) const fn null() -> Self {
        AddressSpace(CriticalLock::new(PML4::null()))
    }

    pub fn new() -> Self {
        let mut new_pml4 = PML4::null();
        let kernel_pml4 = KERNEL_ADDRESS_SPACE.0.lock();

        for i in 256..512 {
            new_pml4.set_entry(i, kernel_pml4[i]);
        }

        AddressSpace(CriticalLock::new(new_pml4))
    }

    pub(crate) unsafe fn set_as_active(&self) {
        set_cr3(PhysicalAddress::new(self.0.lock().as_ptr()).into_usize())
    }

    pub(super) fn identity_map(&self, pdpts: &mut [PDPT]) {
        self.0.lock().identity_map(pdpts)
    }
}
