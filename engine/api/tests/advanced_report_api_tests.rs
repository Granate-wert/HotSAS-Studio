use hotsas_api::{AdvancedReportExportRequestDto, AdvancedReportRequestDto, HotSasApi};
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

fn fake_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
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
    ))
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

#[test]
fn list_report_section_capabilities_returns_non_empty_list() {
    let api = fake_api();
    let capabilities = api.list_report_section_capabilities().unwrap();
    assert!(!capabilities.is_empty());
    let kinds: Vec<_> = capabilities.iter().map(|c| c.kind.clone()).collect();
    assert!(kinds.contains(&"ProjectInfo".to_string()));
    assert!(kinds.contains(&"SchematicSummary".to_string()));
}

#[test]
fn generate_advanced_report_without_project_returns_empty_report() {
    let api = fake_api();
    let request = AdvancedReportRequestDto {
        report_id: "test-1".to_string(),
        title: "Test Report".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string(), "SchematicSummary".to_string()],
        export_options: hotsas_api::ReportExportOptionsDto {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    };
    let report = api.generate_advanced_report(request).unwrap();
    assert_eq!(report.id, "test-1");
    assert_eq!(report.title, "Test Report");
    assert_eq!(report.report_type, "ProjectSummary");
    assert!(!report.sections.is_empty());
}

#[test]
fn export_advanced_report_returns_success() {
    let api = fake_api();
    let request = AdvancedReportRequestDto {
        report_id: "test-2".to_string(),
        title: "Export Test".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string()],
        export_options: hotsas_api::ReportExportOptionsDto {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    };
    api.generate_advanced_report(request).unwrap();

    let export_request = AdvancedReportExportRequestDto {
        report_id: "test-2".to_string(),
        format: "markdown".to_string(),
        output_path: None,
    };
    let result = api.export_advanced_report(export_request).unwrap();
    assert!(result.success);
    assert!(!result.content.is_empty());
    assert_eq!(result.format, "markdown");
}

#[test]
fn get_last_advanced_report_returns_none_initially() {
    let api = fake_api();
    let report = api.get_last_advanced_report().unwrap();
    assert!(report.is_none());
}

#[test]
fn get_last_advanced_report_returns_generated_report() {
    let api = fake_api();
    let request = AdvancedReportRequestDto {
        report_id: "test-3".to_string(),
        title: "Last Report Test".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string()],
        export_options: hotsas_api::ReportExportOptionsDto {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    };
    api.generate_advanced_report(request).unwrap();

    let last = api.get_last_advanced_report().unwrap();
    assert!(last.is_some());
    assert_eq!(last.unwrap().id, "test-3");
}

#[test]
fn export_html_format_returns_html_content() {
    let api = fake_api();
    let request = AdvancedReportRequestDto {
        report_id: "test-4".to_string(),
        title: "HTML Export Test".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string()],
        export_options: hotsas_api::ReportExportOptionsDto {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    };
    api.generate_advanced_report(request).unwrap();

    let export_request = AdvancedReportExportRequestDto {
        report_id: "test-4".to_string(),
        format: "html".to_string(),
        output_path: None,
    };
    let result = api.export_advanced_report(export_request).unwrap();
    assert!(result.success);
    assert!(
        result.content.contains("<html>")
            || result.content.contains("<body>")
            || result.content.contains("<h1>")
    );
}

#[test]
fn export_json_format_returns_valid_json_string() {
    let api = fake_api();
    let request = AdvancedReportRequestDto {
        report_id: "test-5".to_string(),
        title: "JSON Export Test".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string()],
        export_options: hotsas_api::ReportExportOptionsDto {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    };
    api.generate_advanced_report(request).unwrap();

    let export_request = AdvancedReportExportRequestDto {
        report_id: "test-5".to_string(),
        format: "json".to_string(),
        output_path: None,
    };
    let result = api.export_advanced_report(export_request).unwrap();
    assert!(result.success);
    // Verify JSON starts with { and contains expected keys
    assert!(result.content.starts_with('{'));
    assert!(result.content.contains("id") || result.content.contains("sections"));
}

#[test]
fn export_csv_summary_format_returns_csv_lines() {
    let api = fake_api();
    let request = AdvancedReportRequestDto {
        report_id: "test-6".to_string(),
        title: "CSV Export Test".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string(), "SchematicSummary".to_string()],
        export_options: hotsas_api::ReportExportOptionsDto {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    };
    api.generate_advanced_report(request).unwrap();

    let export_request = AdvancedReportExportRequestDto {
        report_id: "test-6".to_string(),
        format: "csv_summary".to_string(),
        output_path: None,
    };
    let result = api.export_advanced_report(export_request).unwrap();
    assert!(result.success);
    let lines: Vec<&str> = result.content.lines().collect();
    assert!(lines.len() >= 2);
}
