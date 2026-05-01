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
