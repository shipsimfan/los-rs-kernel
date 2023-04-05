use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name, Interpreter, Result},
    namespace::objects::Mutex,
    parser,
};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, mutex: parser::Mutex) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Mutex ({}, {})",
        mutex.name(),
        mutex.sync_level()
    );

    let parent = get_parent!(interpreter, mutex.name())?;

    let mutex_object = Mutex::new(
        Some(&parent),
        unwrap_object_name!(mutex.name())?,
        mutex.sync_level(),
    );

    add_child!(parent, mutex_object, mutex.name())
}
