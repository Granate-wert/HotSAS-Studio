import { invoke } from "@tauri-apps/api/core";
import type {
  AdvancedReportDto,
  AdvancedReportExportRequestDto,
  AdvancedReportExportResultDto,
  AdvancedReportRequestDto,
  ApiErrorDto,
  AppDiagnosticsReportDto,
  ApplyNotebookValueRequestDto,
  AssignComponentRequestDto,
  AttachImportedModelRequestDto,
  CircuitValidationReportDto,
  ComponentDetailsDto,
  ComponentLibraryDto,
  ComponentSearchRequestDto,
  ComponentSearchResultDto,
  ComponentSummaryDto,
  ExportCapabilityDto,
  ExportHistoryEntryDto,
  ExportRequestDto,
  ExportResultDto,
  FormulaCalculationRequestDto,
  FormulaDetailsDto,
  FormulaEvaluationResultDto,
  FormulaPackDto,
  FormulaResultDto,
  FormulaSummaryDto,
  ImportedModelDetailsDto,
  ImportedModelSummaryDto,
  NgspiceAvailabilityDto,
  NotebookEvaluationRequestDto,
  NotebookEvaluationResultDto,
  NotebookStateDto,
  PreferredValueDto,
  ProjectDto,
  ProjectPackageManifestDto,
  ProjectPackageValidationReportDto,
  ReportSectionCapabilityDto,
  SaveProjectDto,
  SelectedComponentDto,
  SelectedRegionAnalysisRequestDto,
  SelectedRegionAnalysisResultDto,
  SelectedRegionIssueDto,
  SelectedRegionPreviewDto,
  SimulationResultDto,
  SimulationRunRequestDto,
  SpiceImportReportDto,
  SpiceImportRequestDto,
  SpicePinMappingRequestDto,
  SpicePinMappingValidationReportDto,
  TouchstoneImportReportDto,
  TouchstoneImportRequestDto,
  ProductWorkflowStatusDto,
  VerticalSliceDto,
  DcdcCalculationResultDto,
  DcdcInputDto,
  DcdcNetlistPreviewRequestDto,
  DcdcMockTransientRequestDto,
  DcdcTemplateDto,
  ComponentParameterSchemaDto,
  ComponentParameterIssueDto,
  TypedComponentParametersDto,
} from "../types";

async function invokeCommand<T>(command: string, args?: Record<string, unknown>) {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw new Error(errorMessage(error));
  }
}

function errorMessage(error: unknown) {
  if (typeof error !== "string") {
    return error instanceof Error ? error.message : String(error);
  }

  try {
    const dto = JSON.parse(error) as ApiErrorDto;
    return dto.message || error;
  } catch {
    return error;
  }
}

