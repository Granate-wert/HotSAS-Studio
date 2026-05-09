use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectSessionState {
    pub current_project_id: Option<String>,
    pub current_project_name: Option<String>,
    pub current_project_path: Option<String>,
    pub dirty: bool,
    pub last_saved_at: Option<String>,
    pub last_loaded_at: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecentProjectEntry {
    pub path: String,
    pub display_name: String,
    pub last_opened_at: String,
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSaveResult {
    pub project_id: String,
    pub path: String,
    pub saved_at: String,
    pub warnings: Vec<ProjectPersistenceWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectOpenResult {
    pub project: crate::CircuitProject,
    pub path: String,
    pub opened_at: String,
    pub validation_warnings: Vec<ProjectPersistenceWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectPersistenceWarning {
    pub code: String,
    pub message: String,
    pub severity: ProjectPersistenceWarningSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectPersistenceWarningSeverity {
    Info,
    Warning,
    Error,
}
