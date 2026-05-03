use hotsas_application::{NgspiceSimulationService, SimulationEngineChoice};
use hotsas_core::{
    rc_low_pass_project, EngineeringUnit, GraphPoint, GraphSeries, SimulationProfile,
    SimulationResult, SimulationStatus, SimulationType, ValueWithUnit,
};
use hotsas_ports::{PortError, SimulationEnginePort};
use std::collections::BTreeMap;
use std::sync::Arc;

struct FakeMockEngine;

impl SimulationEnginePort for FakeMockEngine {
    fn engine_name(&self) -> &str {
        "fake-mock"
    }

    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Ok(SimulationResult {
            id: "mock-ac".to_string(),
            profile_id: profile.id.clone(),
            status: SimulationStatus::Completed,
            engine: "mock".to_string(),
            graph_series: vec![GraphSeries {
                name: "Gain".to_string(),
                x_unit: EngineeringUnit::Hertz,
                y_unit: EngineeringUnit::Unitless,
                points: vec![GraphPoint { x: 100.0, y: -3.0 }],
                metadata: BTreeMap::new(),
            }],
            measurements: BTreeMap::new(),
            warnings: vec![],
            errors: vec![],
            raw_data_path: None,
            metadata: BTreeMap::new(),
        })
    }

    fn run_operating_point(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Ok(SimulationResult {
            id: "mock-op".to_string(),
            profile_id: profile.id.clone(),
            status: SimulationStatus::Completed,
            engine: "mock".to_string(),
            graph_series: vec![],
            measurements: BTreeMap::from([(
                "v(net_out)".to_string(),
                ValueWithUnit::new_si(0.5, EngineeringUnit::Volt),
            )]),
            warnings: vec![],
            errors: vec![],
            raw_data_path: None,
            metadata: BTreeMap::new(),
        })
    }

    fn run_transient(
        &self,
        _project: &hotsas_core::CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Ok(SimulationResult {
            id: "mock-tran".to_string(),
            profile_id: profile.id.clone(),
            status: SimulationStatus::Completed,
            engine: "mock".to_string(),
            graph_series: vec![GraphSeries {
                name: "V(out)".to_string(),
                x_unit: EngineeringUnit::Second,
                y_unit: EngineeringUnit::Volt,
                points: vec![GraphPoint { x: 0.0, y: 0.0 }],
                metadata: BTreeMap::new(),
            }],
            measurements: BTreeMap::new(),
            warnings: vec![],
            errors: vec![],
            raw_data_path: None,
            metadata: BTreeMap::new(),
        })
    }
}

struct AlwaysUnavailableNgspiceEngine;

impl SimulationEnginePort for AlwaysUnavailableNgspiceEngine {
    fn engine_name(&self) -> &str {
        "ngspice"
    }

    fn check_availability(&self) -> Result<hotsas_core::NgspiceAvailability, PortError> {
        Ok(hotsas_core::NgspiceAvailability {
            available: false,
            executable_path: None,
            version: None,
            message: Some("ngspice not installed".to_string()),
            warnings: vec![],
        })
    }

    fn run_ac_sweep(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation("ngspice unavailable".to_string()))
    }

    fn run_operating_point(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation("ngspice unavailable".to_string()))
    }

    fn run_transient(
        &self,
        _project: &hotsas_core::CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation("ngspice unavailable".to_string()))
    }
}

fn demo_project() -> hotsas_core::CircuitProject {
    let mut project = rc_low_pass_project();
    project.simulation_profiles = vec![SimulationProfile {
        id: "ac-sweep".to_string(),
        simulation_type: SimulationType::AcSweep,
        parameters: BTreeMap::new(),
        requested_outputs: vec!["gain_db".to_string(), "phase_deg".to_string()],
    }];
    project
}

#[test]
fn auto_falls_back_to_mock_when_ngspice_unavailable() {
    let service = NgspiceSimulationService::new(
        Arc::new(FakeMockEngine),
        Arc::new(AlwaysUnavailableNgspiceEngine),
    );
    let project = demo_project();
    let result = service
        .run_ac_sweep(&project, SimulationEngineChoice::Auto)
        .expect("Auto should fallback to mock");
    assert_eq!(result.engine, "mock");
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("falling back to mock engine")));
}

#[test]
fn direct_ngspice_returns_controlled_unavailable_error() {
    let service = NgspiceSimulationService::new(
        Arc::new(FakeMockEngine),
        Arc::new(AlwaysUnavailableNgspiceEngine),
    );
    let project = demo_project();
    let result = service.run_ac_sweep(&project, SimulationEngineChoice::Ngspice);
    assert!(result.is_err());
}

#[test]
fn mock_simulation_still_returns_graph_series() {
    let service = NgspiceSimulationService::new(
        Arc::new(FakeMockEngine),
        Arc::new(AlwaysUnavailableNgspiceEngine),
    );
    let project = demo_project();
    let result = service
        .run_ac_sweep(&project, SimulationEngineChoice::Mock)
        .expect("mock should work");
    assert_eq!(result.engine, "mock");
    assert!(!result.graph_series.is_empty());
}

#[test]
fn successful_fake_ngspice_run_produces_simulation_result() {
    // This test uses the mock as the "ngspice" engine to verify the service plumbing
    let service = NgspiceSimulationService::new(Arc::new(FakeMockEngine), Arc::new(FakeMockEngine));
    let project = demo_project();
    let result = service
        .run_ac_sweep(&project, SimulationEngineChoice::Ngspice)
        .expect("should succeed when ngspice engine is fake-mock");
    assert!(!result.graph_series.is_empty());
}

#[test]
fn history_tracks_runs() {
    let service = NgspiceSimulationService::new(
        Arc::new(FakeMockEngine),
        Arc::new(AlwaysUnavailableNgspiceEngine),
    );
    let project = demo_project();
    let _ = service.run_ac_sweep(&project, SimulationEngineChoice::Mock);
    let history = service.list_simulation_history();
    assert_eq!(history.len(), 1);
}
