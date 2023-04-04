use super::{Interpreter, Result};
use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name, Error},
    namespace::objects::Method,
    parser,
};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, method: parser::Method) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Method ({}, {}, {}, {})",
        method.name(),
        method.arg_count(),
        method.serialized(),
        method.sync_level()
    );

    let parent = get_parent!(interpreter, method.name())?;

    let method_object = Method::new(
        Some(&parent),
        unwrap_object_name!(method.name())?,
        method.arg_count(),
        method.serialized(),
        method.sync_level(),
        method.method_size(),
    );

    add_child!(parent, method_object, method.name())
}
