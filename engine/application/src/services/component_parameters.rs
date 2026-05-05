use hotsas_core::{
    schema_for_category, BjtParameters, CapacitorParameters, ComponentDefinition,
    ComponentInstance, ComponentParameterSchema, ComponentParameterValue, DiodeParameters,
    InductorParameters, MosfetParameters, OpAmpParameters, RegulatorParameters,
    ResistorParameters,
};
use std::collections::BTreeMap;

/// Service for typed component parameter operations.
///
/// Provides schema-aware validation, typed bundle extraction, and
/// instance override validation without mutating the library.
#[derive(Clone)]
pub struct ComponentParameterService;

impl ComponentParameterService {
    pub fn new() -> Self {
        Self
    }

    /// Get the parameter schema for a component category.
    pub fn schema_for_category(&self, category: &str) -> Option<ComponentParameterSchema> {
        schema_for_category(category)
    }

    /// Validate a component definition's parameters against its category schema.
    pub fn validate_component(&self, component: &ComponentDefinition) -> Vec<ParameterIssue> {
        let Some(schema) = self.schema_for_category(&component.category) else {
            return vec![];
        };
        schema
            .validate_map(&component.parameters)
            .into_iter()
            .map(|(key, error)| ParameterIssue {
                key,
                message: error.to_string(),
                severity: IssueSeverity::Error,
            })
            .collect()
    }

    /// Validate instance parameter overrides against the definition's schema.
    pub fn validate_instance_overrides(
        &self,
        component: &ComponentDefinition,
        instance: &ComponentInstance,
    ) -> Vec<ParameterIssue> {
        let Some(schema) = self.schema_for_category(&component.category) else {
            return vec![];
        };

        let mut issues = Vec::new();
        for (key, value) in &instance.overridden_parameters {
            if let Some(def) = schema.get_definition(key) {
                if let Err(e) = def.validate(value) {
                    issues.push(ParameterIssue {
                        key: key.clone(),
                        message: e.to_string(),
                        severity: IssueSeverity::Error,
                    });
                }
            } else {
                issues.push(ParameterIssue {
                    key: key.clone(),
                    message: format!("Unknown parameter '{}' for category '{}'", key, component.category),
                    severity: IssueSeverity::Warning,
                });
            }
        }
        issues
    }

    // ------------------------------------------------------------------
    // Typed bundle extraction
    // ------------------------------------------------------------------

    pub fn resistor_parameters(&self, component: &ComponentDefinition) -> Option<ResistorParameters> {
        ResistorParameters::from_map(&component.parameters)
    }

    pub fn capacitor_parameters(&self, component: &ComponentDefinition) -> Option<CapacitorParameters> {
        CapacitorParameters::from_map(&component.parameters)
    }

    pub fn inductor_parameters(&self, component: &ComponentDefinition) -> Option<InductorParameters> {
        InductorParameters::from_map(&component.parameters)
    }

    pub fn diode_parameters(&self, component: &ComponentDefinition) -> DiodeParameters {
        DiodeParameters::from_map(&component.parameters)
    }

    pub fn bjt_parameters(&self, component: &ComponentDefinition) -> BjtParameters {
        BjtParameters::from_map(&component.parameters)
    }

    pub fn mosfet_parameters(&self, component: &ComponentDefinition) -> MosfetParameters {
        MosfetParameters::from_map(&component.parameters)
    }

    pub fn op_amp_parameters(&self, component: &ComponentDefinition) -> OpAmpParameters {
        OpAmpParameters::from_map(&component.parameters)
    }

    pub fn regulator_parameters(&self, component: &ComponentDefinition) -> RegulatorParameters {
        RegulatorParameters::from_map(&component.parameters)
    }

    // ------------------------------------------------------------------
    // Instance value resolution
    // ------------------------------------------------------------------

