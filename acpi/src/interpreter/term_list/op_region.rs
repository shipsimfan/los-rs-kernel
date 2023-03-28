use super::{Interpreter, Result};
use crate::parser::OpRegion;
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

    Ok(())
}
