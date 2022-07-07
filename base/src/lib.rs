#![no_std]
#![feature(map_first_last)]
#![feature(generic_associated_types)]

pub mod bootloader;
pub mod critical;
pub mod error;
pub mod hash_map;
pub mod logging;
pub mod map;
pub mod multi_owner;
pub mod pinned_box;
pub mod queue;

extern crate alloc;

pub const MODULE_NAME: &str = "Base";
