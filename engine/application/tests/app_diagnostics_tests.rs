use hotsas_application::{AppDiagnosticsService, AppServices};
use hotsas_core::{
    CircuitProject, EngineeringUnit, ModuleStatus, ProjectPackageManifest,
    ProjectPackageValidationReport, ReadinessStatus, ReportModel, SimulationProfile,
    SimulationResult, ValueWithUnit,
};
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
        Arc::new(FakeSpiceParser),
        Arc::new(FakeTouchstoneParser),
    )
}

struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(&self, _path: &Path, _project: &CircuitProject) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
        Ok(hotsas_core::rc_low_pass_project())
    }
}

struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        Ok(ValueWithUnit::new_si(159.154943, EngineeringUnit::Hertz))
    }

    fn evaluate_formula(
        &self,
        formula: &hotsas_core::FormulaDefinition,
        variables: &std::collections::BTreeMap<String, ValueWithUnit>,
    ) -> Result<hotsas_core::FormulaEvaluationResult, PortError> {
        Ok(hotsas_core::FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: "eq-1".to_string(),
            expression: "1/(2*PI*R*C)".to_string(),
            inputs: variables.clone(),
            outputs: {
                let mut m = std::collections::BTreeMap::new();
                m.insert(
                    "fc".to_string(),
                    ValueWithUnit::new_si(159.154943, EngineeringUnit::Hertz),
                );
                m
            },
            warnings: vec![],
        })
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("V1\nR1\nC1\n.ac".to_string())
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
        Ok(SimulationResult {
            id: "sim-1".to_string(),
            profile_id: "ac-sweep".to_string(),
            status: hotsas_core::SimulationStatus::Completed,
            engine: "fake".to_string(),
            graph_series: vec![],
            measurements: std::collections::BTreeMap::new(),
            warnings: vec![],
            errors: vec![],
            raw_data_path: None,
            metadata: std::collections::BTreeMap::new(),
        })
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        Ok("# Report".to_string())
    }
    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        Ok("<pre># Report</pre>".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("Designator,Quantity,Value,Unit,Description\n".to_string())
    }
    fn export_bom_json(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("[]".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(&self, _simulation: &SimulationResult) -> Result<String, PortError> {
        Ok("frequency,gain_db\n".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        Ok("{}".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("<svg></svg>".to_string())
    }
}

struct FakeSpiceParser;

impl hotsas_ports::SpiceModelParserPort for FakeSpiceParser {
    fn parse_spice_models_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::SpiceImportReport, hotsas_ports::PortError> {
        Ok(hotsas_core::SpiceImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            source: hotsas_core::ImportedModelSource {
                file_name: None,
                file_path: None,
                source_format: "spice".to_string(),
                content_hash: None,
            },
            models: vec![hotsas_core::SpiceModelDefinition {
                id: "NPN".to_string(),
                name: "NPN".to_string(),
                kind: hotsas_core::SpiceModelKind::BjtNpn,
                source: hotsas_core::ImportedModelSource {
                    file_name: None,
                    file_path: None,
                    source_format: "spice".to_string(),
                    content_hash: None,
                },
                raw_line: ".model NPN NPN".to_string(),
                parameters: vec![],
                warnings: vec![],
            }],
            subcircuits: vec![],
            warnings: vec![],
            errors: vec![],
        })
    }
}

struct FakeTouchstoneParser;

impl hotsas_ports::TouchstoneParserPort for FakeTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::TouchstoneImportReport, hotsas_ports::PortError> {
        Ok(hotsas_core::TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: Some(hotsas_core::TouchstoneNetworkData {
                id: "s2p-1".to_string(),
                name: "S2P".to_string(),
                port_count: 2,
                frequency_unit: hotsas_core::TouchstoneFrequencyUnit::Hz,
                parameter_format: hotsas_core::TouchstoneParameterFormat::RI,
                reference_impedance_ohm: 50.0,
                points: vec![hotsas_core::SParameterPoint {
                    frequency_hz: 1e9,
                    values: vec![hotsas_core::ComplexValue { re: 0.9, im: 0.1 }],
                }],
                source: hotsas_core::ImportedModelSource {
                    file_name: None,
                    file_path: None,
                    source_format: "touchstone".to_string(),
                    content_hash: None,
                },
                warnings: vec![],
            }),
            warnings: vec![],
            errors: vec![],
        })
    }
}

