use hotsas_adapters::{
    BomCsvExporter, CircuitProjectPackageStorage, ComponentLibraryJsonExporter,
    CsvSimulationDataExporter, FormulaPackFileLoader, JsonComponentLibraryStorage,
    JsonProjectStorage, MarkdownReportExporter, MockSimulationEngine, NgspiceSimulationAdapter,
    SimpleFormulaEngine, SimpleSpiceModelParser, SimpleTouchstoneParser,
    SvgSchematicExporter, UserCircuitSpiceNetlistExporter,
};
use hotsas_api::{
    AddComponentRequestDto, ApiError, AppDiagnosticsReportDto, ApplyNotebookValueRequestDto,
    AssignComponentRequestDto, AssignModelRequestDto, CircuitValidationReportDto,
    ComponentDetailsDto, ComponentLibraryDto, ComponentModelAssignmentDto,
    ComponentParameterIssueDto, ComponentParameterSchemaDto, ComponentSearchRequestDto,
    ComponentSearchResultDto, ComponentSummaryDto, ConnectPinsRequestDto,
    DcdcCalculationResultDto, DcdcInputDto, DcdcMockTransientRequestDto,
    DcdcNetlistPreviewRequestDto, DcdcTemplateDto, DeleteComponentRequestDto, DeleteWireRequestDto,
    ExportCapabilityDto, ExportHistoryEntryDto, ExportRequestDto, ExportResultDto,
    FormulaCalculationRequestDto, FormulaDetailsDto, FormulaEvaluationResultDto, FormulaPackDto,
    FormulaResultDto, FormulaSummaryDto, HotSasApi, MoveComponentRequestDto, NetlistPreviewDto,
    NgspiceAvailabilityDto, NgspiceDiagnosticsDto, NotebookEvaluationRequestDto,
    NotebookEvaluationResultDto, NotebookStateDto, PlaceComponentRequestDto, PlaceableComponentDto,
    PreferredValueDto, ProductWorkflowStatusDto, ProjectDto, ProjectOpenRequestDto,
    ProjectOpenResultDto, ProjectPackageManifestDto, ProjectPackageValidationReportDto,
    ProjectSaveResultDto, ProjectSessionStateDto, RecentProjectEntryDto, RenameNetRequestDto,
    SaveProjectDto, SchematicEditResultDto, SchematicSelectionDetailsDto,
    SchematicSelectionRequestDto, SchematicToolCapabilityDto, SelectedComponentDto,
    SelectedRegionAnalysisRequestDto, SelectedRegionAnalysisResultDto, SelectedRegionIssueDto,
    SelectedRegionPreviewDto, SimulationDiagnosticMessageDto, SimulationGraphViewDto,
    SimulationPreflightResultDto, SimulationProbeDto, SimulationResultDto,
    SimulationRunHistoryEntryDto, SimulationRunRequestDto, SpiceModelReferenceDto,
    TypedComponentParametersDto, UndoRedoStateDto, UpdateQuickParameterRequestDto,
    UserCircuitSimulationProfileDto, UserCircuitSimulationRunDto, VerticalSliceDto,
    CircuitAnalysisPortDto, FilterAnalysisDiagnosticDto, FilterNetworkAnalysisRequestDto,
    FilterNetworkAnalysisResultDto, AnalyzeTouchstoneRequestDto, SParameterAnalysisResultDto,
};
use hotsas_application::{AppServices, ApplicationError};
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::sync::Arc;
use tauri::{Manager, State};

fn init_logging(app: &tauri::App) {
    let log_dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("logs");
    if let Err(error) = std::fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {error}");
    }
    let log_file = log_dir.join("hotsas.log");
    let file_result = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file);

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();

    loggers.push(TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    ));

    loggers.push(WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        file_result.unwrap_or_else(|error| {
            eprintln!("Failed to open log file {log_file:?}: {error}");
            std::fs::File::create(std::env::temp_dir().join("hotsas_fallback.log"))
                .expect("Cannot create fallback log file")
        }),
    ));

    if let Err(error) = CombinedLogger::init(loggers) {
        eprintln!("Failed to initialize logger: {error}");
    } else {
        log::info!("HotSAS Studio starting up. Log file: {log_file:?}");
    }
}

#[tauri::command]
fn preview_selected_region(
    api: State<'_, HotSasApi>,
    component_ids: Vec<String>,
) -> Result<SelectedRegionPreviewDto, String> {
    log::info!(
        "COMMAND preview_selected_region: {} components",
        component_ids.len()
    );
    let result = api
        .preview_selected_region(component_ids)
        .map_err(tauri_error);
    log_command_result("preview_selected_region", &result);
    result
}

#[tauri::command]
fn analyze_selected_region(
    api: State<'_, HotSasApi>,
    request: SelectedRegionAnalysisRequestDto,
) -> Result<SelectedRegionAnalysisResultDto, String> {
    log::info!("COMMAND analyze_selected_region");
    let result = api.analyze_selected_region(request).map_err(tauri_error);
    log_command_result("analyze_selected_region", &result);
    result
}

#[tauri::command]
fn validate_selected_region(
    api: State<'_, HotSasApi>,
    request: SelectedRegionAnalysisRequestDto,
) -> Result<Vec<SelectedRegionIssueDto>, String> {
    log::info!("COMMAND validate_selected_region");
    let result = api.validate_selected_region(request).map_err(tauri_error);
    log_command_result("validate_selected_region", &result);
    result
}

#[tauri::command]
fn list_export_capabilities(api: State<'_, HotSasApi>) -> Result<Vec<ExportCapabilityDto>, String> {
    log::info!("COMMAND list_export_capabilities");
    let result = api.list_export_capabilities().map_err(tauri_error);
    log_command_result("list_export_capabilities", &result);
    result
}

#[tauri::command]
fn export_file(
    api: State<'_, HotSasApi>,
    request: ExportRequestDto,
) -> Result<ExportResultDto, String> {
    log::info!("COMMAND export_file format={}", request.format);
    let result = api.export(request).map_err(tauri_error);
    log_command_result("export_file", &result);
    result
}

#[tauri::command]
fn export_history(api: State<'_, HotSasApi>) -> Result<Vec<ExportHistoryEntryDto>, String> {
    log::info!("COMMAND export_history");
    let result = api.export_history().map_err(tauri_error);
    log_command_result("export_history", &result);
    result
}

#[tauri::command]
fn write_log(level: String, message: String) {
    match level.as_str() {
        "error" => log::error!("[frontend] {message}"),
        "warn" => log::warn!("[frontend] {message}"),
        "info" => log::info!("[frontend] {message}"),
        "debug" => log::debug!("[frontend] {message}"),
        "trace" => log::trace!("[frontend] {message}"),
        _ => log::info!("[frontend] {message}"),
    }
}

