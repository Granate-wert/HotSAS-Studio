use hotsas_api::{FormulaPackDto, FormulaSummaryDto, HotSasApi};
use hotsas_application::AppServices;
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, FormulaPack, ReportModel, SimulationProfile,
    SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::path::Path;
use std::sync::Arc;

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
