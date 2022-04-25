use core::arch::asm;

#[no_mangle]
pub static mut LOCAL_CRITICAL_COUNT: usize = 0;

#[inline(always)]
pub unsafe fn enter_local() {
    asm!("cli");
    LOCAL_CRITICAL_COUNT += 1;
}

#[inline(always)]
pub unsafe fn leave_local() {
    leave_local_without_sti();
    if LOCAL_CRITICAL_COUNT == 0 {
        asm!("sti");
    }
}

#[inline(always)]
pub unsafe fn leave_local_without_sti() {
    LOCAL_CRITICAL_COUNT -= 1;
}