#[tauri::command]
fn get_project_session_state(api: State<'_, HotSasApi>) -> Result<ProjectSessionStateDto, String> {
    log::info!("COMMAND get_project_session_state");
    let result = api.get_project_session_state().map_err(tauri_error);
    log_command_result("get_project_session_state", &result);
    result
}

#[tauri::command]
fn save_current_project(api: State<'_, HotSasApi>) -> Result<ProjectSaveResultDto, String> {
    log::info!("COMMAND save_current_project");
    let result = api.save_current_project().map_err(tauri_error);
    log_command_result("save_current_project", &result);
    result
}

#[tauri::command]
fn save_project_as(
    api: State<'_, HotSasApi>,
    path: String,
) -> Result<ProjectSaveResultDto, String> {
    log::info!("COMMAND save_project_as path={path}");
    let result = api.save_project_as(path).map_err(tauri_error);
    log_command_result("save_project_as", &result);
    result
}

#[tauri::command]
fn open_project_package(
    api: State<'_, HotSasApi>,
    request: ProjectOpenRequestDto,
) -> Result<ProjectOpenResultDto, String> {
    log::info!("COMMAND open_project_package path={}", request.path);
    let result = api.open_project_package(request).map_err(tauri_error);
    log_command_result("open_project_package", &result);
    result
}

#[tauri::command]
fn list_recent_projects(api: State<'_, HotSasApi>) -> Result<Vec<RecentProjectEntryDto>, String> {
    log::info!("COMMAND list_recent_projects");
    let result = api.list_recent_projects().map_err(tauri_error);
    log_command_result("list_recent_projects", &result);
    result
}

#[tauri::command]
fn remove_recent_project(api: State<'_, HotSasApi>, path: String) -> Result<(), String> {
    log::info!("COMMAND remove_recent_project path={path}");
    let result = api.remove_recent_project(path).map_err(tauri_error);
    log_command_result("remove_recent_project", &result);
    result
}

#[tauri::command]
fn clear_missing_recent_projects(api: State<'_, HotSasApi>) -> Result<usize, String> {
    log::info!("COMMAND clear_missing_recent_projects");
    let result = api.clear_missing_recent_projects().map_err(tauri_error);
    log_command_result("clear_missing_recent_projects", &result);
    result
}

#[tauri::command]
fn create_rc_low_pass_demo_project(api: State<'_, HotSasApi>) -> Result<ProjectDto, String> {
    log::info!("COMMAND create_rc_low_pass_demo_project");
    let result = api.create_rc_low_pass_demo_project().map_err(tauri_error);
    log_command_result("create_rc_low_pass_demo_project", &result);
    result
}

#[tauri::command]
fn calculate_rc_low_pass(api: State<'_, HotSasApi>) -> Result<FormulaResultDto, String> {
    log::info!("COMMAND calculate_rc_low_pass");
    let result = api.calculate_rc_low_pass().map_err(tauri_error);
    log_command_result("calculate_rc_low_pass", &result);
    result
}

#[tauri::command]
fn nearest_e24_for_resistor(api: State<'_, HotSasApi>) -> Result<PreferredValueDto, String> {
    log::info!("COMMAND nearest_e24_for_resistor");
    let result = api.nearest_e24_for_resistor().map_err(tauri_error);
    log_command_result("nearest_e24_for_resistor", &result);
    result
}

#[tauri::command]
fn nearest_e24(
    api: State<'_, HotSasApi>,
    value: String,
    unit: Option<String>,
) -> Result<PreferredValueDto, String> {
    log::info!("COMMAND nearest_e24 value={value} unit={unit:?}");
    let result = api.nearest_e24(value, unit).map_err(tauri_error);
    log_command_result("nearest_e24", &result);
    result
}

#[tauri::command]
fn generate_spice_netlist(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND generate_spice_netlist");
    let result = api.generate_spice_netlist().map_err(tauri_error);
    log_command_result("generate_spice_netlist", &result);
    result
}

#[tauri::command]
fn run_mock_ac_simulation(api: State<'_, HotSasApi>) -> Result<SimulationResultDto, String> {
    log::info!("COMMAND run_mock_ac_simulation");
    let result = api.run_mock_ac_simulation().map_err(tauri_error);
    log_command_result("run_mock_ac_simulation", &result);
    result
}

#[tauri::command]
fn check_ngspice_availability(api: State<'_, HotSasApi>) -> Result<NgspiceAvailabilityDto, String> {
    log::info!("COMMAND check_ngspice_availability");
    let result = api.check_ngspice_availability().map_err(tauri_error);
    log_command_result("check_ngspice_availability", &result);
    result
}

#[tauri::command]
fn run_simulation(
    api: State<'_, HotSasApi>,
    request: SimulationRunRequestDto,
) -> Result<SimulationResultDto, String> {
    log::info!(
        "COMMAND run_simulation engine={} analysis={}",
        request.engine,
        request.analysis_kind
    );
    let result = api.run_simulation(request).map_err(tauri_error);
    log_command_result("run_simulation", &result);
    result
}

#[tauri::command]
fn simulation_history(api: State<'_, HotSasApi>) -> Result<Vec<SimulationResultDto>, String> {
    log::info!("COMMAND simulation_history");
    let result = api.simulation_history().map_err(tauri_error);
    log_command_result("simulation_history", &result);
    result
}

#[tauri::command]
fn import_spice_model(
    api: State<'_, HotSasApi>,
    request: hotsas_api::SpiceImportRequestDto,
) -> Result<hotsas_api::SpiceImportReportDto, String> {
    log::info!("COMMAND import_spice_model");
    let result = api.import_spice_model(request).map_err(tauri_error);
    log_command_result("import_spice_model", &result);
    result
}

#[tauri::command]
fn import_touchstone_model(
    api: State<'_, HotSasApi>,
    request: hotsas_api::TouchstoneImportRequestDto,
) -> Result<hotsas_api::TouchstoneImportReportDto, String> {
    log::info!("COMMAND import_touchstone_model");
    let result = api.import_touchstone_model(request).map_err(tauri_error);
    log_command_result("import_touchstone_model", &result);
    result
}

#[tauri::command]
fn list_imported_models(
    api: State<'_, HotSasApi>,
) -> Result<Vec<hotsas_api::ImportedModelSummaryDto>, String> {
    log::info!("COMMAND list_imported_models");
    let result = api.list_imported_models().map_err(tauri_error);
    log_command_result("list_imported_models", &result);
    result
}

