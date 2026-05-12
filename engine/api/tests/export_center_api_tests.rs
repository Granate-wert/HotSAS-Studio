use hotsas_api::{ExportRequestDto, HotSasApi};
use hotsas_application::AppServices;
use hotsas_core::{
    CircuitProject, ProjectPackageManifest, ProjectPackageValidationReport, ReportModel,
    SimulationProfile, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, FormulaEnginePort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, StoragePort,
};
use std::path::Path;
use std::sync::Arc;

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

#[test]
fn list_export_capabilities_returns_all_nine_formats() {
    let api = fake_api();
    let capabilities = api.list_export_capabilities().unwrap();
    assert_eq!(capabilities.len(), 9);
    let ids: Vec<_> = capabilities.iter().map(|c| c.format.clone()).collect();
    assert!(ids.contains(&"markdown_report".to_string()));
    assert!(ids.contains(&"html_report".to_string()));
    assert!(ids.contains(&"spice_netlist".to_string()));
    assert!(ids.contains(&"csv_simulation_data".to_string()));
    assert!(ids.contains(&"bom_csv".to_string()));
    assert!(ids.contains(&"bom_json".to_string()));
    assert!(ids.contains(&"component_library_json".to_string()));
    assert!(ids.contains(&"svg_schematic".to_string()));
    assert!(ids.contains(&"altium_workflow_package".to_string()));
}

#[test]
fn export_without_project_returns_state_error() {
    let api = fake_api();
    let request = ExportRequestDto {
        format: "spice_netlist".to_string(),
        write_to_file: false,
        output_dir: None,
    };
    let result = api.export(request);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("create or open a project first"),
        "expected state error, got: {err}"
    );
}

#[test]
fn export_spice_netlist_with_project_returns_success() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let request = ExportRequestDto {
        format: "spice_netlist".to_string(),
        write_to_file: false,
        output_dir: None,
    };
    let result = api.export(request).unwrap();
    assert!(result.success);
    assert_eq!(result.format, "spice_netlist");
    assert!(result.content.contains("V1"));
}

#[test]
fn export_bom_csv_contains_expected_headers() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let request = ExportRequestDto {
        format: "bom_csv".to_string(),
        write_to_file: false,
        output_dir: None,
    };
    let result = api.export(request).unwrap();
    assert!(result.success);
    assert!(result
        .content
        .starts_with("Designator,Quantity,Value,Unit,Description"));
}

#[test]
fn export_svg_schematic_contains_svg_tag() {
    let api = fake_api();
    api.create_rc_low_pass_demo_project().unwrap();
    let request = ExportRequestDto {
        format: "svg_schematic".to_string(),
        write_to_file: false,
        output_dir: None,
    };
    let result = api.export(request).unwrap();
    assert!(result.success);
    assert!(result.content.contains("<svg"));
    assert!(result.content.contains("</svg>"));
}

#[test]
fn export_history_returns_empty_list_initially() {
    let api = fake_api();
    let history = api.export_history().unwrap();
    assert!(history.is_empty());
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
        _path: &Path,
    ) -> Result<hotsas_core::ComponentLibrary, hotsas_ports::PortError> {
        Err(PortError::Storage("not implemented".to_string()))
    }
    fn save_library_to_path(
        &self,
        _path: &Path,
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
        Ok(ValueWithUnit::new_si(
            159.154943,
            hotsas_core::EngineeringUnit::Hertz,
        ))
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok(
            "V1 net_in 0 AC 1\nR1 net_in net_out 10k\nC1 net_out 0 100n\n.ac dec 100 10 1e6\n.end"
                .to_string(),
        )
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
            id: "mock".to_string(),
            profile_id: "default".to_string(),
            status: hotsas_core::SimulationStatus::Completed,
            engine: "mock".to_string(),
            graph_series: vec![hotsas_core::GraphSeries {
                name: "Gain".to_string(),
                x_unit: hotsas_core::EngineeringUnit::Hertz,
                y_unit: hotsas_core::EngineeringUnit::Unitless,
                points: vec![hotsas_core::GraphPoint { x: 10.0, y: 0.0 }],
                metadata: std::collections::BTreeMap::new(),
            }],
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
        Ok("Designator,Quantity,Value,Unit,Description\nR1,1,10k,Ohm,Resistor\n".to_string())
    }
    fn export_bom_json(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("[{\"designator\":\"R1\"}]".to_string())
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(&self, _simulation: &SimulationResult) -> Result<String, PortError> {
        Ok("frequency,gain_db\n10,0\n".to_string())
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
