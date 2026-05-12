use hotsas_application::AppServices;
use hotsas_core::{
    CircuitProject, ComponentDefinition, ComponentInstance, ComponentLibrary, EngineeringUnit,
    NgspiceAvailability, ProjectPackageManifest, ProjectPackageValidationReport, SimulationModel,
    SimulationModelKind, SimulationProbe, SimulationProbeKind, SimulationProbeTarget,
    SimulationStatus, ValueWithUnit,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ComponentLibraryPort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, SpiceModelParserPort, StoragePort,
    TouchstoneParserPort,
};
use std::collections::BTreeMap;
use std::sync::Arc;

// ─── Fake Adapters ───────────────────────────────────────────────────────────

struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(
        &self,
        _path: &std::path::Path,
        _project: &CircuitProject,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &std::path::Path) -> Result<CircuitProject, PortError> {
        Ok(rc_low_pass_project_with_ground())
    }
}

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &std::path::Path,
        _project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError> {
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
    ) -> Result<ProjectPackageValidationReport, PortError> {
        Ok(ProjectPackageValidationReport {
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
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        Ok(ValueWithUnit::new_si(159.154943, EngineeringUnit::Hertz))
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
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
        _project: &CircuitProject,
        profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Ok(successful_sim_result(profile, "mock"))
    }

    fn run_operating_point(
        &self,
        _project: &CircuitProject,
        profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Ok(successful_sim_result(profile, "mock"))
    }

    fn run_transient(
        &self,
        _project: &CircuitProject,
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

    fn check_availability(&self) -> Result<NgspiceAvailability, PortError> {
        Ok(NgspiceAvailability {
            available: false,
            executable_path: None,
            version: None,
            message: Some("not installed".to_string()),
            warnings: vec![],
        })
    }

    fn run_ac_sweep(
        &self,
        _project: &CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("unavailable".to_string()))
    }

    fn run_operating_point(
        &self,
        _project: &CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Err(PortError::Simulation("unavailable".to_string()))
    }

    fn run_transient(
        &self,
        _project: &CircuitProject,
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
    fn export_bom_csv(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
    fn export_bom_json(&self, _project: &CircuitProject) -> Result<String, PortError> {
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
    fn export_svg_schematic(&self, _project: &CircuitProject) -> Result<String, PortError> {
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
    let mut measurements = BTreeMap::new();
    measurements.insert(
        "V(net_out)".to_string(),
        ValueWithUnit::new_si(3.3, EngineeringUnit::Volt),
    );
    hotsas_core::SimulationResult {
        id: "mock-run".to_string(),
        profile_id: profile.id.clone(),
        status: SimulationStatus::Completed,
        engine: engine.to_string(),
        graph_series: vec![hotsas_core::GraphSeries {
            name: "V(net_out)".to_string(),
            x_unit: EngineeringUnit::Hertz,
            y_unit: EngineeringUnit::Volt,
            points: vec![hotsas_core::GraphPoint { x: 100.0, y: 3.3 }],
            metadata: BTreeMap::new(),
        }],
        measurements,
        warnings: vec![],
        errors: vec![],
        raw_data_path: None,
        metadata: BTreeMap::new(),
    }
}

fn rc_low_pass_project_with_ground() -> CircuitProject {
    let mut project = hotsas_core::rc_low_pass_project();
    project.schematic.components.push(ComponentInstance {
        instance_id: "GND1".to_string(),
        definition_id: "ground_reference".to_string(),
        selected_symbol_id: None,
        selected_footprint_id: None,
        selected_simulation_model_id: None,
        position: hotsas_core::Point::new(430.0, 320.0),
        rotation_degrees: 0.0,
        connected_nets: vec![hotsas_core::ConnectedPin {
            component_id: "GND1".to_string(),
            pin_id: "gnd".to_string(),
            net_id: "gnd".to_string(),
        }],
        overridden_parameters: BTreeMap::new(),
        notes: None,
    });
    project
}

fn build_services() -> AppServices {
    AppServices::new(
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
    )
}

fn mock_profile() -> hotsas_core::UserCircuitSimulationProfile {
    hotsas_core::UserCircuitSimulationProfile {
        id: "mock-ac".to_string(),
        name: "AC Sweep".to_string(),
        analysis_type: hotsas_core::UserCircuitAnalysisType::AcSweep,
        engine: hotsas_core::UserCircuitSimulationEngine::Mock,
        probes: vec![],
        ac: Some(hotsas_core::AcSweepSettings {
            start_hz: 10.0,
            stop_hz: 1_000_000.0,
            points_per_decade: 100,
        }),
        transient: None,
        op: None,
    }
}

// ─── Integration Tests ───────────────────────────────────────────────────────

#[test]
fn ngspice_missing_returns_structured_diagnostics() {
    let services = build_services();
    let diagnostics = services
        .simulation_diagnostics_service()
        .check_ngspice_diagnostics()
        .unwrap();
    assert!(!diagnostics.availability.available);
    assert_eq!(
        diagnostics.availability.message,
        Some("not installed".to_string())
    );
    assert!(
        diagnostics
            .errors
            .iter()
            .any(|e| e.code == "NGSPICE_UNAVAILABLE"),
        "expected NGSPICE_UNAVAILABLE error"
    );
    assert!(
        diagnostics.errors.iter().any(|e| e
            .suggested_fix
            .as_ref()
            .unwrap()
            .contains("Install ngspice")),
        "expected suggested fix mentioning ngspice installation"
    );
}

#[test]
fn auto_mode_fallback_reports_mock_engine_not_ngspice() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = hotsas_core::UserCircuitSimulationProfile {
        id: "auto-ac".to_string(),
        name: "Auto AC".to_string(),
        analysis_type: hotsas_core::UserCircuitAnalysisType::AcSweep,
        engine: hotsas_core::UserCircuitSimulationEngine::Auto,
        probes: vec![],
        ac: Some(hotsas_core::AcSweepSettings {
            start_hz: 10.0,
            stop_hz: 1_000_000.0,
            points_per_decade: 100,
        }),
        transient: None,
        op: None,
    };
    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();
    assert_eq!(run.engine_used, "mock");
    assert!(
        run.warnings
            .iter()
            .any(|w| w.message.contains("ngspice unavailable")),
        "expected ngspice unavailable warning in run"
    );
}

#[test]
fn diagnose_missing_ground_returns_suggested_fix() {
    let services = build_services();
    let mut project = rc_low_pass_project_with_ground();
    // Remove ground component
    project
        .schematic
        .components
        .retain(|c| !c.definition_id.contains("ground"));
    let profile = mock_profile();
    let diagnostics = services
        .simulation_diagnostics_service()
        .diagnose_simulation_preflight(&project, &profile)
        .unwrap();
    let ground_diag = diagnostics
        .iter()
        .find(|d| d.code == "NO_GROUND")
        .expect("expected NO_GROUND diagnostic");
    assert_eq!(
        ground_diag.severity,
        hotsas_core::SimulationDiagnosticSeverity::Warning
    );
    assert!(
        ground_diag
            .suggested_fix
            .as_ref()
            .unwrap()
            .contains("ground"),
        "expected suggested fix mentioning ground"
    );
}

#[test]
fn diagnose_invalid_probe_returns_blocking_message() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let mut profile = mock_profile();
    profile.probes.push(SimulationProbe {
        id: "bad-probe".to_string(),
        label: "Bad".to_string(),
        kind: SimulationProbeKind::NodeVoltage,
        target: SimulationProbeTarget::Net {
            net_id: "nonexistent-net".to_string(),
        },
        unit: Some(EngineeringUnit::Volt),
    });
    let diagnostics = services
        .simulation_diagnostics_service()
        .diagnose_simulation_preflight(&project, &profile)
        .unwrap();
    let probe_diag = diagnostics
        .iter()
        .find(|d| d.code == "INVALID_PROBE_NET")
        .expect("expected INVALID_PROBE_NET diagnostic");
    assert_eq!(
        probe_diag.severity,
        hotsas_core::SimulationDiagnosticSeverity::Blocking
    );
    assert!(
        probe_diag
            .suggested_fix
            .as_ref()
            .unwrap()
            .contains("existing net"),
        "expected suggested fix mentioning existing net"
    );
}

#[test]
fn preflight_includes_model_mapping_diagnostics_with_component_and_model_ids() {
    let services = build_services();
    let mut project = rc_low_pass_project_with_ground();
    project.schematic.components = vec![
        component_instance("X1", "custom_unknown"),
        component_instance("U1", "generic_op_amp"),
        component_instance("Rmissing", "generic_resistor"),
        component_instance("Ubad", "bad_subckt"),
    ];
    let mut library = ComponentLibrary {
        components: vec![
            generic_op_amp_definition(),
            resistor_without_required_parameter(),
            bad_subckt_definition(),
        ],
        ..hotsas_core::built_in_component_library()
    };
    library
        .components
        .retain(|component| component.id != "generic_resistor" || component.parameters.is_empty());

    let diagnostics = services
        .simulation_diagnostics_service()
        .diagnose_simulation_preflight_with_library(&project, &mock_profile(), &library)
        .unwrap();

    assert_model_diagnostic(
        &diagnostics,
        "MISSING_MODEL",
        hotsas_core::SimulationDiagnosticSeverity::Blocking,
        "X1",
        None,
    );
    assert_model_diagnostic(
        &diagnostics,
        "PLACEHOLDER_MODEL",
        hotsas_core::SimulationDiagnosticSeverity::Warning,
        "U1",
        Some("generic_op_amp_model"),
    );
    assert_model_diagnostic(
        &diagnostics,
        "MISSING_MODEL_PARAMETER",
        hotsas_core::SimulationDiagnosticSeverity::Blocking,
        "Rmissing",
        Some("builtin_resistor_primitive"),
    );
    assert_model_diagnostic(
        &diagnostics,
        "INVALID_PIN_MAPPING",
        hotsas_core::SimulationDiagnosticSeverity::Blocking,
        "Ubad",
        Some("bad_subckt_model"),
    );
}

#[test]
fn run_history_adds_and_lists_runs() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    services.simulation_history_service().add_run(&run).unwrap();
    let history = services
        .simulation_history_service()
        .list_runs(&project.id)
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].run_id, run.id);
    assert_eq!(history[0].profile_name, "AC Sweep");
    assert_eq!(history[0].engine_used, "mock");
}