#[tauri::command]
fn get_imported_model(
    api: State<'_, HotSasApi>,
    model_id: String,
) -> Result<hotsas_api::ImportedModelDetailsDto, String> {
    log::info!("COMMAND get_imported_model");
    let result = api.get_imported_model(model_id).map_err(tauri_error);
    log_command_result("get_imported_model", &result);
    result
}

#[tauri::command]
fn validate_spice_pin_mapping(
    api: State<'_, HotSasApi>,
    request: hotsas_api::SpicePinMappingRequestDto,
) -> Result<hotsas_api::SpicePinMappingValidationReportDto, String> {
    log::info!("COMMAND validate_spice_pin_mapping");
    let result = api.validate_spice_pin_mapping(request).map_err(tauri_error);
    log_command_result("validate_spice_pin_mapping", &result);
    result
}

#[tauri::command]
fn attach_imported_model_to_component(
    api: State<'_, HotSasApi>,
    request: hotsas_api::AttachImportedModelRequestDto,
) -> Result<hotsas_api::ComponentDetailsDto, String> {
    log::info!("COMMAND attach_imported_model_to_component");
    let result = api
        .attach_imported_model_to_component(request)
        .map_err(tauri_error);
    log_command_result("attach_imported_model_to_component", &result);
    result
}

#[tauri::command]
fn export_markdown_report(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND export_markdown_report");
    let result = api.export_markdown_report().map_err(tauri_error);
    log_command_result("export_markdown_report", &result);
    result
}

#[tauri::command]
fn export_html_report(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND export_html_report");
    let result = api.export_html_report().map_err(tauri_error);
    log_command_result("export_html_report", &result);
    result
}

#[tauri::command]
fn save_project_json(api: State<'_, HotSasApi>, path: String) -> Result<SaveProjectDto, String> {
    log::info!("COMMAND save_project_json path={path}");
    let result = api.save_project_json(path).map_err(tauri_error);
    log_command_result("save_project_json", &result);
    result
}

#[tauri::command]
fn save_project_package(
    api: State<'_, HotSasApi>,
    package_dir: String,
) -> Result<ProjectPackageManifestDto, String> {
    log::info!("COMMAND save_project_package package_dir={package_dir}");
    let result = api.save_project_package(package_dir).map_err(tauri_error);
    log_command_result("save_project_package", &result);
    result
}

#[tauri::command]
fn load_project_package(
    api: State<'_, HotSasApi>,
    package_dir: String,
) -> Result<ProjectDto, String> {
    log::info!("COMMAND load_project_package package_dir={package_dir}");
    let result = api.load_project_package(package_dir).map_err(tauri_error);
    log_command_result("load_project_package", &result);
    result
}

#[tauri::command]
fn validate_project_package(
    api: State<'_, HotSasApi>,
    package_dir: String,
) -> Result<ProjectPackageValidationReportDto, String> {
    log::info!("COMMAND validate_project_package package_dir={package_dir}");
    let result = api
        .validate_project_package(package_dir)
        .map_err(tauri_error);
    log_command_result("validate_project_package", &result);
    result
}

#[tauri::command]
fn run_vertical_slice_preview(api: State<'_, HotSasApi>) -> Result<VerticalSliceDto, String> {
    log::info!("COMMAND run_vertical_slice_preview");
    let result = api.run_vertical_slice_preview().map_err(tauri_error);
    log_command_result("run_vertical_slice_preview", &result);
    result
}

#[tauri::command]
fn get_selected_component(
    api: State<'_, HotSasApi>,
    instance_id: String,
) -> Result<SelectedComponentDto, String> {
    log::info!("COMMAND get_selected_component instance_id={instance_id}");
    let result = api.get_selected_component(instance_id).map_err(tauri_error);
    log_command_result("get_selected_component", &result);
    result
}

#[tauri::command]
fn update_component_parameter(
    api: State<'_, HotSasApi>,
    instance_id: String,
    parameter_name: String,
    value: String,
    unit: Option<String>,
) -> Result<ProjectDto, String> {
    log::info!("COMMAND update_component_parameter instance_id={instance_id} parameter={parameter_name} value={value} unit={unit:?}");
    let result = api
        .update_component_parameter(instance_id, parameter_name, value, unit)
        .map_err(tauri_error);
    log_command_result("update_component_parameter", &result);
    result
}

#[tauri::command]
fn validate_current_circuit(
    api: State<'_, HotSasApi>,
) -> Result<CircuitValidationReportDto, String> {
    log::info!("COMMAND validate_current_circuit");
    let result = api.validate_current_circuit().map_err(tauri_error);
    log_command_result("validate_current_circuit", &result);
    result
}

#[tauri::command]
fn load_formula_packs(api: State<'_, HotSasApi>) -> Result<Vec<FormulaPackDto>, String> {
    log::info!("COMMAND load_formula_packs");
    let loader = FormulaPackFileLoader;
    let packs = loader
        .load_builtin_packs()
        .map_err(|error| tauri_error(ApiError::Application(ApplicationError::Port(error))))?;
    log::info!("Loaded {} formula pack(s)", packs.len());
    let result = api.load_formula_packs(packs).map_err(tauri_error);
    log_command_result("load_formula_packs", &result);
    result
}

#[tauri::command]
fn list_formulas(api: State<'_, HotSasApi>) -> Result<Vec<FormulaSummaryDto>, String> {
    log::info!("COMMAND list_formulas");
    let result = api.list_formulas().map_err(tauri_error);
    log_command_result("list_formulas", &result);
    result
}

#[tauri::command]
fn list_formula_categories(api: State<'_, HotSasApi>) -> Result<Vec<String>, String> {
    log::info!("COMMAND list_formula_categories");
    let result = api.list_formula_categories().map_err(tauri_error);
    log_command_result("list_formula_categories", &result);
    result
}

#[tauri::command]
fn get_formula(api: State<'_, HotSasApi>, id: String) -> Result<FormulaDetailsDto, String> {
    log::info!("COMMAND get_formula id={id}");
    let result = api.get_formula(id).map_err(tauri_error);
    log_command_result("get_formula", &result);
    result
}

#[tauri::command]
fn get_formula_pack_metadata(api: State<'_, HotSasApi>) -> Result<Vec<FormulaPackDto>, String> {
    log::info!("COMMAND get_formula_pack_metadata");
    let result = api.get_formula_pack_metadata().map_err(tauri_error);
    log_command_result("get_formula_pack_metadata", &result);
    result
}

