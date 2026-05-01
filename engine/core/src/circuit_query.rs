use crate::{CircuitProject, ComponentInstance, CoreError, ValueWithUnit};

pub struct CircuitQueryService;

impl CircuitQueryService {
    pub fn get_component<'a>(
        project: &'a CircuitProject,
        instance_id: &str,
    ) -> Option<&'a ComponentInstance> {
        project
            .schematic
            .components
            .iter()
            .find(|component| component.instance_id == instance_id)
    }

    pub fn get_component_parameter<'a>(
        project: &'a CircuitProject,
        instance_id: &str,
        parameter_name: &str,
    ) -> Option<&'a ValueWithUnit> {
        Self::get_component(project, instance_id)
            .and_then(|component| component.overridden_parameters.get(parameter_name))
    }

    pub fn require_component_parameter(
        project: &CircuitProject,
        instance_id: &str,
        parameter_name: &str,
    ) -> Result<ValueWithUnit, CoreError> {
        Self::get_component_parameter(project, instance_id, parameter_name)
            .cloned()
            .ok_or_else(|| CoreError::MissingParameter {
                component_id: instance_id.to_string(),
                parameter: parameter_name.to_string(),
            })
    }
}
