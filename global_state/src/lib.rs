#![no_std]
use base::{CriticalLock, InterruptController, MemoryManager};

pub struct GlobalState {
    interrupt_controller: &'static CriticalLock<InterruptController>,
    memory_manager: &'static MemoryManager,
}

//static mut GLOBAL_STATE: Option<Arc<GlobalState>> = None;

impl GlobalState {
    pub fn initialize(memory_map: *const uefi::memory::raw::MemoryMap) {
        //assert!(unsafe { GLOBAL_STATE.is_none() });

        // Initialize static entities (IDT & Memory manager)
        let interrupt_controller = InterruptController::get();
        interrupt_controller.lock().initialize();

        let memory_manager = MemoryManager::get();
        memory_manager.initialize(memory_map);

        // Create global state
        loop {}

        /*
        *unsafe { &mut GLOBAL_STATE } = Some(Arc::new(GlobalState {
            interrupt_controller,
            memory_manager,
        }));
        */
    }

    /*pub fn get() -> &'static Arc<GlobalState> {
        unsafe { GLOBAL_STATE.as_ref().unwrap() }
    }*/

    pub fn interrupt_controller(&self) -> &CriticalLock<InterruptController> {
        self.interrupt_controller
    }

    pub fn memory_manager(&self) -> &MemoryManager {
        self.memory_manager
    }
}
