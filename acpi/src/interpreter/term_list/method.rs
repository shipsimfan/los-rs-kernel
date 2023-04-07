use super::{Interpreter, Result};
use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name},
    namespace::objects::Method,
    parser,
};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    method: parser::Method<'a>,
) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Method ({}, {}, {}, {})",
        method.name(),
        method.arg_count(),
        method.serialized(),
        method.sync_level()
    );

    if interpreter.executing_method() {
        todo!();
    }

    let parent = get_parent!(interpreter, method.name())?;

    let method_object = Method::new(
        Some(&parent),
        unwrap_object_name!(method.name())?,
        method.arg_count(),
        method.serialized(),
        method.sync_level(),
        method.term_list().clone(),
        interpreter.wide_integers(),
    );

    add_child!(parent, method_object, method.name())
}
