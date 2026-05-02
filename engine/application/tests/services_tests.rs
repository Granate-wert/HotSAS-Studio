use hotsas_application::AppServices;
use hotsas_core::{
    rc_low_pass_project, CircuitProject, EngineeringUnit, ReportModel, SimulationProfile,
    SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::path::Path;
use std::sync::Arc;

#[test]
fn app_services_create_demo_and_select_nearest_e24() {
    let services = fake_services();

    let project = services.create_rc_low_pass_demo_project();
    let nearest = services.nearest_e24_for_resistor(&project).unwrap();

    assert_eq!(project.id, "rc-low-pass-demo");
    assert_eq!(nearest.nearest.unit, EngineeringUnit::Ohm);
    assert!(nearest.nearest.si_value() > 0.0);
}

#[test]
fn formula_service_returns_error_when_required_parameter_is_missing() {
    let services = fake_services();
    let mut project = rc_low_pass_project();
    project
        .schematic
        .components
        .iter_mut()
        .find(|component| component.instance_id == "R1")
        .unwrap()
        .overridden_parameters
        .remove("resistance");

    let result = services.calculate_rc_low_pass_cutoff(&project);

    assert!(result.is_err(), "missing R1.resistance must return Err");
}

#[test]
fn simulation_service_returns_error_when_ac_profile_is_missing() {
    let services = fake_services();
    let mut project = rc_low_pass_project();
    project.simulation_profiles.clear();

    let result = services.run_mock_ac_simulation(&project);

    assert!(result.is_err(), "missing ac-sweep profile must return Err");
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
        Ok(rc_low_pass_project())
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
