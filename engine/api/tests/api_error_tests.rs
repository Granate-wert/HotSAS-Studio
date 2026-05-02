use hotsas_api::{ApiError, HotSasApi};
use hotsas_application::{AppServices, ApplicationError};
use hotsas_core::{
    CircuitProject, ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::path::Path;
use std::sync::Arc;

#[test]
fn api_error_dto_has_stable_codes_messages_and_details() {
    let invalid = ApiError::InvalidInput("unsupported unit: Volt".to_string()).to_dto();
    assert_eq!(invalid.code, "invalid_input");
    assert!(invalid.message.contains("unsupported unit"));
    assert_eq!(invalid.details.as_deref(), Some("unsupported unit: Volt"));

    let state = ApiError::State("create or open a project first".to_string()).to_dto();
    assert_eq!(state.code, "state_error");
    assert!(state.message.contains("state error"));
    assert_eq!(
        state.details.as_deref(),
        Some("create or open a project first")
    );

    let port = ApiError::Application(ApplicationError::Port(PortError::Storage(
        "disk unavailable".to_string(),
    )))
    .to_dto();
    assert_eq!(port.code, "port_error");
    assert!(port.message.contains("storage error"));
    assert!(port
        .details
        .as_deref()
        .unwrap()
        .contains("disk unavailable"));
}

#[test]
fn facade_returns_state_errors_before_project_is_created() {
    let api = HotSasApi::new(fake_services());

    for (name, result) in [
        ("calculate", api.calculate_rc_low_pass().map(|_| ())),
        ("nearest_e24", api.nearest_e24_for_resistor().map(|_| ())),
        ("netlist", api.generate_spice_netlist().map(|_| ())),
        ("markdown", api.export_markdown_report().map(|_| ())),
        (
            "save",
            api.save_project_json("unused/project.json".to_string())
                .map(|_| ()),
        ),
    ] {
        let dto = result.unwrap_err().to_dto();

        assert_eq!(dto.code, "state_error", "{name} must return state_error");
        assert!(
            dto.message.contains("create or open a project first"),
            "{name} returned unexpected message: {}",
            dto.message
        );
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
        panic!("state tests must not call storage")
    }

    fn load_project(&self, _path: &Path) -> Result<CircuitProject, PortError> {
        panic!("state tests must not call storage")
    }
}

struct FakeFormulaEngine;

impl FormulaEnginePort for FakeFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        _resistance: &ValueWithUnit,
        _capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        panic!("state tests must not call formula engine")
    }
}

struct FakeNetlistExporter;

impl NetlistExporterPort for FakeNetlistExporter {
    fn export_spice_netlist(&self, _project: &CircuitProject) -> Result<String, PortError> {
        panic!("state tests must not call netlist exporter")
    }
}

struct FakeSimulationEngine;

impl SimulationEnginePort for FakeSimulationEngine {
    fn run_ac_sweep(
        &self,
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        panic!("state tests must not call simulation engine")
    }
}

struct FakeReportExporter;

impl ReportExporterPort for FakeReportExporter {
    fn export_markdown(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("state tests must not call report exporter")
    }

    fn export_html(&self, _report: &ReportModel) -> Result<String, PortError> {
        panic!("state tests must not call report exporter")
    }
}