#[tauri::command]
fn calculate_formula(
    api: State<'_, HotSasApi>,
    request: FormulaCalculationRequestDto,
) -> Result<FormulaEvaluationResultDto, String> {
    log::info!(
        "COMMAND calculate_formula formula_id={}",
        request.formula_id
    );
    for variable in &request.variables {
        log::info!(
            "  variable {} = {} (unit: {:?})",
            variable.name,
            variable.value,
            variable.unit
        );
    }
    let result = api.calculate_formula(request).map_err(tauri_error);
    log_command_result("calculate_formula", &result);
    result
}

#[tauri::command]
fn evaluate_notebook_input(
    api: State<'_, HotSasApi>,
    request: NotebookEvaluationRequestDto,
) -> Result<NotebookEvaluationResultDto, String> {
    log::info!("COMMAND evaluate_notebook_input input={}", request.input);
    let result = api.evaluate_notebook_input(request).map_err(tauri_error);
    log_command_result("evaluate_notebook_input", &result);
    result
}

#[tauri::command]
fn get_notebook_state(api: State<'_, HotSasApi>) -> Result<NotebookStateDto, String> {
    log::info!("COMMAND get_notebook_state");
    let result = api.get_notebook_state().map_err(tauri_error);
    log_command_result("get_notebook_state", &result);
    result
}

#[tauri::command]
fn clear_notebook(api: State<'_, HotSasApi>) -> Result<NotebookStateDto, String> {
    log::info!("COMMAND clear_notebook");
    let result = api.clear_notebook().map_err(tauri_error);
    log_command_result("clear_notebook", &result);
    result
}

#[tauri::command]
fn apply_notebook_output_to_component(
    api: State<'_, HotSasApi>,
    request: ApplyNotebookValueRequestDto,
) -> Result<ProjectDto, String> {
    log::info!(
        "COMMAND apply_notebook_output_to_component instance_id={} parameter={} output={}",
        request.instance_id,
        request.parameter_name,
        request.output_name
    );
    let result = api
        .apply_notebook_output_to_component(request)
        .map_err(tauri_error);
    log_command_result("apply_notebook_output_to_component", &result);
    result
}

#[tauri::command]
fn load_builtin_component_library(
    api: State<'_, HotSasApi>,
) -> Result<ComponentLibraryDto, String> {
    log::info!("COMMAND load_builtin_component_library");
    let result = api.load_builtin_component_library().map_err(tauri_error);
    log_command_result("load_builtin_component_library", &result);
    result
}

#[tauri::command]
fn list_components(api: State<'_, HotSasApi>) -> Result<Vec<ComponentSummaryDto>, String> {
    log::info!("COMMAND list_components");
    let result = api.list_components().map_err(tauri_error);
    log_command_result("list_components", &result);
    result
}

#[tauri::command]
fn search_components(
    api: State<'_, HotSasApi>,
    request: ComponentSearchRequestDto,
) -> Result<ComponentSearchResultDto, String> {
    log::info!("COMMAND search_components");
    let result = api.search_components(request).map_err(tauri_error);
    log_command_result("search_components", &result);
    result
}

#[tauri::command]
fn get_component_details(
    api: State<'_, HotSasApi>,
    component_id: String,
) -> Result<ComponentDetailsDto, String> {
    log::info!("COMMAND get_component_details component_id={component_id}");
    let result = api.get_component_details(component_id).map_err(tauri_error);
    log_command_result("get_component_details", &result);
    result
}

#[tauri::command]
fn assign_component_to_selected_instance(
    api: State<'_, HotSasApi>,
    request: AssignComponentRequestDto,
) -> Result<ProjectDto, String> {
    log::info!(
        "COMMAND assign_component_to_selected_instance instance_id={} component_definition_id={}",
        request.instance_id,
        request.component_definition_id
    );
    let result = api
        .assign_component_to_selected_instance(request)
        .map_err(tauri_error);
    log_command_result("assign_component_to_selected_instance", &result);
    result
}

#[tauri::command]
async fn get_app_diagnostics(api: State<'_, HotSasApi>) -> Result<AppDiagnosticsReportDto, String> {
    let result = api.get_app_diagnostics().map_err(tauri_error);
    log_command_result("get_app_diagnostics", &result);
    result
}

#[tauri::command]
async fn run_readiness_self_check(
    api: State<'_, HotSasApi>,
) -> Result<AppDiagnosticsReportDto, String> {
    let result = api.run_readiness_self_check().map_err(tauri_error);
    log_command_result("run_readiness_self_check", &result);
    result
}

#[tauri::command]
async fn get_product_workflow_status(
    api: State<'_, HotSasApi>,
) -> Result<ProductWorkflowStatusDto, String> {
    let result = api.get_product_workflow_status().map_err(tauri_error);
    log_command_result("get_product_workflow_status", &result);
    result
}

#[tauri::command]
async fn run_product_beta_self_check(
    api: State<'_, HotSasApi>,
) -> Result<ProductWorkflowStatusDto, String> {
    let result = api.run_product_beta_self_check().map_err(tauri_error);
    log_command_result("run_product_beta_self_check", &result);
    result
}

#[tauri::command]
async fn create_integrated_demo_project(api: State<'_, HotSasApi>) -> Result<ProjectDto, String> {
    let result = api.create_integrated_demo_project().map_err(tauri_error);
    log_command_result("create_integrated_demo_project", &result);
    result
}

#[tauri::command]
fn get_component_parameter_schema(
    api: State<'_, HotSasApi>,
    category: String,
) -> Result<Option<ComponentParameterSchemaDto>, String> {
    log::info!("COMMAND get_component_parameter_schema category={category}");
    let result = api
        .get_component_parameter_schema(category)
        .map_err(tauri_error);
    log_command_result("get_component_parameter_schema", &result);
    result
}

#[tauri::command]
fn validate_component_parameters(
    api: State<'_, HotSasApi>,
    component_id: String,
) -> Result<Vec<ComponentParameterIssueDto>, String> {
    log::info!("COMMAND validate_component_parameters component_id={component_id}");
    let result = api
        .validate_component_parameters(component_id)
        .map_err(tauri_error);
    log_command_result("validate_component_parameters", &result);
    result
}

#[tauri::command]
fn get_typed_component_parameters(
    api: State<'_, HotSasApi>,
    component_id: String,
) -> Result<TypedComponentParametersDto, String> {
    log::info!("COMMAND get_typed_component_parameters component_id={component_id}");
    let result = api
        .get_typed_component_parameters(component_id)
        .map_err(tauri_error);
    log_command_result("get_typed_component_parameters", &result);
    result
}

