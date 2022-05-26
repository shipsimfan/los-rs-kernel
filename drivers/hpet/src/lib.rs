#![no_std]

use alloc::boxed::Box;
use base::{error::HPET_DRIVER_MODULE_NUMBER, log_error, log_info, multi_owner::Owner};
use device::Device;
use memory::KERNEL_VMA;
use process::ProcessTypes;

extern crate alloc;

struct HPET;

#[derive(Debug)]
struct NotSupportedError;

const MODULE_NAME: &str = "HPET";

const GENERAL_CAPABILITIES_REG: isize = 0x00 / 8;
const GENERAL_CONFIG_REG: isize = 0x10 / 8;
const MAIN_COUNTER_REG: isize = 0xF0 / 8;

const TIMER_IRQ: u8 = 0;

static mut MILLISECOND_TICK: Option<unsafe fn()> = None;

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . .");

    // Get the HPET table
    let hpet = match acpi::get_table::<acpi::HPET, T>() {
        Some(table) => table,
        None => return log_error!("\nFailed to get table"),
    };

    // Verify address space
    if hpet.address.address_space_id != 0 {
        return log_error!(
            "\nInvalid address space ({})",
            hpet.address.address_space_id
        );
    }

    // Save and allocate hpet address
    let address = hpet.address.address as usize + KERNEL_VMA;
    memory::map_virtual_memory(address, hpet.address.address as usize);
    let address = address as *mut u64;

    // Disable HPET
    unsafe { *(address.offset(GENERAL_CONFIG_REG)) = 0 };

    // Save minimum tick
    let minimun_tick = unsafe { *(address.offset(GENERAL_CAPABILITIES_REG)) >> 32 & 0xFFFFFFFF };

    // Setup IRQ
    if !interrupts::irqs::install_irq_handler(0, irq_handler, 0) {
        return log_error!("\nFailed to install IRQ!");
    }

    // Initialize timers
    let num_timers =
        ((unsafe { *address.offset(GENERAL_CAPABILITIES_REG) >> 8 } & 0x01) + 1) as isize;
    let mut i: isize = 0;
    while i < num_timers {
        let mut timer_cap = unsafe { *address.offset(timer_config_reg(i)) };
        timer_cap &= !(1 << 14); // Disable FSB
        timer_cap &= !(1 << 3); // Disable periodic
        timer_cap &= !(1 << 2); // Disable interrupts
        unsafe { *address.offset(timer_config_reg(i)) = timer_cap };

        i += 1;
    }

    // Create and register the timer device
    if device::register_device::<T>("/hpet", Owner::new(Box::new(HPET) as Box<dyn Device>)).is_err()
    {
        return log_error!("Failed to register device!");
    }

    match time::register_system_timer::<T>("/hpet") {
        Ok(millisecond_tick) => unsafe { MILLISECOND_TICK = Some(millisecond_tick) },
        Err(_) => return log_error!("Failed to register HPET as system timer!"),
    }

    // Start the timer
    let timer_val = 1000000000000 / minimun_tick;
    unsafe { *address.offset(GENERAL_CONFIG_REG) = 3 };

    // Start the millisecond clock
    unsafe {
        *address.offset(timer_config_reg(0)) = TIMER_IRQ as u64 | (1 << 2) | (1 << 3) | (1 << 6);
        *address.offset(timer_compare_reg(0)) = *address.offset(MAIN_COUNTER_REG) + timer_val;
        *address.offset(timer_compare_reg(0)) = timer_val;
    }

    log_info!("Initialized!");
}

const fn timer_config_reg(n: isize) -> isize {
    (0x100 + 0x20 * n) / 8
}

const fn timer_compare_reg(n: isize) -> isize {
    (0x108 + 0x20 * n) / 8
}

unsafe fn irq_handler(_context: usize) {
    match MILLISECOND_TICK {
        Some(millisecond_tick) => millisecond_tick(),
        None => {}
    }
}

impl Device for HPET {
    fn read(&self, _: usize, _: &mut [u8]) -> base::error::Result<usize> {
        Err(Box::new(NotSupportedError))
    }

    fn write(&mut self, _: usize, _: &[u8]) -> base::error::Result<usize> {
        Err(Box::new(NotSupportedError))
    }

    fn read_register(&mut self, _: usize) -> base::error::Result<usize> {
        Err(Box::new(NotSupportedError))
    }

    fn write_register(&mut self, _: usize, _: usize) -> base::error::Result<()> {
        Err(Box::new(NotSupportedError))
    }

    fn ioctrl(&mut self, _: usize, _: usize) -> base::error::Result<usize> {
        Err(Box::new(NotSupportedError))
    }
}

impl base::error::Error for NotSupportedError {
    fn module_number(&self) -> i32 {
        HPET_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::NotSupported
    }
}

impl core::fmt::Display for NotSupportedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HPET does not support interactions")
    }
}
