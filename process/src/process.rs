use crate::{
    queue_thread, thread_queue::ThreadQueue, CurrentQueue, ProcessTypes, Thread, ThreadFunction,
};
use alloc::{boxed::Box, string::String, vec::Vec};
use base::{
    map::{Map, Mappable, INVALID_ID},
    multi_owner::{Owner, Reference},
};
use memory::AddressSpace;

pub trait ProcessOwner<T: ProcessTypes> {
    fn new_daemon() -> Self;

    fn insert_process(&mut self, process: Reference<Process<T>>);
    fn drop_process(&mut self, id: isize);
}

pub trait Signals: Clone {
    fn new() -> Self;
}

pub struct Process<T: ProcessTypes + 'static> {
    id: isize,
    threads: Map<Reference<Thread<T>>>,
    address_space: AddressSpace,
    owner: Owner<T::Owner>,
    exit_queue: ThreadQueue<T>,
    exit_status: isize,
    _descriptors: T::Descriptor,
    _process_time: isize,
    _name: String,
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
            _descriptors: descriptors,
            _process_time: 0,
            _name: name,
            signals,
        });

        owner.lock(|owner| owner.insert_process(process.as_ref()));

        process
    }

    pub fn create_thread(
        process: Owner<Self>,
        entry: ThreadFunction,
        context: usize,
    ) -> Owner<Thread<T>> {
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

    pub fn remove_thread(&mut self, id: isize) {
        self.threads.remove(id);
    }

    pub fn set_exit_status(&mut self, exit_status: isize) {
        self.exit_status = exit_status;
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

        self.owner.lock(|owner| owner.drop_process(self.id));

        while let Some(thread) = self.exit_queue.pop() {
            thread.lock(|thread| thread.set_queue_data(self.exit_status));
            queue_thread(thread);
        }
    }
}
