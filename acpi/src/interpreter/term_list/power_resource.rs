use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name, Interpreter, Result},
    namespace::objects::PowerResource,
    parser,
};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    mut power_resource: parser::PowerResource<'a>,
) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "PowerResource ({}, {}, {})",
        power_resource.name(),
        power_resource.system_level(),
        power_resource.resource_order(),
    );

    if interpreter.executing_method() {
        todo!();
    }

    let parent = get_parent!(interpreter, power_resource.name())?;
    let power_resource_object = PowerResource::new(
        Some(&parent),
        unwrap_object_name!(power_resource.name())?,
        power_resource.system_level(),
        power_resource.resource_order(),
    );
    add_child!(parent, power_resource_object.clone(), power_resource.name())?;

    interpreter.push_current_node(power_resource_object);
    super::execute(interpreter, power_resource.term_list())?;
    interpreter.pop_current_node();

    Ok(())
}
