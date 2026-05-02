use hotsas_core::{
    ProjectPackageFiles, ProjectPackageManifest, ProjectPackageType, ProjectPackageValidationReport,
};

#[test]
fn manifest_serializes_and_deserializes() {
    let manifest = ProjectPackageManifest::new(
        "test-id".to_string(),
        "Test Project".to_string(),
        "2024-01-01T00:00:00Z".to_string(),
        "2024-01-02T00:00:00Z".to_string(),
    );
    let json = serde_json::to_string(&manifest).unwrap();
    let deserialized: ProjectPackageManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(manifest.project_id, deserialized.project_id);
    assert_eq!(manifest.project_name, deserialized.project_name);
    assert_eq!(manifest.format_version, deserialized.format_version);
    assert_eq!(manifest.project_type, ProjectPackageType::CircuitProject);
}

#[test]
fn default_files_paths_match_expected() {
    let files = ProjectPackageFiles::default();
    assert_eq!(files.schematic, "schematic.json");
    assert_eq!(files.components, "components.json");
    assert_eq!(files.formulas, "formulas.json");
    assert_eq!(files.simulation_profiles, "simulation_profiles.json");
    assert_eq!(files.reports_index, "reports/index.json");
    assert_eq!(files.results_index, "results/index.json");
}

#[test]
fn validation_report_can_represent_missing_files() {
    let report = ProjectPackageValidationReport {
        valid: false,
        package_dir: "/test".to_string(),
        missing_files: vec!["project.json".to_string()],
        warnings: vec!["old format".to_string()],
        errors: vec!["missing manifest".to_string()],
    };
    assert!(!report.valid);
    assert_eq!(report.missing_files.len(), 1);
}
