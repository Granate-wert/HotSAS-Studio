use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NgspiceAvailability {
    pub available: bool,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NgspiceRunStatus {
    Success,
    Failed,
    TimedOut,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NgspiceRunMetadata {
    pub run_id: String,
    pub engine: String,
    pub command: Vec<String>,
    pub working_directory: String,
    pub netlist_path: String,
    pub stdout_path: Option<String>,
    pub stderr_path: Option<String>,
    pub raw_output_path: Option<String>,
    pub parsed_output_path: Option<String>,
    pub exit_code: Option<i32>,
    pub elapsed_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationAnalysisKind {
    OperatingPoint,
    DcSweep,
    AcSweep,
    Transient,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NgspiceSimulationRequest {
    pub project_id: String,
    pub profile_id: String,
    pub netlist: String,
    pub analysis_kind: SimulationAnalysisKind,
    pub output_variables: Vec<String>,
    pub timeout_ms: u64,
}
