import { create } from "zustand";
import type {
  CircuitValidationReportDto,
  FormulaResultDto,
  NotebookEvaluationResultDto,
  NotebookStateDto,
  PreferredValueDto,
  ProjectDto,
  SelectedComponentDto,
  SimulationResultDto,
} from "../types";

type HotSasState = {
  project: ProjectDto | null;
  formulaResult: FormulaResultDto | null;
  preferredValue: PreferredValueDto | null;
  netlist: string;
  simulation: SimulationResultDto | null;
  markdownReport: string;
  htmlReport: string;
  savePath: string;
  packagePath: string;
  packageResult: string | null;
  busy: boolean;
  error: string | null;
  selectedComponentId: string | null;
  selectedComponent: SelectedComponentDto | null;
  validationReport: CircuitValidationReportDto | null;
  notebookState: NotebookStateDto | null;
  lastNotebookResult: NotebookEvaluationResultDto | null;
  setProject: (project: ProjectDto) => void;
  setFormulaResult: (result: FormulaResultDto) => void;
  setPreferredValue: (result: PreferredValueDto) => void;
  setNetlist: (netlist: string) => void;
  setSimulation: (simulation: SimulationResultDto) => void;
  setMarkdownReport: (report: string) => void;
  setHtmlReport: (report: string) => void;
  setSavePath: (path: string) => void;
  setPackagePath: (path: string) => void;
  setPackageResult: (result: string | null) => void;
  setBusy: (busy: boolean) => void;
  setError: (error: string | null) => void;
  setSelectedComponentId: (id: string | null) => void;
  setSelectedComponent: (component: SelectedComponentDto | null) => void;
  setValidationReport: (report: CircuitValidationReportDto | null) => void;
  setNotebookState: (state: NotebookStateDto | null) => void;
  setLastNotebookResult: (result: NotebookEvaluationResultDto | null) => void;
};

export const useHotSasStore = create<HotSasState>((set) => ({
  project: null,
  formulaResult: null,
  preferredValue: null,
  netlist: "",
  simulation: null,
  markdownReport: "",
  htmlReport: "",
  savePath: "shared/test_projects/rc_low_pass_demo.circuit/project.json",
  packagePath: "shared/test_projects/rc_low_pass_demo.circuit",
  packageResult: null,
  busy: false,
  error: null,
  selectedComponentId: null,
  selectedComponent: null,
  validationReport: null,
  notebookState: null,
  lastNotebookResult: null,
  setProject: (project) => set({ project }),
  setFormulaResult: (formulaResult) => set({ formulaResult }),
  setPreferredValue: (preferredValue) => set({ preferredValue }),
  setNetlist: (netlist) => set({ netlist }),
  setSimulation: (simulation) => set({ simulation }),
  setMarkdownReport: (markdownReport) => set({ markdownReport }),
  setHtmlReport: (htmlReport) => set({ htmlReport }),
  setSavePath: (savePath) => set({ savePath }),
  setPackagePath: (packagePath) => set({ packagePath }),
  setPackageResult: (packageResult) => set({ packageResult }),
  setBusy: (busy) => set({ busy }),
  setError: (error) => set({ error }),
  setSelectedComponentId: (selectedComponentId) => set({ selectedComponentId }),
  setSelectedComponent: (selectedComponent) => set({ selectedComponent }),
  setValidationReport: (validationReport) => set({ validationReport }),
  setNotebookState: (notebookState) => set({ notebookState }),
  setLastNotebookResult: (lastNotebookResult) => set({ lastNotebookResult }),
}));
