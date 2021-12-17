use crate::{
    device::{inb, outb},
    log, logln, time,
};

#[repr(u8)]
enum Register {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    Day = 0x07,
    Month = 0x08,
    Year = 0x09,
    StatusA = 0x0A,
    StatusB = 0x0B,
}

const MONTH_TO_DAYS: [isize; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

fn read_register(register: Register) -> u8 {
    unsafe { crate::critical::enter_local() };

    outb(0x70, register as u8);
    let ret = inb(0x71);

    unsafe { crate::critical::leave_local() };

    ret
}

fn is_leap_year(year: isize) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}

pub fn initialize() {
    log!("Initializing RTC . . . ");

    while read_register(Register::StatusA) & 0x80 == 0 {}

    time::sync_offset();

    while read_register(Register::StatusA) & 0x80 != 0 {}

    let mut second = read_register(Register::Seconds) as isize;
    let mut minute = read_register(Register::Minutes) as isize;
    let mut hour = read_register(Register::Hours) as isize;
    let mut day = read_register(Register::Day) as isize;
    let mut month = read_register(Register::Month) as isize;
    let mut year = read_register(Register::Year) as isize;

    let reg_b = read_register(Register::StatusB);
    if reg_b & 0x04 == 0 {
        // Convert from BCD
        second = (second & 0x0F) + ((second / 16) * 10);
        minute = (minute & 0x0F) + ((minute / 16) * 10);
        hour = (hour & 0x0F) + (((hour & 0x70) / 16) * 10) | (hour & 0x80);
        day = (day & 0x0F) + ((day / 16) * 10);
        month = (month & 0x0F) + ((month / 16) * 10);
        year = (year & 0x0F) + ((year / 16) * 10);
    }

    if reg_b & 0x02 == 0 {
        if hour & 0x80 == 1 {
            hour = ((hour & 0x7F) + 12) % 24;
        }
    }

    year += 2000;
    month -= 1;
    day -= 1;

    let mut yday = MONTH_TO_DAYS[month as usize] + day;
    if is_leap_year(year) && month > 1 {
        yday += 1;
    }

    let mut new_epoch = second + minute * 60 + hour * 3600 + yday * 86400;

    for year in 1970..year {
        if is_leap_year(year) {
            new_epoch += 366 * 86400;
        } else {
            new_epoch += 365 * 86400;
        }
    }

    time::set_epoch_time(new_epoch);

    logln!("OK!");
}
