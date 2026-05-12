use hotsas_application::{AppServices, ProductWorkflowService};
use hotsas_core::{
    CircuitProject, EngineeringUnit, ProjectPackageManifest, ProjectPackageValidationReport,
    ReportModel, SimulationProfile, SimulationResult, ValueWithUnit, WorkflowStatusKind,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ComponentLibraryPort, FormulaEnginePort,
    NetlistExporterPort, PortError, ProjectPackageStoragePort, ReportExporterPort,
    SchematicExporterPort, SimulationDataExporterPort, SimulationEnginePort, SpiceModelParserPort,
    StoragePort, TouchstoneParserPort,
};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Default)]
struct FakeComponentLibraryStorage;

impl ComponentLibraryPort for FakeComponentLibraryStorage {
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

struct FakeStorage;

impl StoragePort for FakeStorage {
    fn save_project(&self, _path: &Path, _project: &CircuitProject) -> Result<(), PortError> {
        Ok(())
    }
    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
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
            id: "fake".to_string(),
            profile_id: "ac".to_string(),
            status: hotsas_core::SimulationStatus::Completed,
            engine: "fake".to_string(),
            graph_series: vec![],
            warnings: vec![],
            errors: vec![],
            measurements: std::collections::BTreeMap::new(),
            metadata: Default::default(),
            raw_data_path: None,
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
                raw_line: ".model NPN NPN(IS=1e-15 BF=100)".to_string(),
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

impl TouchstoneParserPort for FakeTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<hotsas_core::TouchstoneImportReport, PortError> {
        Ok(hotsas_core::TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: Some(hotsas_core::TouchstoneNetworkData {
                id: "smoke".to_string(),
                name: "smoke".to_string(),
                port_count: 2,
                frequency_unit: hotsas_core::TouchstoneFrequencyUnit::Hz,
                parameter_format: hotsas_core::TouchstoneParameterFormat::RI,
                reference_impedance_ohm: 50.0,
                points: vec![hotsas_core::SParameterPoint {
                    frequency_hz: 1.0e9,
                    values: vec![
                        hotsas_core::ComplexValue { re: 0.9, im: 0.1 },
                        hotsas_core::ComplexValue { re: 0.1, im: 0.9 },
                    ],
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

fn fake_services() -> AppServices {
    AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
        Arc::new(FakeComponentLibraryStorage::default()),
        Arc::new(FakeBomExporter::default()),
        Arc::new(FakeSimulationDataExporter::default()),
        Arc::new(FakeComponentLibraryExporter::default()),
        Arc::new(FakeSchematicExporter::default()),
        Arc::new(FakeSpiceParser),
        Arc::new(FakeTouchstoneParser),
    )
}

#[test]
fn workflow_status_contains_expected_steps() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.get_product_workflow_status(&services, None);

    let step_ids: Vec<String> = status.workflow_steps.iter().map(|s| s.id.clone()).collect();
    assert!(step_ids.contains(&"project".to_string()));
    assert!(step_ids.contains(&"schematic".to_string()));
    assert!(step_ids.contains(&"formula_library".to_string()));
    assert!(step_ids.contains(&"engineering_notebook".to_string()));
    assert!(step_ids.contains(&"component_library".to_string()));
    assert!(step_ids.contains(&"model_import".to_string()));
    assert!(step_ids.contains(&"simulation".to_string()));
    assert!(step_ids.contains(&"selected_region".to_string()));
    assert!(step_ids.contains(&"export_center".to_string()));
    assert!(step_ids.contains(&"diagnostics".to_string()));
}

#[test]
fn workflow_status_marks_formula_library_ready() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.get_product_workflow_status(&services, None);

    let formula_step = status
        .workflow_steps
        .iter()
        .find(|s| s.id == "formula_library")
        .expect("formula_library step exists");
    assert_eq!(formula_step.status, WorkflowStatusKind::Ready);
}

#[test]
fn workflow_status_marks_component_library_ready_or_limited() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.get_product_workflow_status(&services, None);

    let comp_module = status
        .module_statuses
        .iter()
        .find(|m| m.id == "component_library")
        .expect("component_library module exists");
    assert!(
        comp_module.status == WorkflowStatusKind::Ready
            || comp_module.status == WorkflowStatusKind::Limited,
        "component_library should be Ready or Limited, got {:?}",
        comp_module.status
    );
}

#[test]
fn workflow_status_reports_ngspice_as_controlled_status() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.get_product_workflow_status(&services, None);

    let sim_module = status
        .module_statuses
        .iter()
        .find(|m| m.id == "simulation")
        .expect("simulation module exists");
    assert!(
        sim_module.status == WorkflowStatusKind::Ready
            || sim_module.status == WorkflowStatusKind::Limited,
        "simulation should be Ready or Limited, got {:?}",
        sim_module.status
    );
}

#[test]
fn self_check_creates_rc_demo_without_panic() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.run_product_beta_self_check(&services);

    assert!(status.current_project.is_some());
    assert_eq!(
        status.current_project.as_ref().unwrap().project_name,
        "RC Low-Pass Demo"
    );
}

#[test]
fn self_check_calculates_rc_cutoff() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.run_product_beta_self_check(&services);

    let formula_module = status
        .module_statuses
        .iter()
        .find(|m| m.id == "formula_calculation")
        .expect("formula_calculation module exists");
    assert_eq!(formula_module.status, WorkflowStatusKind::Ready);
}

#[test]
fn self_check_lists_export_capabilities() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.run_product_beta_self_check(&services);

    let export_module = status
        .module_statuses
        .iter()
        .find(|m| m.id == "export_center")
        .expect("export_center module exists");
    assert_eq!(export_module.status, WorkflowStatusKind::Ready);
}

#[test]
fn self_check_reports_import_models_status() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.run_product_beta_self_check(&services);

    let spice_module = status
        .module_statuses
        .iter()
        .find(|m| m.id == "spice_parser")
        .expect("spice_parser module exists");
    assert_eq!(spice_module.status, WorkflowStatusKind::Ready);

    let touchstone_module = status
        .module_statuses
        .iter()
        .find(|m| m.id == "touchstone_parser")
        .expect("touchstone_parser module exists");
    assert_eq!(touchstone_module.status, WorkflowStatusKind::Ready);
}

#[test]
fn self_check_collects_warnings_without_failure() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let status = workflow.run_product_beta_self_check(&services);

    assert!(status.blockers.is_empty());
}

#[test]
fn create_integrated_demo_project_returns_project() {
    let services = fake_services();
    let workflow = ProductWorkflowService::new();
    let project = workflow.create_integrated_demo_project(&services);

    assert_eq!(project.name, "RC Low-Pass Demo");
    assert!(!project.schematic.components.is_empty());
}
