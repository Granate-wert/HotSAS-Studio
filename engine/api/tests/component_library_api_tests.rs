use hotsas_api::HotSasApi;
use hotsas_application::AppServices;
use hotsas_core::{
    CircuitProject, EngineeringUnit, ProjectPackageManifest, ProjectPackageValidationReport,
    ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
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
    fn save_project(&self, _path: &Path, _project: &CircuitProject) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not found".to_string()))
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &Path,
        _project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
    fn load_project_package(&self, _package_dir: &Path) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
    fn validate_project_package(
        &self,
        _package_dir: &Path,
    ) -> Result<ProjectPackageValidationReport, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
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
        Ok(ValueWithUnit::new_si(159.0, EngineeringUnit::Hertz))
    }
}

#[derive(Debug, Default)]
struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("* netlist".to_string())
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
        Ok("# report".to_string())
    }
    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        Ok("<html></html>".to_string())
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
fn load_builtin_component_library_returns_metadata() {
    let api = fake_api();
    let lib = api.load_builtin_component_library().unwrap();
    assert!(!lib.id.is_empty());
    assert!(!lib.components.is_empty());
}

#[test]
fn list_components_returns_non_empty_list() {
    let api = fake_api();
    let list = api.list_components().unwrap();
    assert!(!list.is_empty());
}

#[test]
fn search_components_resistor_returns_generic_resistor() {
    let api = fake_api();
    let result = api
        .search_components(hotsas_api::ComponentSearchRequestDto {
            search: Some("resistor".to_string()),
            category: None,
            tags: vec![],
            manufacturer: None,
            has_symbol: None,
            has_footprint: None,
            has_simulation_model: None,
        })
        .unwrap();
    assert!(result.components.iter().any(|c| c.id == "generic_resistor"));
}

#[test]
fn get_component_details_generic_resistor_returns_parameters() {
    let api = fake_api();
    let details = api
        .get_component_details("generic_resistor".to_string())
        .unwrap();
    assert_eq!(details.id, "generic_resistor");
    assert!(!details.parameters.is_empty());
}

#[test]
fn assign_component_to_selected_instance_without_project_returns_state_error() {
    let api = fake_api();
    let result = api.assign_component_to_selected_instance(hotsas_api::AssignComponentRequestDto {
        instance_id: "R1".to_string(),
        component_definition_id: "generic_resistor".to_string(),
        selected_symbol_id: None,
        selected_footprint_id: None,
        selected_simulation_model_id: None,
    });
    assert!(result.is_err());
}

#[test]
fn create_rc_demo_then_assign_generic_resistor_to_r1_works() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.assign_component_to_selected_instance(hotsas_api::AssignComponentRequestDto {
        instance_id: "R1".to_string(),
        component_definition_id: "generic_resistor".to_string(),
        selected_symbol_id: Some("resistor".to_string()),
        selected_footprint_id: None,
        selected_simulation_model_id: None,
    });
    assert!(result.is_ok());
    let project = result.unwrap();
    let r1 = project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    assert_eq!(r1.definition_id, "generic_resistor");
}

#[test]
fn assigning_missing_component_returns_error() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.assign_component_to_selected_instance(hotsas_api::AssignComponentRequestDto {
        instance_id: "R1".to_string(),
        component_definition_id: "missing_component".to_string(),
        selected_symbol_id: None,
        selected_footprint_id: None,
        selected_simulation_model_id: None,
    });
    assert!(result.is_err());
}

#[test]
fn assigning_missing_instance_returns_error() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.assign_component_to_selected_instance(hotsas_api::AssignComponentRequestDto {
        instance_id: "MISSING".to_string(),
        component_definition_id: "generic_resistor".to_string(),
        selected_symbol_id: None,
        selected_footprint_id: None,
        selected_simulation_model_id: None,
    });
    assert!(result.is_err());
}