#[tauri::command]
fn list_schematic_editor_capabilities(
    api: State<'_, HotSasApi>,
) -> Result<Vec<SchematicToolCapabilityDto>, String> {
    log::info!("COMMAND list_schematic_editor_capabilities");
    let result = api
        .list_schematic_editor_capabilities()
        .map_err(tauri_error);
    log_command_result("list_schematic_editor_capabilities", &result);
    result
}

#[tauri::command]
fn add_schematic_component(
    api: State<'_, HotSasApi>,
    request: AddComponentRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!(
        "COMMAND add_schematic_component kind={}",
        request.component_kind
    );
    let result = api.add_schematic_component(request).map_err(tauri_error);
    log_command_result("add_schematic_component", &result);
    result
}

#[tauri::command]
fn move_schematic_component(
    api: State<'_, HotSasApi>,
    request: MoveComponentRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!(
        "COMMAND move_schematic_component instance_id={}",
        request.instance_id
    );
    let result = api.move_schematic_component(request).map_err(tauri_error);
    log_command_result("move_schematic_component", &result);
    result
}

#[tauri::command]
fn delete_schematic_component(
    api: State<'_, HotSasApi>,
    request: DeleteComponentRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!(
        "COMMAND delete_schematic_component instance_id={}",
        request.instance_id
    );
    let result = api.delete_schematic_component(request).map_err(tauri_error);
    log_command_result("delete_schematic_component", &result);
    result
}

#[tauri::command]
fn connect_schematic_pins(
    api: State<'_, HotSasApi>,
    request: ConnectPinsRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!("COMMAND connect_schematic_pins");
    let result = api.connect_schematic_pins(request).map_err(tauri_error);
    log_command_result("connect_schematic_pins", &result);
    result
}

#[tauri::command]
fn rename_schematic_net(
    api: State<'_, HotSasApi>,
    request: RenameNetRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!("COMMAND rename_schematic_net net_id={}", request.net_id);
    let result = api.rename_schematic_net(request).map_err(tauri_error);
    log_command_result("rename_schematic_net", &result);
    result
}

#[tauri::command]
fn list_placeable_components(
    api: State<'_, HotSasApi>,
) -> Result<Vec<PlaceableComponentDto>, String> {
    log::info!("COMMAND list_placeable_components");
    let result = api.list_placeable_components().map_err(tauri_error);
    log_command_result("list_placeable_components", &result);
    result
}

#[tauri::command]
fn place_schematic_component(
    api: State<'_, HotSasApi>,
    request: PlaceComponentRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!(
        "COMMAND place_schematic_component definition_id={}",
        request.component_definition_id
    );
    let result = api.place_schematic_component(request).map_err(tauri_error);
    log_command_result("place_schematic_component", &result);
    result
}

#[tauri::command]
fn delete_schematic_wire(
    api: State<'_, HotSasApi>,
    request: DeleteWireRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!("COMMAND delete_schematic_wire wire_id={}", request.wire_id);
    let result = api.delete_schematic_wire(request).map_err(tauri_error);
    log_command_result("delete_schematic_wire", &result);
    result
}

#[tauri::command]
fn update_schematic_quick_parameter(
    api: State<'_, HotSasApi>,
    request: UpdateQuickParameterRequestDto,
) -> Result<SchematicEditResultDto, String> {
    log::info!(
        "COMMAND update_schematic_quick_parameter component_id={} parameter={}",
        request.component_id,
        request.parameter_id
    );
    let result = api
        .update_schematic_quick_parameter(request)
        .map_err(tauri_error);
    log_command_result("update_schematic_quick_parameter", &result);
    result
}

#[tauri::command]
fn get_schematic_selection_details(
    api: State<'_, HotSasApi>,
    request: SchematicSelectionRequestDto,
) -> Result<SchematicSelectionDetailsDto, String> {
    log::info!(
        "COMMAND get_schematic_selection_details kind={} id={}",
        request.kind,
        request.id
    );
    let result = api
        .get_schematic_selection_details(request)
        .map_err(tauri_error);
    log_command_result("get_schematic_selection_details", &result);
    result
}

#[tauri::command]
fn undo_schematic_edit(api: State<'_, HotSasApi>) -> Result<ProjectDto, String> {
    log::info!("COMMAND undo_schematic_edit");
    let result = api.undo_schematic_edit().map_err(tauri_error);
    log_command_result("undo_schematic_edit", &result);
    result
}

#[tauri::command]
fn redo_schematic_edit(api: State<'_, HotSasApi>) -> Result<ProjectDto, String> {
    log::info!("COMMAND redo_schematic_edit");
    let result = api.redo_schematic_edit().map_err(tauri_error);
    log_command_result("redo_schematic_edit", &result);
    result
}

#[tauri::command]
fn get_schematic_undo_redo_state(api: State<'_, HotSasApi>) -> Result<UndoRedoStateDto, String> {
    log::info!("COMMAND get_schematic_undo_redo_state");
    let result = api.get_schematic_undo_redo_state().map_err(tauri_error);
    log_command_result("get_schematic_undo_redo_state", &result);
    result
}

#[tauri::command]
fn generate_current_schematic_netlist_preview(
    api: State<'_, HotSasApi>,
) -> Result<NetlistPreviewDto, String> {
    log::info!("COMMAND generate_current_schematic_netlist_preview");
    let result = api
        .generate_current_schematic_netlist_preview()
        .map_err(tauri_error);
    log_command_result("generate_current_schematic_netlist_preview", &result);
    result
}

fn log_command_result<T>(name: &str, result: &Result<T, String>) {
    match result {
        Ok(_) => log::info!("Command {name} succeeded"),
        Err(e) => log::error!("Command {name} failed: {e}"),
    }
}

fn tauri_error(error: ApiError) -> String {
    serde_json::to_string(&error.to_dto()).unwrap_or_else(|_| error.to_string())
}

#[tauri::command]
fn calculate_dcdc(
    api: State<'_, HotSasApi>,
    request: DcdcInputDto,
) -> Result<DcdcCalculationResultDto, String> {
    log::info!("COMMAND calculate_dcdc: topology={}", request.topology);
    let result = api.calculate_dcdc(request).map_err(tauri_error);
    log_command_result("calculate_dcdc", &result);
    result
}

#[tauri::command]
fn list_dcdc_templates(api: State<'_, HotSasApi>) -> Result<Vec<DcdcTemplateDto>, String> {
    log::info!("COMMAND list_dcdc_templates");
    let result = api.list_dcdc_templates().map_err(tauri_error);
    log_command_result("list_dcdc_templates", &result);
    result
}

