use hotsas_api::{
    AcSweepSettingsDto, HotSasApi, OperatingPointSettingsDto, TransientSettingsDto,
    UserCircuitSimulationProfileDto,
};
use hotsas_application::AppServices;
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ComponentLibraryPort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, SpiceModelParserPort, StoragePort,
    TouchstoneParserPort,
};
use std::sync::Arc;

// ─── Fake Adapters ───────────────────────────────────────────────────────────

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
        _package_dir: &std::path::Path,
    ) -> Result<hotsas_core::CircuitProject, PortError> {
        Ok(hotsas_core::rc_low_pass_project())
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

fn mock_auto_profile() -> UserCircuitSimulationProfileDto {
    UserCircuitSimulationProfileDto {
        id: "auto-ac".to_string(),
        name: "Auto AC".to_string(),
        analysis_type: "AcSweep".to_string(),
        engine: "Auto".to_string(),
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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn check_ngspice_diagnostics_returns_structured_data() {
    let api = build_test_api();
    let diag = api.check_ngspice_diagnostics().unwrap();
    assert!(!diag.availability.available);
    assert_eq!(diag.availability.message, Some("not installed".to_string()));
    assert!(
        diag.errors.iter().any(|e| e.code == "NGSPICE_UNAVAILABLE"),
        "expected NGSPICE_UNAVAILABLE error"
    );
}

#[test]
fn diagnose_simulation_preflight_returns_diagnostics() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let diagnostics = api
        .diagnose_simulation_preflight(mock_ac_profile())
        .unwrap();
    // RC low-pass project has components, nets, but no ground in default template
    assert!(
        diagnostics.iter().any(|d| d.code == "NO_GROUND"),
        "expected NO_GROUND warning"
    );
    assert!(
        diagnostics.iter().any(|d| d.code == "NO_PROBES"),
        "expected NO_PROBES info"
    );
}

#[test]
fn full_workflow_api_preflight_run_history_graph_export() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();

    // 1. Preflight
    let preflight = api
        .diagnose_simulation_preflight(mock_ac_profile())
        .unwrap();
    let blocking: Vec<_> = preflight
        .iter()
        .filter(|d| d.severity == "Blocking")
        .collect();
    assert!(
        blocking.is_empty(),
        "preflight should have no blocking errors"
    );

    // 2. Run
    let run = api
        .run_current_circuit_simulation(mock_ac_profile())
        .unwrap();
    assert_eq!(run.status, "Succeeded");

    // 3. History
    api.add_run_to_history().unwrap();
    let history = api.list_simulation_history().unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].profile_name, "AC Sweep");
    assert_eq!(history[0].engine_used, "mock");

    // 4. Graph
    let view = api.build_simulation_graph_view().unwrap();
    assert!(!view.series.is_empty());
    assert!(view.x_axis.scale == "Log" || view.x_axis.scale == "Linear");

    // 5. Export
    let csv = api.export_run_series_csv().unwrap();
    assert!(csv.starts_with("series_id"));
    let json = api.export_run_series_json().unwrap();
    assert!(json.contains("run_id"));

    // 6. Last run diagnostics
    let last_run_diag = api.diagnose_last_simulation_run().unwrap();
    // Auto-mock fallback should be reported if applicable, but this was Mock engine
    assert!(
        last_run_diag.iter().all(|d| d.code != "RUN_FAILED"),
        "successful run should not have RUN_FAILED diagnostic"
    );
}

#[test]
fn auto_fallback_api_reports_mock_in_run_diagnostics() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let run = api
        .run_current_circuit_simulation(mock_auto_profile())
        .unwrap();
    assert_eq!(run.status, "Succeeded");
    assert_eq!(run.engine_used, "mock");
    assert!(
        run.warnings
            .iter()
            .any(|w| w.message.contains("ngspice unavailable")),
        "expected ngspice unavailable warning"
    );
}

#[test]
fn simulation_history_delete_and_clear_work_through_api() {
    let api = build_test_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let run = api
        .run_current_circuit_simulation(mock_ac_profile())
        .unwrap();
    api.add_run_to_history().unwrap();

    let history = api.list_simulation_history().unwrap();
    assert_eq!(history.len(), 1);

    api.delete_simulation_history_run(run.id.clone()).unwrap();
    let history_after_delete = api.list_simulation_history().unwrap();
    assert!(history_after_delete.is_empty());

    // Re-add and clear
    api.add_run_to_history().unwrap();
    api.clear_simulation_history().unwrap();
    let history_after_clear = api.list_simulation_history().unwrap();
    assert!(history_after_clear.is_empty());
}
