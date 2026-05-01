use hotsas_core::{
    CircuitProject, ReportModel, SimulationProfile, SimulationResult, ValueWithUnit,
};
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum PortError {
    Storage(String),
    Formula(String),
    Export(String),
    Simulation(String),
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(message) => write!(f, "storage error: {message}"),
            Self::Formula(message) => write!(f, "formula error: {message}"),
            Self::Export(message) => write!(f, "export error: {message}"),
            Self::Simulation(message) => write!(f, "simulation error: {message}"),
        }
    }
}

impl std::error::Error for PortError {}

pub trait StoragePort: Send + Sync {
    fn save_project(&self, path: &Path, project: &CircuitProject) -> Result<(), PortError>;
    fn load_project(&self, path: &Path) -> Result<CircuitProject, PortError>;
}

pub trait FormulaEnginePort: Send + Sync {
    fn calculate_rc_low_pass_cutoff(
        &self,
        resistance: &ValueWithUnit,
        capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError>;
}

pub trait NetlistExporterPort: Send + Sync {
    fn export_spice_netlist(&self, project: &CircuitProject) -> Result<String, PortError>;
}

pub trait SimulationEnginePort: Send + Sync {
    fn run_ac_sweep(
        &self,
        project: &CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError>;
}

pub trait ReportExporterPort: Send + Sync {
    fn export_markdown(&self, report: &ReportModel) -> Result<String, PortError>;
    fn export_html(&self, report: &ReportModel) -> Result<String, PortError>;
}
