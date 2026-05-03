use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImportedModelKind {
    SpiceModel,
    SpiceSubcircuit,
    TouchstoneNetwork,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelImportStatus {
    Parsed,
    ParsedWithWarnings,
    Failed,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportedModelSource {
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub source_format: String,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiceModelKind {
    Diode,
    BjtNpn,
    BjtPnp,
    MosfetN,
    MosfetP,
    JfetN,
    JfetP,
    Resistor,
    Capacitor,
    Inductor,
    Subcircuit,
    OpAmpMacroModel,
    IcMacroModel,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpiceModelParameter {
    pub name: String,
    pub value: String,
    pub unit_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpiceModelDefinition {
    pub id: String,
    pub name: String,
    pub kind: SpiceModelKind,
    pub source: ImportedModelSource,
    pub raw_line: String,
    pub parameters: Vec<SpiceModelParameter>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpiceSubcircuitDefinition {
    pub id: String,
    pub name: String,
    pub pins: Vec<String>,
    pub body: Vec<String>,
    pub source: ImportedModelSource,
    pub detected_kind: SpiceModelKind,
    pub parameters: Vec<SpiceModelParameter>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpicePinMapping {
    pub model_id: String,
    pub component_definition_id: String,
    pub mappings: Vec<SpicePinMappingEntry>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpicePinMappingEntry {
    pub model_pin: String,
    pub component_pin: String,
    pub role_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpiceImportReport {
    pub status: ModelImportStatus,
    pub source: ImportedModelSource,
    pub models: Vec<SpiceModelDefinition>,
    pub subcircuits: Vec<SpiceSubcircuitDefinition>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TouchstoneParameterFormat {
    RI,
    MA,
    DB,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TouchstoneFrequencyUnit {
    Hz,
    KHz,
    MHz,
    GHz,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexValue {
    pub re: f64,
    pub im: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterPoint {
    pub frequency_hz: f64,
    pub values: Vec<ComplexValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchstoneNetworkData {
    pub id: String,
    pub name: String,
    pub port_count: usize,
    pub frequency_unit: TouchstoneFrequencyUnit,
    pub parameter_format: TouchstoneParameterFormat,
    pub reference_impedance_ohm: f64,
    pub points: Vec<SParameterPoint>,
    pub source: ImportedModelSource,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchstoneImportReport {
    pub status: ModelImportStatus,
    pub network: Option<TouchstoneNetworkData>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportedSimulationModelReference {
    pub id: String,
    pub model_kind: ImportedModelKind,
    pub display_name: String,
    pub source_file_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportedModelSummary {
    pub id: String,
    pub kind: ImportedModelKind,
    pub name: String,
    pub source_format: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportedModelDetails {
    pub id: String,
    pub kind: ImportedModelKind,
    pub name: String,
    pub source: ImportedModelSource,
    pub spice_model: Option<SpiceModelDefinition>,
    pub spice_subcircuit: Option<SpiceSubcircuitDefinition>,
    pub touchstone_network: Option<TouchstoneNetworkData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpicePinMappingRequest {
    pub model_id: String,
    pub component_definition_id: String,
    pub mappings: Vec<SpicePinMappingEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpicePinMappingValidationReport {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachImportedModelRequest {
    pub model_id: String,
    pub component_definition_id: String,
    pub pin_mapping: Option<SpicePinMappingRequest>,
}
