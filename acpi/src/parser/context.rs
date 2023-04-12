use super::{Error, Result};
use crate::Path;
use alloc::vec::Vec;
use base::Logger;

pub(super) struct Context {
    logger: Logger,

    current_location: Path,
    location_stack: Vec<Path>,

    method_list: Vec<(Path, u8)>,
}

impl Context {
    pub(super) fn new(logger: Logger) -> Self {
        Context {
            logger,
            current_location: Path::new(crate::PathPrefix::Root, Vec::new(), None),
            location_stack: Vec::new(),
            method_list: Vec::new(),
        }
    }

    pub(super) fn logger(&self) -> &Logger {
        &self.logger
    }

    pub(super) fn get_method_argument_count(&self, method: &Path) -> usize {
        // TODO: Perform a proper search for the method

        for (m, _) in &self.method_list {
            if m.r#final().unwrap() == method.r#final().unwrap() {
                panic!("Found! ({})", m);
            }
        }

        0
    }

    pub(super) fn add_method(
        &mut self,
        path: &Path,
        argument_count: u8,
        offset: usize,
    ) -> Result<()> {
        let path = self.current_location.join(path);

        for (method, _) in &self.method_list {
            if *method == path {
                return Err(Error::name_collision(path, offset, "Method"));
            }
        }

        self.method_list.push((path, argument_count));

        Ok(())
    }

    pub(super) fn push_path(&mut self, path: &Path) {
        // Create the new path
        let mut new_path = self.current_location.join(path);

        // Set the current path to the new path
        core::mem::swap(&mut new_path, &mut self.current_location);

        // Push the old path (which is now in "new_path" due to the swap)
        self.location_stack.push(new_path);
    }

    pub(super) fn pop_path(&mut self) {
        self.current_location = self.location_stack.pop().unwrap();
    }
}
