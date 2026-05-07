use crate::{EngineeringUnit, ValueWithUnit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserCircuitAnalysisType {
    OperatingPoint,
    AcSweep,
    Transient,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserCircuitSimulationEngine {
    Mock,
    Ngspice,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationProbe {
    pub id: String,
    pub label: String,
    pub kind: SimulationProbeKind,
    pub target: SimulationProbeTarget,
    pub unit: Option<EngineeringUnit>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationProbeKind {
    NodeVoltage,
    ComponentCurrent,
    DifferentialVoltage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationProbeTarget {
    Net {
        net_id: String,
    },
    ComponentPin {
        component_id: String,
        pin_id: String,
    },
    Component {
        component_id: String,
    },
    NetPair {
        positive_net_id: String,
        negative_net_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCircuitSimulationProfile {
    pub id: String,
    pub name: String,
    pub analysis_type: UserCircuitAnalysisType,
    pub engine: UserCircuitSimulationEngine,
    pub probes: Vec<SimulationProbe>,
    pub ac: Option<AcSweepSettings>,
    pub transient: Option<TransientSettings>,
    pub op: Option<OperatingPointSettings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcSweepSettings {
    pub start_hz: f64,
    pub stop_hz: f64,
    pub points_per_decade: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientSettings {
    pub step_seconds: f64,
    pub stop_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatingPointSettings {
    pub include_node_voltages: bool,
    pub include_branch_currents: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCircuitSimulationRun {
    pub id: String,
    pub project_id: String,
    pub profile: UserCircuitSimulationProfile,
    pub generated_netlist: String,
    pub status: UserCircuitSimulationStatus,
    pub engine_used: String,
    pub warnings: Vec<SimulationWorkflowWarning>,
    pub errors: Vec<SimulationWorkflowError>,
    pub result: Option<UserCircuitSimulationResult>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserCircuitSimulationStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserCircuitSimulationResult {
    pub summary: Vec<SimulationMeasurement>,
    pub series: Vec<SimulationSeries>,
    pub raw_output_excerpt: Option<String>,
    pub netlist_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationMeasurement {
    pub name: String,
    pub value: ValueWithUnit,
    pub unit_symbol: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationSeries {
    pub id: String,
    pub label: String,
    pub x_unit: Option<EngineeringUnit>,
    pub y_unit: Option<EngineeringUnit>,
    pub points: Vec<SimulationPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationPreflightResult {
    pub can_run: bool,
    pub blocking_errors: Vec<SimulationWorkflowError>,
    pub warnings: Vec<SimulationWorkflowWarning>,
    pub generated_netlist_preview: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationWorkflowError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationWorkflowWarning {
    pub code: String,
    pub message: String,
}
