use crate::{
    interpreter::{Error, Interpreter, Result},
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

    let parent_rc = interpreter
        .get_node(mutex.name(), false)
        .ok_or_else(|| Error::UnknownName(mutex.name().clone()))?;

    let mut parent_ref = parent_rc.borrow_mut();
    let parent = parent_ref
        .as_children_mut()
        .ok_or_else(|| Error::InvalidParent(mutex.name().clone()))?;

    parent.add_child(Mutex::new(
        Some(&parent_rc),
        mutex
            .name()
            .name()
            .ok_or_else(|| Error::InvalidName(mutex.name().clone()))?,
        mutex.sync_level(),
    ));

    drop(parent_ref);
    interpreter.display_namespace();

    Ok(())
}
