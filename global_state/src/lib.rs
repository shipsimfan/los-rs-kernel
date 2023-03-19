#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use base::{BootVideo, CriticalLock, InterruptController, Level, Logger, MemoryManager, MemoryMap};

pub struct GlobalState {
    interrupt_controller: &'static CriticalLock<InterruptController>,
    memory_manager: &'static MemoryManager,
}

static mut GLOBAL_STATE: Option<Arc<GlobalState>> = None;

impl GlobalState {
    pub fn initialize<M: MemoryMap, B: BootVideo>(memory_map: M, boot_video: &CriticalLock<B>) {
        //assert!(unsafe { GLOBAL_STATE.is_none() });

        let logger = Logger::new("Global State");
        logger.log(Level::Info, "Initializing");

        // Initialize static entities (IDT & Memory manager)
        let interrupt_controller = InterruptController::get();
        interrupt_controller.lock().initialize();

        let memory_manager = MemoryManager::get();
        let framebuffer_memory = boot_video.lock().framebuffer_memory();
        memory_manager.initialize(memory_map, framebuffer_memory);

        // Create global state
        let global_state = Arc::new(GlobalState {
            interrupt_controller,
            memory_manager,
        });
        *unsafe { &mut GLOBAL_STATE } = Some(global_state.clone());

        logger.log(Level::Info, "Global state initialized");
    }

    pub fn get() -> &'static Arc<GlobalState> {
        unsafe { GLOBAL_STATE.as_ref().unwrap() }
    }

    pub fn interrupt_controller(&self) -> &CriticalLock<InterruptController> {
        self.interrupt_controller
    }

    pub fn memory_manager(&self) -> &MemoryManager {
        self.memory_manager
    }
}
