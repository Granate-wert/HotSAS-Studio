use hotsas_api::{FormulaPackDto, FormulaSummaryDto, HotSasApi};
use hotsas_application::AppServices;
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, FormulaPack, ReportModel, SimulationProfile,
    SimulationResult, ValueWithUnit,
};
use hotsas_core::{ProjectPackageManifest, ProjectPackageValidationReport};
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

#[test]
fn api_loads_formula_pack_metadata_and_lists_formulas() {
    let api = HotSasApi::new(fake_services());

    let metadata = api
        .load_formula_packs(vec![pack("filters", vec![rc_low_pass_formula()])])
        .unwrap();
    let formulas = api.list_formulas().unwrap();
    let categories = api.list_formula_categories().unwrap();

    assert_eq!(pack_ids(&metadata), ["filters"]);
    assert_eq!(formula_ids(&formulas), ["rc_low_pass_cutoff"]);
    assert_eq!(categories, ["filters/passive"]);
}

#[test]
fn api_returns_formula_details_and_not_found_error() {
    let api = HotSasApi::new(fake_services());
    api.load_formula_packs(vec![pack("filters", vec![rc_low_pass_formula()])])
        .unwrap();

    let details = api.get_formula("rc_low_pass_cutoff".to_string()).unwrap();
    let missing = api.get_formula("missing".to_string()).unwrap_err().to_dto();

    assert_eq!(details.id, "rc_low_pass_cutoff");
    assert_eq!(details.variables[0].name, "C");
    assert_eq!(details.equations[0].id, "cutoff");
    assert_eq!(details.outputs[0].name, "fc");
    assert_eq!(
        details.linked_circuit_template_id.as_deref(),
        Some("rc_low_pass_template")
    );
    assert_eq!(missing.code, "formula_not_found");
}

fn pack_ids(metadata: &[FormulaPackDto]) -> Vec<&str> {
    metadata.iter().map(|pack| pack.pack_id.as_str()).collect()
}

fn formula_ids(formulas: &[FormulaSummaryDto]) -> Vec<&str> {
    formulas.iter().map(|formula| formula.id.as_str()).collect()
}

fn pack(id: &str, formulas: Vec<hotsas_core::FormulaDefinition>) -> FormulaPack {
    FormulaPack {
        pack_id: id.to_string(),
        title: id.to_string(),
        version: "0.1.0".to_string(),
        formulas,
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
        panic!("formula registry API tests must not call storage")
    }
}

struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        panic!("formula registry API tests must not call formula engine")
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        panic!("formula registry API tests must not call netlist exporter")
    }
}

struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn run_ac_sweep(
        &self,
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        panic!("formula registry API tests must not call simulation engine")
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("formula registry API tests must not call report exporter")
    }

    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("formula registry API tests must not call report exporter")
    }
}

#[derive(Debug, Default)]
struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        panic!("formula registry API tests must not call bom exporter")
    }
    fn export_bom_json(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        panic!("formula registry API tests must not call bom exporter")
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, PortError> {
        panic!("formula registry API tests must not call simulation data exporter")
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        panic!("formula registry API tests must not call component library exporter")
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        panic!("formula registry API tests must not call schematic exporter")
    }
}