#[test]
fn diagnostics_report_contains_expected_module_ids() {
    let services = fake_services();
    let service = AppDiagnosticsService::new();
    let report = service.get_app_diagnostics(&services);

    let ids: Vec<&str> = report.modules.iter().map(|m| m.id.as_str()).collect();
    assert!(ids.contains(&"formula_registry"));
    assert!(ids.contains(&"component_library"));
    assert!(ids.contains(&"export_center"));
    assert!(ids.contains(&"simulation"));
    assert!(ids.contains(&"import_models"));
    assert!(ids.contains(&"project_package"));
    assert!(ids.contains(&"schematic_editor"));
    assert!(ids.contains(&"engineering_notebook"));
    assert!(ids.contains(&"selected_region"));
}

#[test]
fn component_library_module_reports_ready_or_limited_not_panic() {
    let services = fake_services();
    let service = AppDiagnosticsService::new();
    let report = service.get_app_diagnostics(&services);

    let module = report
        .modules
        .iter()
        .find(|m| m.id == "component_library")
        .expect("component_library module must exist");

    assert!(
        module.status == ModuleStatus::Ready || module.status == ModuleStatus::Limited,
        "component library must report ready or limited, got {:?}",
        module.status
    );
}

#[test]
fn export_center_module_reports_nine_capabilities() {
    let services = fake_services();
    let service = AppDiagnosticsService::new();
    let report = service.get_app_diagnostics(&services);

    let module = report
        .modules
        .iter()
        .find(|m| m.id == "export_center")
        .expect("export_center module must exist");

    assert_eq!(module.status, ModuleStatus::Ready);
    assert!(
        module.summary.contains("9"),
        "export center must report 9 capabilities"
    );
}

#[test]
fn simulation_module_handles_ngspice_unavailable_as_limited_not_error() {
    let services = fake_services();
    let service = AppDiagnosticsService::new();
    let report = service.get_app_diagnostics(&services);

    let module = report
        .modules
        .iter()
        .find(|m| m.id == "simulation")
        .expect("simulation module must exist");

    assert!(
        module.status == ModuleStatus::Ready || module.status == ModuleStatus::Limited,
        "simulation must report ready or limited (mock available), got {:?}",
        module.status
    );
}

#[test]
fn import_models_module_reports_spice_and_touchstone_support() {
    let services = fake_services();
    let service = AppDiagnosticsService::new();
    let report = service.get_app_diagnostics(&services);

    let module = report
        .modules
        .iter()
        .find(|m| m.id == "import_models")
        .expect("import_models module must exist");

    assert_eq!(module.status, ModuleStatus::Ready);
    assert!(
        module.summary.to_lowercase().contains("spice"),
        "import models must mention SPICE"
    );
    assert!(
        module.summary.to_lowercase().contains("touchstone"),
        "import models must mention Touchstone"
    );
}

#[test]
fn readiness_self_check_returns_checks_with_expected_statuses() {
    let services = fake_services();
    let service = AppDiagnosticsService::new();
    let report = service.run_readiness_self_check(&services);

    assert!(!report.checks.is_empty(), "self-check must produce checks");

    let formula_check = report
        .checks
        .iter()
        .find(|c| c.id == "formula_calculation")
        .expect("formula_calculation check must exist");
    assert_eq!(formula_check.status, ReadinessStatus::Pass);

    let export_check = report
        .checks
        .iter()
        .find(|c| c.id == "export_capabilities")
        .expect("export_capabilities check must exist");
    assert_eq!(export_check.status, ReadinessStatus::Pass);

    let mock_sim_check = report
        .checks
        .iter()
        .find(|c| c.id == "mock_simulation")
        .expect("mock_simulation check must exist");
    assert_eq!(mock_sim_check.status, ReadinessStatus::Pass);

    let spice_check = report
        .checks
        .iter()
        .find(|c| c.id == "spice_parser")
        .expect("spice_parser check must exist");
    assert_eq!(spice_check.status, ReadinessStatus::Pass);

    let touchstone_check = report
        .checks
        .iter()
        .find(|c| c.id == "touchstone_parser")
        .expect("touchstone_parser check must exist");
    assert_eq!(touchstone_check.status, ReadinessStatus::Pass);
}
