#![no_std]

use core::mem::ManuallyDrop;

use alloc::{borrow::ToOwned, boxed::Box, string::String};
use base::{
    log_info,
    multi_owner::{Owner, Reference},
};
use context::KernelspaceContext;
use filesystem::{DirectoryDescriptor, WorkingDirectory, OPEN_READ};
use process::{current_thread, Process};
use process_types::ProcessTypes;
use sessions::Session;

mod context;
mod elf;
mod stdio;

pub use stdio::{StandardIO, StandardIOType};

extern crate alloc;

const MODULE_NAME: &str = "Program Loader";

pub fn execute_session<S1: AsRef<str>, S2: AsRef<str>>(
    filepath: &str,
    args: &[S1],
    environment: &[S2],
    standard_io: StandardIO,
    session: Owner<Box<dyn Session<ProcessTypes>>>,
    inherit_signals: bool,
) -> base::error::Result<Reference<Process<ProcessTypes>>> {
    do_execute::<S1, S2>(
        filepath,
        args,
        environment,
        standard_io,
        session,
        inherit_signals,
    )
}

pub fn execute<S1: AsRef<str>, S2: AsRef<str>>(
    filepath: &str,
    args: &[S1],
    environment: &[S2],
    standard_io: StandardIO,
    inherit_signals: bool,
) -> base::error::Result<Reference<Process<ProcessTypes>>> {
    let session = process::current_thread::<ProcessTypes>()
        .lock(|thread| thread.process().lock(|process| process.owner().clone()));
    do_execute::<S1, S2>(
        filepath,
        args,
        environment,
        standard_io,
        session,
        inherit_signals,
    )
}

#[allow(unused)]
fn do_execute<S1: AsRef<str>, S2: AsRef<str>>(
    filepath: &str,
    args: &[S1],
    environment: &[S2],
    standard_io: StandardIO,
    session: Owner<Box<dyn Session<ProcessTypes>>>,
    inherit_signals: bool,
) -> base::error::Result<Reference<Process<ProcessTypes>>> {
    // Open file
    let mut file = filesystem::open::<ProcessTypes>(filepath, OPEN_READ, None)?;

    // Verify executable
    elf::verify_executable(&mut file)?;

    // Determine working directory
    let working_directory = match current_thread::<ProcessTypes>().lock(|thread| {
        thread.process().lock(|process| {
            let descriptors: &process_types::Descriptors<ProcessTypes> = process.descriptors();
            match descriptors.working_directory() {
                Some(working_directory) => Some(DirectoryDescriptor::new(
                    working_directory.get_directory().clone(),
                )),
                None => None,
            }
        })
    }) {
        Some(working_directory) => working_directory,
        None => {
            let mut iter = filepath.split(|c| -> bool { c == '\\' || c == '/' });
            iter.next_back();

            let mut path = String::new();
            while let Some(part) = iter.next() {
                path.push_str(part);
                path.push('/');
            }

            path.pop();
            filesystem::open_directory(&path, None)?
        }
    };

    // Copy descriptors
    let descriptors = standard_io.build_descriptors(working_directory);

    // Build kernel space context
    let context = KernelspaceContext::new(file, args, environment, standard_io);

    // Find name
    let name = filepath.split(&['/', '\\']).last().unwrap().to_owned();

    // Create new thread
    Ok(process::create_process::<ProcessTypes>(
        load_process,
        context.as_ref() as *const _ as usize,
        descriptors,
        name,
        inherit_signals,
    ))
}

fn load_process(context: usize) -> isize {
    // Convert the context
    let mut context =
        unsafe { ManuallyDrop::new(Box::from_raw(context as *mut KernelspaceContext)) };

    // Load the executable

    // Create the userspace context

    log_info!("Testing: {}", context.file().tell());

    // Drop the kernelspace context
    unsafe { ManuallyDrop::drop(&mut context) };

    // Create the userspace thread

    0
}