#[tauri::command]
fn generate_dcdc_netlist_preview(
    api: State<'_, HotSasApi>,
    request: DcdcNetlistPreviewRequestDto,
) -> Result<String, String> {
    log::info!("COMMAND generate_dcdc_netlist_preview");
    let result = api
        .generate_dcdc_netlist_preview(request)
        .map_err(tauri_error);
    log_command_result("generate_dcdc_netlist_preview", &result);
    result
}

#[tauri::command]
fn run_dcdc_mock_transient_preview(
    api: State<'_, HotSasApi>,
    request: DcdcMockTransientRequestDto,
) -> Result<SimulationResultDto, String> {
    log::info!("COMMAND run_dcdc_mock_transient_preview");
    let result = api
        .run_dcdc_mock_transient_preview(request)
        .map_err(tauri_error);
    log_command_result("run_dcdc_mock_transient_preview", &result);
    result
}

#[tauri::command]
fn list_report_section_capabilities(
    api: State<'_, HotSasApi>,
) -> Result<Vec<hotsas_api::ReportSectionCapabilityDto>, String> {
    log::info!("COMMAND list_report_section_capabilities");
    let result = api.list_report_section_capabilities().map_err(tauri_error);
    log_command_result("list_report_section_capabilities", &result);
    result
}

#[tauri::command]
fn generate_advanced_report(
    api: State<'_, HotSasApi>,
    request: hotsas_api::AdvancedReportRequestDto,
) -> Result<hotsas_api::AdvancedReportDto, String> {
    log::info!("COMMAND generate_advanced_report");
    let result = api.generate_advanced_report(request).map_err(tauri_error);
    log_command_result("generate_advanced_report", &result);
    result
}

#[tauri::command]
fn export_advanced_report(
    api: State<'_, HotSasApi>,
    request: hotsas_api::AdvancedReportExportRequestDto,
) -> Result<hotsas_api::AdvancedReportExportResultDto, String> {
    log::info!("COMMAND export_advanced_report");
    let result = api.export_advanced_report(request).map_err(tauri_error);
    log_command_result("export_advanced_report", &result);
    result
}

#[tauri::command]
fn get_last_advanced_report(
    api: State<'_, HotSasApi>,
) -> Result<Option<hotsas_api::AdvancedReportDto>, String> {
    log::info!("COMMAND get_last_advanced_report");
    let result = api.get_last_advanced_report().map_err(tauri_error);
    log_command_result("get_last_advanced_report", &result);
    result
}

#[tauri::command]
fn list_user_circuit_simulation_profiles(
    api: State<'_, HotSasApi>,
) -> Result<Vec<UserCircuitSimulationProfileDto>, String> {
    log::info!("COMMAND list_user_circuit_simulation_profiles");
    let result = api.list_user_circuit_simulation_profiles().map_err(tauri_error);
    log_command_result("list_user_circuit_simulation_profiles", &result);
    result
}

#[tauri::command]
fn suggest_user_circuit_simulation_probes(
    api: State<'_, HotSasApi>,
) -> Result<Vec<SimulationProbeDto>, String> {
    log::info!("COMMAND suggest_user_circuit_simulation_probes");
    let result = api.suggest_user_circuit_simulation_probes().map_err(tauri_error);
    log_command_result("suggest_user_circuit_simulation_probes", &result);
    result
}

#[tauri::command]
fn validate_current_circuit_for_simulation(
    api: State<'_, HotSasApi>,
    profile: UserCircuitSimulationProfileDto,
) -> Result<SimulationPreflightResultDto, String> {
    log::info!("COMMAND validate_current_circuit_for_simulation");
    let result = api.validate_current_circuit_for_simulation(profile).map_err(tauri_error);
    log_command_result("validate_current_circuit_for_simulation", &result);
    result
}

#[tauri::command]
fn run_current_circuit_simulation(
    api: State<'_, HotSasApi>,
    profile: UserCircuitSimulationProfileDto,
) -> Result<UserCircuitSimulationRunDto, String> {
    log::info!("COMMAND run_current_circuit_simulation");
    let result = api.run_current_circuit_simulation(profile).map_err(tauri_error);
    log_command_result("run_current_circuit_simulation", &result);
    result
}

#[tauri::command]
fn get_last_user_circuit_simulation(
    api: State<'_, HotSasApi>,
) -> Result<Option<UserCircuitSimulationRunDto>, String> {
    log::info!("COMMAND get_last_user_circuit_simulation");
    let result = api.get_last_user_circuit_simulation().map_err(tauri_error);
    log_command_result("get_last_user_circuit_simulation", &result);
    result
}

#[tauri::command]
fn clear_last_user_circuit_simulation(api: State<'_, HotSasApi>) -> Result<(), String> {
    log::info!("COMMAND clear_last_user_circuit_simulation");
    let result = api.clear_last_user_circuit_simulation().map_err(tauri_error);
    log_command_result("clear_last_user_circuit_simulation", &result);
    result
}

#[tauri::command]
fn add_last_simulation_to_advanced_report(api: State<'_, HotSasApi>) -> Result<ProjectDto, String> {
    log::info!("COMMAND add_last_simulation_to_advanced_report");
    let result = api.add_last_simulation_to_advanced_report().map_err(tauri_error);
    log_command_result("add_last_simulation_to_advanced_report", &result);
    result
}

#[tauri::command]
fn check_ngspice_diagnostics(api: State<'_, HotSasApi>) -> Result<NgspiceDiagnosticsDto, String> {
    log::info!("COMMAND check_ngspice_diagnostics");
    let result = api.check_ngspice_diagnostics().map_err(tauri_error);
    log_command_result("check_ngspice_diagnostics", &result);
    result
}

#[tauri::command]
fn diagnose_simulation_preflight(
    api: State<'_, HotSasApi>,
    profile: UserCircuitSimulationProfileDto,
) -> Result<Vec<SimulationDiagnosticMessageDto>, String> {
    log::info!("COMMAND diagnose_simulation_preflight");
    let result = api.diagnose_simulation_preflight(profile).map_err(tauri_error);
    log_command_result("diagnose_simulation_preflight", &result);
    result
}

#[tauri::command]
fn diagnose_last_simulation_run(
    api: State<'_, HotSasApi>,
) -> Result<Vec<SimulationDiagnosticMessageDto>, String> {
    log::info!("COMMAND diagnose_last_simulation_run");
    let result = api.diagnose_last_simulation_run().map_err(tauri_error);
    log_command_result("diagnose_last_simulation_run", &result);
    result
}

#[tauri::command]
fn add_run_to_history(api: State<'_, HotSasApi>) -> Result<(), String> {
    log::info!("COMMAND add_run_to_history");
    let result = api.add_run_to_history().map_err(tauri_error);
    log_command_result("add_run_to_history", &result);
    result
}