    /// Resolve the effective value for a parameter key on an instance.
    /// Returns the override if present, otherwise the definition default.
    pub fn resolve_parameter(
        &self,
        component: &ComponentDefinition,
        instance: &ComponentInstance,
        key: &str,
    ) -> Option<ComponentParameterValue> {
        if let Some(value) = instance.overridden_parameters.get(key) {
            return Some(ComponentParameterValue {
                value: value.clone(),
                source: hotsas_core::ComponentParameterSource::Override,
                note: None,
            });
        }
        component.parameters.get(key).map(|value| ComponentParameterValue {
            value: value.clone(),
            source: hotsas_core::ComponentParameterSource::Default,
            note: None,
        })
    }

    /// Build a full resolved parameter map for an instance.
    pub fn resolve_all_parameters(
        &self,
        component: &ComponentDefinition,
        instance: &ComponentInstance,
    ) -> BTreeMap<String, ComponentParameterValue> {
        let mut result = BTreeMap::new();

        // Start with definition defaults.
        for (key, value) in &component.parameters {
            result.insert(
                key.clone(),
                ComponentParameterValue {
                    value: value.clone(),
                    source: hotsas_core::ComponentParameterSource::Default,
                    note: None,
                },
            );
        }

        // Apply overrides.
        for (key, value) in &instance.overridden_parameters {
            result.insert(
                key.clone(),
                ComponentParameterValue {
                    value: value.clone(),
                    source: hotsas_core::ComponentParameterSource::Override,
                    note: None,
                },
            );
        }

        result
    }
}

impl Default for ComponentParameterService {
    fn default() -> Self {
        Self::new()
    }
}

/// Severity of a parameter issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// A single parameter validation issue.
#[derive(Debug, Clone)]
pub struct ParameterIssue {
    pub key: String,
    pub message: String,
    pub severity: IssueSeverity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{
        built_in_component_library, ComponentParameterKind, ComponentParameterSource,
        EngineeringUnit, Point, ValueWithUnit,
    };

    #[test]
    fn service_returns_resistor_schema() {
        let svc = ComponentParameterService::new();
        let schema = svc.schema_for_category("Resistor").unwrap();
        let def = schema.get_definition("resistance").unwrap();
        assert_eq!(def.kind, ComponentParameterKind::Primary);
    }

    #[test]
    fn service_validates_good_resistor() {
        let lib = built_in_component_library();
        let svc = ComponentParameterService::new();
        let r = lib.components.iter().find(|c| c.id == "generic_resistor").unwrap();
        let issues = svc.validate_component(r);
        assert!(issues.is_empty(), "expected no issues, got {:?}", issues);
    }

    #[test]
    fn service_extracts_resistor_parameters() {
        let lib = built_in_component_library();
        let svc = ComponentParameterService::new();
        let r = lib.components.iter().find(|c| c.id == "generic_resistor").unwrap();
        let params = svc.resistor_parameters(r).unwrap();
        assert_eq!(params.resistance.si_value(), 10_000.0);
    }

    #[test]
    fn service_resolves_instance_override() {
        let lib = built_in_component_library();
        let svc = ComponentParameterService::new();
        let r = lib.components.iter().find(|c| c.id == "generic_resistor").unwrap();

        let mut instance = ComponentInstance {
            instance_id: "R1".to_string(),
            definition_id: r.id.clone(),
            selected_symbol_id: None,
            selected_footprint_id: None,
            selected_simulation_model_id: None,
            overridden_parameters: BTreeMap::new(),
            position: Point { x: 0.0, y: 0.0 },
            rotation_degrees: 0.0,
            connected_nets: vec![],
            notes: None,
        };
        instance.overridden_parameters.insert(
            "resistance".to_string(),
            ValueWithUnit::parse_with_default("4.7k", EngineeringUnit::Ohm).unwrap(),
        );

        let resolved = svc.resolve_parameter(r, &instance, "resistance").unwrap();
        assert_eq!(resolved.value.si_value(), 4_700.0);
        assert_eq!(resolved.source, ComponentParameterSource::Override);
    }

    #[test]
    fn service_returns_none_for_unknown_category_schema() {
        let svc = ComponentParameterService::new();
        assert!(svc.schema_for_category("QuantumFluxCapacitor").is_none());
    }
}
