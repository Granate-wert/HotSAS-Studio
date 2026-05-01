use hotsas_core::{
    calculate_error_percent, nearest_preferred_value, rc_low_pass_formula, rc_low_pass_project,
    BomLine, CircuitProject, CoreError, EngineeringUnit, PreferredValueResult,
    PreferredValueSeries, ReportModel, ReportSection, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum ApplicationError {
    Core(CoreError),
    Port(PortError),
    MissingProjectState(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(error) => write!(f, "{error}"),
            Self::Port(error) => write!(f, "{error}"),
            Self::MissingProjectState(message) => write!(f, "missing project state: {message}"),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<CoreError> for ApplicationError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}

impl From<PortError> for ApplicationError {
    fn from(value: PortError) -> Self {
        Self::Port(value)
    }
}

#[derive(Clone)]
pub struct AppServices {
    storage: Arc<dyn StoragePort>,
    formula_engine: Arc<dyn FormulaEnginePort>,
    netlist_exporter: Arc<dyn NetlistExporterPort>,
    simulation_engine: Arc<dyn SimulationEnginePort>,
    report_exporter: Arc<dyn ReportExporterPort>,
}

impl AppServices {
    pub fn new(
        storage: Arc<dyn StoragePort>,
        formula_engine: Arc<dyn FormulaEnginePort>,
        netlist_exporter: Arc<dyn NetlistExporterPort>,
        simulation_engine: Arc<dyn SimulationEnginePort>,
        report_exporter: Arc<dyn ReportExporterPort>,
    ) -> Self {
        Self {
            storage,
            formula_engine,
            netlist_exporter,
            simulation_engine,
            report_exporter,
        }
    }

    pub fn create_rc_low_pass_demo_project(&self) -> CircuitProject {
        rc_low_pass_project()
    }

    pub fn calculate_rc_low_pass_cutoff(
        &self,
        project: &CircuitProject,
    ) -> Result<ValueWithUnit, ApplicationError> {
        let resistance = component_parameter(project, "R1", "resistance")?;
        let capacitance = component_parameter(project, "C1", "capacitance")?;
        Ok(self
            .formula_engine
            .calculate_rc_low_pass_cutoff(&resistance, &capacitance)?)
    }

    pub fn nearest_e24(
        &self,
        requested_value: ValueWithUnit,
    ) -> Result<PreferredValueResult, ApplicationError> {
        Ok(nearest_preferred_value(
            requested_value,
            PreferredValueSeries::E24,
        )?)
    }

    pub fn generate_spice_netlist(
        &self,
        project: &CircuitProject,
    ) -> Result<String, ApplicationError> {
        Ok(self.netlist_exporter.export_spice_netlist(project)?)
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

    pub fn build_report_model(
        &self,
        project: &CircuitProject,
        cutoff_frequency: &ValueWithUnit,
        nearest_e24: &PreferredValueResult,
        netlist: &str,
        simulation_result: &SimulationResult,
    ) -> ReportModel {
        let formula = rc_low_pass_formula();
        let resistance = component_parameter(project, "R1", "resistance").ok();
        let capacitance = component_parameter(project, "C1", "capacitance").ok();
        let mut export_settings = BTreeMap::new();
        export_settings.insert("primary_format".to_string(), "markdown".to_string());
        export_settings.insert("html_available".to_string(), "true".to_string());
        export_settings.insert("pdf_status".to_string(), "placeholder".to_string());

        ReportModel {
            id: "rc-low-pass-report".to_string(),
            title: format!("{} Report", project.name),
            sections: vec![
                ReportSection {
                    title: "Project".to_string(),
                    body_markdown: format!(
                        "Project `{}` uses format version `{}` and engine version `{}`.",
                        project.name, project.format_version, project.engine_version
                    ),
                },
                ReportSection {
                    title: "Formula".to_string(),
                    body_markdown: format!(
                        "`fc = 1 / (2*pi*R*C)` = `{:.6} Hz`.",
                        cutoff_frequency.si_value()
                    ),
                },
                ReportSection {
                    title: "Preferred Value".to_string(),
                    body_markdown: format!(
                        "Nearest {} value for `{:.6} {}` is `{:.6} {}` with error `{:.4}%`.",
                        nearest_e24.series.label(),
                        nearest_e24.requested_value.si_value(),
                        nearest_e24.requested_value.unit.symbol(),
                        nearest_e24.nearest.si_value(),
                        nearest_e24.nearest.unit.symbol(),
                        calculate_error_percent(
                            nearest_e24.requested_value.si_value(),
                            nearest_e24.nearest.si_value()
                        )
                    ),
                },
                ReportSection {
                    title: "SPICE Netlist".to_string(),
                    body_markdown: format!("```spice\n{netlist}\n```"),
                },
                ReportSection {
                    title: "Mock AC Simulation".to_string(),
                    body_markdown: format!(
                        "Simulation status: `{:?}`. Series count: `{}`.",
                        simulation_result.status,
                        simulation_result.graph_series.len()
                    ),
                },
            ],
            included_schematic_images: vec![],
            included_formulas: vec![formula],
            included_simulation_results: vec![simulation_result.clone()],
            included_bom: vec![
                BomLine {
                    designator: "R1".to_string(),
                    quantity: 1,
                    value: resistance,
                    description: "RC low-pass resistor".to_string(),
                },
                BomLine {
                    designator: "C1".to_string(),
                    quantity: 1,
                    value: capacitance,
                    description: "RC low-pass capacitor".to_string(),
                },
            ],
            export_settings,
        }
    }

    pub fn export_markdown_report(&self, report: &ReportModel) -> Result<String, ApplicationError> {
        Ok(self.report_exporter.export_markdown(report)?)
    }

    pub fn export_html_report(&self, report: &ReportModel) -> Result<String, ApplicationError> {
        Ok(self.report_exporter.export_html(report)?)
    }

    pub fn save_project(
        &self,
        path: &Path,
        project: &CircuitProject,
    ) -> Result<(), ApplicationError> {
        Ok(self.storage.save_project(path, project)?)
    }

    pub fn load_project(&self, path: &Path) -> Result<CircuitProject, ApplicationError> {
        Ok(self.storage.load_project(path)?)
    }
}

pub fn component_parameter(
    project: &CircuitProject,
    component_id: &str,
    parameter: &str,
) -> Result<ValueWithUnit, CoreError> {
    project
        .schematic
        .components
        .iter()
        .find(|component| component.instance_id == component_id)
        .and_then(|component| component.overridden_parameters.get(parameter))
        .cloned()
        .ok_or_else(|| CoreError::MissingParameter {
            component_id: component_id.to_string(),
            parameter: parameter.to_string(),
        })
}

pub fn parse_requested_e24_value(
    input: &str,
    unit: EngineeringUnit,
) -> Result<ValueWithUnit, ApplicationError> {
    Ok(ValueWithUnit::parse_with_default(input, unit)?)
}
