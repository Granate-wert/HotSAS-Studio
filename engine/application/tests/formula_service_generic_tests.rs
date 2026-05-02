use hotsas_application::{AppServices, ApplicationError, FormulaRegistryService};
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, EngineeringUnit, FormulaDefinition, FormulaOutput,
    FormulaPack, ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

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

fn fake_services() -> AppServices {
    AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeFormulaEngine),
        Arc::new(FakeNetlistExporter),
        Arc::new(FakeSimulationEngine),
        Arc::new(FakeReportExporter),
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
