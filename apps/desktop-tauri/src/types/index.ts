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

export type ComponentDto = {
  instance_id: string;
  definition_id: string;
  x: number;
  y: number;
  rotation_degrees: number;
  parameters: ParameterDto[];
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

export type FormulaDetailsDto = {
  id: string;
  title: string;
  category: string;
  description: string;
  variables: FormulaVariableDto[];
  equations: FormulaEquationDto[];
  outputs: FormulaOutputDto[];
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
  graph_series: GraphSeriesDto[];
  warnings: string[];
  errors: string[];
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

export type ApiErrorDto = {
  code: string;
  message: string;
  details?: string | null;
};
