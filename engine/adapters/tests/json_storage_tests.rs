use hotsas_adapters::JsonProjectStorage;
use hotsas_core::{rc_low_pass_project, CircuitQueryService};
use hotsas_ports::StoragePort;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn json_storage_roundtrips_rc_low_pass_project() {
    let storage = JsonProjectStorage;
    let project = rc_low_pass_project();
    let path = temp_path("roundtrip").join("nested").join("project.json");

    storage.save_project(&path, &project).unwrap();
    let loaded = storage.load_project(&path).unwrap();

    assert_eq!(loaded.id, project.id);
    assert_eq!(loaded.name, project.name);
    assert_eq!(loaded.format_version, project.format_version);
    assert_eq!(
        loaded.schematic.components.len(),
        project.schematic.components.len()
    );
    assert_eq!(loaded.schematic.nets.len(), project.schematic.nets.len());
    assert!(
        CircuitQueryService::get_component_parameter(&loaded, "R1", "resistance").is_some(),
        "loaded project must preserve R1 resistance"
    );
    assert!(
        CircuitQueryService::get_component_parameter(&loaded, "C1", "capacitance").is_some(),
        "loaded project must preserve C1 capacitance"
    );
}

#[test]
fn json_storage_creates_parent_directories_on_save() {
    let storage = JsonProjectStorage;
    let path = temp_path("parent-dirs")
        .join("a")
        .join("b")
        .join("project.json");

    storage.save_project(&path, &rc_low_pass_project()).unwrap();

    assert!(path.exists(), "save_project must create parent directories");
}

#[test]
fn json_storage_returns_errors_for_missing_invalid_or_empty_files() {
    let storage = JsonProjectStorage;
    let dir = temp_path("invalid-loads");

    assert!(storage.load_project(&dir.join("missing.json")).is_err());

    fs::create_dir_all(&dir).unwrap();
    let invalid = dir.join("invalid.json");
    fs::write(&invalid, "{not valid json").unwrap();
    assert!(storage.load_project(&invalid).is_err());

    let empty = dir.join("empty.json");
    fs::write(&empty, "").unwrap();
    assert!(storage.load_project(&empty).is_err());
}

fn temp_path(label: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("hotsas-{label}-{timestamp}"))
}
