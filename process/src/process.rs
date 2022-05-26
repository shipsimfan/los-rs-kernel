use crate::{queue_thread, thread_queue::ThreadQueue, CurrentQueue, ProcessTypes, Thread};
use alloc::{boxed::Box, string::String, vec::Vec};
use base::{
    map::{Map, Mappable, INVALID_ID},
    multi_owner::{Owner, Reference},
};
use memory::AddressSpace;

pub trait ProcessOwner<T: ProcessTypes> {
    fn insert_process(&mut self, process: Reference<Process<T>>);
    fn remove_process(&mut self, id: isize);
}

pub trait Signals: Clone {
    type UserspaceContext;

    fn new() -> Self;

    fn handle(&mut self, userspace_context: (Self::UserspaceContext, u64)) -> SignalHandleReturn;
}

pub trait Descriptors {
    fn working_directory_string(&self) -> String;
}

pub enum SignalHandleReturn {
    None,
    Kill(isize),
    Userspace(u64, usize, u64),
}

pub struct Process<T: ProcessTypes + 'static> {
    id: isize,
    threads: Map<Reference<Thread<T>>>,
    address_space: AddressSpace,
    owner: Owner<T::Owner>,
    exit_queue: ThreadQueue<T>,
    exit_status: isize,
    descriptors: T::Descriptor,
    process_time: isize,
    name: String,
    signals: T::Signals,
}

impl<T: ProcessTypes> Process<T> {
    pub fn new(
        owner: Owner<T::Owner>,
        descriptors: T::Descriptor,
        signals: T::Signals,
        name: String,
    ) -> Owner<Self> {
        let process = Owner::new(Process {
            id: INVALID_ID,
            threads: Map::new(),
            address_space: AddressSpace::new(),
            owner: owner.clone(),
            exit_queue: ThreadQueue::new(),
            exit_status: 128, // Random exit
            descriptors,
            process_time: 0,
            name: name,
            signals,
        });

        owner.lock(|owner| owner.insert_process(process.as_ref()));

        process
    }

    pub fn create_thread(process: Owner<Self>, entry: usize, context: usize) -> Owner<Thread<T>> {
        let thread = Thread::new(process.clone(), entry, context);
        process.lock(|process| process.threads.insert(thread.as_ref()));
        thread
    }

    pub fn owner(&self) -> &Owner<T::Owner> {
        &self.owner
    }

    pub fn signals(&self) -> &T::Signals {
        &self.signals
    }

    pub fn set_address_space_as_current(&self) {
        self.address_space.set_as_current();
    }

    pub fn threads(&self) -> Box<[Reference<Thread<T>>]> {
        let mut threads = Vec::with_capacity(self.threads.len());
        for thread in self.threads.iter() {
            threads.push(thread.clone());
        }

        threads.into_boxed_slice()
    }

    pub fn exit_queue(&self) -> CurrentQueue<T> {
        self.exit_queue.current_queue()
    }

    pub fn descriptors(&self) -> &T::Descriptor {
        &self.descriptors
    }

    pub fn time(&self) -> isize {
        self.process_time
    }

    pub fn get_thread(&self, id: isize) -> Option<&Reference<Thread<T>>> {
        self.threads.get(id)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn descriptors_mut(&mut self) -> &mut T::Descriptor {
        &mut self.descriptors
    }

    pub fn signals_mut(&mut self) -> &mut T::Signals {
        &mut self.signals
    }

    pub fn handle_signals(
        &mut self,
        userspace_context: (
            <<T as ProcessTypes>::Signals as Signals>::UserspaceContext,
            u64,
        ),
    ) -> SignalHandleReturn {
        self.signals.handle(userspace_context)
    }

    pub fn remove_thread(&mut self, id: isize) {
        self.threads.remove(id);
    }

    pub fn set_exit_status(&mut self, exit_status: isize) {
        self.exit_status = exit_status;
    }

    pub fn increase_time(&mut self, amount: isize) {
        self.process_time += amount;
    }
}

impl<T: ProcessTypes> Mappable for Process<T> {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id;
    }
}

impl<T: ProcessTypes> Drop for Process<T> {
    fn drop(&mut self) {
        unsafe { self.address_space.free() };

        self.owner.lock(|owner| owner.remove_process(self.id));

        while let Some(thread) = self.exit_queue.pop() {
            thread.lock(|thread| thread.set_queue_data(self.exit_status));
            queue_thread(thread);
        }
    }
}
