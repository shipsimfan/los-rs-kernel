use super::{Interpreter, Result};
use crate::{
    interpreter::{add_child, data_object, get_parent, unwrap_object_name},
    namespace::objects::Name,
    parser,
};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, name: parser::Name) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Name ({}, {})",
        name.name(),
        name.data_object()
    );

    if interpreter.executing_method() {
        todo!();
    }

    let parent = get_parent!(interpreter, name.name())?;

    let data_object = data_object::execute(interpreter, name.data_object(), name.name())?;

    let name_object = Name::new(
        Some(&parent),
        unwrap_object_name!(name.name())?,
        data_object,
    );

    add_child!(parent, name_object, name.name())
}
