import { invoke } from "@tauri-apps/api/core";
import type {
  ApiErrorDto,
  FormulaCalculationRequestDto,
  FormulaDetailsDto,
  FormulaEvaluationResultDto,
  FormulaPackDto,
  FormulaResultDto,
  FormulaSummaryDto,
  PreferredValueDto,
  ProjectDto,
  ProjectPackageManifestDto,
  ProjectPackageValidationReportDto,
  SaveProjectDto,
  SimulationResultDto,
  VerticalSliceDto,
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
  loadFormulaPacks: () => invokeCommand<FormulaPackDto[]>("load_formula_packs"),
  listFormulas: () => invokeCommand<FormulaSummaryDto[]>("list_formulas"),
  listFormulaCategories: () => invokeCommand<string[]>("list_formula_categories"),
  getFormula: (id: string) => invokeCommand<FormulaDetailsDto>("get_formula", { id }),
  getFormulaPackMetadata: () => invokeCommand<FormulaPackDto[]>("get_formula_pack_metadata"),
  calculateFormula: (request: FormulaCalculationRequestDto) =>
    invokeCommand<FormulaEvaluationResultDto>("calculate_formula", { request }),
  writeLog: (level: string, message: string) =>
    invokeCommand<void>("write_log", { level, message }),
};
