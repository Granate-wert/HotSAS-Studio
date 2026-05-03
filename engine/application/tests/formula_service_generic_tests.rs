use hotsas_application::{AppServices, ApplicationError, FormulaRegistryService};
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, EngineeringUnit, FormulaDefinition, FormulaOutput,
    FormulaPack, ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
use hotsas_core::{ProjectPackageManifest, ProjectPackageValidationReport};
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

#[test]
fn formula_service_calculates_formula_from_registry() {
    let services = fake_services();
    let registry = FormulaRegistryService::new(vec![pack(vec![rc_low_pass_formula()])]).unwrap();
    let result = services
        .formula_service()
        .calculate_formula(
            &registry,
            "rc_low_pass_cutoff",
            BTreeMap::from([
                ("R".to_string(), value("10k", EngineeringUnit::Ohm)),
                ("C".to_string(), value("100n", EngineeringUnit::Farad)),
            ]),
        )
        .unwrap();

    assert_eq!(result.outputs["fc"].unit, EngineeringUnit::Hertz);
    assert!((result.outputs["fc"].si_value() - 159.154943).abs() < 0.000001);
}

#[test]
fn formula_service_reports_missing_formula() {
    let services = fake_services();
    let registry = FormulaRegistryService::new(vec![pack(vec![rc_low_pass_formula()])]).unwrap();

    let error = services
        .formula_service()
        .calculate_formula(&registry, "missing", BTreeMap::new())
        .unwrap_err();

    assert!(matches!(
        error,
        ApplicationError::FormulaNotFound(id) if id == "missing"
    ));
}

#[test]
fn formula_service_calculate_rc_low_pass_compatibility_path_still_works() {
    let services = fake_services();
    let project = hotsas_core::rc_low_pass_project();

    let result = services.calculate_rc_low_pass_cutoff(&project).unwrap();

    assert_eq!(result.unit, EngineeringUnit::Hertz);
    assert!((result.si_value() - 159.154943).abs() < 0.000001);
}

fn pack(formulas: Vec<FormulaDefinition>) -> FormulaPack {
    FormulaPack {
        pack_id: "filters".to_string(),
        title: "Filters".to_string(),
        version: "0.1.0".to_string(),
        formulas,
    }
}

fn value(input: &str, unit: EngineeringUnit) -> ValueWithUnit {
    ValueWithUnit::parse_with_default(input, unit).unwrap()
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
        formula: &FormulaDefinition,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<hotsas_core::FormulaEvaluationResult, PortError> {
        Ok(hotsas_core::FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: formula.equations[0].id.clone(),
            expression: formula.equations[0].expression.clone(),
            inputs: variables.clone(),
            outputs: BTreeMap::from([(
                "fc".to_string(),
                ValueWithUnit::new_si(159.154943, EngineeringUnit::Hertz),
            )]),
            warnings: vec![],
        })
    }

    fn evaluate_expression(
        &self,
        _expression: &str,
        _variables: &BTreeMap<String, ValueWithUnit>,
        _expected_outputs: &BTreeMap<String, FormulaOutput>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        Ok(BTreeMap::new())
    }

    fn validate_expression(
        &self,
        expression: &str,
    ) -> Result<hotsas_core::FormulaExpressionValidationResult, PortError> {
        Ok(hotsas_core::FormulaExpressionValidationResult {
            expression: expression.to_string(),
            supported: true,
            reason: None,
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
        Err(PortError::Simulation(
            "fake simulation is not needed by this test".to_string(),
        ))
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
