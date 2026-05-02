use hotsas_application::{AppServices, ProjectPackageService};
use hotsas_core::{
    rc_low_pass_project, CircuitProject, ProjectPackageManifest, ProjectPackageValidationReport,
    ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ProjectPackageStoragePort,
    ReportExporterPort, SimulationEnginePort, StoragePort,
};
use std::path::Path;
use std::sync::Arc;

fn temp_package_dir() -> std::path::PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp = std::env::temp_dir().join(format!("hotsas_test_{timestamp}.circuit"));
    let _ = std::fs::remove_dir_all(&temp);
    temp
}

#[derive(Debug, Default)]
struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(&self, _path: &Path, _project: &CircuitProject) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
}

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Default)]
struct FakeProjectPackageStorage {
    projects: Mutex<HashMap<std::path::PathBuf, CircuitProject>>,
}

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError> {
        self.projects
            .lock()
            .unwrap()
            .insert(package_dir.to_path_buf(), project.clone());
        Ok(ProjectPackageManifest::new(
            project.id.clone(),
            project.name.clone(),
            "2024-01-01T00:00:00Z".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        ))
    }

    fn load_project_package(&self, package_dir: &Path) -> Result<CircuitProject, PortError> {
        self.projects
            .lock()
            .unwrap()
            .get(package_dir)
            .cloned()
            .ok_or_else(|| PortError::Storage("not found".to_string()))
    }

    fn validate_project_package(
        &self,
        _package_dir: &Path,
    ) -> Result<ProjectPackageValidationReport, PortError> {
        Ok(ProjectPackageValidationReport {
            valid: true,
            package_dir: "".to_string(),
            missing_files: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }
}

#[derive(Debug, Default)]
struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        Err(PortError::Formula("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn run_ac_sweep(
        &self,
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
}

fn fake_services() -> AppServices {
    AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
    )
}

#[test]
fn project_package_service_save_load_roundtrip() {
    let dir = temp_package_dir();
    let storage =
        Arc::new(FakeProjectPackageStorage::default()) as Arc<dyn ProjectPackageStoragePort>;
    let service = ProjectPackageService::new(storage);
    let project = rc_low_pass_project();

    let manifest = service.save_project_package(&dir, &project).unwrap();
    assert_eq!(manifest.project_id, project.id);

    let loaded = service.load_project_package(&dir).unwrap();
    assert_eq!(loaded.id, project.id);
    assert_eq!(loaded.name, project.name);
}

#[test]
fn app_services_exposes_project_package_service() {
    let services = fake_services();
    let dir = temp_package_dir();
    let project = rc_low_pass_project();

    let manifest = services.save_project_package(&dir, &project).unwrap();
    assert!(!manifest.project_id.is_empty());
}
