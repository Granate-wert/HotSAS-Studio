use hotsas_application::{AppServices, FormulaRegistryService};
use hotsas_core::{
    ohms_law_formula, rc_low_pass_formula, voltage_divider_formula, CircuitProject,
    EngineeringUnit, FormulaDefinition, FormulaPack, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ProjectPackageStoragePort,
    ReportExporterPort, SimulationEnginePort, StoragePort,
};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

fn fake_services() -> AppServices {
    AppServices::new(
        Arc::new(FakeStorage),
        Arc::new(FakeProjectPackageStorage::default()),
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
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        Ok(ValueWithUnit::parse_with_default("159.15Hz", EngineeringUnit::Hertz).unwrap())
    }

    fn evaluate_formula(
        &self,
        formula: &FormulaDefinition,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<hotsas_core::FormulaEvaluationResult, PortError> {
        let output = match formula.id.as_str() {
            "ohms_law" => (
                "V".to_string(),
                ValueWithUnit::new_si(20.0, EngineeringUnit::Volt),
            ),
            "voltage_divider" => (
                "Vout".to_string(),
                ValueWithUnit::new_si(2.5, EngineeringUnit::Volt),
            ),
            _ => (
                "fc".to_string(),
                ValueWithUnit::new_si(159.154943, EngineeringUnit::Hertz),
            ),
        };
        Ok(hotsas_core::FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: formula.equations[0].id.clone(),
            expression: formula.equations[0].expression.clone(),
            inputs: variables.clone(),
            outputs: BTreeMap::from([output]),
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

fn pack(formulas: Vec<FormulaDefinition>) -> FormulaPack {
    FormulaPack {
        pack_id: "basic".to_string(),
        title: "Basic".to_string(),
        version: "0.1.0".to_string(),
        formulas,
    }
}

fn registry() -> FormulaRegistryService {
    FormulaRegistryService::new(vec![pack(vec![
        rc_low_pass_formula(),
        ohms_law_formula(),
        voltage_divider_formula(),
    ])])
    .unwrap()
}

#[test]
fn assignment_creates_variable() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "R = 10k",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.variables.contains_key("R"));
}

#[test]
fn formula_call_with_literal_values() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "rc_low_pass_cutoff(R=10k, C=100n)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.outputs.contains_key("fc"));
}

#[test]
fn formula_call_with_variables() {
    let services = fake_services();
    let reg = registry();
    let mut scope = BTreeMap::new();
    scope.insert(
        "R".to_string(),
        ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap(),
    );
    scope.insert(
        "C".to_string(),
        ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap(),
    );
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "rc_low_pass_cutoff(R=R, C=C)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.outputs.contains_key("fc"));
}

#[test]
fn ohms_law_formula_call() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "ohms_law(I=2m, R=10k)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.outputs.contains_key("V"));
}

#[test]
fn voltage_divider_formula_call() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "voltage_divider(Vin=5, R1=10k, R2=10k)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.outputs.contains_key("Vout"));
}

#[test]
fn nearest_e24_returns_expected_value() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "nearestE(15.93k, E24, Ohm)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.outputs.contains_key("nearest"));
}

#[test]
fn nearest_e96_returns_expected_value() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "nearestE(15.93k, E96, Ohm)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "success");
    assert!(result.outputs.contains_key("nearest"));
}

#[test]
fn unsupported_expression_returns_controlled_unsupported() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "sin(5)",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "unsupported");
    assert!(result
        .message
        .as_ref()
        .unwrap()
        .contains("Unsupported notebook expression in v1.4"));
}

#[test]
fn malformed_input_returns_controlled_error() {
    let services = fake_services();
    let reg = registry();
    let scope = BTreeMap::new();
    let result = services
        .engineering_notebook_service()
        .evaluate_input(
            "= 10k",
            &scope,
            &reg,
            services.preferred_values_service(),
            services.formula_service(),
        )
        .unwrap();
    assert_eq!(result.status.as_str(), "unsupported");
}
