use hotsas_api::HotSasApi;
use hotsas_application::AppServices;
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, FormulaEnginePort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, StoragePort,
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

#[derive(Debug, Default)]
struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
    fn export_bom_json(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Ok("".to_string())
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
        Arc::new(FakeBomExporter),
        Arc::new(FakeSimulationDataExporter),
        Arc::new(FakeComponentLibraryExporter),
        Arc::new(FakeSchematicExporter),
    ))
}

#[test]
fn get_selected_component_r1_returns_parameters() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let selected = api.get_selected_component("R1".to_string()).unwrap();
    assert_eq!(selected.instance_id, "R1");
    assert_eq!(selected.component_kind, "resistor");
    assert!(selected.parameters.iter().any(|p| p.name == "resistance"));
    assert!(selected.symbol.is_some());
}

#[test]
fn get_selected_component_missing_id_returns_error() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.get_selected_component("MISSING".to_string());
    assert!(result.is_err());
}

#[test]
fn update_component_parameter_r1_resistance_changes_project() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let updated = api
        .update_component_parameter(
            "R1".to_string(),
            "resistance".to_string(),
            "4.7k".to_string(),
            Some("Ohm".to_string()),
        )
        .unwrap();
    let r1 = updated
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    let resistance = r1
        .parameters
        .iter()
        .find(|p| p.name == "resistance")
        .unwrap();
    assert!(resistance.value.display.contains("4700"));
}

#[test]
fn update_component_parameter_invalid_value_returns_error() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.update_component_parameter(
        "R1".to_string(),
        "resistance".to_string(),
        "invalid".to_string(),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn validate_current_circuit_returns_report() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let report = api.validate_current_circuit().unwrap();
    assert!(report.valid);
    assert!(report.errors.is_empty());
}