#[test]
fn run_history_delete_removes_one_run() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    services.simulation_history_service().add_run(&run).unwrap();
    assert_eq!(
        services
            .simulation_history_service()
            .list_runs(&project.id)
            .unwrap()
            .len(),
        1
    );

    services
        .simulation_history_service()
        .delete_run(&project.id, &run.id)
        .unwrap();

    let history = services
        .simulation_history_service()
        .list_runs(&project.id)
        .unwrap();
    assert!(history.is_empty());
}

#[test]
fn run_history_clear_removes_all_runs() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    services.simulation_history_service().add_run(&run).unwrap();
    services
        .simulation_history_service()
        .clear_runs(&project.id)
        .unwrap();

    let history = services
        .simulation_history_service()
        .list_runs(&project.id)
        .unwrap();
    assert!(history.is_empty());
}

#[test]
fn graph_view_contains_series_metadata() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    let view = services
        .simulation_graph_service()
        .build_graph_view(&run)
        .unwrap();
    assert!(!view.series.is_empty());
    assert!(view.series.iter().all(|s| !s.id.is_empty()));
    assert!(view.series.iter().all(|s| !s.label.is_empty()));
    assert!(view.series.iter().all(|s| s.points_count > 0));
}

#[test]
fn export_run_series_csv_has_headers_and_points() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    let csv = services
        .simulation_graph_service()
        .export_run_series_csv(&run)
        .unwrap();
    assert!(csv.starts_with("series_id,series_label,x,y"));
    assert!(csv.lines().count() > 1);
}

