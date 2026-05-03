use hotsas_api::{
    FormulaCalculationRequestDto, FormulaPackDto, FormulaVariableInputDto, HotSasApi,
};
use hotsas_application::AppServices;
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, EngineeringUnit, FormulaDefinition, FormulaEquation,
    FormulaEvaluationResult, FormulaExpressionValidationResult, FormulaOutput, FormulaPack,
    FormulaVariable, ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
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
fn api_calculates_rc_low_pass_formula_generically() {
    let api = api_with_pack(vec![rc_low_pass_formula()]);

    let result = api
        .calculate_formula(FormulaCalculationRequestDto {
            formula_id: "rc_low_pass_cutoff".to_string(),
            variables: vec![input("R", "10k", None), input("C", "100n", Some("F"))],
        })
        .unwrap();

    assert_eq!(result.formula_id, "rc_low_pass_cutoff");
    assert_eq!(result.outputs[0].name, "fc");
    assert_eq!(result.outputs[0].value.unit, "Hz");
    assert!((result.outputs[0].value.si_value - 159.154943).abs() < 0.000001);
}

#[test]
fn api_calculates_ohms_law_formula_generically() {
    let api = api_with_pack(vec![ohms_law_formula()]);

    let result = api
        .calculate_formula(FormulaCalculationRequestDto {
            formula_id: "ohms_law".to_string(),
            variables: vec![input("I", "2m", Some("A")), input("R", "10k", Some("Ohm"))],
        })
        .unwrap();

    assert_eq!(result.outputs[0].name, "V");
    assert_eq!(result.outputs[0].value.unit, "V");
    assert!((result.outputs[0].value.si_value - 20.0).abs() < 0.000001);
}

#[test]
fn api_reports_missing_formula_missing_variable_and_unsupported_expression() {
    let api = api_with_pack(vec![rc_low_pass_formula(), unsupported_formula()]);

    let missing_formula = api
        .calculate_formula(FormulaCalculationRequestDto {
            formula_id: "missing".to_string(),
            variables: vec![],
        })
        .unwrap_err()
        .to_dto();
    assert_eq!(missing_formula.code, "formula_not_found");

    let missing_variable = api
        .calculate_formula(FormulaCalculationRequestDto {
            formula_id: "rc_low_pass_cutoff".to_string(),
            variables: vec![input("R", "10k", Some("Ohm"))],
        })
        .unwrap_err()
        .to_dto();
    assert!(
        matches!(
            missing_variable.code.as_str(),
            "invalid_input" | "port_error"
        ),
        "unexpected code: {}",
        missing_variable.code
    );

    let unsupported = api
        .calculate_formula(FormulaCalculationRequestDto {
            formula_id: "unsupported".to_string(),
            variables: vec![input("Y", "4", None)],
        })
        .unwrap_err()
        .to_dto();
    assert_eq!(unsupported.code, "port_error");
    assert!(unsupported.message.contains("unsupported expression"));
}

fn api_with_pack(formulas: Vec<FormulaDefinition>) -> HotSasApi {
    let api = HotSasApi::new(fake_services());
    let metadata = api.load_formula_packs(vec![pack(formulas)]).unwrap();
    assert_eq!(pack_ids(&metadata), ["test_pack"]);
    api
}

fn pack_ids(metadata: &[FormulaPackDto]) -> Vec<&str> {
    metadata.iter().map(|pack| pack.pack_id.as_str()).collect()
}

fn input(name: &str, value: &str, unit: Option<&str>) -> FormulaVariableInputDto {
    FormulaVariableInputDto {
        name: name.to_string(),
        value: value.to_string(),
        unit: unit.map(str::to_string),
    }
}

fn pack(formulas: Vec<FormulaDefinition>) -> FormulaPack {
    FormulaPack {
        pack_id: "test_pack".to_string(),
        title: "Test Pack".to_string(),
        version: "0.1.0".to_string(),
        formulas,
    }
}

fn ohms_law_formula() -> FormulaDefinition {
    FormulaDefinition {
        id: "ohms_law".to_string(),
        title: "Ohm's Law".to_string(),
        category: "basic/dc".to_string(),
        description: "Relation between voltage, current, and resistance.".to_string(),
        equations: vec![FormulaEquation {
            id: "ohms_law".to_string(),
            latex: "V = I R".to_string(),
            expression: "V = I * R".to_string(),
            solve_for: vec!["V".to_string(), "I".to_string(), "R".to_string()],
        }],
        variables: BTreeMap::from([
            (
                "I".to_string(),
                FormulaVariable {
                    unit: EngineeringUnit::Ampere,
                    description: "Current".to_string(),
                    default: None,
                },
            ),
            (
                "R".to_string(),
                FormulaVariable {
                    unit: EngineeringUnit::Ohm,
                    description: "Resistance".to_string(),
                    default: Some(
                        ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap(),
                    ),
                },
            ),
        ]),
        outputs: BTreeMap::from([(
            "V".to_string(),
            FormulaOutput {
                unit: EngineeringUnit::Volt,
                description: "Voltage".to_string(),
            },
        )]),
        assumptions: vec![],
        limitations: vec![],
        linked_circuit_template_id: None,
        mapping: None,
        default_simulation_profile: None,
        examples: vec![],
    }
}

