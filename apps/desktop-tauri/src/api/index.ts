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
  DeleteWireRequestDto,
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
  NetlistPreviewDto,
  NgspiceAvailabilityDto,
  NotebookEvaluationRequestDto,
  NotebookEvaluationResultDto,
  NotebookStateDto,
  PlaceableComponentDto,
  PlaceComponentRequestDto,
  PreferredValueDto,
  ProjectDto,
  ProjectPackageManifestDto,
  ProjectPackageValidationReportDto,
  ReportSectionCapabilityDto,
  SaveProjectDto,
  SchematicEditResultDto,
  SchematicSelectionDetailsDto,
  SchematicSelectionRequestDto,
  SchematicToolCapabilityDto,
  SelectedComponentDto,
  SelectedRegionAnalysisRequestDto,
  SelectedRegionAnalysisResultDto,
  SelectedRegionIssueDto,
  SelectedRegionPreviewDto,
  SimulationPreflightResultDto,
  SimulationProbeDto,
  SimulationResultDto,
  SimulationRunRequestDto,
  SpiceImportReportDto,
  UserCircuitSimulationProfileDto,
  UserCircuitSimulationRunDto,
  SpiceImportRequestDto,
  SpicePinMappingRequestDto,
  SpicePinMappingValidationReportDto,
  TouchstoneImportReportDto,
  TouchstoneImportRequestDto,
  UndoRedoStateDto,
  UpdateQuickParameterRequestDto,
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
  AddComponentRequestDto,
  MoveComponentRequestDto,
  DeleteComponentRequestDto,
  ConnectPinsRequestDto,
  RenameNetRequestDto,
  ProjectSessionStateDto,
  ProjectSaveResultDto,
  ProjectOpenRequestDto,
  ProjectOpenResultDto,
  RecentProjectEntryDto,
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
  getLastAdvancedReport: () => invokeCommand<AdvancedReportDto | null>("get_last_advanced_report"),
  writeLog: (level: string, message: string) =>
    invokeCommand<void>("write_log", { level, message }),
  // v2.4 typed component parameters
  getComponentParameterSchema: (category: string) =>
    invokeCommand<ComponentParameterSchemaDto | null>("get_component_parameter_schema", {
      category,
    }),
  validateComponentParameters: (componentId: string) =>
    invokeCommand<ComponentParameterIssueDto[]>("validate_component_parameters", { componentId }),
  getTypedComponentParameters: (componentId: string) =>
    invokeCommand<TypedComponentParametersDto>("get_typed_component_parameters", { componentId }),
  // v2.5 schematic editor hardening
  listSchematicEditorCapabilities: () =>
    invokeCommand<SchematicToolCapabilityDto[]>("list_schematic_editor_capabilities"),
  addSchematicComponent: (request: AddComponentRequestDto) =>
    invokeCommand<SchematicEditResultDto>("add_schematic_component", { request }),
  moveSchematicComponent: (request: MoveComponentRequestDto) =>
    invokeCommand<SchematicEditResultDto>("move_schematic_component", { request }),
  deleteSchematicComponent: (request: DeleteComponentRequestDto) =>
    invokeCommand<SchematicEditResultDto>("delete_schematic_component", { request }),
  connectSchematicPins: (request: ConnectPinsRequestDto) =>
    invokeCommand<SchematicEditResultDto>("connect_schematic_pins", { request }),
  renameSchematicNet: (request: RenameNetRequestDto) =>
    invokeCommand<SchematicEditResultDto>("rename_schematic_net", { request }),
  // v2.8 interactive schematic editing
  listPlaceableComponents: () =>
    invokeCommand<PlaceableComponentDto[]>("list_placeable_components"),
  placeSchematicComponent: (request: PlaceComponentRequestDto) =>
    invokeCommand<SchematicEditResultDto>("place_schematic_component", { request }),
  deleteSchematicWire: (request: DeleteWireRequestDto) =>
    invokeCommand<SchematicEditResultDto>("delete_schematic_wire", { request }),
  updateSchematicQuickParameter: (request: UpdateQuickParameterRequestDto) =>
    invokeCommand<SchematicEditResultDto>("update_schematic_quick_parameter", { request }),
  getSchematicSelectionDetails: (request: SchematicSelectionRequestDto) =>
    invokeCommand<SchematicSelectionDetailsDto>("get_schematic_selection_details", { request }),
  undoSchematicEdit: () => invokeCommand<ProjectDto>("undo_schematic_edit"),
  redoSchematicEdit: () => invokeCommand<ProjectDto>("redo_schematic_edit"),
  getSchematicUndoRedoState: () => invokeCommand<UndoRedoStateDto>("get_schematic_undo_redo_state"),
  generateCurrentSchematicNetlistPreview: () =>
    invokeCommand<NetlistPreviewDto>("generate_current_schematic_netlist_preview"),
  // v2.6 project persistence
  getProjectSessionState: () => invokeCommand<ProjectSessionStateDto>("get_project_session_state"),
  saveCurrentProject: () => invokeCommand<ProjectSaveResultDto>("save_current_project"),
  saveProjectAs: (path: string) => invokeCommand<ProjectSaveResultDto>("save_project_as", { path }),
  openProjectPackage: (request: ProjectOpenRequestDto) =>
    invokeCommand<ProjectOpenResultDto>("open_project_package", { request }),
  listRecentProjects: () => invokeCommand<RecentProjectEntryDto[]>("list_recent_projects"),
  removeRecentProject: (path: string) => invokeCommand<void>("remove_recent_project", { path }),
  clearMissingRecentProjects: () => invokeCommand<number>("clear_missing_recent_projects"),
  // v2.9 user-circuit simulation workflow
  listUserCircuitSimulationProfiles: () =>
    invokeCommand<UserCircuitSimulationProfileDto[]>("list_user_circuit_simulation_profiles"),
  suggestUserCircuitSimulationProbes: () =>
    invokeCommand<SimulationProbeDto[]>("suggest_user_circuit_simulation_probes"),
  validateCurrentCircuitForSimulation: (profile: UserCircuitSimulationProfileDto) =>
    invokeCommand<SimulationPreflightResultDto>("validate_current_circuit_for_simulation", {
      profile,
    }),
  runCurrentCircuitSimulation: (profile: UserCircuitSimulationProfileDto) =>
    invokeCommand<UserCircuitSimulationRunDto>("run_current_circuit_simulation", { profile }),
  getLastUserCircuitSimulation: () =>
    invokeCommand<UserCircuitSimulationRunDto | null>("get_last_user_circuit_simulation"),
  clearLastUserCircuitSimulation: () => invokeCommand<void>("clear_last_user_circuit_simulation"),
  addLastSimulationToAdvancedReport: () =>
    invokeCommand<ProjectDto>("add_last_simulation_to_advanced_report"),
};
