use hotsas_api::HotSasApi;
use hotsas_application::AppServices;
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ProjectPackageStoragePort,
    ReportExporterPort, SimulationEnginePort, StoragePort,
};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Default)]
struct FakeComponentLibraryStorage;

impl hotsas_ports::ComponentLibraryPort for FakeComponentLibraryStorage {
    fn load_builtin_library(
        &self,
    ) -> Result<hotsas_core::ComponentLibrary, hotsas_ports::PortError> {
        Ok(hotsas_core::built_in_component_library())
    }
    fn load_library_from_path(
        &self,
        _path: &std::path::Path,
    ) -> Result<hotsas_core::ComponentLibrary, hotsas_ports::PortError> {
        Err(hotsas_ports::PortError::Storage(
            "not implemented".to_string(),
        ))
    }
    fn save_library_to_path(
        &self,
        _path: &std::path::Path,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<(), hotsas_ports::PortError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(
        &self,
        _path: &Path,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<hotsas_core::CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &Path,
        project: &hotsas_core::CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, PortError> {
        Ok(hotsas_core::ProjectPackageManifest::new(
            project.id.clone(),
            project.name.clone(),
            "2024-01-01T00:00:00Z".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        ))
    }

    fn load_project_package(
        &self,
        _package_dir: &Path,
    ) -> Result<hotsas_core::CircuitProject, PortError> {
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

#[derive(Debug, Default)]
struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &hotsas_core::ValueWithUnit,
        _capacitance: &hotsas_core::ValueWithUnit,
    ) -> Result<hotsas_core::ValueWithUnit, PortError> {
        Err(PortError::Formula("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("not implemented".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
    fn export_html(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Err(PortError::Export("not implemented".to_string()))
    }
}

fn fake_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
        Arc::new(FakeComponentLibraryStorage),
    ))
}

#[test]
fn save_project_package_without_project_returns_state_error() {
    let api = fake_api();
    let result = api.save_project_package("/test/package.circuit".to_string());
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("create or open a project first"),
        "expected state error, got: {err}"
    );
}

#[test]
fn create_demo_then_save_project_package_returns_manifest() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.save_project_package("/test/package.circuit".to_string());
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert!(!manifest.project_id.is_empty());
}

#[test]
fn validate_project_package_returns_valid() {
    let api = fake_api();
    let result = api.validate_project_package("/test/package.circuit".to_string());
    assert!(result.is_ok());
    assert!(result.unwrap().valid);
}
