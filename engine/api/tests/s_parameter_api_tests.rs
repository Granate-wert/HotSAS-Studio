use hotsas_api::{
    AddSParameterAnalysisToReportRequestDto, AnalyzeTouchstoneRequestDto,
    ExportSParameterCsvRequestDto, HotSasApi,
};
use hotsas_application::AppServices;
use hotsas_core::{
    CircuitProject, ComplexValue, EngineeringUnit, ImportedModelSource, ProjectPackageManifest,
    ProjectPackageValidationReport, ReportModel, SParameterPoint, SimulationProfile,
    SimulationResult, TouchstoneFrequencyUnit, TouchstoneImportReport, TouchstoneNetworkData,
    TouchstoneParameterFormat, ValueWithUnit,
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
        _variables: &std::collections::BTreeMap<String, ValueWithUnit>,
    ) -> Result<hotsas_core::FormulaEvaluationResult, PortError> {
        Ok(hotsas_core::FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: "eq-1".to_string(),
            expression: "1/(2*PI*R*C)".to_string(),
            inputs: std::collections::BTreeMap::new(),
            outputs: std::collections::BTreeMap::new(),
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
            source: ImportedModelSource {
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

impl hotsas_ports::TouchstoneParserPort for FakeTouchstoneParser {
    fn parse_touchstone_from_str(
        &self,
        _source_name: Option<String>,
        _content: &str,
    ) -> Result<TouchstoneImportReport, hotsas_ports::PortError> {
        Ok(TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: Some(TouchstoneNetworkData {
                id: "net-1".to_string(),
                name: "test.s2p".to_string(),
                port_count: 2,
                frequency_unit: TouchstoneFrequencyUnit::MHz,
                parameter_format: TouchstoneParameterFormat::RI,
                reference_impedance_ohm: 50.0,
                points: vec![
                    SParameterPoint {
                        frequency_hz: 1e6,
                        values: vec![
                            ComplexValue { re: 0.5, im: 0.0 },
                            ComplexValue { re: 0.9, im: 0.1 },
                            ComplexValue { re: 0.9, im: 0.1 },
                            ComplexValue { re: 0.4, im: 0.0 },
                        ],
                    },
                    SParameterPoint {
                        frequency_hz: 10e6,
                        values: vec![
                            ComplexValue { re: 0.3, im: 0.1 },
                            ComplexValue { re: 0.8, im: 0.2 },
                            ComplexValue { re: 0.8, im: 0.2 },
                            ComplexValue { re: 0.3, im: 0.1 },
                        ],
                    },
                ],
                source: ImportedModelSource {
                    file_name: Some("test.s2p".to_string()),
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
        Arc::new(FakeComponentLibraryStorage),
        Arc::new(FakeBomExporter),
        Arc::new(FakeSimulationDataExporter),
        Arc::new(FakeComponentLibraryExporter),
        Arc::new(FakeSchematicExporter),
        Arc::new(FakeSpiceParser),
        Arc::new(FakeTouchstoneParser),
    )
}

fn setup_api() -> HotSasApi {
    HotSasApi::new(fake_services())
}

#[test]
fn analyze_touchstone_s_parameters_returns_result() {
    let api = setup_api();
    let request = AnalyzeTouchstoneRequestDto {
        source_name: Some("test.s2p".to_string()),
        content: "# dummy content".to_string(),
    };
    let result = api
        .analyze_touchstone_s_parameters(request)
        .expect("should analyze");
    assert_eq!(result.dataset.port_count, 2);
    assert!(!result.curve_points.is_empty());
    assert!(result.can_plot_s11);
    assert!(result.can_plot_s21);
}

#[test]
fn get_last_s_parameter_analysis_after_analyze() {
    let api = setup_api();
    let request = AnalyzeTouchstoneRequestDto {
        source_name: Some("test.s2p".to_string()),
        content: "# dummy".to_string(),
    };
    api.analyze_touchstone_s_parameters(request)
        .expect("should analyze");
    let last = api
        .get_last_s_parameter_analysis()
        .expect("should have last result")
        .expect("should have result");
    assert_eq!(last.dataset.port_count, 2);
}

#[test]
fn clear_last_s_parameter_analysis_works() {
    let api = setup_api();
    let request = AnalyzeTouchstoneRequestDto {
        source_name: Some("test.s2p".to_string()),
        content: "# dummy".to_string(),
    };
    api.analyze_touchstone_s_parameters(request)
        .expect("should analyze");
    assert!(api.get_last_s_parameter_analysis().unwrap().is_some());
    api.clear_last_s_parameter_analysis();
    assert!(api.get_last_s_parameter_analysis().unwrap().is_none());
}

#[test]
fn export_s_parameter_csv_uses_last_result() {
    let api = setup_api();
    let request = AnalyzeTouchstoneRequestDto {
        source_name: Some("test.s2p".to_string()),
        content: "# dummy".to_string(),
    };
    api.analyze_touchstone_s_parameters(request)
        .expect("should analyze");
    let csv = api.export_s_parameter_csv().expect("should export csv");
    assert!(csv.contains("frequency_hz"));
    assert!(csv.contains("s11_db"));
}

#[test]
fn add_s_parameter_analysis_to_report_adds_section() {
    let api = setup_api();
    let request = AnalyzeTouchstoneRequestDto {
        source_name: Some("test.s2p".to_string()),
        content: "# dummy".to_string(),
    };
    api.analyze_touchstone_s_parameters(request)
        .expect("should analyze");
    let result = api
        .add_s_parameter_analysis_to_advanced_report()
        .expect("should add to report");
    assert!(
        !result.sections.is_empty(),
        "report should have sections after adding s-parameter analysis"
    );
}