fn unsupported_formula() -> FormulaDefinition {
    let mut formula = ohms_law_formula();
    formula.id = "unsupported".to_string();
    formula.equations[0].expression = "X = sqrt(Y)".to_string();
    formula.variables = BTreeMap::from([(
        "Y".to_string(),
        FormulaVariable {
            unit: EngineeringUnit::Unitless,
            description: "Input".to_string(),
            default: None,
        },
    )]);
    formula.outputs = BTreeMap::from([(
        "X".to_string(),
        FormulaOutput {
            unit: EngineeringUnit::Unitless,
            description: "Unsupported".to_string(),
        },
    )]);
    formula
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
        panic!("formula calculation API tests must not call storage")
    }
}

struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        panic!("generic API tests must use evaluate_formula")
    }

    fn evaluate_formula(
        &self,
        formula: &FormulaDefinition,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<FormulaEvaluationResult, PortError> {
        let expression = &formula.equations[0].expression;
        let outputs = self.evaluate_expression(expression, variables, &formula.outputs)?;
        Ok(FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: formula.equations[0].id.clone(),
            expression: expression.clone(),
            inputs: variables.clone(),
            outputs,
            warnings: vec![],
        })
    }

    fn evaluate_expression(
        &self,
        expression: &str,
        variables: &BTreeMap<String, ValueWithUnit>,
        _expected_outputs: &BTreeMap<String, FormulaOutput>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        match expression.replace(' ', "").as_str() {
            "fc=1/(2*pi*R*C)" => {
                let r = variables
                    .get("R")
                    .ok_or_else(|| PortError::Formula("missing variable R".to_string()))?;
                let c = variables
                    .get("C")
                    .ok_or_else(|| PortError::Formula("missing variable C".to_string()))?;
                Ok(BTreeMap::from([(
                    "fc".to_string(),
                    ValueWithUnit::new_si(
                        1.0 / (2.0 * std::f64::consts::PI * r.si_value() * c.si_value()),
                        EngineeringUnit::Hertz,
                    ),
                )]))
            }
            "V=I*R" => {
                let i = variables
                    .get("I")
                    .ok_or_else(|| PortError::Formula("missing variable I".to_string()))?;
                let r = variables
                    .get("R")
                    .ok_or_else(|| PortError::Formula("missing variable R".to_string()))?;
                Ok(BTreeMap::from([(
                    "V".to_string(),
                    ValueWithUnit::new_si(i.si_value() * r.si_value(), EngineeringUnit::Volt),
                )]))
            }
            _ => Err(PortError::Formula(format!(
                "unsupported expression: {expression}"
            ))),
        }
    }

    fn validate_expression(
        &self,
        expression: &str,
    ) -> Result<FormulaExpressionValidationResult, PortError> {
        Ok(FormulaExpressionValidationResult {
            expression: expression.to_string(),
            supported: true,
            reason: None,
        })
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call netlist exporter")
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
        panic!("formula calculation API tests must not call simulation engine")
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call report exporter")
    }

    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call report exporter")
    }
}

#[derive(Debug, Default)]
struct FakeBomExporter;

impl BomExporterPort for FakeBomExporter {
    fn export_bom_csv(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call bom exporter")
    }
    fn export_bom_json(&self, _project: &hotsas_core::CircuitProject) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call bom exporter")
    }
}

#[derive(Debug, Default)]
struct FakeSimulationDataExporter;

impl SimulationDataExporterPort for FakeSimulationDataExporter {
    fn export_simulation_csv(
        &self,
        _simulation: &hotsas_core::SimulationResult,
    ) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call simulation data exporter")
    }
}

#[derive(Debug, Default)]
struct FakeComponentLibraryExporter;

impl ComponentLibraryExporterPort for FakeComponentLibraryExporter {
    fn export_component_library_json(
        &self,
        _library: &hotsas_core::ComponentLibrary,
    ) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call component library exporter")
    }
}

#[derive(Debug, Default)]
struct FakeSchematicExporter;

impl SchematicExporterPort for FakeSchematicExporter {
    fn export_svg_schematic(
        &self,
        _project: &hotsas_core::CircuitProject,
    ) -> Result<String, PortError> {
        panic!("formula calculation API tests must not call schematic exporter")
    }
}
