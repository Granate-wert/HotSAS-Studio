import { create } from "zustand";
import type {
  AdvancedReportDto,
  AppDiagnosticsReportDto,
  CircuitValidationReportDto,
  ComponentDetailsDto,
  ComponentLibraryDto,
  ComponentSearchResultDto,
  ExportCapabilityDto,
  ExportHistoryEntryDto,
  ExportResultDto,
  FormulaResultDto,
  ImportedModelDetailsDto,
  ImportedModelSummaryDto,
  NgspiceAvailabilityDto,
  NotebookEvaluationResultDto,
  NotebookStateDto,
  PreferredValueDto,
  ProductWorkflowStatusDto,
  ProjectDto,
  ReportSectionCapabilityDto,
  SchematicEditResultDto,
  SchematicToolCapabilityDto,
  SelectedComponentDto,
  SelectedRegionAnalysisResultDto,
  SelectedRegionPreviewDto,
  SimulationResultDto,
  SpiceImportReportDto,
  TouchstoneImportReportDto,
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
  exportCapabilities: ExportCapabilityDto[];
  lastExportResult: ExportResultDto | null;
  exportHistory: ExportHistoryEntryDto[];
  ngspiceAvailability: NgspiceAvailabilityDto | null;
  selectedSimulationEngine: string;
  simulationHistory: SimulationResultDto[];
  isSimulationRunning: boolean;
  simulationError: string | null;
  spiceImportReport: SpiceImportReportDto | null;
  touchstoneImportReport: TouchstoneImportReportDto | null;
  importedModels: ImportedModelSummaryDto[];
  selectedImportedModel: ImportedModelDetailsDto | null;
  appDiagnostics: AppDiagnosticsReportDto | null;
  readinessSelfCheckResult: AppDiagnosticsReportDto | null;
  diagnosticsLoading: boolean;
  diagnosticsError: string | null;
  productWorkflowStatus: ProductWorkflowStatusDto | null;
  productWorkflowLoading: boolean;
  productWorkflowError: string | null;
  reportSectionCapabilities: ReportSectionCapabilityDto[];
  lastAdvancedReport: AdvancedReportDto | null;
  advancedReportPreview: AdvancedReportDto | null;
  advancedReportExportResult: string | null;
  advancedReportLoading: boolean;
  advancedReportError: string | null;
  // v2.5 schematic editor
  schematicEditorCapabilities: SchematicToolCapabilityDto[];
  schematicEditLoading: boolean;
  schematicEditError: string | null;
  pendingConnectionStart: { componentId: string; pinId: string } | null;
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
  setExportCapabilities: (capabilities: ExportCapabilityDto[]) => void;
  setLastExportResult: (result: ExportResultDto | null) => void;
  setExportHistory: (history: ExportHistoryEntryDto[]) => void;
  setNgspiceAvailability: (availability: NgspiceAvailabilityDto | null) => void;
  setSelectedSimulationEngine: (engine: string) => void;
  setSimulationHistory: (history: SimulationResultDto[]) => void;
  setIsSimulationRunning: (running: boolean) => void;
  setSimulationError: (error: string | null) => void;
  setSpiceImportReport: (report: SpiceImportReportDto | null) => void;
  setTouchstoneImportReport: (report: TouchstoneImportReportDto | null) => void;
  setImportedModels: (models: ImportedModelSummaryDto[]) => void;
  setSelectedImportedModel: (model: ImportedModelDetailsDto | null) => void;
  setAppDiagnostics: (report: AppDiagnosticsReportDto | null) => void;
  setReadinessSelfCheckResult: (report: AppDiagnosticsReportDto | null) => void;
  setDiagnosticsLoading: (loading: boolean) => void;
  setDiagnosticsError: (error: string | null) => void;
  setProductWorkflowStatus: (status: ProductWorkflowStatusDto | null) => void;
  setProductWorkflowLoading: (loading: boolean) => void;
  setProductWorkflowError: (error: string | null) => void;
  setReportSectionCapabilities: (capabilities: ReportSectionCapabilityDto[]) => void;
  setLastAdvancedReport: (report: AdvancedReportDto | null) => void;
  setAdvancedReportPreview: (preview: AdvancedReportDto | null) => void;
  setAdvancedReportExportResult: (result: string | null) => void;
  setAdvancedReportLoading: (loading: boolean) => void;
  setAdvancedReportError: (error: string | null) => void;
  setSchematicEditorCapabilities: (capabilities: SchematicToolCapabilityDto[]) => void;
  setSchematicEditLoading: (loading: boolean) => void;
  setSchematicEditError: (error: string | null) => void;
  setPendingConnectionStart: (start: { componentId: string; pinId: string } | null) => void;
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
  exportCapabilities: [],
  lastExportResult: null,
  exportHistory: [],
  ngspiceAvailability: null,
  selectedSimulationEngine: "auto",
  simulationHistory: [],
  isSimulationRunning: false,
  simulationError: null,
  spiceImportReport: null,
  touchstoneImportReport: null,
  importedModels: [],
  selectedImportedModel: null,
  appDiagnostics: null,
  readinessSelfCheckResult: null,
  diagnosticsLoading: false,
  diagnosticsError: null,
  productWorkflowStatus: null,
  productWorkflowLoading: false,
  productWorkflowError: null,
  reportSectionCapabilities: [],
  lastAdvancedReport: null,
  advancedReportPreview: null,
  advancedReportExportResult: null,
  advancedReportLoading: false,
  advancedReportError: null,
  schematicEditorCapabilities: [],
  schematicEditLoading: false,
  schematicEditError: null,
  pendingConnectionStart: null,
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
  setSelectedRegionComponentIds: (selectedRegionComponentIds) =>
    set({ selectedRegionComponentIds }),
  setSelectedRegionPreview: (selectedRegionPreview) => set({ selectedRegionPreview }),
  setSelectedRegionAnalysisResult: (selectedRegionAnalysisResult) =>
    set({ selectedRegionAnalysisResult }),
  setExportCapabilities: (exportCapabilities) => set({ exportCapabilities }),
  setLastExportResult: (lastExportResult) => set({ lastExportResult }),
  setExportHistory: (exportHistory) => set({ exportHistory }),
  setNgspiceAvailability: (ngspiceAvailability) => set({ ngspiceAvailability }),
  setSelectedSimulationEngine: (selectedSimulationEngine) => set({ selectedSimulationEngine }),
  setSimulationHistory: (simulationHistory) => set({ simulationHistory }),
  setIsSimulationRunning: (isSimulationRunning) => set({ isSimulationRunning }),
  setSimulationError: (simulationError) => set({ simulationError }),
  setSpiceImportReport: (spiceImportReport) => set({ spiceImportReport }),
  setTouchstoneImportReport: (touchstoneImportReport) => set({ touchstoneImportReport }),
  setImportedModels: (importedModels) => set({ importedModels }),
  setSelectedImportedModel: (selectedImportedModel) => set({ selectedImportedModel }),
  setAppDiagnostics: (appDiagnostics) => set({ appDiagnostics }),
  setReadinessSelfCheckResult: (readinessSelfCheckResult) => set({ readinessSelfCheckResult }),
  setDiagnosticsLoading: (diagnosticsLoading) => set({ diagnosticsLoading }),
  setDiagnosticsError: (diagnosticsError) => set({ diagnosticsError }),
  setProductWorkflowStatus: (productWorkflowStatus) => set({ productWorkflowStatus }),
  setProductWorkflowLoading: (productWorkflowLoading) => set({ productWorkflowLoading }),
  setProductWorkflowError: (productWorkflowError) => set({ productWorkflowError }),
  setReportSectionCapabilities: (reportSectionCapabilities) => set({ reportSectionCapabilities }),
  setLastAdvancedReport: (lastAdvancedReport) => set({ lastAdvancedReport }),
  setAdvancedReportPreview: (advancedReportPreview) => set({ advancedReportPreview }),
  setAdvancedReportExportResult: (advancedReportExportResult) =>
    set({ advancedReportExportResult }),
  setAdvancedReportLoading: (advancedReportLoading) => set({ advancedReportLoading }),
  setAdvancedReportError: (advancedReportError) => set({ advancedReportError }),
  setSchematicEditorCapabilities: (schematicEditorCapabilities) =>
    set({ schematicEditorCapabilities }),
  setSchematicEditLoading: (schematicEditLoading) => set({ schematicEditLoading }),
  setSchematicEditError: (schematicEditError) => set({ schematicEditError }),
  setPendingConnectionStart: (pendingConnectionStart) => set({ pendingConnectionStart }),
}));