#[test]
fn export_run_series_json_is_valid() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    let json_str = services
        .simulation_graph_service()
        .export_run_series_json(&run)
        .unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).expect("valid JSON");
    assert!(json["run_id"].is_string());
    assert!(json["profile"].is_string());
    assert!(json["engine"].is_string());
    assert!(json["series"].is_array());
}

#[test]
fn report_section_contains_diagnostics_summary() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    let section = services
        .simulation_workflow_service()
        .simulation_result_to_report_section(&run)
        .unwrap();
    assert_eq!(section.title, "Simulation Results");
    assert!(!section.blocks.is_empty());
}

#[test]
fn full_workflow_preflight_run_history_graph_export() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = mock_profile();

    // 1. Preflight
    let preflight = services
        .simulation_diagnostics_service()
        .diagnose_simulation_preflight(&project, &profile)
        .unwrap();
    let blocking: Vec<_> = preflight
        .iter()
        .filter(|d| {
            matches!(
                d.severity,
                hotsas_core::SimulationDiagnosticSeverity::Blocking
            )
        })
        .collect();
    assert!(
        blocking.is_empty(),
        "preflight should have no blocking errors"
    );

    // 2. Run
    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();
    assert_eq!(
        run.status,
        hotsas_core::UserCircuitSimulationStatus::Succeeded
    );

    // 3. History
    services.simulation_history_service().add_run(&run).unwrap();
    let history = services
        .simulation_history_service()
        .list_runs(&project.id)
        .unwrap();
    assert_eq!(history.len(), 1);

    // 4. Graph
    let view = services
        .simulation_graph_service()
        .build_graph_view(&run)
        .unwrap();
    assert!(!view.series.is_empty());

    // 5. Export
    let csv = services
        .simulation_graph_service()
        .export_run_series_csv(&run)
        .unwrap();
    assert!(csv.starts_with("series_id"));
    let json = services
        .simulation_graph_service()
        .export_run_series_json(&run)
        .unwrap();
    assert!(json.contains("run_id"));
}

