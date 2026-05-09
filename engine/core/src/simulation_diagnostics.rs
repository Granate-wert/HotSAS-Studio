use crate::NgspiceAvailability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NgspiceDiagnostics {
    pub availability: NgspiceAvailability,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub checked_at: String,
    pub warnings: Vec<SimulationDiagnosticMessage>,
    pub errors: Vec<SimulationDiagnosticMessage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationDiagnosticMessage {
    pub code: String,
    pub severity: SimulationDiagnosticSeverity,
    pub title: String,
    pub message: String,
    pub related_entity: Option<SimulationDiagnosticEntityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_model_id: Option<String>,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationDiagnosticSeverity {
    Info,
    Warning,
    Error,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationDiagnosticEntityRef {
    pub kind: SimulationDiagnosticEntityKind,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationDiagnosticEntityKind {
    Component,
    Net,
    Probe,
    Profile,
    Engine,
}
