use crate::services::SimulationWorkflowService;
use crate::services::{
    AdvancedReportService, DcdcCalculatorService, ProjectSessionService,
    SimulationDiagnosticsService, SimulationGraphService, SimulationHistoryService,
};
use crate::{
    ApplicationError, CircuitTemplateService, CircuitValidationService, ComponentLibraryService,
    ComponentModelMappingService, ComponentParameterService, EngineeringNotebookService,
    ExportCenterService, ExportService, FormulaService, ModelImportService,
    NetlistGenerationService, NgspiceSimulationService, PreferredValuesService,
    ProjectPackageService, ProjectService, SchematicEditingService, SelectedRegionAnalysisService,
    SimulationEngineChoice, SimulationService,
};
use hotsas_core::{
    CircuitProject, PreferredValueResult, ProjectPackageManifest, ProjectPackageValidationReport,
    ReportModel, SimulationResult, ValueWithUnit,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ComponentLibraryPort, FormulaEnginePort,
    NetlistExporterPort, ProjectPackageStoragePort, ReportExporterPort, SchematicExporterPort,
    SimulationDataExporterPort, SimulationEnginePort, SpiceModelParserPort, StoragePort,
    TouchstoneParserPort,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppServices {
    project_service: ProjectService,
    project_package_service: ProjectPackageService,
    formula_service: FormulaService,
    preferred_values_service: PreferredValuesService,
    circuit_template_service: CircuitTemplateService,
    circuit_validation_service: CircuitValidationService,
    engineering_notebook_service: EngineeringNotebookService,
    netlist_generation_service: NetlistGenerationService,
    simulation_service: SimulationService,
    ngspice_simulation_service: NgspiceSimulationService,
    export_service: ExportService,
    export_center_service: ExportCenterService,
    component_library_service: ComponentLibraryService,
    component_parameter_service: ComponentParameterService,
    schematic_editing_service: SchematicEditingService,
    selected_region_analysis_service: SelectedRegionAnalysisService,
    model_import_service: ModelImportService,
    dcdc_calculator_service: DcdcCalculatorService,
    advanced_report_service: AdvancedReportService,
    project_session_service: ProjectSessionService,
    simulation_workflow_service: SimulationWorkflowService,
    simulation_diagnostics_service: SimulationDiagnosticsService,
    simulation_history_service: SimulationHistoryService,
    simulation_graph_service: SimulationGraphService,
    component_model_mapping_service: ComponentModelMappingService,
}

impl AppServices {
    pub fn new(
        storage: Arc<dyn StoragePort>,
        project_package_storage: Arc<dyn ProjectPackageStoragePort>,
        formula_engine: Arc<dyn FormulaEnginePort>,
        netlist_exporter: Arc<dyn NetlistExporterPort>,
        mock_engine: Arc<dyn SimulationEnginePort>,
        ngspice_engine: Arc<dyn SimulationEnginePort>,
        report_exporter: Arc<dyn ReportExporterPort>,
        component_library_port: Arc<dyn ComponentLibraryPort>,
        bom_exporter: Arc<dyn BomExporterPort>,
        simulation_data_exporter: Arc<dyn SimulationDataExporterPort>,
        library_exporter: Arc<dyn ComponentLibraryExporterPort>,
        schematic_exporter: Arc<dyn SchematicExporterPort>,
        spice_parser: Arc<dyn SpiceModelParserPort>,
        touchstone_parser: Arc<dyn TouchstoneParserPort>,
    ) -> Self {
        Self {
            project_service: ProjectService::new(storage),
            project_package_service: ProjectPackageService::new(project_package_storage.clone()),
            formula_service: FormulaService::new(formula_engine),
            preferred_values_service: PreferredValuesService,
            circuit_template_service: CircuitTemplateService,
            circuit_validation_service: CircuitValidationService::new(),
            engineering_notebook_service: EngineeringNotebookService::new(),
            netlist_generation_service: NetlistGenerationService::new(netlist_exporter.clone()),
            simulation_service: SimulationService::new(mock_engine.clone()),
            ngspice_simulation_service: NgspiceSimulationService::new(
                mock_engine.clone(),
                ngspice_engine.clone(),
            ),
            simulation_workflow_service: SimulationWorkflowService::new(
                netlist_exporter.clone(),
                NgspiceSimulationService::new(mock_engine.clone(), ngspice_engine.clone()),
            ),
            simulation_diagnostics_service: SimulationDiagnosticsService::new(ngspice_engine),
            simulation_history_service: SimulationHistoryService::new(),
            simulation_graph_service: SimulationGraphService::new(),
            component_model_mapping_service: ComponentModelMappingService::new(),
            export_service: ExportService::new(report_exporter.clone()),
            export_center_service: ExportCenterService::new(
                report_exporter,
                netlist_exporter,
                bom_exporter,
                simulation_data_exporter,
                library_exporter,
                schematic_exporter,
            ),
            component_library_service: ComponentLibraryService::new(component_library_port),
            component_parameter_service: ComponentParameterService::new(),
            schematic_editing_service: SchematicEditingService::new(),
            selected_region_analysis_service: SelectedRegionAnalysisService::new(),
            model_import_service: ModelImportService::new(spice_parser, touchstone_parser),
            dcdc_calculator_service: crate::services::DcdcCalculatorService::new(),
            advanced_report_service: AdvancedReportService::new(),
            project_session_service: ProjectSessionService::new(
                std::env::temp_dir().join("hotsas_session.json"),
                project_package_storage.clone(),
            ),
        }
    }

    pub fn set_project_session_settings_path(&mut self, path: PathBuf) {
        self.project_session_service =
            ProjectSessionService::new(path, self.project_package_service.storage());
    }

    pub fn project_service(&self) -> &ProjectService {
        &self.project_service
    }

    pub fn project_package_service(&self) -> &ProjectPackageService {
        &self.project_package_service
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

    pub fn circuit_validation_service(&self) -> &CircuitValidationService {
        &self.circuit_validation_service
    }

    pub fn engineering_notebook_service(&self) -> &EngineeringNotebookService {
        &self.engineering_notebook_service
    }

    pub fn netlist_generation_service(&self) -> &NetlistGenerationService {
        &self.netlist_generation_service
    }

    pub fn simulation_service(&self) -> &SimulationService {
        &self.simulation_service
    }

    pub fn ngspice_simulation_service(&self) -> &NgspiceSimulationService {
        &self.ngspice_simulation_service
    }

    pub fn export_service(&self) -> &ExportService {
        &self.export_service
    }

    pub fn export_center_service(&self) -> &ExportCenterService {
        &self.export_center_service
    }

    pub fn component_library_service(&self) -> &ComponentLibraryService {
        &self.component_library_service
    }

    pub fn component_parameter_service(&self) -> &ComponentParameterService {
        &self.component_parameter_service
    }

    pub fn schematic_editing_service(&self) -> &SchematicEditingService {
        &self.schematic_editing_service
    }

    pub fn selected_region_analysis_service(&self) -> &SelectedRegionAnalysisService {
        &self.selected_region_analysis_service
    }

    pub fn model_import_service(&self) -> &ModelImportService {
        &self.model_import_service
    }

    pub fn dcdc_calculator_service(&self) -> &DcdcCalculatorService {
        &self.dcdc_calculator_service
    }

    pub fn advanced_report_service(&self) -> &AdvancedReportService {
        &self.advanced_report_service
    }

    pub fn project_session_service(&self) -> &ProjectSessionService {
        &self.project_session_service
    }

    pub fn simulation_workflow_service(&self) -> &SimulationWorkflowService {
        &self.simulation_workflow_service
    }

    pub fn simulation_diagnostics_service(&self) -> &SimulationDiagnosticsService {
        &self.simulation_diagnostics_service
    }

    pub fn simulation_history_service(&self) -> &SimulationHistoryService {
        &self.simulation_history_service
    }

    pub fn simulation_graph_service(&self) -> &SimulationGraphService {
        &self.simulation_graph_service
    }

    pub fn component_model_mapping_service(&self) -> &ComponentModelMappingService {
        &self.component_model_mapping_service
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

    pub fn check_ngspice_availability(
        &self,
    ) -> Result<hotsas_core::NgspiceAvailability, ApplicationError> {
        self.ngspice_simulation_service.check_ngspice_availability()
    }

    pub fn run_simulation(
        &self,
        project: &CircuitProject,
        engine: &str,
        analysis: &str,
    ) -> Result<SimulationResult, ApplicationError> {
        let choice: SimulationEngineChoice = engine
            .parse()
            .map_err(|e: String| ApplicationError::InvalidInput(e))?;
        match analysis {
            "ac_sweep" => self
                .ngspice_simulation_service
                .run_ac_sweep(project, choice),
            "operating_point" => self
                .ngspice_simulation_service
                .run_operating_point(project, choice),
            "transient" => self
                .ngspice_simulation_service
                .run_transient(project, choice),
            other => Err(ApplicationError::InvalidInput(format!(
                "unknown analysis: {other}"
            ))),
        }
    }

    pub fn simulation_history(&self) -> Vec<SimulationResult> {
        self.ngspice_simulation_service.list_simulation_history()
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

    pub fn save_project_package(
        &self,
        package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, ApplicationError> {
        self.project_package_service
            .save_project_package(package_dir, project)
    }

    pub fn load_project_package(
        &self,
        package_dir: &Path,
    ) -> Result<CircuitProject, ApplicationError> {
        self.project_package_service
            .load_project_package(package_dir)
    }

    pub fn validate_project_package(
        &self,
        package_dir: &Path,
    ) -> Result<ProjectPackageValidationReport, ApplicationError> {
        self.project_package_service
            .validate_project_package(package_dir)
    }

    pub fn validate_circuit(
        &self,
        project: &CircuitProject,
    ) -> hotsas_core::CircuitValidationReport {
        self.circuit_validation_service.validate(&project.schematic)
    }
}
