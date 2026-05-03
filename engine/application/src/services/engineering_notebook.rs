use crate::{ApplicationError, FormulaRegistryService};
use hotsas_core::{
    nearest_preferred_value, EngineeringNotebook, EngineeringUnit, NotebookBlockKind,
    NotebookEvaluationResult, NotebookEvaluationStatus, PreferredValueSeries, ValueWithUnit,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct EngineeringNotebookService;

impl EngineeringNotebookService {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate_input(
        &self,
        input: &str,
        variable_scope: &BTreeMap<String, ValueWithUnit>,
        formula_registry: &FormulaRegistryService,
        _preferred_values_service: &crate::PreferredValuesService,
        formula_service: &crate::FormulaService,
    ) -> Result<NotebookEvaluationResult, ApplicationError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(NotebookEvaluationResult::error(input, "empty input"));
        }

        if let Some(result) = self.parse_assignment(trimmed, variable_scope) {
            return Ok(result);
        }

        if let Some(result) =
            self.parse_formula_call(trimmed, variable_scope, formula_registry, formula_service)
        {
            return Ok(result);
        }

        if let Some(result) =
            self.parse_preferred_value_command(trimmed, variable_scope, _preferred_values_service)
        {
            return Ok(result);
        }

        Ok(NotebookEvaluationResult::unsupported(
            input,
            "Unsupported notebook expression in v1.4",
        ))
    }

    pub fn clear_notebook(&self) -> EngineeringNotebook {
        EngineeringNotebook::new("default", "Engineering Notebook")
    }

    pub fn apply_result_to_component(
        &self,
        project: &mut hotsas_core::CircuitProject,
        instance_id: &str,
        parameter_name: &str,
        value: ValueWithUnit,
    ) -> Result<(), ApplicationError> {
        let component = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == instance_id)
            .ok_or_else(|| {
                ApplicationError::InvalidInput(format!("component '{}' not found", instance_id))
            })?;
        component
            .overridden_parameters
            .insert(parameter_name.to_string(), value);
        Ok(())
    }

    fn parse_assignment(
        &self,
        input: &str,
        variable_scope: &BTreeMap<String, ValueWithUnit>,
    ) -> Option<NotebookEvaluationResult> {
        let parts: Vec<&str> = input.splitn(2, '=').collect();
        if parts.len() != 2 {
            return None;
        }
        let name = parts[0].trim();
        let value_str = parts[1].trim();
        if name.is_empty() || value_str.is_empty() {
            return None;
        }
        if name.contains('(') || name.contains(')') {
            return None;
        }

        let parsed =
            ValueWithUnit::parse_with_default(value_str, EngineeringUnit::Unitless).ok()?;
        let mut variables = variable_scope.clone();
        variables.insert(name.to_string(), parsed.clone());

        let mut outputs = BTreeMap::new();
        outputs.insert(name.to_string(), parsed);

        Some(NotebookEvaluationResult {
            input: input.to_string(),
            status: NotebookEvaluationStatus::Success,
            kind: NotebookBlockKind::Assignment,
            outputs,
            variables,
            message: None,
            warnings: Vec::new(),
        })
    }

    fn parse_formula_call(
        &self,
        input: &str,
        variable_scope: &BTreeMap<String, ValueWithUnit>,
        formula_registry: &FormulaRegistryService,
        formula_service: &crate::FormulaService,
    ) -> Option<NotebookEvaluationResult> {
        let open_paren = input.find('(')?;
        let close_paren = input.rfind(')')?;
        if close_paren <= open_paren {
            return None;
        }

        let formula_id = input[..open_paren].trim();
        let args_str = &input[open_paren + 1..close_paren];

        let formula = formula_registry.get_formula(formula_id).ok()?;
        let args = self.parse_key_value_args(args_str, variable_scope)?;

        let result = formula_service
            .calculate_formula_from_definition(&formula, args)
            .ok()?;

        let mut outputs = BTreeMap::new();
        for (name, value) in &result.outputs {
            outputs.insert(name.clone(), value.clone());
        }

        Some(NotebookEvaluationResult {
            input: input.to_string(),
            status: NotebookEvaluationStatus::Success,
            kind: NotebookBlockKind::FormulaCall,
            outputs,
            variables: variable_scope.clone(),
            message: None,
            warnings: result.warnings.clone(),
        })
    }

    fn parse_preferred_value_command(
        &self,
        input: &str,
        variable_scope: &BTreeMap<String, ValueWithUnit>,
        _preferred_values_service: &crate::PreferredValuesService,
    ) -> Option<NotebookEvaluationResult> {
        let open_paren = input.find('(')?;
        let close_paren = input.rfind(')')?;
        if close_paren <= open_paren {
            return None;
        }

        let command = input[..open_paren].trim();
        let args_str = &input[open_paren + 1..close_paren];
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
        if args.len() < 2 {
            return None;
        }

        let value_str = args[0];
        let series_str = args[1];
        let unit_str = args.get(2).copied().unwrap_or("");

        let series = match series_str {
            "E3" => PreferredValueSeries::E3,
            "E6" => PreferredValueSeries::E6,
            "E12" => PreferredValueSeries::E12,
            "E24" => PreferredValueSeries::E24,
            "E48" => PreferredValueSeries::E48,
            "E96" => PreferredValueSeries::E96,
            "E192" => PreferredValueSeries::E192,
            _ => return None,
        };

        let unit = match unit_str {
            "Ohm" => EngineeringUnit::Ohm,
            "F" => EngineeringUnit::Farad,
            "Hz" => EngineeringUnit::Hertz,
            "V" => EngineeringUnit::Volt,
            "A" => EngineeringUnit::Ampere,
            _ => EngineeringUnit::Unitless,
        };

        let value = self.resolve_value(value_str, variable_scope)?;
        let value_with_unit = ValueWithUnit::new_si(value.si_value(), unit);

        let result = match command {
            "nearestE" => nearest_preferred_value(value_with_unit.clone(), series).ok()?,
            "lowerE" => {
                let pv = nearest_preferred_value(value_with_unit.clone(), series).ok()?;
                let lower = pv.lower?;
                hotsas_core::PreferredValueResult {
                    requested_value: value_with_unit,
                    series,
                    lower: None,
                    nearest: lower,
                    higher: None,
                    error_percent: 0.0,
                }
            }
            "higherE" => {
                let pv = nearest_preferred_value(value_with_unit.clone(), series).ok()?;
                let higher = pv.higher?;
                hotsas_core::PreferredValueResult {
                    requested_value: value_with_unit,
                    series,
                    lower: None,
                    nearest: higher,
                    higher: None,
                    error_percent: 0.0,
                }
            }
            _ => return None,
        };

        let mut outputs = BTreeMap::new();
        outputs.insert("nearest".to_string(), result.nearest);
        if let Some(lower) = result.lower {
            outputs.insert("lower".to_string(), lower);
        }
        if let Some(higher) = result.higher {
            outputs.insert("higher".to_string(), higher);
        }
        outputs.insert(
            "error_percent".to_string(),
            ValueWithUnit::new_si(result.error_percent, EngineeringUnit::Unitless),
        );

        Some(NotebookEvaluationResult {
            input: input.to_string(),
            status: NotebookEvaluationStatus::Success,
            kind: NotebookBlockKind::PreferredValue,
            outputs,
            variables: variable_scope.clone(),
            message: None,
            warnings: Vec::new(),
        })
    }

    fn parse_key_value_args(
        &self,
        args_str: &str,
        variable_scope: &BTreeMap<String, ValueWithUnit>,
    ) -> Option<BTreeMap<String, ValueWithUnit>> {
        let mut result = BTreeMap::new();
        for part in args_str.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let kv: Vec<&str> = part.splitn(2, '=').collect();
            if kv.len() != 2 {
                return None;
            }
            let name = kv[0].trim();
            let value_str = kv[1].trim();
            let value = self.resolve_value(value_str, variable_scope)?;
            result.insert(name.to_string(), value);
        }
        Some(result)
    }

    fn resolve_value(
        &self,
        value_str: &str,
        variable_scope: &BTreeMap<String, ValueWithUnit>,
    ) -> Option<ValueWithUnit> {
        if let Some(existing) = variable_scope.get(value_str) {
            return Some(existing.clone());
        }
        ValueWithUnit::parse_with_default(value_str, EngineeringUnit::Unitless).ok()
    }
}
