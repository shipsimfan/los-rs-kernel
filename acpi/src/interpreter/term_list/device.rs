use super::{Interpreter, Result};
use crate::{interpreter::Error, namespace::objects::Device, parser};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, device: parser::Device) -> Result<()> {
    log_debug!(interpreter.logger(), "Device ({})", device.name());

    let parent_rc = interpreter
        .get_node(device.name(), false)
        .ok_or_else(|| Error::UnknownName(device.name().clone()))?;

    let mut parent_ref = parent_rc.borrow_mut();
    let parent = parent_ref
        .as_children_mut()
        .ok_or_else(|| Error::InvalidParent(device.name().clone()))?;

    parent.add_child(Device::new(
        Some(&parent_rc),
        device
            .name()
            .name()
            .ok_or_else(|| Error::InvalidName(device.name().clone()))?,
    ));

    drop(parent_ref);
    interpreter.display_namespace();

    Ok(())
}
