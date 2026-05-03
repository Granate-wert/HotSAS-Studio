use crate::{EngineeringUnit, ValueWithUnit};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitProject {
    pub id: String,
    pub name: String,
    pub format_version: String,
    pub engine_version: String,
    pub project_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub schematic: CircuitModel,
    pub simulation_profiles: Vec<SimulationProfile>,
    pub linked_libraries: Vec<String>,
    pub reports: Vec<ReportModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitModel {
    pub id: String,
    pub title: String,
    pub components: Vec<ComponentInstance>,
    pub wires: Vec<Wire>,
    pub nets: Vec<Net>,
    pub labels: Vec<CircuitLabel>,
    pub probes: Vec<Probe>,
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
    pub description: Option<String>,
    pub parameters: BTreeMap<String, ValueWithUnit>,
    pub ratings: BTreeMap<String, ValueWithUnit>,
    pub symbol_ids: Vec<String>,
    pub footprint_ids: Vec<String>,
    pub simulation_models: Vec<SimulationModel>,
    pub datasheets: Vec<String>,
    pub tags: Vec<String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    pub instance_id: String,
    pub definition_id: String,
    pub selected_symbol_id: Option<String>,
    pub selected_footprint_id: Option<String>,
    pub selected_simulation_model_id: Option<String>,
    pub position: Point,
    pub rotation_degrees: f64,
    pub connected_nets: Vec<ConnectedPin>,
    pub overridden_parameters: BTreeMap<String, ValueWithUnit>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectedPin {
    pub pin_id: String,
    pub net_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationModel {
    pub id: String,
    pub model_type: String,
    pub source_path: Option<String>,
    pub raw_model: Option<String>,
    pub raw_model_id: Option<String>,
    pub pin_mapping: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolDefinition {
    pub id: String,
    pub name: String,
    pub component_category: String,
    pub graphical_primitives: Vec<GraphicalPrimitive>,
    pub pins: Vec<SymbolPin>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphicalPrimitive {
    pub primitive_type: String,
    pub points: Vec<Point>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolPin {
    pub pin_number: String,
    pub pin_name: String,
    pub electrical_type: String,
    pub position: Point,
    pub orientation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FootprintDefinition {
    pub id: String,
    pub name: String,
    pub package_name: String,
    pub standard: Option<String>,
    pub pads: Vec<Pad>,
    pub courtyard: Vec<Point>,
    pub silkscreen: Vec<GraphicalPrimitive>,
    pub assembly_outline: Vec<Point>,
    pub body_size: Option<Size3d>,
    pub pitch: Option<ValueWithUnit>,
    pub height: Option<ValueWithUnit>,
    pub model_3d_path: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pad {
    pub pad_number: String,
    pub pad_type: String,
    pub shape: String,
    pub position: Point,
    pub size: Size2d,
    pub drill: Option<ValueWithUnit>,
    pub layers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Net {
    pub id: String,
    pub name: String,
    pub connected_pins: Vec<ConnectedPin>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    pub id: String,
    pub from: CircuitEndpoint,
    pub to: CircuitEndpoint,
    pub net_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitEndpoint {
    pub component_id: Option<String>,
    pub pin_id: Option<String>,
    pub point: Point,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitLabel {
    pub id: String,
    pub text: String,
    pub position: Point,
    pub net_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Probe {
    pub id: String,
    pub probe_type: ProbeType,
    pub target: String,
    pub reference_node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProbeType {
    Voltage,
    Current,
    Power,
    Differential,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaDefinition {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub equations: Vec<FormulaEquation>,
    pub variables: BTreeMap<String, FormulaVariable>,
    pub outputs: BTreeMap<String, FormulaOutput>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub linked_circuit_template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<BTreeMap<String, String>>,
    pub default_simulation_profile: Option<SimulationProfile>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaPack {
    pub pack_id: String,
    pub title: String,
    pub version: String,
    pub formulas: Vec<FormulaDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormulaPackMetadata {
    pub pack_id: String,
    pub title: String,
    pub version: String,
    pub formula_count: usize,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormulaPackSource {
    BuiltIn,
    FilePath(String),
    DirectoryPath(String),
    UserProvided,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormulaPackValidationError {
    pub code: String,
    pub message: String,
    pub formula_id: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaEquation {
    pub id: String,
    pub latex: String,
    pub expression: String,
    pub solve_for: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaVariable {
    pub unit: EngineeringUnit,
    pub description: String,
    pub default: Option<ValueWithUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaOutput {
    pub unit: EngineeringUnit,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaEvaluationRequest {
    pub formula_id: String,
    pub variables: BTreeMap<String, ValueWithUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaVariableValue {
    pub name: String,
    pub value: ValueWithUnit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaOutputValue {
    pub name: String,
    pub value: ValueWithUnit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaEvaluationResult {
    pub formula_id: String,
    pub equation_id: String,
    pub expression: String,
    pub inputs: BTreeMap<String, ValueWithUnit>,
    pub outputs: BTreeMap<String, ValueWithUnit>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormulaExpressionValidationResult {
    pub expression: String,
    pub supported: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitTemplate {
    pub id: String,
    pub title: String,
    pub components: Vec<ComponentInstance>,
    pub wires: Vec<Wire>,
    pub named_nodes: BTreeMap<String, String>,
    pub input_ports: Vec<String>,
    pub output_ports: Vec<String>,
    pub probes: Vec<Probe>,
    pub default_parameters: BTreeMap<String, ValueWithUnit>,
    pub compatible_formula_ids: Vec<String>,
    pub simulation_profiles: Vec<SimulationProfile>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaCircuitBinding {
    pub formula_id: String,
    pub circuit_template_id: String,
    pub variable_mapping: BTreeMap<String, String>,
    pub input_mapping: BTreeMap<String, String>,
    pub output_mapping: BTreeMap<String, String>,
    pub probe_mapping: BTreeMap<String, String>,
    pub graph_mapping: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationProfile {
    pub id: String,
    pub simulation_type: SimulationType,
    pub parameters: BTreeMap<String, ValueWithUnit>,
    pub requested_outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationType {
    OperatingPoint,
    DcSweep,
    AcSweep,
    Transient,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationResult {
    pub id: String,
    pub profile_id: String,
    pub status: SimulationStatus,
    pub engine: String,
    pub graph_series: Vec<GraphSeries>,
    pub measurements: BTreeMap<String, ValueWithUnit>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub raw_data_path: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationStatus {
    NotStarted,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphSeries {
    pub name: String,
    pub x_unit: EngineeringUnit,
    pub y_unit: EngineeringUnit,
    pub points: Vec<GraphPoint>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportModel {
    pub id: String,
    pub title: String,
    pub sections: Vec<ReportSection>,
    pub included_schematic_images: Vec<String>,
    pub included_formulas: Vec<FormulaDefinition>,
    pub included_simulation_results: Vec<SimulationResult>,
    pub included_bom: Vec<BomLine>,
    pub export_settings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportSection {
    pub title: String,
    pub body_markdown: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BomLine {
    pub designator: String,
    pub quantity: u32,
    pub value: Option<ValueWithUnit>,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size2d {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size3d {
    pub width: f64,
    pub depth: f64,
    pub height: f64,
}
