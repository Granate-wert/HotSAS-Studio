use hotsas_api::{HotSasApi, SimulationRunRequestDto};
use hotsas_application::AppServices;
use hotsas_ports::SimulationEnginePort;
use std::sync::Arc;

struct FakeMockEngine;

impl SimulationEnginePort for FakeMockEngine {
    fn engine_name(&self) -> &str {
        "mock"
    }

    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, hotsas_ports::PortError> {
        Ok(hotsas_core::SimulationResult {
            id: "mock-ac".to_string(),
            profile_id: profile.id.clone(),
            status: hotsas_core::SimulationStatus::Completed,
            engine: "mock".to_string(),
            graph_series: vec![],
            measurements: std::collections::BTreeMap::new(),
            warnings: vec![],
            errors: vec![],
            raw_data_path: None,
            metadata: std::collections::BTreeMap::new(),
        })
    }
}

struct FakeUnavailableNgspiceEngine;

impl SimulationEnginePort for FakeUnavailableNgspiceEngine {
    fn engine_name(&self) -> &str {
        "ngspice"
    }

    fn check_availability(
        &self,
    ) -> Result<hotsas_core::NgspiceAvailability, hotsas_ports::PortError> {
        Ok(hotsas_core::NgspiceAvailability {
            available: false,
            executable_path: None,
            version: None,
            message: Some("not installed".to_string()),
            warnings: vec![],
        })
    }

    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, hotsas_ports::PortError> {
        Err(hotsas_ports::PortError::Simulation(
            "unavailable".to_string(),
        ))
    }

    fn run_operating_point(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, hotsas_ports::PortError> {
        Err(hotsas_ports::PortError::Simulation(
            "unavailable".to_string(),
        ))
    }

    fn run_transient(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, hotsas_ports::PortError> {
        Err(hotsas_ports::PortError::Simulation(
            "unavailable".to_string(),
        ))
    }
}

struct FakeStorage;

impl hotsas_ports::StoragePort for FakeStorage {
    fn save_project(
        &self,
        _path: &std::path::Path,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<(), hotsas_ports::PortError> {
        Ok(())
    }
    fn load_project(
        &self,
        _path: &std::path::Path,
    ) -> Result<hotsas_core::CircuitProject, hotsas_ports::PortError> {
        Ok(hotsas_core::rc_low_pass_project())
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl hotsas_ports::ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &std::path::Path,
        project: &hotsas_core::CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, hotsas_ports::PortError> {
        Ok(hotsas_core::ProjectPackageManifest::new(
            project.id.clone(),
            project.name.clone(),
            "2024-01-01T00:00:00Z".to_string(),
            "2024-01-01T00:00:00Z".to_string(),
        ))
    }
    fn load_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::CircuitProject, hotsas_ports::PortError> {
        Ok(hotsas_core::rc_low_pass_project())
    }
    fn validate_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::ProjectPackageValidationReport, hotsas_ports::PortError> {
        Ok(hotsas_core::ProjectPackageValidationReport {
            valid: true,
            package_dir: "".to_string(),
            missing_files: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }
}

struct FakeFormulaEngine;

impl hotsas_ports::FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &hotsas_core::ValueWithUnit,
        _capacitance: &hotsas_core::ValueWithUnit,
    ) -> Result<hotsas_core::ValueWithUnit, hotsas_ports::PortError> {
        Ok(hotsas_core::ValueWithUnit::new_si(
            159.154943,
            hotsas_core::EngineeringUnit::Hertz,
        ))
    }
}

struct FakeNetlistExporter;

impl hotsas_ports::NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok(
            "V1 net_in 0 AC 1\nR1 net_in net_out 10k\nC1 net_out 0 100n\n.ac dec 100 10 1e6\n.end"
                .to_string(),
        )
    }
}

struct FakeReportExporter;

impl hotsas_ports::ReportExporterPort for FakeReportExporter {
    fn export_markdown(
        &self,
        _report: &hotsas_core::ReportModel,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("# Report".to_string())
    }
    fn export_html(
        &self,
        _report: &hotsas_core::ReportModel,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("<html></html>".to_string())
    }
}

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
struct FakeBomExporter;

impl hotsas_ports::BomExporterPort for FakeBomExporter {
    fn export_bom_csv(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("".to_string())
    }
    fn export_bom_json(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl hotsas_ports::SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl hotsas_ports::ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl hotsas_ports::SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, hotsas_ports::PortError> {
        Ok("".to_string())
    }
}

fn build_test_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeMockEngine),
        Arc::new(FakeUnavailableNgspiceEngine),
        Arc::new(FakeReportExporter),
        Arc::new(FakeComponentLibraryStorage),
        Arc::new(FakeBomExporter),
        Arc::new(FakeSimulationDataExporter),
        Arc::new(FakeComponentLibraryExporter),
        Arc::new(FakeSchematicExporter),
    ))
}

#[test]
fn check_ngspice_availability_returns_dto() {
    let api = build_test_api();
    let dto = api.check_ngspice_availability().expect("should return DTO");
    assert!(!dto.available);
    assert!(dto.message.is_some());
}

#[test]
fn run_simulation_mock_returns_dto() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().expect("demo project");
    let request = SimulationRunRequestDto {
        engine: "mock".to_string(),
        analysis_kind: "ac_sweep".to_string(),
        profile_id: None,
        output_variables: vec![],
        timeout_ms: None,
    };
    let dto = api.run_simulation(request).expect("should return DTO");
    assert_eq!(dto.engine, "mock");
}

#[test]
fn run_simulation_ngspice_unavailable_returns_controlled_error() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().expect("demo project");
    let request = SimulationRunRequestDto {
        engine: "ngspice".to_string(),
        analysis_kind: "ac_sweep".to_string(),
        profile_id: None,
        output_variables: vec![],
        timeout_ms: None,
    };
    let result = api.run_simulation(request);
    assert!(result.is_err());
}

#[test]
fn simulation_history_returns_last_runs() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().expect("demo project");
    let request = SimulationRunRequestDto {
        engine: "mock".to_string(),
        analysis_kind: "ac_sweep".to_string(),
        profile_id: None,
        output_variables: vec![],
        timeout_ms: None,
    };
    api.run_simulation(request).expect("should run");
    let history = api.simulation_history().expect("should return history");
    assert_eq!(history.len(), 1);
}
