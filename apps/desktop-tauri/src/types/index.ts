export type ValueDto = {
  original: string;
  si_value: number;
  unit: string;
  display: string;
};

export type ParameterDto = {
  name: string;
  value: ValueDto;
};

export type PinDto = {
  id: string;
  name: string;
  number: string;
  electrical_type: string;
  x: number;
  y: number;
  side: string;
};

export type SymbolDto = {
  id: string;
  title: string;
  component_kind: string;
  width: number;
  height: number;
  pins: PinDto[];
};

export type ConnectedPinDto = {
  pin_id: string;
  net_id: string;
};

export type ComponentDto = {
  instance_id: string;
  definition_id: string;
  component_kind: string;
  display_label: string;
  x: number;
  y: number;
  rotation_degrees: number;
  parameters: ParameterDto[];
  symbol: SymbolDto | null;
  pins: PinDto[];
  connected_nets: ConnectedPinDto[];
};

export type WireDto = {
  id: string;
  from_component_id: string | null;
  to_component_id: string | null;
  net_id: string;
};

export type NetDto = {
  id: string;
  name: string;
};

export type CircuitDto = {
  id: string;
  title: string;
  components: ComponentDto[];
  wires: WireDto[];
  nets: NetDto[];
};

export type ProjectDto = {
  id: string;
  name: string;
  format_version: string;
  engine_version: string;
  project_type: string;
  schematic: CircuitDto;
};

export type FormulaResultDto = {
  formula_id: string;
  output_name: string;
  value: ValueDto;
  expression: string;
};

export type FormulaPackDto = {
  pack_id: string;
  title: string;
  version: string;
  formula_count: number;
  categories: string[];
};

export type FormulaSummaryDto = {
  id: string;
  title: string;
  category: string;
  description: string;
  linked_circuit_template_id: string | null;
};

export type FormulaVariableDto = {
  name: string;
  unit: string;
  description: string;
  default: ValueDto | null;
};

export type FormulaEquationDto = {
  id: string;
  latex: string;
  expression: string;
  solve_for: string[];
};

export type FormulaOutputDto = {
  name: string;
  unit: string;
  description: string;
};

export type FormulaExampleValueDto = {
  name: string;
  value: string;
};

export type FormulaExampleDto = {
  title: string;
  inputs: FormulaExampleValueDto[];
  expected_outputs: FormulaExampleValueDto[];
  notes: string | null;
};

export type FormulaDetailsDto = {
  id: string;
  title: string;
  category: string;
  description: string;
  variables: FormulaVariableDto[];
  equations: FormulaEquationDto[];
  outputs: FormulaOutputDto[];
  assumptions: string[];
  limitations: string[];
  examples: FormulaExampleDto[];
  linked_circuit_template_id: string | null;
  mapping: Record<string, string> | null;
  default_simulation: string | null;
};

export type FormulaVariableInputDto = {
  name: string;
  value: string;
  unit?: string | null;
};

export type FormulaCalculationRequestDto = {
  formula_id: string;
  variables: FormulaVariableInputDto[];
};

export type FormulaOutputValueDto = {
  name: string;
  value: ValueDto;
};

export type FormulaEvaluationResultDto = {
  formula_id: string;
  equation_id: string;
  expression: string;
  outputs: FormulaOutputValueDto[];
  warnings: string[];
};

export type PreferredValueDto = {
  requested_value: ValueDto;
  series: string;
  lower: ValueDto | null;
  nearest: ValueDto;
  higher: ValueDto | null;
  error_percent: number;
};

export type GraphSeriesDto = {
  name: string;
  x_unit: string;
  y_unit: string;
  points: Array<[number, number]>;
};

export type SimulationResultDto = {
  id: string;
  profile_id: string;
  status: string;
  engine: string;
  graph_series: GraphSeriesDto[];
  warnings: string[];
  errors: string[];
};

export type NgspiceAvailabilityDto = {
  available: boolean;
  executablePath?: string | null;
  version?: string | null;
  message?: string | null;
  warnings: string[];
};

export type SimulationRunRequestDto = {
  engine: string;
  analysis_kind: string;
  profile_id?: string | null;
  output_variables: string[];
  timeout_ms?: number | null;
};

export type SaveProjectDto = {
  path: string;
};

export type VerticalSliceDto = {
  project: ProjectDto;
  cutoff_frequency: FormulaResultDto;
  nearest_e24: PreferredValueDto;
  spice_netlist: string;
  simulation: SimulationResultDto;
  markdown_report: string;
  html_report: string;
};

