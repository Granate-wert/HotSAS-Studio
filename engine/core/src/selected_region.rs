use crate::{EngineeringUnit, ValueWithUnit};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedCircuitRegion {
    pub id: String,
    pub title: String,
    pub component_ids: Vec<String>,
    pub internal_nets: Vec<String>,
    pub boundary_nets: Vec<String>,
    pub input_port: Option<RegionPort>,
    pub output_port: Option<RegionPort>,
    pub reference_node: Option<String>,
    pub analysis_direction: RegionAnalysisDirection,
    pub analysis_mode: RegionAnalysisMode,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionPort {
    pub positive_net: String,
    pub negative_net: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionAnalysisDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionAnalysisMode {
    Structural,
    TemplateBased,
    NumericMock,
    AllAvailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedRegionPreview {
    pub region: SelectedCircuitRegion,
    pub selected_components: Vec<RegionComponentSummary>,
    pub detected_internal_nets: Vec<RegionNetSummary>,
    pub detected_boundary_nets: Vec<RegionNetSummary>,
    pub suggested_input_nets: Vec<String>,
    pub suggested_output_nets: Vec<String>,
    pub suggested_reference_nodes: Vec<String>,
    pub warnings: Vec<SelectedRegionIssue>,
    pub errors: Vec<SelectedRegionIssue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionComponentSummary {
    pub instance_id: String,
    pub definition_id: Option<String>,
    pub component_kind: String,
    pub display_label: String,
    pub connected_nets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionNetSummary {
    pub net_id: String,
    pub net_name: String,
    pub connected_selected_components: Vec<String>,
    pub connected_external_components: Vec<String>,
    pub is_ground: bool,
    pub role_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedRegionIssue {
    pub code: String,
    pub severity: SelectedRegionIssueSeverity,
    pub message: String,
    pub component_id: Option<String>,
    pub net_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectedRegionIssueSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedRegionAnalysisRequest {
    pub component_ids: Vec<String>,
    pub input_port: Option<RegionPort>,
    pub output_port: Option<RegionPort>,
    pub reference_node: Option<String>,
    pub analysis_direction: RegionAnalysisDirection,
    pub analysis_mode: RegionAnalysisMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedRegionAnalysisResult {
    pub region: SelectedCircuitRegion,
    pub status: SelectedRegionAnalysisStatus,
    pub summary: String,
    pub matched_template: Option<MatchedRegionTemplate>,
    pub equivalent_circuit: Option<EquivalentCircuitSummary>,
    pub transfer_function: Option<RegionTransferFunction>,
    pub measurements: Vec<RegionMeasurement>,
    pub graph_specs: Vec<RegionGraphSpec>,
    pub netlist_fragment: Option<RegionNetlistFragment>,
    pub warnings: Vec<SelectedRegionIssue>,
    pub errors: Vec<SelectedRegionIssue>,
    pub report_section_markdown: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectedRegionAnalysisStatus {
    Success,
    Partial,
    Unsupported,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchedRegionTemplate {
    pub template_id: String,
    pub title: String,
    pub confidence: f64,
    pub formula_ids: Vec<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquivalentCircuitSummary {
    pub title: String,
    pub description: String,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionTransferFunction {
    pub expression: String,
    pub latex: Option<String>,
    pub output_name: String,
    pub unit: Option<EngineeringUnit>,
    pub availability_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionMeasurement {
    pub name: String,
    pub value: Option<ValueWithUnit>,
    pub description: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionGraphSpec {
    pub id: String,
    pub title: String,
    pub x_unit: Option<EngineeringUnit>,
    pub y_unit: Option<EngineeringUnit>,
    pub description: String,
    pub available: bool,
    pub unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionNetlistFragment {
    pub title: String,
    pub format: String,
    pub content: String,
    pub warnings: Vec<String>,
}

pub fn normalize_component_ids(component_ids: &[String]) -> Vec<String> {
    let mut result = component_ids.to_vec();
    result.sort();
    result.dedup();
    result
}

pub fn region_has_component(region: &SelectedCircuitRegion, component_id: &str) -> bool {
    region.component_ids.iter().any(|id| id == component_id)
}

pub fn is_region_configured(region: &SelectedCircuitRegion) -> bool {
    region.input_port.is_some() && region.output_port.is_some() && region.reference_node.is_some()
}

pub fn selected_region_summary(region: &SelectedCircuitRegion) -> String {
    format!(
        "Selected region '{}' with {} component(s), {} internal net(s), {} boundary net(s)",
        region.title,
        region.component_ids.len(),
        region.internal_nets.len(),
        region.boundary_nets.len()
    )
}
