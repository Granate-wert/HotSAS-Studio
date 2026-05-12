use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedModelAssetKind {
    SpiceModel,
    SpiceSubcircuit,
    TouchstoneDataset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedModelAssetSource {
    ImportedFile,
    BuiltIn,
    UserProvided,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistedModelAssetStatus {
    Present,
    Missing,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedModelAsset {
    pub id: String,
    pub name: String,
    pub kind: PersistedModelAssetKind,
    pub source: PersistedModelAssetSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_asset_path: Option<String>,
    pub status: PersistedModelAssetStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub compatibility: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PersistedModelCatalog {
    pub assets: Vec<PersistedModelAsset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedInstanceModelAssignment {
    pub instance_id: String,
    pub component_definition_id: String,
    pub model_asset_id: String,
    pub pin_mappings: Vec<crate::ComponentPinMapping>,
    pub parameter_bindings: Vec<crate::ModelParameterBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelAssetValidationDiagnostic {
    pub code: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectModelPersistenceSummary {
    pub asset_count: usize,
    pub spice_model_count: usize,
    pub subcircuit_count: usize,
    pub touchstone_dataset_count: usize,
    pub component_assignment_count: usize,
    pub instance_assignment_count: usize,
    pub missing_asset_reference_count: usize,
    pub stale_assignment_count: usize,
    pub diagnostics: Vec<ModelAssetValidationDiagnostic>,
    pub ready: bool,
}