export type ProjectPackageFilesDto = {
  schematic: string;
  components: string;
  formulas: string;
  simulation_profiles: string;
  reports_index: string;
  results_index: string;
};

export type ProjectPackageManifestDto = {
  format_version: string;
  engine_version: string;
  project_id: string;
  project_name: string;
  project_type: string;
  created_at: string;
  updated_at: string;
  files: ProjectPackageFilesDto;
};

export type ProjectPackageValidationReportDto = {
  valid: boolean;
  package_dir: string;
  missing_files: string[];
  warnings: string[];
  errors: string[];
};

export type ComponentParameterDto = {
  name: string;
  value: string;
  unit?: string | null;
};

export type SelectedComponentDto = {
  instance_id: string;
  component_kind: string;
  title: string;
  parameters: ComponentParameterDto[];
  symbol: SymbolDto | null;
};

export type CircuitValidationIssueDto = {
  code: string;
  message: string;
  component_id: string | null;
  net_id: string | null;
};

export type CircuitValidationReportDto = {
  valid: boolean;
  warnings: CircuitValidationIssueDto[];
  errors: CircuitValidationIssueDto[];
};

export type ApiErrorDto = {
  code: string;
  message: string;
  details?: string | null;
};

export type NotebookVariableDto = {
  name: string;
  value: ValueDto;
};

export type NotebookOutputDto = {
  name: string;
  value: ValueDto;
};

export type NotebookEvaluationRequestDto = {
  input: string;
};

export type NotebookEvaluationResultDto = {
  input: string;
  status: string;
  kind: string;
  outputs: NotebookOutputDto[];
  variables: NotebookVariableDto[];
  message?: string | null;
  warnings: string[];
};

export type NotebookHistoryEntryDto = {
  id: string;
  input: string;
  result_summary: string;
  status: string;
};

export type NotebookStateDto = {
  variables: NotebookVariableDto[];
  history: NotebookHistoryEntryDto[];
};

export type ApplyNotebookValueRequestDto = {
  instance_id: string;
  parameter_name: string;
  output_name: string;
};

export type ComponentLibraryDto = {
  id: string;
  title: string;
  version: string;
  components: ComponentSummaryDto[];
  categories: string[];
  tags: string[];
};

export type ComponentSummaryDto = {
  id: string;
  name: string;
  category: string;
  manufacturer?: string | null;
  part_number?: string | null;
  description?: string | null;
  tags: string[];
  has_symbol: boolean;
  has_footprint: boolean;
  has_simulation_model: boolean;
};

export type ComponentDetailsDto = {
  id: string;
  name: string;
  category: string;
  manufacturer?: string | null;
  part_number?: string | null;
  description?: string | null;
  parameters: ComponentParameterDto[];
  ratings: ComponentParameterDto[];
  symbol_ids: string[];
  footprint_ids: string[];
  simulation_models: SimulationModelDto[];
  datasheets: string[];
  tags: string[];
  metadata: KeyValueDto[];
  symbol_preview?: SymbolDto | null;
  footprint_previews: FootprintDto[];
};

export type ComponentSearchRequestDto = {
  search?: string | null;
  category?: string | null;
  tags: string[];
  manufacturer?: string | null;
  has_symbol?: boolean | null;
  has_footprint?: boolean | null;
  has_simulation_model?: boolean | null;
};

export type ComponentSearchResultDto = {
  components: ComponentSummaryDto[];
  total_count: number;
  categories: string[];
  tags: string[];
};

export type AssignComponentRequestDto = {
  instance_id: string;
  component_definition_id: string;
  selected_symbol_id?: string | null;
  selected_footprint_id?: string | null;
  selected_simulation_model_id?: string | null;
};

export type FootprintDto = {
  id: string;
  name: string;
  package_name: string;
  pad_count: number;
  metadata: KeyValueDto[];
};

export type SimulationModelDto = {
  id: string;
  model_type: string;
  source_path?: string | null;
};

export type KeyValueDto = {
  key: string;
  value: string;
};

export type RegionPortDto = {
  positive_net: string;
  negative_net: string | null;
  label: string | null;
};

export type RegionAnalysisDirection = "LeftToRight" | "RightToLeft" | "Custom";
export type RegionAnalysisMode = "Structural" | "TemplateBased" | "NumericMock" | "AllAvailable";

