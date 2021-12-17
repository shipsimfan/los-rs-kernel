#[inline(always)]
pub unsafe fn enter_local() {
    asm!("cli");
}

#[inline(always)]
pub unsafe fn leave_local() {
    asm!("sti");
}
