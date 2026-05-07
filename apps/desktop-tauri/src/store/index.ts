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
  NetlistPreviewDto,
  NgspiceAvailabilityDto,
  NotebookEvaluationResultDto,
  NotebookStateDto,
  PlaceableComponentDto,
  PreferredValueDto,
  ProductWorkflowStatusDto,
  ProjectDto,
  ReportSectionCapabilityDto,
  SchematicEditResultDto,
  SchematicSelectionDetailsDto,
  SchematicToolCapabilityDto,
  UndoRedoStateDto,
  ProjectOpenResultDto,
  ProjectSaveResultDto,
  ProjectSessionStateDto,
  RecentProjectEntryDto,
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
  // v2.8 interactive schematic editing
  schematicToolMode: "select" | "place" | "wire" | "delete";
  placeableComponents: PlaceableComponentDto[];
  pendingPlaceComponent: PlaceableComponentDto | null;
  pendingWireStart: { componentId: string; pinId: string } | null;
  selectedSchematicEntity: { kind: "component" | "wire" | "net"; id: string } | null;
  schematicSelectionDetails: SchematicSelectionDetailsDto | null;
  undoRedoState: UndoRedoStateDto | null;
  netlistPreview: NetlistPreviewDto | null;
  schematicInteractionLoading: boolean;
  schematicInteractionError: string | null;
  // v2.6 project persistence
  projectSessionState: ProjectSessionStateDto | null;
  recentProjects: RecentProjectEntryDto[];
  projectPersistenceLoading: boolean;
  projectPersistenceError: string | null;
  lastProjectSaveResult: ProjectSaveResultDto | null;
  lastProjectOpenResult: ProjectOpenResultDto | null;
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
  setSchematicToolMode: (mode: "select" | "place" | "wire" | "delete") => void;
  setPlaceableComponents: (components: PlaceableComponentDto[]) => void;
  setPendingPlaceComponent: (component: PlaceableComponentDto | null) => void;
  setPendingWireStart: (start: { componentId: string; pinId: string } | null) => void;
  setSelectedSchematicEntity: (
    entity: { kind: "component" | "wire" | "net"; id: string } | null,
  ) => void;
  setSchematicSelectionDetails: (details: SchematicSelectionDetailsDto | null) => void;
  setUndoRedoState: (state: UndoRedoStateDto | null) => void;
  setNetlistPreview: (preview: NetlistPreviewDto | null) => void;
  setSchematicInteractionLoading: (loading: boolean) => void;
  setSchematicInteractionError: (error: string | null) => void;
  setProjectSessionState: (
    state:
      | ProjectSessionStateDto
      | null
      | ((prev: ProjectSessionStateDto | null) => ProjectSessionStateDto | null),
  ) => void;
  setRecentProjects: (projects: RecentProjectEntryDto[]) => void;
  setProjectPersistenceLoading: (loading: boolean) => void;
  setProjectPersistenceError: (error: string | null) => void;
  setLastProjectSaveResult: (result: ProjectSaveResultDto | null) => void;
  setLastProjectOpenResult: (result: ProjectOpenResultDto | null) => void;
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
  // v2.8 interactive schematic editing
  schematicToolMode: "select",
  placeableComponents: [],
  pendingPlaceComponent: null,
  pendingWireStart: null,
  selectedSchematicEntity: null,
  schematicSelectionDetails: null,
  undoRedoState: null,
  netlistPreview: null,
  schematicInteractionLoading: false,
  schematicInteractionError: null,
  projectSessionState: null,
  recentProjects: [],
  projectPersistenceLoading: false,
  projectPersistenceError: null,
  lastProjectSaveResult: null,
  lastProjectOpenResult: null,
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
  setPendingConnectionStart: (
    pendingConnectionStart: { componentId: string; pinId: string } | null,
  ) => set({ pendingConnectionStart }),
  setSchematicToolMode: (schematicToolMode) => set({ schematicToolMode }),
  setPlaceableComponents: (placeableComponents) => set({ placeableComponents }),
  setPendingPlaceComponent: (pendingPlaceComponent) => set({ pendingPlaceComponent }),
  setPendingWireStart: (pendingWireStart) => set({ pendingWireStart }),
  setSelectedSchematicEntity: (selectedSchematicEntity) => set({ selectedSchematicEntity }),
  setSchematicSelectionDetails: (schematicSelectionDetails) => set({ schematicSelectionDetails }),
  setUndoRedoState: (undoRedoState) => set({ undoRedoState }),
  setNetlistPreview: (netlistPreview) => set({ netlistPreview }),
  setSchematicInteractionLoading: (schematicInteractionLoading) =>
    set({ schematicInteractionLoading }),
  setSchematicInteractionError: (schematicInteractionError) => set({ schematicInteractionError }),
  setProjectSessionState: (projectSessionState) =>
    set((state) => ({
      projectSessionState:
        typeof projectSessionState === "function"
          ? projectSessionState(state.projectSessionState)
          : projectSessionState,
    })),
  setRecentProjects: (recentProjects: RecentProjectEntryDto[]) => set({ recentProjects }),
  setProjectPersistenceLoading: (projectPersistenceLoading: boolean) =>
    set({ projectPersistenceLoading }),
  setProjectPersistenceError: (projectPersistenceError: string | null) =>
    set({ projectPersistenceError }),
  setLastProjectSaveResult: (lastProjectSaveResult: ProjectSaveResultDto | null) =>
    set({ lastProjectSaveResult }),
  setLastProjectOpenResult: (lastProjectOpenResult: ProjectOpenResultDto | null) =>
    set({ lastProjectOpenResult }),
}));
