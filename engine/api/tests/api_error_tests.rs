use hotsas_api::{ApiError, HotSasApi};
use hotsas_application::{AppServices, ApplicationError};
use hotsas_core::{
    CircuitProject, ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
use hotsas_core::{ProjectPackageManifest, ProjectPackageValidationReport};
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

#[test]
fn api_error_dto_has_stable_codes_messages_and_details() {
    let invalid = ApiError::InvalidInput("unsupported unit: Volt".to_string()).to_dto();
    assert_eq!(invalid.code, "invalid_input");
    assert!(invalid.message.contains("unsupported unit"));
    assert_eq!(invalid.details.as_deref(), Some("unsupported unit: Volt"));

    let state = ApiError::State("create or open a project first".to_string()).to_dto();
    assert_eq!(state.code, "state_error");
    assert!(state.message.contains("state error"));
    assert_eq!(
        state.details.as_deref(),
        Some("create or open a project first")
    );

    let port = ApiError::Application(ApplicationError::Port(PortError::Storage(
        "disk unavailable".to_string(),
    )))
    .to_dto();
    assert_eq!(port.code, "port_error");
    assert!(port.message.contains("storage error"));
    assert!(port
        .details
        .as_deref()
        .unwrap()
        .contains("disk unavailable"));
}

#[test]
fn facade_returns_state_errors_before_project_is_created() {
    let api = HotSasApi::new(fake_services());

    for (name, result) in [
        ("calculate", api.calculate_rc_low_pass().map(|_| ())),
        ("nearest_e24", api.nearest_e24_for_resistor().map(|_| ())),
        ("netlist", api.generate_spice_netlist().map(|_| ())),
        ("markdown", api.export_markdown_report().map(|_| ())),
        (
            "save",
            api.save_project_json("unused/project.json".to_string())
                .map(|_| ()),
        ),
    ] {
        let dto = result.unwrap_err().to_dto();

        assert_eq!(dto.code, "state_error", "{name} must return state_error");
        assert!(
            dto.message.contains("create or open a project first"),
            "{name} returned unexpected message: {}",
            dto.message
        );
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError> {
        Ok(ProjectPackageManifest::new(
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

fn fake_services() -> AppServices {
    AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
        Arc::new(FakeComponentLibraryStorage),
        Arc::new(FakeBomExporter),
        Arc::new(FakeSimulationDataExporter),
        Arc::new(FakeComponentLibraryExporter),
        Arc::new(FakeSchematicExporter),
    )
}

struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(&self, _path: &Path, _project: &CircuitProject) -> Result<(), PortError> {
        panic!("state tests must not call storage")
    }

    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
        panic!("state tests must not call storage")
    }
}

struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        panic!("state tests must not call formula engine")
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        panic!("state tests must not call netlist exporter")
    }
}

struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn engine_name(&self) -> &str {
        "fake"
    }
    fn run_ac_sweep(
        &self,
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        panic!("state tests must not call simulation engine")
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("state tests must not call report exporter")
    }

    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("state tests must not call report exporter")
    }
}

#[derive(Debug, Default)]
struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        panic!("state tests must not call bom exporter")
    }
    fn export_bom_json(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        panic!("state tests must not call bom exporter")
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, PortError> {
        panic!("state tests must not call simulation data exporter")
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        panic!("state tests must not call component library exporter")
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        panic!("state tests must not call schematic exporter")
    }
}
