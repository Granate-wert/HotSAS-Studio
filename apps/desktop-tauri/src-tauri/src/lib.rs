use hotsas_adapters::{
    CircuitProjectPackageStorage, FormulaPackFileLoader, JsonProjectStorage, MarkdownReportExporter,
    MockSimulationEngine, SimpleFormulaEngine, SpiceNetlistExporter,
};
use hotsas_api::{
    ApiError, CircuitValidationReportDto, FormulaCalculationRequestDto, FormulaDetailsDto,
    FormulaEvaluationResultDto, FormulaPackDto, FormulaResultDto, FormulaSummaryDto, HotSasApi,
    PreferredValueDto, ProjectDto, ProjectPackageManifestDto, ProjectPackageValidationReportDto,
    SaveProjectDto, SelectedComponentDto, SimulationResultDto, VerticalSliceDto,
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
    let result = api.validate_project_package(package_dir).map_err(tauri_error);
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
    log::info!("COMMAND calculate_formula formula_id={}", request.formula_id);
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

fn log_command_result<T>(name: &str, result: &Result<T, String>) {
    match result {
        Ok(_) => log::info!("Command {name} succeeded"),
        Err(e) => log::error!("Command {name} failed: {e}"),
    }
}

fn tauri_error(error: ApiError) -> String {
    serde_json::to_string(&error.to_dto()).unwrap_or_else(|_| error.to_string())
}

fn build_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
        Arc::new(JsonProjectStorage),
        Arc::new(CircuitProjectPackageStorage::default()),
        Arc::new(SimpleFormulaEngine),
        Arc::new(SpiceNetlistExporter),
        Arc::new(MockSimulationEngine),
        Arc::new(MarkdownReportExporter),
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            init_logging(app);
            Ok(())
        })
        .manage(build_api())
        .invoke_handler(tauri::generate_handler![
            create_rc_low_pass_demo_project,
            calculate_rc_low_pass,
            nearest_e24_for_resistor,
            nearest_e24,
            generate_spice_netlist,
            run_mock_ac_simulation,
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
            write_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running HotSAS Studio");
}
