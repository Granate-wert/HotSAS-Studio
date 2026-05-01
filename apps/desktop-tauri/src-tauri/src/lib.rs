use hotsas_adapters::{
    JsonProjectStorage, MarkdownReportExporter, MockSimulationEngine, SimpleFormulaEngine,
    SpiceNetlistExporter,
};
use hotsas_api::{
    ApiError, FormulaResultDto, HotSasApi, PreferredValueDto, ProjectDto, SaveProjectDto,
    SimulationResultDto, VerticalSliceDto,
};
use hotsas_application::AppServices;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
fn create_rc_low_pass_demo_project(api: State<'_, HotSasApi>) -> Result<ProjectDto, String> {
    api.create_rc_low_pass_demo_project().map_err(tauri_error)
}

#[tauri::command]
fn calculate_rc_low_pass(api: State<'_, HotSasApi>) -> Result<FormulaResultDto, String> {
    api.calculate_rc_low_pass().map_err(tauri_error)
}

#[tauri::command]
fn nearest_e24_for_resistor(api: State<'_, HotSasApi>) -> Result<PreferredValueDto, String> {
    api.nearest_e24_for_resistor().map_err(tauri_error)
}

#[tauri::command]
fn nearest_e24(
    api: State<'_, HotSasApi>,
    value: String,
    unit: Option<String>,
) -> Result<PreferredValueDto, String> {
    api.nearest_e24(value, unit).map_err(tauri_error)
}

#[tauri::command]
fn generate_spice_netlist(api: State<'_, HotSasApi>) -> Result<String, String> {
    api.generate_spice_netlist().map_err(tauri_error)
}

#[tauri::command]
fn run_mock_ac_simulation(api: State<'_, HotSasApi>) -> Result<SimulationResultDto, String> {
    api.run_mock_ac_simulation().map_err(tauri_error)
}

#[tauri::command]
fn export_markdown_report(api: State<'_, HotSasApi>) -> Result<String, String> {
    api.export_markdown_report().map_err(tauri_error)
}

#[tauri::command]
fn export_html_report(api: State<'_, HotSasApi>) -> Result<String, String> {
    api.export_html_report().map_err(tauri_error)
}

#[tauri::command]
fn save_project_json(api: State<'_, HotSasApi>, path: String) -> Result<SaveProjectDto, String> {
    api.save_project_json(path).map_err(tauri_error)
}

#[tauri::command]
fn run_vertical_slice_preview(api: State<'_, HotSasApi>) -> Result<VerticalSliceDto, String> {
    api.run_vertical_slice_preview().map_err(tauri_error)
}

fn tauri_error(error: ApiError) -> String {
    serde_json::to_string(&error.to_dto()).unwrap_or_else(|_| error.to_string())
}

fn build_api() -> HotSasApi {
    HotSasApi::new(AppServices::new(
        Arc::new(JsonProjectStorage),
        Arc::new(SimpleFormulaEngine),
        Arc::new(SpiceNetlistExporter),
        Arc::new(MockSimulationEngine),
        Arc::new(MarkdownReportExporter),
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            run_vertical_slice_preview
        ])
        .run(tauri::generate_context!())
        .expect("error while running HotSAS Studio");
}
