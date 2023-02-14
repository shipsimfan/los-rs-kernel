use super::IDT;

extern "C" {
    fn load_idt(idtr: *const IDTR);
}

#[repr(packed, C)]
#[allow(unused)]
pub(super) struct IDTR {
    limit: u16,
    address: *const IDT,
}

impl IDTR {
    pub(super) fn load_idt(idt: &IDT) {
        unsafe {
            load_idt(&IDTR {
                limit: core::mem::size_of::<IDT>() as u16,
                address: idt,
            })
        };
    }
}
