use super::{Interpreter, Result};
use crate::{
    interpreter::{add_child, argument, get_parent, unwrap_object_name, unwrap_type},
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

    let offset = unwrap_type!(
        argument::execute(interpreter, op_region.offset(), op_region.name())?,
        Integer,
        op_region.name()
    )?;

    let length = unwrap_type!(
        argument::execute(interpreter, op_region.length(), op_region.name())?,
        Integer,
        op_region.name()
    )?;

    let parent = get_parent!(interpreter, op_region.name())?;

    let op_region_object = OperationRegion::new(
        Some(&parent),
        unwrap_object_name!(op_region.name())?,
        op_region.region_space(),
        offset,
        length,
    );

    add_child!(parent, op_region_object, op_region.name())
}
