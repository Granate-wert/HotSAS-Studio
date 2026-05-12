use hotsas_application::AppServices;
use hotsas_core::{
    CircuitProject, ComponentInstance, EngineeringUnit, OperatingPointSettings,
    ProjectPackageManifest, ProjectPackageValidationReport, SimulationProbe, SimulationProbeKind,
    SimulationProbeTarget, SimulationStatus, TransientSettings, UserCircuitAnalysisType,
    UserCircuitSimulationEngine, UserCircuitSimulationProfile, ValueWithUnit,
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
    // Add a ground reference component so ground check passes without warning
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

// ─── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn list_default_profiles_returns_three_profiles() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profiles = services
        .simulation_workflow_service()
        .list_default_simulation_profiles(&project)
        .unwrap();
    assert_eq!(profiles.len(), 4);
    assert!(profiles.iter().any(|p| p.id == "mock-op"));
    assert!(profiles.iter().any(|p| p.id == "mock-ac"));
    assert!(profiles.iter().any(|p| p.id == "mock-transient"));
    assert!(profiles.iter().any(|p| p.id == "auto-ac"));
}

#[test]
fn suggest_probes_returns_node_voltage_probes() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let probes = services
        .simulation_workflow_service()
        .suggest_simulation_probes(&project)
        .unwrap();
    assert!(!probes.is_empty());
    assert!(probes
        .iter()
        .all(|p| matches!(p.kind, SimulationProbeKind::NodeVoltage)));
    assert!(probes.iter().any(|p| p.label.contains("Vin")));
    assert!(probes.iter().any(|p| p.label.contains("Vout")));
}

#[test]
fn validate_circuit_with_valid_project_returns_can_run() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "test-profile".to_string(),
        name: "Test".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Mock,
        probes: vec![],
        ac: Some(hotsas_core::AcSweepSettings {
            start_hz: 10.0,
            stop_hz: 1_000_000.0,
            points_per_decade: 100,
        }),
        transient: None,
        op: None,
    };
    let result = services
        .simulation_workflow_service()
        .validate_circuit_for_simulation(&project, &profile)
        .unwrap();
    assert!(
        result.can_run,
        "Expected can_run=true, got errors: {:?}",
        result.blocking_errors
    );
    assert!(result.generated_netlist_preview.is_some());
    assert!(result.blocking_errors.is_empty());
}

#[test]
fn validate_circuit_without_components_fails() {
    let services = build_services();
    let mut project = rc_low_pass_project_with_ground();
    project.schematic.components.clear();
    let profile = UserCircuitSimulationProfile {
        id: "test-profile".to_string(),
        name: "Test".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Mock,
        probes: vec![],
        ac: None,
        transient: None,
        op: None,
    };
    let result = services
        .simulation_workflow_service()
        .validate_circuit_for_simulation(&project, &profile)
        .unwrap();
    assert!(!result.can_run);
    assert!(result
        .blocking_errors
        .iter()
        .any(|e| e.code == "NO_COMPONENTS"));
}

#[test]
fn validate_circuit_invalid_probe_net_fails() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "test-profile".to_string(),
        name: "Test".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Mock,
        probes: vec![SimulationProbe {
            id: "bad-probe".to_string(),
            label: "Bad".to_string(),
            kind: SimulationProbeKind::NodeVoltage,
            target: SimulationProbeTarget::Net {
                net_id: "nonexistent-net".to_string(),
            },
            unit: Some(EngineeringUnit::Volt),
        }],
        ac: None,
        transient: None,
        op: None,
    };
    let result = services
        .simulation_workflow_service()
        .validate_circuit_for_simulation(&project, &profile)
        .unwrap();
    assert!(!result.can_run);
    assert!(result
        .blocking_errors
        .iter()
        .any(|e| e.code == "INVALID_PROBE_NET"));
}

