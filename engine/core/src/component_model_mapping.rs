use serde::{Deserialize, Serialize};

/// Status of a SPICE model assignment for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentModelAssignmentStatus {
    /// No model is assigned.
    Missing,
    /// A placeholder model is assigned (not production-accurate).
    Placeholder,
    /// A builtin primitive model is assigned (R, C, L, V, GND).
    AssignedBuiltin,
    /// An imported model (.model or .subckt) is assigned.
    AssignedImported,
    /// A user-manually assigned model is present.
    AssignedManual,
    /// The assignment is invalid (broken pin mapping, missing params, etc.).
    Invalid,
}

/// Kind of SPICE model reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpiceModelReferenceKind {
    /// Built-in SPICE primitive (R, C, L, V, I, etc.).
    PrimitiveModel,
    /// A .model definition.
    Subcircuit,
    /// A behavioral model.
    Behavioral,
    /// A placeholder / simplified model.
    Placeholder,
}

/// Source of a SPICE model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpiceModelSource {
    /// Provided by the builtin component library.
    Builtin,
    /// Imported from an external file.
    ImportedFile,
    /// Assigned manually by the user.
    UserAssigned,
    /// Generated fallback when nothing better is available.
    GeneratedFallback,
    /// Unknown or unspecified source.
    Unknown,
}

/// A reference to an available SPICE model that can be assigned to a component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpiceModelReference {
    pub id: String,
    pub display_name: String,
    pub model_kind: SpiceModelReferenceKind,
    pub source: SpiceModelSource,
    pub status: ComponentModelAssignmentStatus,
    pub limitations: Vec<String>,
    pub warnings: Vec<String>,
}

/// Role of a component pin in the context of a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentPinRole {
    Positive,
    Negative,
    Input,
    Output,
    SupplyPositive,
    SupplyNegative,
    Gate,
    Drain,
    Source,
    Base,
    Collector,
    Emitter,
    Anode,
    Cathode,
    Reference,
    Other,
}

/// Mapping between a component pin and a model pin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentPinMapping {
    pub component_pin_id: String,
    pub model_pin_name: String,
    pub model_pin_index: Option<usize>,
    pub role: Option<ComponentPinRole>,
    pub required: bool,
}

/// Binding between a model parameter and a component parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelParameterBinding {
    pub model_parameter_name: String,
    pub component_parameter_id: String,
    pub value_expression: Option<String>,
    pub required: bool,
}

/// Readiness of a single component for simulation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationReadiness {
    pub can_simulate: bool,
    pub can_export_netlist: bool,
    pub uses_placeholder: bool,
    pub blocking_count: usize,
    pub warning_count: usize,
    pub status_label: String,
}

impl SimulationReadiness {
    pub fn ready() -> Self {
        Self {
            can_simulate: true,
            can_export_netlist: true,
            uses_placeholder: false,
            blocking_count: 0,
            warning_count: 0,
            status_label: "Simulation ready".to_string(),
        }
    }

    pub fn placeholder() -> Self {
        Self {
            can_simulate: true,
            can_export_netlist: true,
            uses_placeholder: true,
            blocking_count: 0,
            warning_count: 1,
            status_label: "Placeholder model".to_string(),
        }
    }

    pub fn missing() -> Self {
        Self {
            can_simulate: false,
            can_export_netlist: true,
            uses_placeholder: false,
            blocking_count: 1,
            warning_count: 0,
            status_label: "No SPICE model".to_string(),
        }
    }

    pub fn invalid() -> Self {
        Self {
            can_simulate: false,
            can_export_netlist: false,
            uses_placeholder: false,
            blocking_count: 1,
            warning_count: 0,
            status_label: "Invalid model assignment".to_string(),
        }
    }
}

/// Severity of a model mapping diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelMappingSeverity {
    Info,
    Warning,
    Error,
    Blocking,
}

/// A diagnostic message about a model mapping issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelMappingDiagnostic {
    pub code: String,
    pub severity: ModelMappingSeverity,
    pub title: String,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub related_component_id: Option<String>,
    pub related_model_id: Option<String>,
}

/// Full model assignment state for a component (definition or instance).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentModelAssignment {
    pub component_definition_id: String,
    pub component_instance_id: Option<String>,
    pub model_ref: Option<SpiceModelReference>,
    pub pin_mappings: Vec<ComponentPinMapping>,
    pub parameter_bindings: Vec<ModelParameterBinding>,
    pub status: ComponentModelAssignmentStatus,
    pub readiness: SimulationReadiness,
    pub diagnostics: Vec<ModelMappingDiagnostic>,
}

/// Project-level simulation readiness summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationReadiness {
    pub project_id: String,
    pub can_simulate: bool,
    pub component_count: usize,
    pub ready_count: usize,
    pub placeholder_count: usize,
    pub missing_count: usize,
    pub invalid_count: usize,
    pub blocking_count: usize,
    pub warning_count: usize,
    pub components: Vec<ComponentModelAssignment>,
}
