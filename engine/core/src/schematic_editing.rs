use crate::{CircuitProject, CircuitValidationIssue, Point};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddComponentRequest {
    pub component_kind: String,
    pub component_definition_id: Option<String>,
    pub instance_id: Option<String>,
    pub position: Point,
    pub rotation_deg: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveComponentRequest {
    pub instance_id: String,
    pub position: Point,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteComponentRequest {
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectPinsRequest {
    pub from_component_id: String,
    pub from_pin_id: String,
    pub to_component_id: String,
    pub to_pin_id: String,
    pub net_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameNetRequest {
    pub net_id: String,
    pub new_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchematicEditResult {
    pub project: CircuitProject,
    pub validation_warnings: Vec<CircuitValidationIssue>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchematicToolCapability {
    pub tool_id: String,
    pub label: String,
    pub available: bool,
    pub limitation: Option<String>,
}