fn component_instance(instance_id: &str, definition_id: &str) -> ComponentInstance {
    ComponentInstance {
        instance_id: instance_id.to_string(),
        definition_id: definition_id.to_string(),
        selected_symbol_id: None,
        selected_footprint_id: None,
        selected_simulation_model_id: None,
        position: hotsas_core::Point::new(0.0, 0.0),
        rotation_degrees: 0.0,
        connected_nets: vec![],
        overridden_parameters: BTreeMap::new(),
        notes: None,
    }
}

fn generic_op_amp_definition() -> ComponentDefinition {
    hotsas_core::built_in_component_library()
        .components
        .into_iter()
        .find(|component| component.id == "generic_op_amp")
        .unwrap()
}

fn resistor_without_required_parameter() -> ComponentDefinition {
    let mut definition = hotsas_core::built_in_component_library()
        .components
        .into_iter()
        .find(|component| component.id == "generic_resistor")
        .unwrap();
    definition.parameters.clear();
    definition
}

fn bad_subckt_definition() -> ComponentDefinition {
    ComponentDefinition {
        id: "bad_subckt".to_string(),
        name: "Bad Subckt".to_string(),
        category: "op_amp".to_string(),
        manufacturer: None,
        part_number: None,
        description: None,
        parameters: BTreeMap::new(),
        ratings: BTreeMap::new(),
        symbol_ids: vec!["op_amp".to_string()],
        footprint_ids: vec![],
        simulation_models: vec![SimulationModel {
            id: "bad_subckt_model".to_string(),
            model_type: "spice".to_string(),
            source_path: Some("bad.lib".to_string()),
            raw_model: Some(".subckt BAD IN OUT VCC VEE".to_string()),
            raw_model_id: Some("BAD".to_string()),
            pin_mapping: BTreeMap::from([("IN".to_string(), "missing_pin".to_string())]),
            kind: SimulationModelKind::Subcircuit,
        }],
        datasheets: vec![],
        tags: vec!["test".to_string()],
        metadata: BTreeMap::new(),
    }
}

fn assert_model_diagnostic(
    diagnostics: &[hotsas_core::SimulationDiagnosticMessage],
    code: &str,
    severity: hotsas_core::SimulationDiagnosticSeverity,
    component_id: &str,
    model_id: Option<&str>,
) {
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.code == code
                && diagnostic
                    .related_entity
                    .as_ref()
                    .map(|entity| entity.id.as_str())
                    == Some(component_id)
        })
        .unwrap_or_else(|| panic!("expected {code} for component {component_id}"));
    assert_eq!(diagnostic.severity, severity);
    assert_eq!(diagnostic.related_model_id.as_deref(), model_id);
}
