use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectPackageType {
    CircuitProject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectPackageFiles {
    pub schematic: String,
    pub components: String,
    pub formulas: String,
    pub simulation_profiles: String,
    pub reports_index: String,
    pub results_index: String,
}

impl Default for ProjectPackageFiles {
    fn default() -> Self {
        Self {
            schematic: "schematic.json".to_string(),
            components: "components.json".to_string(),
            formulas: "formulas.json".to_string(),
            simulation_profiles: "simulation_profiles.json".to_string(),
            reports_index: "reports/index.json".to_string(),
            results_index: "results/index.json".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectPackageManifest {
    pub format_version: String,
    pub engine_version: String,
    pub project_id: String,
    pub project_name: String,
    pub project_type: ProjectPackageType,
    pub created_at: String,
    pub updated_at: String,
    pub files: ProjectPackageFiles,
}

pub const CIRCUIT_FORMAT_VERSION: &str = "1.0.0";
pub const CIRCUIT_ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

impl ProjectPackageManifest {
    pub fn new(
        project_id: String,
        project_name: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            format_version: CIRCUIT_FORMAT_VERSION.to_string(),
            engine_version: CIRCUIT_ENGINE_VERSION.to_string(),
            project_id,
            project_name,
            project_type: ProjectPackageType::CircuitProject,
            created_at,
            updated_at,
            files: ProjectPackageFiles::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportIndexEntry {
    pub id: String,
    pub title: String,
    pub path: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportIndex {
    pub reports: Vec<ReportIndexEntry>,
}

impl Default for ReportIndex {
    fn default() -> Self {
        Self { reports: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultIndexEntry {
    pub id: String,
    pub profile_id: String,
    pub path: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultIndex {
    pub results: Vec<ResultIndexEntry>,
}

impl Default for ResultIndex {
    fn default() -> Self {
        Self { results: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectPackageValidationReport {
    pub valid: bool,
    pub package_dir: String,
    pub missing_files: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
