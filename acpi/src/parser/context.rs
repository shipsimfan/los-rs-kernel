use crate::Path;
use alloc::vec::Vec;
use base::Logger;

pub(super) struct Context {
    logger: Logger,

    wide_integers: bool,

    current_location: Path,
    location_stack: Vec<Path>,

    method_list: Vec<(Path, u8)>,
}

impl Context {
    pub(super) fn new(logger: Logger, wide_integers: bool) -> Self {
        Context {
            logger,
            wide_integers,
            current_location: Path::new(crate::PathPrefix::Root, Vec::new(), None),
            location_stack: Vec::new(),
            method_list: Vec::new(),
        }
    }

    pub(super) fn logger(&self) -> &Logger {
        &self.logger
    }
}
