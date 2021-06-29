use crate::{
    device::{self, acpi, Device},
    error, interrupts,
    locks::Mutex,
    log, logln,
    memory::KERNEL_VMA,
};
use alloc::{boxed::Box, sync::Arc};

struct HPET;

const GENERAL_CAPABILITIES_REG: isize = 0x00 / 8;
const GENERAL_CONFIG_REG: isize = 0x10 / 8;
const MAIN_COUNTER_REG: isize = 0xF0 / 8;

const TIMER_IRQ: u8 = 0;

const fn timer_config_reg(n: isize) -> isize {
    (0x100 + 0x20 * n) / 8
}

const fn timer_compare_reg(n: isize) -> isize {
    (0x108 + 0x20 * n) / 8
}

pub fn initialize() {
    log!("Initializing HPET . . . ");

    // Get the HPET table
    let hpet: &acpi::HPET = match acpi::get_table() {
        Err(err) => return logln!("\nFailed to get HPET table: {}", err),
        Ok(table) => table,
    };

    // Verify address space
    if hpet.address.address_space_id != 0 {
        logln!(
            "\nInvalid HPET address space({})",
            hpet.address.address_space_id
        );
        return;
    }

    // Save and allocate hpet address
    let address = hpet.address.address as usize + KERNEL_VMA;
    crate::memory::map_virtual_memory(address, hpet.address.address as usize);
    let address = address as *mut u64;

    // Disable HPET
    unsafe { *(address.offset(GENERAL_CONFIG_REG)) = 0 };

    // Save minimum tick
    let minimun_tick = unsafe { *(address.offset(GENERAL_CAPABILITIES_REG)) >> 32 & 0xFFFFFFFF };

    // Setup IRQ
    if !interrupts::irq::install_irq_handler(0, irq_handler, 0) {
        logln!("\nFailed to install HPET IRQ!");
        return;
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
    if device::register_device("/hpet", Arc::new(Mutex::new(Box::new(HPET {})))).is_err() {
        logln!("Failed to register HPET device!");
        return;
    }

    if crate::time::register_system_timer("/hpet").is_err() {
        logln!("Failed to register HPET as system timer!");
        return;
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

    logln!("\x1B2A2]OK\x1B]!");
}

fn irq_handler(_context: usize) {
    crate::interrupts::irq::end_irq(TIMER_IRQ);
    crate::time::millisecond_tick();
}

impl Device for HPET {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn read_register(&self, _: usize) -> Result<usize, error::Status> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&self, _: usize, _: usize) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn ioctrl(&mut self, _: usize, _: usize) -> error::Result {
        Err(error::Status::NotSupported)
    }
}
