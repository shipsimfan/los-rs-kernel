use crate::stdio::{CStandardIO, StandardIO};
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use core::mem::ManuallyDrop;
use filesystem::FileDescriptor;
use process_types::ProcessTypes;

#[repr(packed(1))]
#[allow(unused)]
pub struct UserspaceContext {
    argc: usize,
    argv: *const *const u8,
    envp: *const *const u8,
    stdio: *const CStandardIO,
    tls_size: usize,
    tls_align: usize,
}

#[allow(unused)]
pub struct KernelspaceContext {
    file: FileDescriptor<ProcessTypes>,
    args: Vec<String>,
    environment: Vec<String>,
    stdio: StandardIO,
}

impl UserspaceContext {
    pub fn new(
        argc: usize,
        argv: *const *const u8,
        envp: *const *const u8,
        stdio: *const CStandardIO,
        tls_size: usize,
        tls_align: usize,
    ) -> Self {
        UserspaceContext {
            argc,
            argv,
            envp,
            stdio,
            tls_size,
            tls_align,
        }
    }
}

impl KernelspaceContext {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(
        file: FileDescriptor<ProcessTypes>,
        args: &[S1],
        environment: &[S2],
        stdio: StandardIO,
    ) -> ManuallyDrop<Box<Self>> {
        let mut args_vec = Vec::with_capacity(args.len());
        for arg in args {
            args_vec.push(arg.as_ref().to_owned());
        }

        let mut environment_vec = Vec::with_capacity(environment.len());
        for var in environment {
            environment_vec.push(var.as_ref().to_owned());
        }

        ManuallyDrop::new(Box::new(KernelspaceContext {
            file,
            args: args_vec,
            environment: environment_vec,
            stdio,
        }))
    }

    pub fn stdio(&self) -> &StandardIO {
        &self.stdio
    }

    pub fn args(&self) -> &[String] {
        self.args.as_slice()
    }

    pub fn environment(&self) -> &[String] {
        self.environment.as_slice()
    }

    pub fn file(&mut self) -> &mut FileDescriptor<ProcessTypes> {
        &mut self.file
    }
}