export type SelectedCircuitRegionDto = {
  id: string;
  title: string;
  component_ids: string[];
  internal_nets: string[];
  boundary_nets: string[];
  input_port: RegionPortDto | null;
  output_port: RegionPortDto | null;
  reference_node: string | null;
  analysis_direction: string;
  analysis_mode: string;
};

export type RegionComponentSummaryDto = {
  instance_id: string;
  definition_id: string | null;
  component_kind: string;
  display_label: string;
  connected_nets: string[];
};

export type RegionNetSummaryDto = {
  net_id: string;
  net_name: string;
  connected_selected_components: string[];
  connected_external_components: string[];
  is_ground: boolean;
  role_hint: string | null;
};

export type SelectedRegionIssueDto = {
  code: string;
  severity: string;
  message: string;
  component_id: string | null;
  net_id: string | null;
};

export type SelectedRegionPreviewDto = {
  region: SelectedCircuitRegionDto;
  selected_components: RegionComponentSummaryDto[];
  detected_internal_nets: RegionNetSummaryDto[];
  detected_boundary_nets: RegionNetSummaryDto[];
  suggested_input_nets: string[];
  suggested_output_nets: string[];
  suggested_reference_nodes: string[];
  warnings: SelectedRegionIssueDto[];
  errors: SelectedRegionIssueDto[];
};

export type MatchedRegionTemplateDto = {
  template_id: string;
  title: string;
  confidence: number;
  formula_ids: string[];
  explanation: string;
};

export type EquivalentCircuitSummaryDto = {
  title: string;
  description: string;
  assumptions: string[];
  limitations: string[];
};

export type RegionTransferFunctionDto = {
  expression: string;
  latex: string | null;
  output_name: string;
  unit: string | null;
  availability_note: string | null;
};

export type RegionMeasurementDto = {
  name: string;
  value: ValueDto | null;
  description: string;
  source: string;
};

export type RegionGraphSpecDto = {
  id: string;
  title: string;
  x_unit: string | null;
  y_unit: string | null;
  description: string;
  available: boolean;
  unavailable_reason: string | null;
};

export type RegionNetlistFragmentDto = {
  title: string;
  format: string;
  content: string;
  warnings: string[];
};

export type SelectedRegionAnalysisResultDto = {
  region: SelectedCircuitRegionDto;
  status: string;
  summary: string;
  matched_template: MatchedRegionTemplateDto | null;
  equivalent_circuit: EquivalentCircuitSummaryDto | null;
  transfer_function: RegionTransferFunctionDto | null;
  measurements: RegionMeasurementDto[];
  graph_specs: RegionGraphSpecDto[];
  netlist_fragment: RegionNetlistFragmentDto | null;
  warnings: SelectedRegionIssueDto[];
  errors: SelectedRegionIssueDto[];
  report_section_markdown: string | null;
};

export type ExportCapabilityDto = {
  format: string;
  label: string;
  description: string;
  file_extension: string;
  available: boolean;
};

export type ExportResultDto = {
  format: string;
  content: string;
  file_path: string | null;
  success: boolean;
  message: string;
};

export type ExportHistoryEntryDto = {
  timestamp: string;
  format: string;
  file_path: string | null;
  success: boolean;
  message: string;
};

export type ExportRequestDto = {
  format: string;
  write_to_file: boolean;
  output_dir?: string | null;
};

export type SelectedRegionAnalysisRequestDto = {
  component_ids: string[];
  input_port: RegionPortDto | null;
  output_port: RegionPortDto | null;
  reference_node: string | null;
  analysis_direction: string;
  analysis_mode: string;
};

export type SpiceModelParameterDto = {
  name: string;
  value: string;
  unit_hint: string | null;
};

export type SpiceModelDto = {
  id: string;
  name: string;
  kind: string;
  parameters: SpiceModelParameterDto[];
  warnings: string[];
};

export type SpiceSubcircuitDto = {
  id: string;
  name: string;
  pins: string[];
  detected_kind: string;
  parameters: SpiceModelParameterDto[];
  warnings: string[];
};

export type SpiceImportRequestDto = {
  source_name: string | null;
  content: string;
};

export type SpiceImportReportDto = {
  status: string;
  models: SpiceModelDto[];
  subcircuits: SpiceSubcircuitDto[];
  warnings: string[];
  errors: string[];
};

