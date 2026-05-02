use crate::ApplicationError;
use crate::FormulaRegistryService;
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, CircuitQueryService, FormulaDefinition,
    FormulaEvaluationResult, ValueWithUnit,
};
use hotsas_ports::FormulaEnginePort;
use std::collections::BTreeMap;
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
        let result = self.calculate_formula_from_definition(
            &rc_low_pass_formula(),
            BTreeMap::from([
                ("R".to_string(), resistance),
                ("C".to_string(), capacitance),
            ]),
        )?;
        result.outputs.get("fc").cloned().ok_or_else(|| {
            ApplicationError::MissingProjectState("formula output fc missing".to_string())
        })
    }

    pub fn calculate_formula(
        &self,
        registry: &FormulaRegistryService,
        formula_id: &str,
        variables: BTreeMap<String, ValueWithUnit>,
    ) -> Result<FormulaEvaluationResult, ApplicationError> {
        let formula = registry.get_formula(formula_id)?;
        self.calculate_formula_from_definition(&formula, variables)
    }

    pub fn calculate_formula_from_definition(
        &self,
        formula: &FormulaDefinition,
        variables: BTreeMap<String, ValueWithUnit>,
    ) -> Result<FormulaEvaluationResult, ApplicationError> {
        Ok(self.formula_engine.evaluate_formula(formula, &variables)?)
    }
}