export const backend = {
  createRcLowPassDemoProject: () => invokeCommand<ProjectDto>("create_rc_low_pass_demo_project"),
  calculateRcLowPass: () => invokeCommand<FormulaResultDto>("calculate_rc_low_pass"),
  nearestE24ForResistor: () => invokeCommand<PreferredValueDto>("nearest_e24_for_resistor"),
  nearestE24: (value: string, unit?: string) =>
    invokeCommand<PreferredValueDto>("nearest_e24", { value, unit }),
  generateSpiceNetlist: () => invokeCommand<string>("generate_spice_netlist"),
  runMockAcSimulation: () => invokeCommand<SimulationResultDto>("run_mock_ac_simulation"),
  checkNgspiceAvailability: () =>
    invokeCommand<NgspiceAvailabilityDto>("check_ngspice_availability"),
  runSimulation: (request: SimulationRunRequestDto) =>
    invokeCommand<SimulationResultDto>("run_simulation", { request }),
  simulationHistory: () => invokeCommand<SimulationResultDto[]>("simulation_history"),
  exportMarkdownReport: () => invokeCommand<string>("export_markdown_report"),
  exportHtmlReport: () => invokeCommand<string>("export_html_report"),
  saveProjectJson: (path: string) => invokeCommand<SaveProjectDto>("save_project_json", { path }),
  saveProjectPackage: (packageDir: string) =>
    invokeCommand<ProjectPackageManifestDto>("save_project_package", { packageDir }),
  loadProjectPackage: (packageDir: string) =>
    invokeCommand<ProjectDto>("load_project_package", { packageDir }),
  validateProjectPackage: (packageDir: string) =>
    invokeCommand<ProjectPackageValidationReportDto>("validate_project_package", { packageDir }),
  runVerticalSlicePreview: () => invokeCommand<VerticalSliceDto>("run_vertical_slice_preview"),
  getSelectedComponent: (instanceId: string) =>
    invokeCommand<SelectedComponentDto>("get_selected_component", { instanceId }),
  updateComponentParameter: (
    instanceId: string,
    parameterName: string,
    value: string,
    unit?: string | null,
  ) =>
    invokeCommand<ProjectDto>("update_component_parameter", {
      instanceId,
      parameterName,
      value,
      unit,
    }),
  validateCurrentCircuit: () =>
    invokeCommand<CircuitValidationReportDto>("validate_current_circuit"),
  loadFormulaPacks: () => invokeCommand<FormulaPackDto[]>("load_formula_packs"),
  listFormulas: () => invokeCommand<FormulaSummaryDto[]>("list_formulas"),
  listFormulaCategories: () => invokeCommand<string[]>("list_formula_categories"),
  getFormula: (id: string) => invokeCommand<FormulaDetailsDto>("get_formula", { id }),
  getFormulaPackMetadata: () => invokeCommand<FormulaPackDto[]>("get_formula_pack_metadata"),
  calculateFormula: (request: FormulaCalculationRequestDto) =>
    invokeCommand<FormulaEvaluationResultDto>("calculate_formula", { request }),
  evaluateNotebookInput: (request: NotebookEvaluationRequestDto) =>
    invokeCommand<NotebookEvaluationResultDto>("evaluate_notebook_input", { request }),
  getNotebookState: () => invokeCommand<NotebookStateDto>("get_notebook_state"),
  clearNotebook: () => invokeCommand<NotebookStateDto>("clear_notebook"),
  applyNotebookOutputToComponent: (request: ApplyNotebookValueRequestDto) =>
    invokeCommand<ProjectDto>("apply_notebook_output_to_component", { request }),
  loadBuiltinComponentLibrary: () =>
    invokeCommand<ComponentLibraryDto>("load_builtin_component_library"),
  listComponents: () => invokeCommand<ComponentSummaryDto[]>("list_components"),
  searchComponents: (request: ComponentSearchRequestDto) =>
    invokeCommand<ComponentSearchResultDto>("search_components", { request }),
  getComponentDetails: (componentId: string) =>
    invokeCommand<ComponentDetailsDto>("get_component_details", { componentId }),
  assignComponentToSelectedInstance: (request: AssignComponentRequestDto) =>
    invokeCommand<ProjectDto>("assign_component_to_selected_instance", { request }),
  previewSelectedRegion: (componentIds: string[]) =>
    invokeCommand<SelectedRegionPreviewDto>("preview_selected_region", { componentIds }),
  analyzeSelectedRegion: (request: SelectedRegionAnalysisRequestDto) =>
    invokeCommand<SelectedRegionAnalysisResultDto>("analyze_selected_region", { request }),
  validateSelectedRegion: (request: SelectedRegionAnalysisRequestDto) =>
    invokeCommand<SelectedRegionIssueDto[]>("validate_selected_region", { request }),
  listExportCapabilities: () => invokeCommand<ExportCapabilityDto[]>("list_export_capabilities"),
  exportFile: (request: ExportRequestDto) =>
    invokeCommand<ExportResultDto>("export_file", { request }),
  exportHistory: () => invokeCommand<ExportHistoryEntryDto[]>("export_history"),
  importSpiceModel: (request: SpiceImportRequestDto) =>
    invokeCommand<SpiceImportReportDto>("import_spice_model", { request }),
  importTouchstoneModel: (request: TouchstoneImportRequestDto) =>
    invokeCommand<TouchstoneImportReportDto>("import_touchstone_model", { request }),
  listImportedModels: () => invokeCommand<ImportedModelSummaryDto[]>("list_imported_models"),
  getImportedModel: (modelId: string) =>
    invokeCommand<ImportedModelDetailsDto>("get_imported_model", { modelId }),
  validateSpicePinMapping: (request: SpicePinMappingRequestDto) =>
    invokeCommand<SpicePinMappingValidationReportDto>("validate_spice_pin_mapping", { request }),
  attachImportedModelToComponent: (request: AttachImportedModelRequestDto) =>
    invokeCommand<ComponentDetailsDto>("attach_imported_model_to_component", { request }),
  getAppDiagnostics: () => invokeCommand<AppDiagnosticsReportDto>("get_app_diagnostics"),
  runReadinessSelfCheck: () => invokeCommand<AppDiagnosticsReportDto>("run_readiness_self_check"),
  getProductWorkflowStatus: () =>
    invokeCommand<ProductWorkflowStatusDto>("get_product_workflow_status"),
  runProductBetaSelfCheck: () =>
    invokeCommand<ProductWorkflowStatusDto>("run_product_beta_self_check"),
  createIntegratedDemoProject: () => invokeCommand<ProjectDto>("create_integrated_demo_project"),
  calculateDcdc: (request: DcdcInputDto) =>
    invokeCommand<DcdcCalculationResultDto>("calculate_dcdc", { request }),
  listDcdcTemplates: () => invokeCommand<DcdcTemplateDto[]>("list_dcdc_templates"),
  generateDcdcNetlistPreview: (request: DcdcNetlistPreviewRequestDto) =>
    invokeCommand<string>("generate_dcdc_netlist_preview", { request }),
  runDcdcMockTransientPreview: (request: DcdcMockTransientRequestDto) =>
    invokeCommand<SimulationResultDto>("run_dcdc_mock_transient_preview", { request }),
  listReportSectionCapabilities: () =>
    invokeCommand<ReportSectionCapabilityDto[]>("list_report_section_capabilities"),
  generateAdvancedReport: (request: AdvancedReportRequestDto) =>
    invokeCommand<AdvancedReportDto>("generate_advanced_report", { request }),
  exportAdvancedReport: (request: AdvancedReportExportRequestDto) =>
    invokeCommand<AdvancedReportExportResultDto>("export_advanced_report", { request }),
  getLastAdvancedReport: () =>
    invokeCommand<AdvancedReportDto | null>("get_last_advanced_report"),
  writeLog: (level: string, message: string) =>
    invokeCommand<void>("write_log", { level, message }),
  // v2.4 typed component parameters
  getComponentParameterSchema: (category: string) =>
    invokeCommand<ComponentParameterSchemaDto | null>("get_component_parameter_schema", { category }),
  validateComponentParameters: (componentId: string) =>
    invokeCommand<ComponentParameterIssueDto[]>("validate_component_parameters", { componentId }),
  getTypedComponentParameters: (componentId: string) =>
    invokeCommand<TypedComponentParametersDto>("get_typed_component_parameters", { componentId }),
};