#[test]
fn run_user_circuit_simulation_mock_ac_succeeds() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "mock-ac".to_string(),
        name: "AC Sweep".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Mock,
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
    assert_eq!(
        run.status,
        hotsas_core::UserCircuitSimulationStatus::Succeeded
    );
    assert_eq!(run.engine_used, "mock");
    assert!(!run.generated_netlist.is_empty());
    assert!(run.result.is_some());
    let result = run.result.unwrap();
    assert!(!result.summary.is_empty());
}

#[test]
fn run_user_circuit_simulation_mock_op_succeeds() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "mock-op".to_string(),
        name: "Operating Point".to_string(),
        analysis_type: UserCircuitAnalysisType::OperatingPoint,
        engine: UserCircuitSimulationEngine::Mock,
        probes: vec![],
        ac: None,
        transient: None,
        op: Some(OperatingPointSettings {
            include_node_voltages: true,
            include_branch_currents: true,
        }),
    };
    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();
    assert_eq!(
        run.status,
        hotsas_core::UserCircuitSimulationStatus::Succeeded
    );
    assert_eq!(run.engine_used, "mock");
    assert!(run.result.is_some());
}

#[test]
fn run_user_circuit_simulation_mock_transient_succeeds() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "mock-transient".to_string(),
        name: "Transient".to_string(),
        analysis_type: UserCircuitAnalysisType::Transient,
        engine: UserCircuitSimulationEngine::Mock,
        probes: vec![],
        ac: None,
        transient: Some(TransientSettings {
            step_seconds: 1e-6,
            stop_seconds: 1e-3,
        }),
        op: None,
    };
    let run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();
    assert_eq!(
        run.status,
        hotsas_core::UserCircuitSimulationStatus::Succeeded
    );
    assert_eq!(run.engine_used, "mock");
    assert!(run.result.is_some());
}

#[test]
fn run_user_circuit_simulation_auto_fallback_to_mock() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "auto-ac".to_string(),
        name: "Auto AC".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Auto,
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
    assert_eq!(
        run.status,
        hotsas_core::UserCircuitSimulationStatus::Succeeded
    );
    // Auto mode with unavailable ngspice falls back to mock
    assert_eq!(run.engine_used, "mock");
    assert!(run
        .warnings
        .iter()
        .any(|w| w.message.contains("ngspice unavailable")));
}

#[test]
fn get_and_clear_last_simulation() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "mock-ac".to_string(),
        name: "AC".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Mock,
        probes: vec![],
        ac: Some(hotsas_core::AcSweepSettings {
            start_hz: 10.0,
            stop_hz: 1_000_000.0,
            points_per_decade: 100,
        }),
        transient: None,
        op: None,
    };

    // Before run: no last simulation
    let before = services
        .simulation_workflow_service()
        .get_last_user_circuit_simulation(&project.id);
    assert!(before.is_none());

    // Run simulation
    let _run = services
        .simulation_workflow_service()
        .run_user_circuit_simulation(&project, profile)
        .unwrap();

    // After run: last simulation exists
    let after = services
        .simulation_workflow_service()
        .get_last_user_circuit_simulation(&project.id);
    assert!(after.is_some());

    // Clear it
    services
        .simulation_workflow_service()
        .clear_last_user_circuit_simulation(&project.id)
        .unwrap();

    let cleared = services
        .simulation_workflow_service()
        .get_last_user_circuit_simulation(&project.id);
    assert!(cleared.is_none());
}

#[test]
fn simulation_result_to_report_section_builds_section() {
    let services = build_services();
    let project = rc_low_pass_project_with_ground();
    let profile = UserCircuitSimulationProfile {
        id: "mock-ac".to_string(),
        name: "AC".to_string(),
        analysis_type: UserCircuitAnalysisType::AcSweep,
        engine: UserCircuitSimulationEngine::Mock,
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
    let section = services
        .simulation_workflow_service()
        .simulation_result_to_report_section(&run)
        .unwrap();
    assert_eq!(section.title, "Simulation Results");
    assert!(!section.blocks.is_empty());
}