#[tauri::command]
fn list_simulation_history(
    api: State<'_, HotSasApi>,
) -> Result<Vec<SimulationRunHistoryEntryDto>, String> {
    log::info!("COMMAND list_simulation_history");
    let result = api.list_simulation_history().map_err(tauri_error);
    log_command_result("list_simulation_history", &result);
    result
}

#[tauri::command]
fn delete_simulation_history_run(
    api: State<'_, HotSasApi>,
    run_id: String,
) -> Result<(), String> {
    log::info!("COMMAND delete_simulation_history_run run_id={run_id}");
    let result = api.delete_simulation_history_run(run_id).map_err(tauri_error);
    log_command_result("delete_simulation_history_run", &result);
    result
}

#[tauri::command]
fn clear_simulation_history(api: State<'_, HotSasApi>) -> Result<(), String> {
    log::info!("COMMAND clear_simulation_history");
    let result = api.clear_simulation_history().map_err(tauri_error);
    log_command_result("clear_simulation_history", &result);
    result
}

#[tauri::command]
fn build_simulation_graph_view(
    api: State<'_, HotSasApi>,
) -> Result<SimulationGraphViewDto, String> {
    log::info!("COMMAND build_simulation_graph_view");
    let result = api.build_simulation_graph_view().map_err(tauri_error);
    log_command_result("build_simulation_graph_view", &result);
    result
}

#[tauri::command]
fn export_run_series_csv(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND export_run_series_csv");
    let result = api.export_run_series_csv().map_err(tauri_error);
    log_command_result("export_run_series_csv", &result);
    result
}

#[tauri::command]
fn export_run_series_json(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND export_run_series_json");
    let result = api.export_run_series_json().map_err(tauri_error);
    log_command_result("export_run_series_json", &result);
    result
}

// ─── v3.1 Component Model Mapping Commands ───

#[tauri::command]
fn list_available_models_for_component(
    api: State<'_, HotSasApi>,
    definition_id: String,
) -> Result<Vec<SpiceModelReferenceDto>, String> {
    log::info!("COMMAND list_available_models_for_component: {definition_id}");
    let result = api
        .list_available_models_for_component(definition_id)
        .map_err(tauri_error);
    log_command_result("list_available_models_for_component", &result);
    result
}

#[tauri::command]
fn get_component_model_assignment(
    api: State<'_, HotSasApi>,
    instance_id: String,
) -> Result<ComponentModelAssignmentDto, String> {
    log::info!("COMMAND get_component_model_assignment: {instance_id}");
    let result = api
        .get_component_model_assignment(instance_id)
        .map_err(tauri_error);
    log_command_result("get_component_model_assignment", &result);
    result
}

#[tauri::command]
fn assign_model_to_instance(
    api: State<'_, HotSasApi>,
    request: AssignModelRequestDto,
) -> Result<ComponentModelAssignmentDto, String> {
    log::info!("COMMAND assign_model_to_instance: {} -> {}", request.instance_id, request.model_id);
    let result = api.assign_model_to_instance(request).map_err(tauri_error);
    log_command_result("assign_model_to_instance", &result);
    result
}

#[tauri::command]
fn evaluate_project_simulation_readiness(
    api: State<'_, HotSasApi>,
) -> Result<hotsas_api::ProjectSimulationReadinessDto, String> {
    log::info!("COMMAND evaluate_project_simulation_readiness");
    let result = api.evaluate_project_simulation_readiness().map_err(tauri_error);
    log_command_result("evaluate_project_simulation_readiness", &result);
    result
}

#[tauri::command]
fn suggest_filter_analysis_ports(
    api: State<'_, HotSasApi>,
    selected_component_ids: Vec<String>,
) -> Result<Vec<CircuitAnalysisPortDto>, String> {
    log::info!("COMMAND suggest_filter_analysis_ports");
    let result = api
        .suggest_filter_analysis_ports(selected_component_ids)
        .map_err(tauri_error);
    log_command_result("suggest_filter_analysis_ports", &result);
    result
}

#[tauri::command]
fn validate_filter_network_analysis_request(
    api: State<'_, HotSasApi>,
    request: FilterNetworkAnalysisRequestDto,
) -> Result<Vec<FilterAnalysisDiagnosticDto>, String> {
    log::info!("COMMAND validate_filter_network_analysis_request");
    let result = api
        .validate_filter_network_analysis_request(request)
        .map_err(tauri_error);
    log_command_result("validate_filter_network_analysis_request", &result);
    result
}

#[tauri::command]
fn run_filter_network_analysis(
    api: State<'_, HotSasApi>,
    request: FilterNetworkAnalysisRequestDto,
) -> Result<FilterNetworkAnalysisResultDto, String> {
    log::info!("COMMAND run_filter_network_analysis");
    let result = api.run_filter_network_analysis(request).map_err(tauri_error);
    log_command_result("run_filter_network_analysis", &result);
    result
}

#[tauri::command]
fn get_last_filter_network_analysis(
    api: State<'_, HotSasApi>,
) -> Result<Option<FilterNetworkAnalysisResultDto>, String> {
    log::info!("COMMAND get_last_filter_network_analysis");
    let result = api.get_last_filter_network_analysis().map_err(tauri_error);
    log_command_result("get_last_filter_network_analysis", &result);
    result
}

#[tauri::command]
fn clear_last_filter_network_analysis(api: State<'_, HotSasApi>) -> Result<(), String> {
    log::info!("COMMAND clear_last_filter_network_analysis");
    let result = api.clear_last_filter_network_analysis().map_err(tauri_error);
    log_command_result("clear_last_filter_network_analysis", &result);
    result
}

#[tauri::command]
fn export_filter_network_analysis_csv(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND export_filter_network_analysis_csv");
    let result = api.export_filter_network_analysis_csv().map_err(tauri_error);
    log_command_result("export_filter_network_analysis_csv", &result);
    result
}

#[tauri::command]
fn add_filter_network_analysis_to_advanced_report(
    api: State<'_, HotSasApi>,
) -> Result<hotsas_api::AdvancedReportDto, String> {
    log::info!("COMMAND add_filter_network_analysis_to_advanced_report");
    let result = api
        .add_filter_network_analysis_to_advanced_report()
        .map_err(tauri_error);
    log_command_result("add_filter_network_analysis_to_advanced_report", &result);
    result
}

