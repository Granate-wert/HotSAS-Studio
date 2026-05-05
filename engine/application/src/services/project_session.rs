use crate::ApplicationError;
use hotsas_core::{
    CircuitProject, ProjectOpenResult, ProjectSaveResult, ProjectSessionState, RecentProjectEntry,
};
use hotsas_ports::ProjectPackageStoragePort;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

const MAX_RECENT_PROJECTS: usize = 20;

#[derive(Clone)]
pub struct ProjectSessionService {
    state: Arc<Mutex<ProjectSessionState>>,
    settings_path: PathBuf,
    package_storage: Arc<dyn ProjectPackageStoragePort>,
}

impl ProjectSessionService {
    pub fn new(
        settings_path: PathBuf,
        package_storage: Arc<dyn ProjectPackageStoragePort>,
    ) -> Self {
        let mut initial = ProjectSessionState::default();
        if let Ok(content) = std::fs::read_to_string(&settings_path) {
            if let Ok(saved) = serde_json::from_str::<SavedSessionState>(&content) {
                initial.current_project_path = saved.current_project_path;
                initial.current_project_name = saved.current_project_name;
                initial.current_project_id = saved.current_project_id;
                initial.last_saved_at = saved.last_saved_at;
                initial.last_loaded_at = saved.last_loaded_at;
            }
        }
        Self {
            state: Arc::new(Mutex::new(initial)),
            settings_path,
            package_storage,
        }
    }

    pub fn get_state(&self) -> ProjectSessionState {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn mark_dirty(&self, _reason: &str) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.dirty = true;
        state.last_error = None;
    }

