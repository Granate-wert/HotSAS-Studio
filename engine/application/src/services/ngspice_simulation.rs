use crate::ApplicationError;
use hotsas_core::{CircuitProject, NgspiceAvailability, SimulationResult};
use hotsas_ports::SimulationEnginePort;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationEngineChoice {
    Mock,
    Ngspice,
    Auto,
}

impl std::str::FromStr for SimulationEngineChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mock" => Ok(Self::Mock),
            "ngspice" => Ok(Self::Ngspice),
            "auto" => Ok(Self::Auto),
            other => Err(format!("unknown engine choice: {other}")),
        }
    }
}

#[derive(Clone)]
pub struct NgspiceSimulationService {
    mock_engine: Arc<dyn SimulationEnginePort>,
    ngspice_engine: Arc<dyn SimulationEnginePort>,
    history: std::sync::Arc<std::sync::Mutex<Vec<SimulationResult>>>,
}

impl NgspiceSimulationService {
    pub fn new(
        mock_engine: Arc<dyn SimulationEnginePort>,
        ngspice_engine: Arc<dyn SimulationEnginePort>,
    ) -> Self {
        Self {
            mock_engine,
            ngspice_engine,
            history: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
        }
    }

    pub fn check_ngspice_availability(&self) -> Result<NgspiceAvailability, ApplicationError> {
        Ok(self.ngspice_engine.check_availability()?)
    }

    pub fn run_ac_sweep(
        &self,
        project: &CircuitProject,
        choice: SimulationEngineChoice,
    ) -> Result<SimulationResult, ApplicationError> {
        let profile = project
            .simulation_profiles
            .iter()
            .find(|p| p.id == "ac-sweep")
            .ok_or_else(|| {
                ApplicationError::MissingProjectState("ac-sweep profile not found".to_string())
            })?;
        self.run_ac_sweep_with_profile(project, profile, choice)
    }

    pub fn run_ac_sweep_with_profile(
        &self,
        project: &CircuitProject,
        profile: &hotsas_core::SimulationProfile,
        choice: SimulationEngineChoice,
    ) -> Result<SimulationResult, ApplicationError> {
        let result = match choice {
            SimulationEngineChoice::Mock => self.mock_engine.run_ac_sweep(project, profile),
            SimulationEngineChoice::Ngspice => self.ngspice_engine.run_ac_sweep(project, profile),
            SimulationEngineChoice::Auto => {
                match self.ngspice_engine.run_ac_sweep(project, profile) {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        let mut fallback = self.mock_engine.run_ac_sweep(project, profile)?;
                        fallback
                            .warnings
                            .push(format!("ngspice failed ({e}); falling back to mock engine"));
                        Ok(fallback)
                    }
                }
            }
        }?;

        self.push_history(result.clone());
        Ok(result)
    }

    pub fn run_operating_point(
        &self,
        project: &CircuitProject,
        choice: SimulationEngineChoice,
    ) -> Result<SimulationResult, ApplicationError> {
        let profile = project.simulation_profiles.first().ok_or_else(|| {
            ApplicationError::MissingProjectState("no simulation profile found".to_string())
        })?;
        self.run_operating_point_with_profile(project, profile, choice)
    }

    pub fn run_operating_point_with_profile(
        &self,
        project: &CircuitProject,
        profile: &hotsas_core::SimulationProfile,
        choice: SimulationEngineChoice,
    ) -> Result<SimulationResult, ApplicationError> {
        let result = match choice {
            SimulationEngineChoice::Mock => self.mock_engine.run_operating_point(project, profile),
            SimulationEngineChoice::Ngspice => {
                self.ngspice_engine.run_operating_point(project, profile)
            }
            SimulationEngineChoice::Auto => {
                match self.ngspice_engine.run_operating_point(project, profile) {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        let mut fallback =
                            self.mock_engine.run_operating_point(project, profile)?;
                        fallback
                            .warnings
                            .push(format!("ngspice failed ({e}); falling back to mock engine"));
                        Ok(fallback)
                    }
                }
            }
        }?;

        self.push_history(result.clone());
        Ok(result)
    }

    pub fn run_transient(
        &self,
        project: &CircuitProject,
        choice: SimulationEngineChoice,
    ) -> Result<SimulationResult, ApplicationError> {
        let profile = project.simulation_profiles.first().ok_or_else(|| {
            ApplicationError::MissingProjectState("no simulation profile found".to_string())
        })?;
        self.run_transient_with_profile(project, profile, choice)
    }

    pub fn run_transient_with_profile(
        &self,
        project: &CircuitProject,
        profile: &hotsas_core::SimulationProfile,
        choice: SimulationEngineChoice,
    ) -> Result<SimulationResult, ApplicationError> {
        let result = match choice {
            SimulationEngineChoice::Mock => self.mock_engine.run_transient(project, profile),
            SimulationEngineChoice::Ngspice => self.ngspice_engine.run_transient(project, profile),
            SimulationEngineChoice::Auto => {
                match self.ngspice_engine.run_transient(project, profile) {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        let mut fallback = self.mock_engine.run_transient(project, profile)?;
                        fallback
                            .warnings
                            .push(format!("ngspice failed ({e}); falling back to mock engine"));
                        Ok(fallback)
                    }
                }
            }
        }?;

        self.push_history(result.clone());
        Ok(result)
    }

    pub fn list_simulation_history(&self) -> Vec<SimulationResult> {
        self.history
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }

    fn push_history(&self, result: SimulationResult) {
        if let Ok(mut guard) = self.history.lock() {
            guard.push(result);
            if guard.len() > 50 {
                guard.remove(0);
            }
        }
    }
}