#[tauri::command]
fn analyze_touchstone_s_parameters(
    api: State<'_, HotSasApi>,
    request: AnalyzeTouchstoneRequestDto,
) -> Result<SParameterAnalysisResultDto, String> {
    log::info!("COMMAND analyze_touchstone_s_parameters");
    let result = api
        .analyze_touchstone_s_parameters(request)
        .map_err(tauri_error);
    log_command_result("analyze_touchstone_s_parameters", &result);
    result
}

#[tauri::command]
fn export_s_parameter_csv(api: State<'_, HotSasApi>) -> Result<String, String> {
    log::info!("COMMAND export_s_parameter_csv");
    let result = api.export_s_parameter_csv().map_err(tauri_error);
    log_command_result("export_s_parameter_csv", &result);
    result
}

#[tauri::command]
fn add_s_parameter_analysis_to_advanced_report(
    api: State<'_, HotSasApi>,
) -> Result<hotsas_api::AdvancedReportDto, String> {
    log::info!("COMMAND add_s_parameter_analysis_to_advanced_report");
    let result = api
        .add_s_parameter_analysis_to_advanced_report()
        .map_err(tauri_error);
    log_command_result("add_s_parameter_analysis_to_advanced_report", &result);
    result
}

#[tauri::command]
fn get_last_s_parameter_analysis(
    api: State<'_, HotSasApi>,
) -> Result<Option<SParameterAnalysisResultDto>, String> {
    log::info!("COMMAND get_last_s_parameter_analysis");
    let result = api.get_last_s_parameter_analysis().map_err(tauri_error);
    log_command_result("get_last_s_parameter_analysis", &result);
    result
}

#[tauri::command]
fn clear_last_s_parameter_analysis(api: State<'_, HotSasApi>) -> Result<(), String> {
    log::info!("COMMAND clear_last_s_parameter_analysis");
    let result = api.clear_last_s_parameter_analysis().map_err(tauri_error);
    log_command_result("clear_last_s_parameter_analysis", &result);
    result
}

fn build_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
        Arc::new(JsonProjectStorage),
        Arc::new(CircuitProjectPackageStorage::default()),
        Arc::new(SimpleFormulaEngine),
        Arc::new(UserCircuitSpiceNetlistExporter),
        Arc::new(MockSimulationEngine),
        Arc::new(NgspiceSimulationAdapter::new()),
        Arc::new(MarkdownReportExporter),
        Arc::new(JsonComponentLibraryStorage),
        Arc::new(BomCsvExporter),
        Arc::new(CsvSimulationDataExporter),
        Arc::new(ComponentLibraryJsonExporter),
        Arc::new(SvgSchematicExporter),
        Arc::new(SimpleSpiceModelParser::new()),
        Arc::new(SimpleTouchstoneParser::new()),
    ))
}

fn build_api_with_app_data_dir(app: &tauri::App) -> HotSasApi {
    let mut api = build_api();
    let app_data_dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir());
    api.services_mut()
        .set_project_session_settings_path(app_data_dir.join("hotsas_session.json"));
    api
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            init_logging(app);
            let api = build_api_with_app_data_dir(app);
            app.manage(api);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_project_session_state,
            save_current_project,
            save_project_as,
            open_project_package,
            list_recent_projects,
            remove_recent_project,
            clear_missing_recent_projects,
            create_rc_low_pass_demo_project,
            calculate_rc_low_pass,
            nearest_e24_for_resistor,
            nearest_e24,
            generate_spice_netlist,
            run_mock_ac_simulation,
            check_ngspice_availability,
            run_simulation,
            simulation_history,
            import_spice_model,
            import_touchstone_model,
            list_imported_models,
            get_imported_model,
            validate_spice_pin_mapping,
            attach_imported_model_to_component,
            export_markdown_report,
            export_html_report,
            save_project_json,
            save_project_package,
            load_project_package,
            validate_project_package,
            run_vertical_slice_preview,
            get_selected_component,
            update_component_parameter,
            validate_current_circuit,
            load_formula_packs,
            list_formulas,
            list_formula_categories,
            get_formula,
            get_formula_pack_metadata,
            calculate_formula,
            evaluate_notebook_input,
            get_notebook_state,
            clear_notebook,
            apply_notebook_output_to_component,
            load_builtin_component_library,
            list_components,
            search_components,
            get_component_details,
            assign_component_to_selected_instance,
            get_component_parameter_schema,
            validate_component_parameters,
            get_typed_component_parameters,
            list_schematic_editor_capabilities,
            add_schematic_component,
            move_schematic_component,
            delete_schematic_component,
            connect_schematic_pins,
            rename_schematic_net,
            list_placeable_components,
            place_schematic_component,
            delete_schematic_wire,
            update_schematic_quick_parameter,
            get_schematic_selection_details,
            undo_schematic_edit,
            redo_schematic_edit,
            get_schematic_undo_redo_state,
            generate_current_schematic_netlist_preview,
            preview_selected_region,
            analyze_selected_region,
            validate_selected_region,
            list_export_capabilities,
            export_file,
            export_history,
            get_app_diagnostics,
            run_readiness_self_check,
            get_product_workflow_status,
            run_product_beta_self_check,
            create_integrated_demo_project,
            calculate_dcdc,
            list_dcdc_templates,
            generate_dcdc_netlist_preview,
            run_dcdc_mock_transient_preview,
            list_report_section_capabilities,
            generate_advanced_report,
            export_advanced_report,
            get_last_advanced_report,
            list_user_circuit_simulation_profiles,
            suggest_user_circuit_simulation_probes,
            validate_current_circuit_for_simulation,
            run_current_circuit_simulation,
            get_last_user_circuit_simulation,
            clear_last_user_circuit_simulation,
            add_last_simulation_to_advanced_report,
            check_ngspice_diagnostics,
            diagnose_simulation_preflight,
            diagnose_last_simulation_run,
            add_run_to_history,
            list_simulation_history,
            delete_simulation_history_run,
            clear_simulation_history,
            build_simulation_graph_view,
            export_run_series_csv,
            export_run_series_json,
            list_available_models_for_component,
            get_component_model_assignment,
            assign_model_to_instance,
            evaluate_project_simulation_readiness,
            suggest_filter_analysis_ports,
            validate_filter_network_analysis_request,
            run_filter_network_analysis,
            get_last_filter_network_analysis,
            clear_last_filter_network_analysis,
            export_filter_network_analysis_csv,
            add_filter_network_analysis_to_advanced_report,
            analyze_touchstone_s_parameters,
            export_s_parameter_csv,
            add_s_parameter_analysis_to_advanced_report,
            get_last_s_parameter_analysis,
            clear_last_s_parameter_analysis,
            write_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running HotSAS Studio");
}
