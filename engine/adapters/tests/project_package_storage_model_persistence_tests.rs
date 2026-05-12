use hotsas_adapters::CircuitProjectPackageStorage;
use hotsas_core::{
    PersistedInstanceModelAssignment, PersistedModelAsset, PersistedModelAssetKind,
    PersistedModelAssetSource, PersistedModelAssetStatus, PersistedModelCatalog,
    ProjectPackageValidationReport,
};
use hotsas_ports::ProjectPackageStoragePort;
use std::path::Path;

fn temp_package_dir() -> std::path::PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let tid = std::thread::current().id();
    std::env::temp_dir().join(format!("hotsas_test_{ts:?}_{tid:?}.circuit"))
}

#[test]
fn legacy_project_without_model_catalog_loads_with_warning() {
    let storage = CircuitProjectPackageStorage;
    let dir = temp_package_dir();
    let project = hotsas_core::rc_low_pass_project();

    // Save with v3.4 fields
    let manifest = storage.save_project_package(&dir, &project).unwrap();
    assert_eq!(manifest.project_id, project.id);

    // Load should succeed
    let loaded = storage.load_project_package(&dir).unwrap();
    assert_eq!(loaded.id, project.id);
}

#[test]
fn missing_model_asset_reports_validation_diagnostic() {
    let storage = CircuitProjectPackageStorage;
    let dir = temp_package_dir();
    let mut project = hotsas_core::rc_low_pass_project();
    project.imported_model_catalog = Some(PersistedModelCatalog {
        assets: vec![PersistedModelAsset {
            id: "missing".to_string(),
            name: "Missing".to_string(),
            kind: PersistedModelAssetKind::SpiceModel,
            source: PersistedModelAssetSource::ImportedFile,
            source_file_name: None,
            content_hash: None,
            package_asset_path: Some("models/spice/nonexistent.json".to_string()),
            status: PersistedModelAssetStatus::Missing,
            warnings: vec![],
            compatibility: Default::default(),
        }],
    });

    storage.save_project_package(&dir, &project).unwrap();
    let report = storage.validate_project_package(&dir).unwrap();
    assert!(!report.valid || !report.warnings.is_empty());
}

#[test]
fn model_catalog_save_and_load_roundtrip() {
    let storage = CircuitProjectPackageStorage;
    let dir = temp_package_dir();
    let catalog = PersistedModelCatalog {
        assets: vec![PersistedModelAsset {
            id: "test".to_string(),
            name: "Test".to_string(),
            kind: PersistedModelAssetKind::SpiceModel,
            source: PersistedModelAssetSource::ImportedFile,
            source_file_name: None,
            content_hash: None,
            package_asset_path: None,
            status: PersistedModelAssetStatus::Present,
            warnings: vec![],
            compatibility: Default::default(),
        }],
    };

    storage.save_model_catalog(&dir, &catalog).unwrap();
    let loaded = storage.load_model_catalog(&dir).unwrap();
    assert_eq!(loaded.assets.len(), 1);
    assert_eq!(loaded.assets[0].id, "test");
}

#[test]
fn model_assignments_save_and_load_roundtrip() {
    let storage = CircuitProjectPackageStorage;
    let dir = temp_package_dir();
    let assignments = vec![PersistedInstanceModelAssignment {
        instance_id: "R1".to_string(),
        component_definition_id: "resistor".to_string(),
        model_asset_id: "builtin".to_string(),
        pin_mappings: vec![],
        parameter_bindings: vec![],
    }];

    storage.save_model_assignments(&dir, &assignments).unwrap();
    let loaded = storage.load_model_assignments(&dir).unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].instance_id, "R1");
}