    pub fn mark_clean(&self) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.dirty = false;
        state.last_error = None;
    }

    pub fn set_current_project(&self, project: &CircuitProject, path: Option<String>) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.current_project_id = Some(project.id.clone());
        state.current_project_name = Some(project.name.clone());
        state.current_project_path = path.clone();
        state.dirty = false;
        drop(state);
        if let Some(p) = path {
            let _ = self.add_recent_project_entry(p, project.name.clone());
        }
        self.save_session_state();
    }

    pub fn clear_current_project(&self) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        *state = ProjectSessionState::default();
        drop(state);
        self.save_session_state();
    }

    pub fn save_current_project(
        &self,
        project: &CircuitProject,
    ) -> Result<ProjectSaveResult, ApplicationError> {
        let path = {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            state
                .current_project_path
                .clone()
                .ok_or_else(|| ApplicationError::Storage("no current project path".to_string()))?
        };
        self.save_project_as(project, path)
    }

    pub fn save_project_as(
        &self,
        project: &CircuitProject,
        path: String,
    ) -> Result<ProjectSaveResult, ApplicationError> {
        let package_dir = Path::new(&path);
        self.package_storage
            .save_project_package(package_dir, project)
            .map_err(|e| ApplicationError::Storage(e.to_string()))?;

        let now = chrono::Local::now().to_rfc3339();
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.dirty = false;
        state.last_saved_at = Some(now.clone());
        state.current_project_path = Some(path.clone());
        state.current_project_name = Some(project.name.clone());
        state.current_project_id = Some(project.id.clone());
        state.last_error = None;
        drop(state);
        self.add_recent_project_entry(path.clone(), project.name.clone())?;
        self.save_session_state();

        Ok(ProjectSaveResult {
            project_id: project.id.clone(),
            path,
            saved_at: now,
            warnings: vec![],
        })
    }

    pub fn open_project_package(
        &self,
        path: String,
        confirm_discard_unsaved: bool,
    ) -> Result<ProjectOpenResult, ApplicationError> {
        {
            let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            if state.dirty && !confirm_discard_unsaved {
                return Err(ApplicationError::State(
                    "unsaved changes: confirm_discard_unsaved is false".to_string(),
                ));
            }
        }

        let package_dir = Path::new(&path);
        let project = self
            .package_storage
            .load_project_package(package_dir)
            .map_err(|e| ApplicationError::Storage(e.to_string()))?;

        let now = chrono::Local::now().to_rfc3339();
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.dirty = false;
        state.last_loaded_at = Some(now.clone());
        state.current_project_path = Some(path.clone());
        state.current_project_name = Some(project.name.clone());
        state.current_project_id = Some(project.id.clone());
        state.last_error = None;
        drop(state);
        self.add_recent_project_entry(path.clone(), project.name.clone())?;
        self.save_session_state();

        Ok(ProjectOpenResult {
            validation_warnings: vec![],
            project,
            path,
            opened_at: now,
        })
    }

    pub fn list_recent_projects(&self) -> Vec<RecentProjectEntry> {
        self.load_recent_projects()
            .into_iter()
            .map(|r| RecentProjectEntry {
                exists: Path::new(&r.path).exists(),
                path: r.path,
                display_name: r.display_name,
                last_opened_at: r.last_opened_at,
            })
            .collect()
    }

    pub fn remove_recent_project(&self, path: String) -> Result<(), ApplicationError> {
        let mut settings = self.load_settings();
        settings.recent_projects.retain(|r| r.path != path);
        self.save_settings(&settings)
            .map_err(|e| ApplicationError::Storage(e.to_string()))
    }

    pub fn clear_missing_recent_projects(&self) -> Result<usize, ApplicationError> {
        let mut settings = self.load_settings();
        let before = settings.recent_projects.len();
        settings
            .recent_projects
            .retain(|r| Path::new(&r.path).exists());
        let after = settings.recent_projects.len();
        self.save_settings(&settings)
            .map_err(|e| ApplicationError::Storage(e.to_string()))?;
        Ok(before - after)
    }

    fn add_recent_project_entry(
        &self,
        path: String,
        display_name: String,
    ) -> Result<(), ApplicationError> {
        let mut settings = self.load_settings();
        let now = chrono::Local::now().to_rfc3339();
        settings.recent_projects.retain(|r| r.path != path);
        settings.recent_projects.insert(
            0,
            RecentProjectRecord {
                path,
                display_name,
                last_opened_at: now,
            },
        );
        if settings.recent_projects.len() > MAX_RECENT_PROJECTS {
            settings.recent_projects.truncate(MAX_RECENT_PROJECTS);
        }
        self.save_settings(&settings)
            .map_err(|e| ApplicationError::Storage(e.to_string()))
    }

    fn load_settings(&self) -> LocalSettings {
        if !self.settings_path.exists() {
            return LocalSettings::default();
        }
        std::fs::read_to_string(&self.settings_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    fn save_settings(&self, settings: &LocalSettings) -> Result<(), std::io::Error> {
        if let Some(parent) = self.settings_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(settings)?;
        std::fs::write(&self.settings_path, content)
    }

    fn load_recent_projects(&self) -> Vec<RecentProjectRecord> {
        self.load_settings().recent_projects
    }

    fn save_session_state(&self) {
        let state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let saved = SavedSessionState {
            current_project_path: state.current_project_path.clone(),
            current_project_name: state.current_project_name.clone(),
            current_project_id: state.current_project_id.clone(),
            last_saved_at: state.last_saved_at.clone(),
            last_loaded_at: state.last_loaded_at.clone(),
        };
        drop(state);
        let _ = self.save_settings_including_state(&saved);
    }

    fn save_settings_including_state(
        &self,
        state: &SavedSessionState,
    ) -> Result<(), std::io::Error> {
        let mut settings = self.load_settings();
        settings.current_project_path = state.current_project_path.clone();
        settings.current_project_name = state.current_project_name.clone();
        settings.current_project_id = state.current_project_id.clone();
        settings.last_saved_at = state.last_saved_at.clone();
        settings.last_loaded_at = state.last_loaded_at.clone();
        self.save_settings(&settings)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct LocalSettings {
    #[serde(default)]
    pub recent_projects: Vec<RecentProjectRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_project_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_project_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_saved_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_loaded_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RecentProjectRecord {
    pub path: String,
    pub display_name: String,
    pub last_opened_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct SavedSessionState {
    pub current_project_path: Option<String>,
    pub current_project_name: Option<String>,
    pub current_project_id: Option<String>,
    pub last_saved_at: Option<String>,
    pub last_loaded_at: Option<String>,
}
