use hotsas_application::ProjectSessionService;
use hotsas_core::CircuitProject;
use hotsas_ports::PortError;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

fn temp_settings_path() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("hotsas_test_{}_{}", std::process::id(), nonce));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("session.json")
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl hotsas_ports::ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, PortError> {
        Ok(hotsas_core::ProjectPackageManifest::new(
            project.id.clone(),
            project.name.clone(),
            "2024-01-01T00:00:00Z".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        ))
    }

    fn load_project_package(&self, _package_dir: &Path) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }

    fn validate_project_package(
        &self,
        _package_dir: &Path,
    ) -> Result<hotsas_core::ProjectPackageValidationReport, PortError> {
        Ok(hotsas_core::ProjectPackageValidationReport {
            valid: true,
            package_dir: "".to_string(),
            missing_files: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }
}

fn mock_storage() -> Arc<dyn hotsas_ports::ProjectPackageStoragePort> {
    Arc::new(FakeProjectPackageStorage::default())
}

fn demo_project() -> CircuitProject {
    hotsas_core::rc_low_pass_project()
}

#[test]
fn new_session_is_clean() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    let state = service.get_state();
    assert!(!state.dirty);
    assert!(state.current_project_id.is_none());
    assert!(state.current_project_path.is_none());
}

#[test]
fn mark_dirty_works() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    service.mark_dirty("test");
    assert!(service.get_state().dirty);
}

#[test]
fn save_project_as_sets_path_and_clears_dirty() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    let project = demo_project();
    let result = service.save_project_as(&project, "test.circuit".to_string());
    assert!(result.is_ok());
    let state = service.get_state();
    assert!(!state.dirty);
    assert_eq!(state.current_project_path, Some("test.circuit".to_string()));
}

#[test]
fn save_current_project_without_path_returns_error() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    let project = demo_project();
    let result = service.save_current_project(&project);
    assert!(result.is_err());
}

#[test]
fn open_project_package_without_confirm_on_dirty_returns_error() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    service.mark_dirty("test");
    let result = service.open_project_package("some.circuit".to_string(), false);
    assert!(result.is_err());
}

#[test]
fn recent_projects_updated_after_save() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    let project = demo_project();
    service
        .save_project_as(&project, "recent.circuit".to_string())
        .unwrap();
    let recent = service.list_recent_projects();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].path, "recent.circuit");
}

#[test]
fn remove_recent_project_works() {
    let service = ProjectSessionService::new(temp_settings_path(), mock_storage());
    let project = demo_project();
    service
        .save_project_as(&project, "a.circuit".to_string())
        .unwrap();
    service
        .remove_recent_project("a.circuit".to_string())
        .unwrap();
    let recent = service.list_recent_projects();
    assert!(recent.is_empty());
}
