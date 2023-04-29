#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(associated_type_bounds)]
#![feature(alloc_error_handler)]

extern crate alloc;

use base::{
    log_info,
    process::{self, wait_thread},
    CriticalLock, LocalState, LogController, Logger, ProcessManager, GDT, KERNEL_VMA, TSS,
};
use core::{arch::asm, ffi::c_void, fmt::Write, ptr::NonNull};
use uefi::UEFIBootVideo;

mod boot;

static BOOT_VIDEO: CriticalLock<UEFIBootVideo> = CriticalLock::new(UEFIBootVideo::null());

extern "C" {
    static STACK_TOP: c_void;
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn kmain(
    graphics_mode: NonNull<uefi::raw::GraphicsMode>,
    memory_map: NonNull<uefi::raw::MemoryMap>,
    rsdp: NonNull<acpi::RSDP>,
    null_gs_ptr: NonNull<*mut c_void>,
) -> ! {
    // Setup boot video
    BOOT_VIDEO.lock().initialize(graphics_mode);
    LogController::get().set_static_output(&BOOT_VIDEO);

    // Create the GDT
    let tss = TSS::new();
    let gdt = GDT::new(&tss);
    gdt.set_active(null_gs_ptr.as_ptr() as usize);

    // Initialize the base state
    base::initialize::<uefi::MemoryMap, _>(memory_map.into(), &BOOT_VIDEO);

    // Create the local state
    let mut local_state_container = LocalState::new(
        &gdt,
        unsafe { &STACK_TOP } as *const _ as usize + KERNEL_VMA,
    );
    let local_state = local_state_container.set_active();

    // Initialize ACPI
    acpi::initialize(rsdp, &BOOT_VIDEO);

    // Create the kinit process
    let kinit = process::spawn_kernel_process(kinit, 69, "kinit");

    ProcessManager::get().r#yield(None);

    panic!("Return from yield in null thread!");
}

fn kinit(context: usize) -> isize {
    let logger = Logger::from("kinit");
    log_info!(logger, "Context: {}", context);

    let id = process::spawn_kernel_process(thread1, 69, "Test").id();
    let result = process::wait_process(id).unwrap();

    log_info!(logger, "Result: {}", result);

    for _ in 0..50 {
        ProcessManager::get().r#yield(None);
    }

    process::exit_process(1);
}

fn thread1(context: usize) -> isize {
    let logger = Logger::from("Thread 1");
    log_info!(logger, "Context: {}", context);

    let id = process::spawn_kernel_thread(None, thread2, 0);

    for i in 0..10 {
        log_info!(logger, "{}", i);
        ProcessManager::get().r#yield(None);

        if i == 5 {
            let exit_code = wait_thread(None, id).unwrap();
            log_info!(logger, "Thread 2 Exit Code: {}", exit_code)
        }
    }

    420
}

fn thread2(context: usize) -> isize {
    let logger = Logger::from("Thread 2");
    log_info!(logger, "Context: {}", context);

    let mut i = 1;
    while i <= 2u32.pow(10) {
        log_info!(logger, "{}", i);
        i *= 2;
        ProcessManager::get().r#yield(None);
    }

    13
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut boot_video = BOOT_VIDEO.lock();

    match info.message() {
        Some(msg) => {
            writeln!(boot_video, "PANIC: {}", msg).ok();
            match info.location() {
                Some(location) => {
                    writeln!(boot_video, "\tLocated at {}", location).ok();
                }
                None => {}
            }
        }
        None => {
            writeln!(boot_video, "{}", info).ok();
        }
    }

    infinite_loop()
}

fn infinite_loop() -> ! {
    loop {
        unsafe { asm!("cli; hlt") };
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    // Cannot print or panic as that requires allocations
    infinite_loop()
}
