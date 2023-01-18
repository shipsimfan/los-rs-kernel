#![no_std]

extern crate alloc;

mod error;
mod kind;

pub use error::Error;
pub use kind::ErrorKind;
