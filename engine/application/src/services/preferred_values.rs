use crate::ApplicationError;
use hotsas_core::{
    nearest_preferred_value, CircuitProject, CircuitQueryService, EngineeringUnit,
    PreferredValueResult, PreferredValueSeries, ValueWithUnit,
};

#[derive(Debug, Clone, Default)]
pub struct PreferredValuesService;

impl PreferredValuesService {
    pub fn nearest_e24(
        &self,
        requested_value: ValueWithUnit,
    ) -> Result<PreferredValueResult, ApplicationError> {
        Ok(nearest_preferred_value(
            requested_value,
            PreferredValueSeries::E24,
        )?)
    }

    pub fn nearest_e24_for_resistor(
        &self,
        project: &CircuitProject,
    ) -> Result<PreferredValueResult, ApplicationError> {
        let resistance =
            CircuitQueryService::require_component_parameter(project, "R1", "resistance")?;
        self.nearest_e24(resistance)
    }
}

pub fn parse_requested_e24_value(
    input: &str,
    unit: EngineeringUnit,
) -> Result<ValueWithUnit, ApplicationError> {
    Ok(ValueWithUnit::parse_with_default(input, unit)?)
}