export type TouchstoneSummaryDto = {
  id: string;
  name: string;
  port_count: number;
  point_count: number;
  start_frequency_hz: number | null;
  stop_frequency_hz: number | null;
  parameter_format: string;
  reference_impedance_ohm: number;
};

export type TouchstoneImportRequestDto = {
  source_name: string | null;
  content: string;
};

export type TouchstoneImportReportDto = {
  status: string;
  summary: TouchstoneSummaryDto | null;
  warnings: string[];
  errors: string[];
};

export type SpicePinMappingEntryDto = {
  model_pin: string;
  component_pin: string;
  role_hint: string | null;
};

export type SpicePinMappingRequestDto = {
  model_id: string;
  component_definition_id: string;
  mappings: SpicePinMappingEntryDto[];
};

export type SpicePinMappingValidationReportDto = {
  valid: boolean;
  warnings: string[];
  errors: string[];
};

export type AttachImportedModelRequestDto = {
  model_id: string;
  component_definition_id: string;
  pin_mapping: SpicePinMappingRequestDto | null;
};

export type ImportedModelSummaryDto = {
  id: string;
  kind: string;
  name: string;
  source_format: string;
};

export type ImportedModelDetailsDto = {
  id: string;
  kind: string;
  name: string;
  source_format: string;
  spice_model: SpiceModelDto | null;
  spice_subcircuit: SpiceSubcircuitDto | null;
  touchstone_summary: TouchstoneSummaryDto | null;
};

export type ReadinessCheckDto = {
  id: string;
  title: string;
  status: string;
  message: string;
};

export type ModuleDiagnosticsDto = {
  id: string;
  title: string;
  status: string;
  summary: string;
  details: Record<string, string>;
};

export type AppDiagnosticsReportDto = {
  app_name: string;
  app_version: string;
  roadmap_stage: string;
  build_profile: string;
  modules: ModuleDiagnosticsDto[];
  checks: ReadinessCheckDto[];
  warnings: string[];
};

export type ProjectSummaryDto = {
  project_id: string;
  project_name: string;
  format_version: string;
  component_count: number;
  net_count: number;
  simulation_profile_count: number;
};

export type WorkflowStepStatusDto = {
  id: string;
  title: string;
  status: string;
  screen_id: string;
  description: string;
  warnings: string[];
};

export type WorkflowModuleStatusDto = {
  id: string;
  title: string;
  status: string;
  details: KeyValueDto[];
};

export type ProductWorkflowStatusDto = {
  app_name: string;
  app_version: string;
  roadmap_stage: string;
  build_profile: string;
  current_project: ProjectSummaryDto | null;
  workflow_steps: WorkflowStepStatusDto[];
  module_statuses: WorkflowModuleStatusDto[];
  blockers: string[];
  warnings: string[];
};

export type DcdcInputDto = {
  topology: string;
  vin: string;
  vout: string;
  iout: string;
  switching_frequency: string;
  inductor: string | null;
  output_capacitor: string | null;
  target_inductor_ripple_percent: number | null;
  estimated_efficiency_percent: number | null;
};

export type DcdcComputedValueDto = {
  id: string;
  label: string;
  value: ValueDto;
  formula: string | null;
  description: string | null;
};

export type DcdcWarningDto = {
  code: string;
  message: string;
  severity: string;
};

export type DcdcSimulationPlanDto = {
  id: string;
  title: string;
  profile_type: string;
  recommended_stop_time: ValueDto;
  recommended_time_step: ValueDto | null;
  signals: string[];
  notes: string[];
};

export type DcdcCalculationResultDto = {
  topology: string;
  operating_mode: string;
  values: DcdcComputedValueDto[];
  assumptions: string[];
  limitations: string[];
  warnings: DcdcWarningDto[];
  simulation_plan: DcdcSimulationPlanDto | null;
  template_id: string | null;
};

export type DcdcTemplateDto = {
  id: string;
  title: string;
  topology: string;
  description: string;
  supported_outputs: string[];
  limitations: string[];
};

export type DcdcNetlistPreviewRequestDto = {
  topology: string;
  vin: string;
  vout: string;
  iout: string;
  switching_frequency: string;
};

export type DcdcMockTransientRequestDto = {
  topology: string;
  vin: string;
  vout: string;
  iout: string;
  switching_frequency: string;
  inductor: string | null;
  output_capacitor: string | null;
  target_inductor_ripple_percent: number | null;
  estimated_efficiency_percent: number | null;
};
