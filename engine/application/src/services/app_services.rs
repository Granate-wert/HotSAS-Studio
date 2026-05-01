use crate::{
    ApplicationError, CircuitTemplateService, ExportService, FormulaService,
    NetlistGenerationService, PreferredValuesService, ProjectService, SimulationService,
};
use hotsas_core::{
    CircuitProject, PreferredValueResult, ReportModel, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, ReportExporterPort, SimulationEnginePort, StoragePort,
};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppServices {
    project_service: ProjectService,
    formula_service: FormulaService,
    preferred_values_service: PreferredValuesService,
    circuit_template_service: CircuitTemplateService,
    netlist_generation_service: NetlistGenerationService,
    simulation_service: SimulationService,
    export_service: ExportService,
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
            project_service: ProjectService::new(storage),
            formula_service: FormulaService::new(formula_engine),
            preferred_values_service: PreferredValuesService,
            circuit_template_service: CircuitTemplateService,
            netlist_generation_service: NetlistGenerationService::new(netlist_exporter),
            simulation_service: SimulationService::new(simulation_engine),
            export_service: ExportService::new(report_exporter),
        }
    }

    pub fn project_service(&self) -> &ProjectService {
        &self.project_service
    }

    pub fn formula_service(&self) -> &FormulaService {
        &self.formula_service
    }

    pub fn preferred_values_service(&self) -> &PreferredValuesService {
        &self.preferred_values_service
    }

    pub fn circuit_template_service(&self) -> &CircuitTemplateService {
        &self.circuit_template_service
    }

    pub fn netlist_generation_service(&self) -> &NetlistGenerationService {
        &self.netlist_generation_service
    }

    pub fn simulation_service(&self) -> &SimulationService {
        &self.simulation_service
    }

    pub fn export_service(&self) -> &ExportService {
        &self.export_service
    }

    pub fn create_rc_low_pass_demo_project(&self) -> CircuitProject {
        self.circuit_template_service
            .create_rc_low_pass_demo_project()
    }

    pub fn calculate_rc_low_pass_cutoff(
        &self,
        project: &CircuitProject,
    ) -> Result<ValueWithUnit, ApplicationError> {
        self.formula_service.calculate_rc_low_pass_cutoff(project)
    }

    pub fn nearest_e24(
        &self,
        requested_value: ValueWithUnit,
    ) -> Result<PreferredValueResult, ApplicationError> {
        self.preferred_values_service.nearest_e24(requested_value)
    }

    pub fn nearest_e24_for_resistor(
        &self,
        project: &CircuitProject,
    ) -> Result<PreferredValueResult, ApplicationError> {
        self.preferred_values_service
            .nearest_e24_for_resistor(project)
    }

    pub fn generate_spice_netlist(
        &self,
        project: &CircuitProject,
    ) -> Result<String, ApplicationError> {
        self.netlist_generation_service
            .generate_spice_netlist(project)
    }

    pub fn run_mock_ac_simulation(
        &self,
        project: &CircuitProject,
    ) -> Result<SimulationResult, ApplicationError> {
        self.simulation_service.run_mock_ac_simulation(project)
    }

    pub fn build_report_model(
        &self,
        project: &CircuitProject,
        cutoff_frequency: &ValueWithUnit,
        nearest_e24: &PreferredValueResult,
        netlist: &str,
        simulation_result: &SimulationResult,
    ) -> ReportModel {
        self.export_service.build_report_model(
            project,
            cutoff_frequency,
            nearest_e24,
            netlist,
            simulation_result,
        )
    }

    pub fn export_markdown_report(&self, report: &ReportModel) -> Result<String, ApplicationError> {
        self.export_service.export_markdown_report(report)
    }

    pub fn export_html_report(&self, report: &ReportModel) -> Result<String, ApplicationError> {
        self.export_service.export_html_report(report)
    }

    pub fn save_project(
        &self,
        path: &Path,
        project: &CircuitProject,
    ) -> Result<(), ApplicationError> {
        self.project_service.save_project(path, project)
    }

    pub fn load_project(&self, path: &Path) -> Result<CircuitProject, ApplicationError> {
        self.project_service.load_project(path)
    }
}
