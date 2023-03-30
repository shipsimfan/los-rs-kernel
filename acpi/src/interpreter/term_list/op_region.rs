use super::{Interpreter, Result};
use crate::{
    interpreter::{argument, DataObject, Error},
    namespace::objects::OperationRegion,
    parser::OpRegion,
};
use base::log_debug;

pub(super) fn execute(interpreter: &mut Interpreter, op_region: OpRegion) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "Op Region ({}, {}, {}, {})",
        op_region.name(),
        op_region.region_space(),
        op_region.offset(),
        op_region.length(),
    );

    let offset = match argument::execute(interpreter, op_region.offset())? {
        DataObject::Integer(value) => value,
        _ => return Err(Error::InvalidType(op_region.name().clone())),
    };

    let length = match argument::execute(interpreter, op_region.length())? {
        DataObject::Integer(value) => value,
        _ => return Err(Error::InvalidType(op_region.name().clone())),
    };

    let parent_rc = interpreter
        .get_node(op_region.name(), false)
        .ok_or_else(|| Error::UnknownName(op_region.name().clone()))?;

    let mut parent_ref = parent_rc.borrow_mut();
    let parent = parent_ref
        .as_children_mut()
        .ok_or_else(|| Error::InvalidParent(op_region.name().clone()))
        .unwrap();

    parent.add_child(OperationRegion::new(
        Some(&parent_rc),
        op_region
            .name()
            .name()
            .ok_or_else(|| Error::InvalidName(op_region.name().clone()))?,
        op_region.region_space(),
        offset,
        length,
    ));

    Ok(())
}
