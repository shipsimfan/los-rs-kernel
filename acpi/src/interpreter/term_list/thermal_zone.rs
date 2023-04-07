use crate::{
    interpreter::{add_child, get_parent, unwrap_object_name, Interpreter, Result},
    namespace::objects::ThermalZone,
    parser,
};
use base::log_debug;

pub(super) fn execute<'a, 'b>(
    interpreter: &mut Interpreter<'a, 'b>,
    mut thermal_zone: parser::ThermalZone<'a>,
) -> Result<()> {
    log_debug!(
        interpreter.logger(),
        "ThermalZone ({})",
        thermal_zone.name()
    );

    if interpreter.executing_method() {
        todo!();
    }

    let parent = get_parent!(interpreter, thermal_zone.name())?;
    let power_resource_object =
        ThermalZone::new(Some(&parent), unwrap_object_name!(thermal_zone.name())?);
    add_child!(parent, power_resource_object.clone(), thermal_zone.name())?;

    interpreter.push_current_node(power_resource_object);
    super::execute(interpreter, thermal_zone.term_list())?;
    interpreter.pop_current_node();

    Ok(())
}
