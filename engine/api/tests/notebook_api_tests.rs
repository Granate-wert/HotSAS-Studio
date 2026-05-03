use hotsas_api::HotSasApi;
use hotsas_application::AppServices;
use hotsas_core::CircuitProject;
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, FormulaEnginePort, NetlistExporterPort,
    PortError, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, StoragePort,
};
use std::collections::BTreeMap;
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

fn fake_services() -> AppServices {
    AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
        Arc::new(FakeComponentLibraryStorage),
        Arc::new(FakeBomExporter),
        Arc::new(FakeSimulationDataExporter),
        Arc::new(FakeComponentLibraryExporter),
        Arc::new(FakeSchematicExporter),
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

#[derive(Debug, Default)]
struct FakeProjectPackageStorage;

impl ProjectPackageStoragePort for FakeProjectPackageStorage {
    fn save_project_package(
        &self,
        _package_dir: &Path,
        _project: &CircuitProject,
    ) -> Result<hotsas_core::ProjectPackageManifest, PortError> {
        Ok(hotsas_core::ProjectPackageManifest {
            format_version: "1.0".to_string(),
            engine_version: "0.1.4".to_string(),
            project_id: "test".to_string(),
            project_name: "test".to_string(),
            project_type: hotsas_core::ProjectPackageType::CircuitProject,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
            files: hotsas_core::ProjectPackageFiles::default(),
        })
    }
    fn load_project_package(&self, _package_dir: &Path) -> Result<CircuitProject, PortError> {
        Ok(hotsas_core::rc_low_pass_project())
    }
    fn validate_project_package(
        &self,
        _package_dir: &Path,
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

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &hotsas_core::ValueWithUnit,
        _capacitance: &hotsas_core::ValueWithUnit,
    ) -> Result<hotsas_core::ValueWithUnit, PortError> {
        Ok(hotsas_core::ValueWithUnit::parse_with_default(
            "159.15Hz",
            hotsas_core::EngineeringUnit::Hertz,
        )
        .unwrap())
    }

    fn evaluate_formula(
        &self,
        formula: &hotsas_core::FormulaDefinition,
        variables: &BTreeMap<String, hotsas_core::ValueWithUnit>,
    ) -> Result<hotsas_core::FormulaEvaluationResult, PortError> {
        Ok(hotsas_core::FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: formula.equations[0].id.clone(),
            expression: formula.equations[0].expression.clone(),
            inputs: variables.clone(),
            outputs: BTreeMap::from([(
                "fc".to_string(),
                hotsas_core::ValueWithUnit::new_si(159.154943, hotsas_core::EngineeringUnit::Hertz),
            )]),
            warnings: vec![],
        })
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        Ok("".to_string())
    }
}

struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn run_ac_sweep(
        &self,
        _project: &CircuitProject,
        _profile: &hotsas_core::SimulationProfile,
    ) -> Result<hotsas_core::SimulationResult, PortError> {
        Ok(hotsas_core::SimulationResult {
            id: "sim-1".to_string(),
            profile_id: "profile-1".to_string(),
            status: hotsas_core::SimulationStatus::Completed,
            graph_series: vec![],
            measurements: BTreeMap::new(),
            warnings: vec![],
            errors: vec![],
            raw_data_path: None,
        })
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Ok("".to_string())
    }
    fn export_html(&self, _report: &hotsas_core::ReportModel) -> Result<String, PortError> {
        Ok("".to_string())
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

fn api() -> HotSasApi {
    HotSasApi::new(fake_services())
}

#[test]
fn evaluate_notebook_input_assignment_returns_variable() {
    let api = api();
    let result = api
        .evaluate_notebook_input(hotsas_api::NotebookEvaluationRequestDto {
            input: "R = 10k".to_string(),
        })
        .unwrap();
    assert_eq!(result.status, "success");
    assert!(!result.variables.is_empty());
}

#[test]
fn evaluate_notebook_input_formula_returns_output() {
    let api = api();
    let result = api
        .evaluate_notebook_input(hotsas_api::NotebookEvaluationRequestDto {
            input: "rc_low_pass_cutoff(R=10k, C=100n)".to_string(),
        })
        .unwrap();
    assert_eq!(result.status, "success");
    assert!(!result.outputs.is_empty());
}

#[test]
fn get_notebook_state_returns_variables_and_history() {
    let api = api();
    api.evaluate_notebook_input(hotsas_api::NotebookEvaluationRequestDto {
        input: "R = 10k".to_string(),
    })
    .unwrap();
    let state = api.get_notebook_state().unwrap();
    assert!(!state.variables.is_empty());
    assert!(!state.history.is_empty());
}

#[test]
fn clear_notebook_clears_state() {
    let api = api();
    api.evaluate_notebook_input(hotsas_api::NotebookEvaluationRequestDto {
        input: "R = 10k".to_string(),
    })
    .unwrap();
    let state = api.clear_notebook().unwrap();
    assert!(state.variables.is_empty());
    assert!(state.history.is_empty());
}

#[test]
fn apply_notebook_output_to_component_without_project_returns_state_error() {
    let api = api();
    api.evaluate_notebook_input(hotsas_api::NotebookEvaluationRequestDto {
        input: "R = 10k".to_string(),
    })
    .unwrap();
    let result = api.apply_notebook_output_to_component(hotsas_api::ApplyNotebookValueRequestDto {
        instance_id: "R1".to_string(),
        parameter_name: "resistance".to_string(),
        output_name: "R".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn unsupported_input_returns_controlled_unsupported_result() {
    let api = api();
    let result = api
        .evaluate_notebook_input(hotsas_api::NotebookEvaluationRequestDto {
            input: "sin(5)".to_string(),
        })
        .unwrap();
    assert_eq!(result.status, "unsupported");
}
