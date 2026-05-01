use crate::ApplicationError;
use hotsas_core::{CircuitProject, SimulationResult};
use hotsas_ports::SimulationEnginePort;
use std::sync::Arc;

#[derive(Clone)]
pub struct SimulationService {
    simulation_engine: Arc<dyn SimulationEnginePort>,
}

impl SimulationService {
    pub fn new(simulation_engine: Arc<dyn SimulationEnginePort>) -> Self {
        Self { simulation_engine }
    }

    pub fn run_mock_ac_simulation(
        &self,
        project: &CircuitProject,
    ) -> Result<SimulationResult, ApplicationError> {
        let profile = project
            .simulation_profiles
            .iter()
            .find(|profile| profile.id == "ac-sweep")
            .ok_or_else(|| {
                ApplicationError::MissingProjectState("ac-sweep profile not found".to_string())
            })?;
        Ok(self.simulation_engine.run_ac_sweep(project, profile)?)
    }
}
