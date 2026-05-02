use hotsas_adapters::CircuitProjectPackageStorage;
use hotsas_core::rc_low_pass_project;
use hotsas_ports::ProjectPackageStoragePort;
use std::path::PathBuf;

fn temp_package_dir() -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp = std::env::temp_dir().join(format!("hotsas_test_{timestamp}.circuit"));
    let _ = std::fs::remove_dir_all(&temp);
    temp
}

#[test]
fn save_package_creates_circuit_folder_and_required_files() {
    let dir = temp_package_dir();
    let storage = CircuitProjectPackageStorage::default();
    let project = rc_low_pass_project();

    let manifest = storage.save_project_package(&dir, &project).unwrap();

    assert!(dir.exists());
    assert!(dir.join("project.json").exists());
    assert!(dir.join(&manifest.files.schematic).exists());
    assert!(dir.join(&manifest.files.simulation_profiles).exists());
    assert!(dir.join(&manifest.files.components).exists());
    assert!(dir.join(&manifest.files.formulas).exists());
    assert!(dir.join(&manifest.files.reports_index).exists());
    assert!(dir.join(&manifest.files.results_index).exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_package_creates_subdirectories() {
    let dir = temp_package_dir();
    let storage = CircuitProjectPackageStorage::default();
    let project = rc_low_pass_project();

    storage.save_project_package(&dir, &project).unwrap();

    assert!(dir.join("reports").exists());
    assert!(dir.join("results").exists());
    assert!(dir.join("models/spice").exists());
    assert!(dir.join("models/touchstone").exists());
    assert!(dir.join("symbols").exists());
    assert!(dir.join("footprints").exists());
    assert!(dir.join("exports").exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn load_package_roundtrip_preserves_project_id_and_name() {
    let dir = temp_package_dir();
    let storage = CircuitProjectPackageStorage::default();
    let project = rc_low_pass_project();

    storage.save_project_package(&dir, &project).unwrap();
    let loaded = storage.load_project_package(&dir).unwrap();

    assert_eq!(loaded.id, project.id);
    assert_eq!(loaded.name, project.name);
    assert_eq!(
        loaded.schematic.components.len(),
        project.schematic.components.len()
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn validate_package_reports_valid_for_complete_package() {
    let dir = temp_package_dir();
    let storage = CircuitProjectPackageStorage::default();
    let project = rc_low_pass_project();

    storage.save_project_package(&dir, &project).unwrap();
    let report = storage.validate_project_package(&dir).unwrap();

    assert!(report.valid);
    assert!(report.errors.is_empty());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn validate_package_reports_missing_project_json() {
    let dir = temp_package_dir();
    let storage = CircuitProjectPackageStorage::default();

    std::fs::create_dir_all(&dir).unwrap();
    let report = storage.validate_project_package(&dir).unwrap();

    assert!(!report.valid);
    assert!(report
        .missing_files
        .iter()
        .any(|f| f.contains("project.json")));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn package_dir_without_circuit_extension_returns_error() {
    let dir = PathBuf::from(std::env::temp_dir().join("hotsas_test_no_extension"));
    let storage = CircuitProjectPackageStorage::default();
    let project = rc_low_pass_project();

    let result = storage.save_project_package(&dir, &project);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&dir);
}
