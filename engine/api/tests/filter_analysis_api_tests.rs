use hotsas_api::{
    FilterAnalysisMethodDto, FilterAnalysisScopeDto, FilterMetricKindDto,
    FilterNetworkAnalysisRequestDto, FrequencySweepScaleDto, FrequencySweepSettingsDto, HotSasApi,
};
use hotsas_application::AppServices;
use hotsas_core::{
    CircuitProject, EngineeringUnit, ProjectPackageManifest, ProjectPackageValidationReport,
    ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
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
    ) -> Result<hotsas_core::TouchstoneImportReport, hotsas_ports::PortError> {
        Ok(hotsas_core::TouchstoneImportReport {
            status: hotsas_core::ModelImportStatus::Parsed,
            network: None,
            warnings: vec![],
            errors: vec![],
        })
    }
}

fn setup_api() -> HotSasApi {
    HotSasApi::new(fake_services())
}

#[test]
fn suggest_filter_analysis_ports_without_project_returns_state_error() {
    let api = setup_api();
    let result = api.suggest_filter_analysis_ports(vec![]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        hotsas_api::ApiError::State(_) => {}
        _ => panic!("Expected State error, got {:?}", err),
    }
}

#[test]
fn validate_filter_network_analysis_request_without_project_returns_state_error() {
    let api = setup_api();
    let request = FilterNetworkAnalysisRequestDto {
        project_id: "test".to_string(),
        scope: FilterAnalysisScopeDto::WholeCircuit,
        selected_component_ids: vec![],
        input_port: hotsas_api::CircuitAnalysisPortDto {
            label: "IN".to_string(),
            positive_net_id: "n1".to_string(),
            negative_net_id: None,
            reference_node_id: Some("gnd".to_string()),
            nominal_impedance_ohm: Some(50.0),
        },
        output_port: hotsas_api::CircuitAnalysisPortDto {
            label: "OUT".to_string(),
            positive_net_id: "n2".to_string(),
            negative_net_id: None,
            reference_node_id: Some("gnd".to_string()),
            nominal_impedance_ohm: Some(50.0),
        },
        sweep: FrequencySweepSettingsDto {
            start_hz: 1.0,
            stop_hz: 1_000_000.0,
            points: 100,
            points_per_decade: None,
            scale: FrequencySweepScaleDto::Logarithmic,
        },
        method: FilterAnalysisMethodDto::Mock,
        source_amplitude_v: Some(1.0),
        requested_metrics: vec![FilterMetricKindDto::CutoffFrequency],
    };
    let result = api.validate_filter_network_analysis_request(request);
    assert!(result.is_err());
}

#[test]
fn run_filter_network_analysis_mock_returns_result_with_points_and_metrics() {
    let api = setup_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let ports = api.suggest_filter_analysis_ports(vec![]).unwrap();
    assert!(
        ports.len() >= 2,
        "Expected at least 2 ports, got {}",
        ports.len()
    );

    let request = FilterNetworkAnalysisRequestDto {
        project_id: api
            .get_project_session_state()
            .unwrap()
            .current_project_id
            .unwrap(),
        scope: FilterAnalysisScopeDto::WholeCircuit,
        selected_component_ids: vec![],
        input_port: ports[0].clone(),
        output_port: ports[1].clone(),
        sweep: FrequencySweepSettingsDto {
            start_hz: 1.0,
            stop_hz: 1_000_000.0,
            points: 100,
            points_per_decade: None,
            scale: FrequencySweepScaleDto::Logarithmic,
        },
        method: FilterAnalysisMethodDto::Mock,
        source_amplitude_v: Some(1.0),
        requested_metrics: vec![
            FilterMetricKindDto::CutoffFrequency,
            FilterMetricKindDto::PeakGain,
        ],
    };

    let result = api.run_filter_network_analysis(request).unwrap();
    eprintln!(
        "DEBUG: method_used={:?}, detected_kind={:?}, points={}, metrics={:?}",
        result.method_used,
        result.detected_filter_kind,
        result.points.len(),
        result
            .metrics
            .iter()
            .map(|m| (m.kind.clone(), m.value))
            .collect::<Vec<_>>()
    );
    assert!(!result.analysis_id.is_empty());
    assert_eq!(result.method_used, FilterAnalysisMethodDto::Mock);
    assert!(!result.points.is_empty());
    assert!(!result.metrics.is_empty());
    assert!(
        result
            .metrics
            .iter()
            .any(|m| m.kind == FilterMetricKindDto::CutoffFrequency),
        "Expected cutoff_frequency metric, got {:?}",
        result
            .metrics
            .iter()
            .map(|m| m.kind.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn get_last_filter_network_analysis_returns_none_initially() {
    let api = setup_api();
    let result = api.get_last_filter_network_analysis().unwrap();
    assert!(result.is_none());
}

#[test]
fn clear_last_filter_network_analysis_works() {
    let api = setup_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let ports = api.suggest_filter_analysis_ports(vec![]).unwrap();
    let request = FilterNetworkAnalysisRequestDto {
        project_id: api
            .get_project_session_state()
            .unwrap()
            .current_project_id
            .unwrap(),
        scope: FilterAnalysisScopeDto::WholeCircuit,
        selected_component_ids: vec![],
        input_port: ports[0].clone(),
        output_port: ports[1].clone(),
        sweep: FrequencySweepSettingsDto {
            start_hz: 1.0,
            stop_hz: 1_000_000.0,
            points: 50,
            points_per_decade: None,
            scale: FrequencySweepScaleDto::Logarithmic,
        },
        method: FilterAnalysisMethodDto::Mock,
        source_amplitude_v: Some(1.0),
        requested_metrics: vec![FilterMetricKindDto::CutoffFrequency],
    };

    api.run_filter_network_analysis(request).unwrap();
    assert!(api.get_last_filter_network_analysis().unwrap().is_some());

    api.clear_last_filter_network_analysis().unwrap();
    assert!(api.get_last_filter_network_analysis().unwrap().is_none());
}

#[test]
fn export_filter_analysis_csv_without_result_returns_state_error() {
    let api = setup_api();
    let result = api.export_filter_network_analysis_csv();
    assert!(result.is_err());
}

#[test]
fn add_filter_network_analysis_to_advanced_report_without_result_returns_state_error() {
    let api = setup_api();
    let result = api.add_filter_network_analysis_to_advanced_report();
    assert!(result.is_err());
}
