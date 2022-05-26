#![no_std]

use core::{mem::ManuallyDrop, ptr::null_mut};

use alloc::{borrow::ToOwned, boxed::Box, string::String};
use base::{
    log_fatal, log_info,
    multi_owner::{Owner, Reference},
};
use context::{KernelspaceContext, UserspaceContext};
use filesystem::{DirectoryDescriptor, WorkingDirectory, OPEN_READ};
use process::{current_thread, exit_process, Process};
use process_types::ProcessTypes;
use sessions::Session;

mod context;
mod elf;
mod stdio;

pub use stdio::{CStandardIO, StandardIO, StandardIOType};

extern crate alloc;

const MODULE_NAME: &str = "Program Loader";

const TLS_LOCATION: usize = 0x700000000000;
const USERSPACE_CONTEXT_LOCATION: *mut UserspaceContext =
    (TLS_LOCATION - core::mem::size_of::<UserspaceContext>()) as *mut UserspaceContext;

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
                    working_directory.directory().clone(),
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
    Ok(process::create_process_owner::<ProcessTypes>(
        load_process,
        context.as_ref() as *const _ as usize,
        descriptors,
        name,
        inherit_signals,
        session,
    ))
}

fn load_process(context: usize) -> isize {
    // Convert the context
    let mut context =
        unsafe { ManuallyDrop::new(Box::from_raw(context as *mut KernelspaceContext)) };

    // Load the executable
    let (entry, tls_size, tls_align) =
        match elf::load_executable(context.file(), TLS_LOCATION as *mut u8) {
            Ok(result) => result,
            Err(error) => {
                unsafe { ManuallyDrop::drop(&mut context) };
                log_fatal!("Error while loading executable: {}", error);
                exit_process::<ProcessTypes>(error.to_status_code());
            }
        };

    // Determine context locations

    /*  |===========| TLS_LOCATION + tls_size
     *  |    TLS    |
     *  |===========| TLS_LOCATION (0x700000000000)
     *  | Userspace |
     *  |  Context  |
     *  |===========| USERSPACE_CONTEXT_LOCATION
     *  |   Stdio   |
     *  |===========| c_stdio_location
     *  | Args List |
     *  |===========| arg_list_start
     *  | Envs List |
     *  |===========| env_list_start
     *  |  Args...  |
     *  |===========|
     *  |  Envs...  |
     *  |===========|
     *
     *  Args and envs are built downwards, putting the first argument at the top
     */

    let c_stdio_location =
        USERSPACE_CONTEXT_LOCATION as usize - core::mem::size_of::<CStandardIO>();
    let arg_list_start =
        c_stdio_location - (core::mem::size_of::<*mut u8>() * (context.args().len() + 1));
    let env_list_start =
        arg_list_start - (core::mem::size_of::<*mut u8>() * (context.environment().len() + 1));

    let arg_list = unsafe {
        core::slice::from_raw_parts_mut(arg_list_start as *mut *mut u8, context.args().len() + 1)
    };
    let env_list = unsafe {
        core::slice::from_raw_parts_mut(
            env_list_start as *mut *mut u8,
            context.environment().len() + 1,
        )
    };

    // Create the userspace context
    unsafe {
        // Copy stdio
        let c_stdio = c_stdio_location as *mut CStandardIO;
        *c_stdio = context.stdio().into_c();

        // Copy context
        *USERSPACE_CONTEXT_LOCATION = UserspaceContext::new(
            context.args().len(),
            arg_list.as_ptr() as *const *const _,
            env_list.as_ptr() as *const *const _,
            c_stdio,
            tls_size,
            tls_align,
        );

        // Copy arguments
        let mut ptr = env_list_start as *mut u8;
        for i in 0..context.args().len() {
            let arg = &context.args()[i];

            // Move pointer
            ptr = ptr.sub(arg.len() + 1);

            // Set list
            arg_list[i] = ptr;

            // Copy string
            let mut p = ptr;
            for byte in arg.as_bytes() {
                *p = *byte;
                p = p.add(1);
            }
            *p = 0;
        }
        arg_list[context.args().len()] = null_mut();

        // Copy environment variables
        for i in 0..context.environment().len() {
            let env = &context.environment()[i];

            // Move pointer
            ptr = ptr.sub(env.len() + 1);

            // Set list
            env_list[i] = ptr;

            // Copy string
            let mut p = ptr;
            for byte in env.as_bytes() {
                *p = *byte;
                p = p.add(1);
            }
            *p = 0;
        }
        env_list[context.environment().len()] = null_mut();
    }

    // Drop the kernelspace context
    unsafe { ManuallyDrop::drop(&mut context) };

    // Create the userspace thread
    process::create_thread_usize::<ProcessTypes>(entry, USERSPACE_CONTEXT_LOCATION as usize);

    log_info!("Testing");

    0
}
