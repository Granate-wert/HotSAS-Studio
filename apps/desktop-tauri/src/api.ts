import { invoke } from '@tauri-apps/api/core';
import type {
  FormulaResultDto,
  PreferredValueDto,
  ProjectDto,
  SaveProjectDto,
  SimulationResultDto,
  VerticalSliceDto,
} from './types';

export const backend = {
  createRcLowPassDemoProject: () =>
    invoke<ProjectDto>('create_rc_low_pass_demo_project'),
  calculateRcLowPass: () => invoke<FormulaResultDto>('calculate_rc_low_pass'),
  nearestE24ForResistor: () =>
    invoke<PreferredValueDto>('nearest_e24_for_resistor'),
  nearestE24: (value: string, unit?: string) =>
    invoke<PreferredValueDto>('nearest_e24', { value, unit }),
  generateSpiceNetlist: () => invoke<string>('generate_spice_netlist'),
  runMockAcSimulation: () =>
    invoke<SimulationResultDto>('run_mock_ac_simulation'),
  exportMarkdownReport: () => invoke<string>('export_markdown_report'),
  exportHtmlReport: () => invoke<string>('export_html_report'),
  saveProjectJson: (path: string) =>
    invoke<SaveProjectDto>('save_project_json', { path }),
  runVerticalSlicePreview: () =>
    invoke<VerticalSliceDto>('run_vertical_slice_preview'),
};
