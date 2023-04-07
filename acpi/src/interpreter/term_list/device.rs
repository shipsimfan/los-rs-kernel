use super::{Interpreter, Result};
use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name},
    namespace::objects::Device,
    parser,
};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    mut device: parser::Device<'a>,
) -> Result<()> {
    log_debug!(interpreter.logger(), "Device ({})", device.name());

    if interpreter.executing_method() {
        todo!();
    }

    let parent = get_parent!(interpreter, device.name())?;
    let device_object = Device::new(Some(&parent), unwrap_object_name!(device.name())?);
    add_child!(parent, device_object.clone(), device.name())?;

    interpreter.push_current_node(device_object);
    super::execute(interpreter, device.term_list())?;
    interpreter.pop_current_node();

    Ok(())
}
