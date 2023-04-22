use super::{Process, Thread, ThreadQueueGuard};
use crate::{CriticalLock, LocalState, Map, MappableMut, MemoryManager, Queue, StandardError};
use alloc::{
    borrow::Cow,
    sync::{Arc, Weak},
};
use core::arch::{asm, global_asm};

pub struct ProcessManager {
    processes: CriticalLock<Map<u64, (u64, Weak<Process>)>>,
    waiting_queue: CriticalLock<Queue<Thread>>,
}

static PROCESS_MANAGER: ProcessManager = ProcessManager::new();

global_asm!(include_str!("process.asm"));

extern "C" {
    fn switch_stacks(stack_save_location: *const usize, stack_load_location: *const usize);
}

impl ProcessManager {
    pub fn get<'a>() -> &'a ProcessManager {
        &PROCESS_MANAGER
    }

    pub(self) const fn new() -> Self {
        ProcessManager {
            processes: CriticalLock::new(Map::new(0)),
            waiting_queue: CriticalLock::new(Queue::new()),
        }
    }

    pub fn create_process<S: Into<Cow<'static, str>>>(
        &self,
        entry: usize,
        context: usize,
        name: S,
    ) -> Arc<Process> {
        // Create the process
        let mut process_inner = Process::new(name);
        let mut process = None;
        self.processes.lock().insert_f(|id| {
            process_inner.set_id(id);
            process = Some(Arc::new(process_inner));
            (*id, Arc::downgrade(process.as_ref().unwrap()))
        });
        let process = process.unwrap();

        // Create the first thread
        let new_thread = process.create_thread(entry, context);
        self.queue_thread(new_thread);

        process
    }

    pub fn get_process(&self, id: u64) -> Result<Arc<Process>, StandardError> {
        match self.processes.lock().get(id) {
            Some((_, process)) => process.upgrade(),
            None => None,
        }
        .ok_or(StandardError::ProcessNotFound)
    }

    pub fn queue_thread(&self, thread: Thread) {
        if thread.is_killed() {
            return;
        }

        self.waiting_queue.lock().push(thread)
    }

    pub fn r#yield(&self, target_queue: Option<ThreadQueueGuard>) {
        unsafe {
            {
                let local_state = LocalState::get();
                let critical_state = local_state.critical_state();

                let key = if target_queue.is_some() {
                    critical_state.enter()
                } else {
                    critical_state.enter_assert()
                };

                let mut local_controller = LocalState::get().process_controller().borrow_mut();
                local_controller.set_key(key);
                local_controller.set_target_queue(target_queue);
            }

            loop {
                let (stack_save_location, stack_load_location) = match self.before_stack_switch() {
                    Some(value) => value,
                    None => continue,
                };

                // Switch stacks
                switch_stacks(stack_save_location, stack_load_location);

                let mut process_controller = LocalState::get().process_controller().borrow_mut();

                // Switch current thread and queue the old thread
                process_controller.set_next_thread_and_queue_old();

                // If we are not in the null thread, return
                if process_controller.current_thread_opt().is_some() {
                    break;
                }
            }

            LocalState::get().critical_state().leave(
                LocalState::get()
                    .process_controller()
                    .borrow_mut()
                    .take_key(),
            );
        }
    }

    pub(super) fn remove_process(&self, id: u64) {
        self.processes.lock().remove(&id);
    }

    fn get_next_thread(&self) -> Option<Thread> {
        let mut waiting_queue = self.waiting_queue.lock();

        while let Some(thread) = waiting_queue.pop() {
            if !thread.is_killed() {
                return Some(thread);
            }
        }

        None
    }

    unsafe fn before_stack_switch(&self) -> Option<(*mut usize, *const usize)> {
        let local_state = LocalState::get();

        let mut local_controller = local_state.process_controller().borrow_mut();

        // Get the next thread
        //  If none, use the null thread
        let next_thread = match self.get_next_thread() {
            Some(thread) => Some(thread),
            None => {
                // If we are currently in the null thread, wait for a thread to queue
                if local_controller.current_thread_opt().is_none() {
                    local_state
                        .critical_state()
                        .leave(local_controller.take_key());
                    drop(local_controller);

                    asm!("hlt");

                    let key = local_state.critical_state().enter_assert();
                    local_controller = local_state.process_controller().borrow_mut();
                    local_controller.set_key(key);

                    return None;
                } else {
                    None
                }
            }
        };

        let current_thread = local_controller.current_thread_opt();

        assert!(current_thread.is_some() || next_thread.is_some());

        // Save & load floats
        match current_thread {
            Some(thread) => thread.save_float(),
            None => local_controller.save_float(),
        }

        match &next_thread {
            Some(thread) => thread.load_float(),
            None => local_controller.load_float(),
        }

        // Load new address space
        match &next_thread {
            Some(thread) => thread.process().address_space().set_as_active(),
            None => MemoryManager::get().kernel_address_space().set_as_active(),
        }

        // Switch the interrupt stack
        match &next_thread {
            Some(thread) => thread.set_interrupt_stack(),
            None => local_controller.set_interrupt_stack(),
        }

        // Get the stack save & load locations
        let stack_save_location = match current_thread {
            Some(thread) => thread.stack_pointer_location(),
            None => local_controller.null_stack_pointer_location(),
        };

        let stack_load_location = match &next_thread {
            Some(thread) => thread.stack_pointer_location(),
            None => local_controller.null_stack_pointer_location(),
        } as *const usize;

        // Set the next thread
        local_controller.set_next_thread(next_thread);

        Some((stack_save_location, stack_load_location))
    }
}
