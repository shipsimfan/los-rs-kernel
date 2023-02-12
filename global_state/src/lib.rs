#![no_std]
use base::{CriticalLock, InterruptController};

pub struct GlobalState {
    interrupt_controller: &'static CriticalLock<InterruptController>,
}

//static mut GLOBAL_STATE: Option<Arc<GlobalState>> = None;

impl GlobalState {
    pub fn initialize() {
        //assert!(unsafe { GLOBAL_STATE.is_none() });

        // Initialize static entities (IDT & Memory manager)
        let interrupt_controller = InterruptController::get();
        interrupt_controller.lock().initialize();

        // Create global state
        loop {}

        /*
        *unsafe { &mut GLOBAL_STATE } = Some(Arc::new(GlobalState {
            interrupt_controller,
        }));
        */
    }

    /*pub fn get() -> &'static Arc<GlobalState> {
        unsafe { GLOBAL_STATE.as_ref().unwrap() }
    }*/

    pub fn interrupt_controller(&self) -> &CriticalLock<InterruptController> {
        self.interrupt_controller
    }
}
