#![no_std]

extern crate alloc;

use alloc::sync::Arc;

pub struct GlobalState {}

impl GlobalState {
    pub fn new() -> Arc<Self> {
        Arc::new(GlobalState {})
    }
}
