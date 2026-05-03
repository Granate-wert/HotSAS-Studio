import { create } from "zustand";
import type {
  CircuitValidationReportDto,
  ComponentDetailsDto,
  ComponentLibraryDto,
  ComponentSearchResultDto,
  FormulaResultDto,
  NotebookEvaluationResultDto,
  NotebookStateDto,
  PreferredValueDto,
  ProjectDto,
  SelectedComponentDto,
  SelectedRegionAnalysisResultDto,
  SelectedRegionPreviewDto,
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
  componentLibrary: ComponentLibraryDto | null;
  componentSearchResult: ComponentSearchResultDto | null;
  selectedLibraryComponentId: string | null;
  selectedLibraryComponent: ComponentDetailsDto | null;
  selectedRegionComponentIds: string[];
  selectedRegionPreview: SelectedRegionPreviewDto | null;
  selectedRegionAnalysisResult: SelectedRegionAnalysisResultDto | null;
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
  clearNotebookState: () => void;
  setComponentLibrary: (library: ComponentLibraryDto | null) => void;
  setComponentSearchResult: (result: ComponentSearchResultDto | null) => void;
  setSelectedLibraryComponentId: (id: string | null) => void;
  setSelectedLibraryComponent: (component: ComponentDetailsDto | null) => void;
  setSelectedRegionComponentIds: (ids: string[]) => void;
  setSelectedRegionPreview: (preview: SelectedRegionPreviewDto | null) => void;
  setSelectedRegionAnalysisResult: (result: SelectedRegionAnalysisResultDto | null) => void;
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
  componentLibrary: null,
  componentSearchResult: null,
  selectedLibraryComponentId: null,
  selectedLibraryComponent: null,
  selectedRegionComponentIds: [],
  selectedRegionPreview: null,
  selectedRegionAnalysisResult: null,
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
  clearNotebookState: () => set({ notebookState: null, lastNotebookResult: null }),
  setComponentLibrary: (componentLibrary) => set({ componentLibrary }),
  setComponentSearchResult: (componentSearchResult) => set({ componentSearchResult }),
  setSelectedLibraryComponentId: (selectedLibraryComponentId) =>
    set({ selectedLibraryComponentId }),
  setSelectedLibraryComponent: (selectedLibraryComponent) => set({ selectedLibraryComponent }),
  setSelectedRegionComponentIds: (selectedRegionComponentIds) => set({ selectedRegionComponentIds }),
  setSelectedRegionPreview: (selectedRegionPreview) => set({ selectedRegionPreview }),
  setSelectedRegionAnalysisResult: (selectedRegionAnalysisResult) =>
    set({ selectedRegionAnalysisResult }),
}));
