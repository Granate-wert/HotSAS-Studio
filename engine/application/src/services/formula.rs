use crate::ApplicationError;
use hotsas_core::{CircuitProject, CircuitQueryService, ValueWithUnit};
use hotsas_ports::FormulaEnginePort;
use std::sync::Arc;

#[derive(Clone)]
pub struct FormulaService {
    formula_engine: Arc<dyn FormulaEnginePort>,
}

impl FormulaService {
    pub fn new(formula_engine: Arc<dyn FormulaEnginePort>) -> Self {
        Self { formula_engine }
    }

    pub fn calculate_rc_low_pass_cutoff(
        &self,
        project: &CircuitProject,
    ) -> Result<ValueWithUnit, ApplicationError> {
        let resistance =
            CircuitQueryService::require_component_parameter(project, "R1", "resistance")?;
        let capacitance =
            CircuitQueryService::require_component_parameter(project, "C1", "capacitance")?;
        Ok(self
            .formula_engine
            .calculate_rc_low_pass_cutoff(&resistance, &capacitance)?)
    }
}
