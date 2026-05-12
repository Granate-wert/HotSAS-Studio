use hotsas_api::{
    AcSweepSettingsDto, HotSasApi, OperatingPointSettingsDto, SimulationPreflightResultDto,
    SimulationProbeDto, SimulationProbeTargetDto, TransientSettingsDto,
    UserCircuitSimulationProfileDto,
};
use hotsas_application::AppServices;
use hotsas_core::{CircuitProject, ProjectPackageManifest, ProjectPackageValidationReport};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ComponentLibraryPort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, SpiceModelParserPort, StoragePort,
    TouchstoneParserPort,
};
use std::sync::Arc;

// ─── Fake Adapters (copied from ngspice_simulation_api_tests.rs) ─────────────

struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(
        &self,
        _path: &std::path::Path,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(
        &self,
        _path: &std::path::Path,
    ) -> Result<hotsas_core::CircuitProject, PortError> {
        Ok(hotsas_core::rc_low_pass_project())
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &std::path::Path,
        _project: &CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, PortError> {
        Ok(ProjectPackageManifest::new(
            "test".to_string(),
            "Test".to_string(),
            "now".to_string(),
            "now".to_string(),
        ))
    }

    fn load_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<CircuitProject, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }

    fn validate_project_package(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::ProjectPackageValidationReport, PortError> {
        Ok(hotsas_core::ProjectPackageValidationReport {
            valid: true,
            package_dir: "".to_string(),
            missing_files: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }

    fn save_model_catalog(
        &self,
        _package_dir: &std::path::Path,
        _catalog: &hotsas_core::PersistedModelCatalog,
    ) -> Result<(), PortError> {
        Ok(())
    }

    fn load_model_catalog(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::PersistedModelCatalog, PortError> {
        Ok(Default::default())
    }

    fn save_model_assignments(
        &self,
        _package_dir: &std::path::Path,
        _assignments: &[hotsas_core::PersistedInstanceModelAssignment],
    ) -> Result<(), PortError> {
        Ok(())
    }

    fn load_model_assignments(
        &self,
        _package_dir: &std::path::Path,
    ) -> Result<Vec<hotsas_core::PersistedInstanceModelAssignment>, PortError> {
        Ok(vec![])
    }
}

struct FakeFormulaEngine;

impl hotsas_ports::FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &hotsas_core::ValueWithUnit,
        _capacitance: &hotsas_core::ValueWithUnit,
    ) -> Result<hotsas_core::ValueWithUnit, PortError> {
        Ok(hotsas_core::ValueWithUnit::new_si(
            159.154943,
            hotsas_core::EngineeringUnit::Hertz,
        ))
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        Ok("V1 net_in 0 AC 1\nR1 net_in net_out 10k\nC1 net_out 0 100n\n.end".to_string())
    }
}

struct FakeMockEngine;

impl SimulationEnginePort for FakeMockEngine {
    fn engine_name(&self) -> &str {
        "mock"
    }

    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Ok(successful_sim_result(profile, "mock"))
    }

    fn run_operating_point(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Ok(successful_sim_result(profile, "mock"))
    }

    fn run_transient(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Ok(successful_sim_result(profile, "mock"))
    }
}

struct FakeUnavailableNgspiceEngine;

impl SimulationEnginePort for FakeUnavailableNgspiceEngine {
    fn engine_name(&self) -> &str {
        "ngspice"
    }

    fn check_availability(&self) -> Result<hotsas_core::NgspiceAvailability, PortError> {
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
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("unavailable".to_string()))
    }

    fn run_operating_point(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("unavailable".to_string()))
    }

    fn run_transient(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("unavailable".to_string()))
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Ok("# Report".to_string())
    }
    fn export_html(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Ok("<html></html>".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryStorage;

impl ComponentLibraryPort for FakeComponentLibraryStorage {
    fn load_builtin_library(&self) -> Result<hotsas_core::ComponentLibrary, PortError> {
        Ok(hotsas_core::built_in_component_library())
    }
    fn load_library_from_path(
        &self,
        _path: &std::path::Path,
    ) -> Result<hotsas_core::ComponentLibrary, PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
    fn save_library_to_path(
        &self,
        _path: &std::path::Path,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<(), PortError> {
        Ok(())
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

struct FakeSpiceParser;

impl SpiceModelParserPort for FakeSpiceParser {
    fn parse_spice_models_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::SpiceImportReport, PortError> {
        Ok(hotsas_core::SpiceImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            source: hotsas_core::ImportedModelSource {
                file_name: None,
                file_path: None,
                source_format: "spice".to_string(),
                content_hash: None,
            },
            models: vec![],
            subcircuits: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }
}

struct FakeTouchstoneParser;

impl TouchstoneParserPort for FakeTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::TouchstoneImportReport, PortError> {
        Ok(hotsas_core::TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: None,
            warnings: vec![],
            errors: vec![],
        })
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn successful_sim_result(
    profile: &hotsas_core::SimulationProfile,
    engine: &str,
) -> hotsas_core::SimulationResult {
    let mut measurements = std::collections::BTreeMap::new();
    measurements.insert(
        "V(net_out)".to_string(),
        hotsas_core::ValueWithUnit::new_si(3.3, hotsas_core::EngineeringUnit::Volt),
    );
    hotsas_core::SimulationResult {
        id: "mock-run".to_string(),
        profile_id: profile.id.clone(),
        status: hotsas_core::SimulationStatus::Completed,
        engine: engine.to_string(),
        graph_series: vec![hotsas_core::GraphSeries {
            name: "V(net_out)".to_string(),
            x_unit: hotsas_core::EngineeringUnit::Hertz,
            y_unit: hotsas_core::EngineeringUnit::Volt,
            points: vec![hotsas_core::GraphPoint { x: 100.0, y: 3.3 }],
            metadata: std::collections::BTreeMap::new(),
        }],
        measurements,
        warnings: vec![],
        errors: vec![],
        raw_data_path: None,
        metadata: std::collections::BTreeMap::new(),
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
        Arc::new(FakeSpiceParser),
        Arc::new(FakeTouchstoneParser),
    ))
}

fn mock_ac_profile() -> UserCircuitSimulationProfileDto {
    UserCircuitSimulationProfileDto {
        id: "mock-ac".to_string(),
        name: "AC Sweep".to_string(),
        analysis_type: "AcSweep".to_string(),
        engine: "Mock".to_string(),
        probes: vec![],
        ac: Some(AcSweepSettingsDto {
            start_hz: 10.0,
            stop_hz: 1_000_000.0,
            points_per_decade: 100,
        }),
        transient: None,
        op: None,
    }
}

fn mock_op_profile() -> UserCircuitSimulationProfileDto {
    UserCircuitSimulationProfileDto {
        id: "mock-op".to_string(),
        name: "Operating Point".to_string(),
        analysis_type: "OperatingPoint".to_string(),
        engine: "Mock".to_string(),
        probes: vec![],
        ac: None,
        transient: None,
        op: Some(OperatingPointSettingsDto {
            include_node_voltages: true,
            include_branch_currents: true,
        }),
    }
}

fn mock_transient_profile() -> UserCircuitSimulationProfileDto {
    UserCircuitSimulationProfileDto {
        id: "mock-transient".to_string(),
        name: "Transient".to_string(),
        analysis_type: "Transient".to_string(),
        engine: "Mock".to_string(),
        probes: vec![],
        ac: None,
        transient: Some(TransientSettingsDto {
            step_seconds: 1e-6,
            stop_seconds: 1e-3,
        }),
        op: None,
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn list_user_circuit_simulation_profiles_returns_profiles() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let profiles = api.list_user_circuit_simulation_profiles().unwrap();
    assert!(!profiles.is_empty());
    assert!(profiles.iter().any(|p| p.id == "mock-ac"));
}

#[test]
fn suggest_user_circuit_simulation_probes_returns_probes() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let probes = api.suggest_user_circuit_simulation_probes().unwrap();
    assert!(!probes.is_empty());
    assert!(probes.iter().all(|p| p.target.net_id.is_some()));
}

#[test]
fn validate_current_circuit_for_simulation_returns_can_run() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api
        .validate_current_circuit_for_simulation(mock_ac_profile())
        .unwrap();
    assert!(
        result.can_run,
        "Expected can_run=true, got errors: {:?}",
        result.blocking_errors
    );
    assert!(result.generated_netlist_preview.is_some());
}

#[test]
fn run_current_circuit_simulation_mock_ac_returns_run_dto() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let run = api
        .run_current_circuit_simulation(mock_ac_profile())
        .unwrap();
    assert_eq!(run.status, "Succeeded");
    assert_eq!(run.engine_used, "mock");
    assert!(!run.generated_netlist.is_empty());
    assert!(run.result.is_some());
}

#[test]
fn run_current_circuit_simulation_mock_op_returns_run_dto() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let run = api
        .run_current_circuit_simulation(mock_op_profile())
        .unwrap();
    assert_eq!(run.status, "Succeeded");
    assert_eq!(run.engine_used, "mock");
    assert!(run.result.is_some());
}

#[test]
fn run_current_circuit_simulation_mock_transient_returns_run_dto() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let run = api
        .run_current_circuit_simulation(mock_transient_profile())
        .unwrap();
    assert_eq!(run.status, "Succeeded");
    assert_eq!(run.engine_used, "mock");
    assert!(run.result.is_some());
}

#[test]
fn get_last_user_circuit_simulation_after_run() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let before = api.get_last_user_circuit_simulation().unwrap();
    assert!(before.is_none());

    api.run_current_circuit_simulation(mock_ac_profile())
        .unwrap();

    let after = api.get_last_user_circuit_simulation().unwrap();
    assert!(after.is_some());
    let run = after.unwrap();
    assert_eq!(run.status, "Succeeded");
}

#[test]
fn clear_last_user_circuit_simulation_works() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();

    api.run_current_circuit_simulation(mock_ac_profile())
        .unwrap();
    assert!(api.get_last_user_circuit_simulation().unwrap().is_some());

    api.clear_last_user_circuit_simulation().unwrap();
    assert!(api.get_last_user_circuit_simulation().unwrap().is_none());
}

#[test]
fn add_last_simulation_to_advanced_report_without_run_fails() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let result = api.add_last_simulation_to_advanced_report();
    assert!(result.is_err());
}
