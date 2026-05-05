import { Button, Group, Paper, Text, TextInput } from "@mantine/core";
import {
  Calculator,
  CircuitBoard,
  FileText,
  FolderOpen,
  Package,
  Play,
  Save,
  Sigma,
} from "lucide-react";
import { useEffect, useMemo } from "react";
import { ImportModelsScreen } from "../screens/ImportModelsScreen";
import { backend } from "../api";
import { AdvancedReportsScreen } from "../screens/AdvancedReportsScreen";
import { DcdcCalculatorScreen } from "../screens/DcdcCalculatorScreen";
import { CalculatorScreen } from "../screens/CalculatorScreen";
import { ComponentLibraryScreen } from "../screens/ComponentLibraryScreen";
import { DiagnosticsScreen } from "../screens/DiagnosticsScreen";
import { ExportScreen } from "../screens/ExportScreen";
import { ProductBetaScreen } from "../screens/ProductBetaScreen";
import { FormulaLibraryScreen } from "../screens/FormulaLibraryScreen";
import { SchematicScreen } from "../screens/SchematicScreen";
import { SimulationScreen } from "../screens/SimulationScreen";
import { StartScreen } from "../screens/StartScreen";
import { useHotSasStore } from "../store";
import type { ScreenId } from "../screens/navigation";
import type { ProjectSessionStateDto } from "../types";
import { ProjectToolbar } from "./project/ProjectToolbar";
import { RecentProjectsPanel } from "./project/RecentProjectsPanel";
import { UnsavedChangesBanner } from "./project/UnsavedChangesBanner";

export function Workbench({ activeScreen }: { activeScreen: ScreenId }) {
  const {
    project,
    formulaResult,
    preferredValue,
    netlist,
    simulation,
    markdownReport,
    htmlReport,
    savePath,
    packagePath,
    packageResult,
    busy,
    error,
    ngspiceAvailability,
    selectedSimulationEngine,
    simulationHistory,
    isSimulationRunning,
    simulationError,
    setProject,
    setFormulaResult,
    setPreferredValue,
    setNetlist,
    setSimulation,
    setMarkdownReport,
    setHtmlReport,
    setSavePath,
    setPackagePath,
    setPackageResult,
    setBusy,
    setError,
    selectedComponentId,
    selectedComponent,
    validationReport,
    exportCapabilities,
    lastExportResult,
    setSelectedComponentId,
    setSelectedComponent,
    setValidationReport,
    setExportCapabilities,
    setLastExportResult,
    setNgspiceAvailability,
    setSelectedSimulationEngine,
    setSimulationHistory,
    setIsSimulationRunning,
    setSimulationError,
    setSpiceImportReport,
    setTouchstoneImportReport,
    setImportedModels,
    setSelectedImportedModel,
    appDiagnostics,
    readinessSelfCheckResult,
    diagnosticsLoading,
    diagnosticsError,
    setAppDiagnostics,
    setReadinessSelfCheckResult,
    setDiagnosticsLoading,
    setDiagnosticsError,
    productWorkflowStatus,
    productWorkflowLoading,
    productWorkflowError,
    setProductWorkflowStatus,
    setProductWorkflowLoading,
    setProductWorkflowError,
    reportSectionCapabilities,
    lastAdvancedReport,
    advancedReportPreview,
    advancedReportExportResult,
    advancedReportLoading,
    advancedReportError,
    setReportSectionCapabilities,
    setLastAdvancedReport,
    setAdvancedReportPreview,
    setAdvancedReportExportResult,
    setAdvancedReportLoading,
    setAdvancedReportError,
    schematicEditorCapabilities,
    schematicEditLoading,
    schematicEditError,
    pendingConnectionStart,
    setSchematicEditorCapabilities,
    setSchematicEditLoading,
    setSchematicEditError,
    setPendingConnectionStart,
    projectSessionState,
    recentProjects,
    projectPersistenceLoading,
    projectPersistenceError,
    lastProjectSaveResult,
    lastProjectOpenResult,
    setProjectSessionState,
    setRecentProjects,
    setProjectPersistenceLoading,
    setProjectPersistenceError,
    setLastProjectSaveResult,
    setLastProjectOpenResult,
  } = useHotSasStore();

  const run = async <T,>(operation: () => Promise<T>, onResult: (result: T) => void) => {
    setBusy(true);
    setError(null);
    try {
      const result = await operation();
      onResult(result);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setBusy(false);
    }
  };

  useEffect(() => {
    actions.loadProjectSessionState();
    actions.listRecentProjects();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const runDiagnostics = async <T,>(operation: () => Promise<T>, onResult: (result: T) => void) => {
    setDiagnosticsLoading(true);
    setDiagnosticsError(null);
    try {
      const result = await operation();
      onResult(result);
    } catch (caught) {
      setDiagnosticsError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setDiagnosticsLoading(false);
    }
  };

  const runAdvancedReport = async <T,>(
    operation: () => Promise<T>,
    onResult: (result: T) => void,
  ) => {
    setAdvancedReportLoading(true);
    setAdvancedReportError(null);
    try {
      const result = await operation();
      onResult(result);
    } catch (caught) {
      setAdvancedReportError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setAdvancedReportLoading(false);
    }
  };

  const actions = useMemo(
    () => ({
      createDemoProject: () => run(backend.createRcLowPassDemoProject, setProject),
      calculateCutoff: () => run(backend.calculateRcLowPass, setFormulaResult),
      selectNearestE24: () => run(backend.nearestE24ForResistor, setPreferredValue),
      generateNetlist: () => run(backend.generateSpiceNetlist, setNetlist),
      runSimulation: () => run(backend.runMockAcSimulation, setSimulation),
      checkNgspice: () => run(backend.checkNgspiceAvailability, setNgspiceAvailability),
      runSimulationWithEngine: (analysis: string) =>
        run(
          () =>
            backend.runSimulation({
              engine: selectedSimulationEngine,
              analysis_kind: analysis,
              profile_id: null,
              output_variables: [],
              timeout_ms: null,
            }),
          (result) => {
            setSimulation(result);
            setSimulationHistory([...simulationHistory, result]);
          },
        ),
      setSimulationEngine: (engine: string) => setSelectedSimulationEngine(engine),
      saveProject: () =>
        run(
          () => backend.saveProjectJson(savePath),
          () => undefined,
        ),
      exportMarkdown: () => run(backend.exportMarkdownReport, setMarkdownReport),
      exportHtml: () => run(backend.exportHtmlReport, setHtmlReport),
      saveProjectPackage: () =>
        run(
          () => backend.saveProjectPackage(packagePath),
          (manifest) => setPackageResult(`Saved package: ${manifest.project_name}`),
        ),
      loadProjectPackage: () =>
        run(
          () => backend.loadProjectPackage(packagePath),
          (project) => {
            setProject(project);
            setPackageResult(`Loaded package: ${project.name}`);
          },
        ),
      validateProjectPackage: () =>
        run(
          () => backend.validateProjectPackage(packagePath),
          (report) =>
            setPackageResult(
              report.valid ? `Package is valid` : `Invalid: ${report.errors.join(", ")}`,
            ),
        ),
      selectComponent: (instanceId: string) =>
        run(
          () => backend.getSelectedComponent(instanceId),
          (component) => {
            setSelectedComponentId(instanceId);
            setSelectedComponent(component);
          },
        ),
      validateCircuit: () =>
        run(
          () => backend.validateCurrentCircuit(),
          (report) => setValidationReport(report),
        ),
      loadExportCapabilities: () => run(backend.listExportCapabilities, setExportCapabilities),
      importSpiceModel: (content: string) =>
        run(
          () => backend.importSpiceModel({ source_name: null, content }),
          (report) => {
            setSpiceImportReport(report);
            run(backend.listImportedModels, setImportedModels);
          },
        ),
      importTouchstoneModel: (content: string) =>
        run(
          () => backend.importTouchstoneModel({ source_name: null, content }),
          (report) => {
            setTouchstoneImportReport(report);
            run(backend.listImportedModels, setImportedModels);
          },
        ),
      listImportedModels: () => run(backend.listImportedModels, setImportedModels),
      getImportedModel: (modelId: string) =>
        run(() => backend.getImportedModel(modelId), setSelectedImportedModel),
      loadAppDiagnostics: () =>
        runDiagnostics(
          () => backend.getAppDiagnostics(),
          (report) => {
            setAppDiagnostics(report);
            setReadinessSelfCheckResult(null);
          },
        ),
      runReadinessSelfCheck: () =>
        runDiagnostics(
          () => backend.runReadinessSelfCheck(),
          (report) => setReadinessSelfCheckResult(report),
        ),
      loadProductWorkflowStatus: () =>
        runDiagnostics(
          () => backend.getProductWorkflowStatus(),
          (status) => setProductWorkflowStatus(status),
        ),
      runProductBetaSelfCheck: () =>
        runDiagnostics(
          () => backend.runProductBetaSelfCheck(),
          (status) => setProductWorkflowStatus(status),
        ),
      createIntegratedDemoProject: () => run(backend.createIntegratedDemoProject, setProject),
      loadReportSectionCapabilities: () =>
        runAdvancedReport(
          () => backend.listReportSectionCapabilities(),
          setReportSectionCapabilities,
        ),
      generateAdvancedReport: (reportType: string, includedSections: string[], title: string) =>
        runAdvancedReport(
          () =>
            backend.generateAdvancedReport({
              report_id: `report-${Date.now()}`,
              title,
              report_type: reportType,
              included_sections: includedSections,
              export_options: {
                include_source_references: true,
                include_graph_references: true,
                include_assumptions: true,
                max_table_rows: null,
              },
              metadata: {},
            }),
          (report) => {
            setLastAdvancedReport(report);
            setAdvancedReportPreview(report);
          },
        ),
      exportAdvancedReport: (reportId: string, format: string, outputPath: string | null) =>
        runAdvancedReport(
          () =>
            backend.exportAdvancedReport({
              report_id: reportId,
              format,
              output_path: outputPath,
            }),
          (result) => {
            setAdvancedReportExportResult(
              result.success
                ? `Exported ${result.format}: ${result.message}`
                : `Export failed: ${result.message}`,
            );
          },
        ),
      exportFile: (format: string, writeToFile: boolean, outputDir: string) =>
        run(
          () =>
            backend.exportFile({
              format,
              write_to_file: writeToFile,
              output_dir: writeToFile ? outputDir : null,
            }),
          (result) => setLastExportResult(result),
        ),
      refreshSelectedComponent: () => {
        if (selectedComponentId) {
          run(
            () => backend.getSelectedComponent(selectedComponentId),
            (component) => setSelectedComponent(component),
          );
        }
      },
      setPendingConnectionStart: (start: { componentId: string; pinId: string } | null) =>
        setPendingConnectionStart(start),
      loadSchematicCapabilities: () =>
        run(backend.listSchematicEditorCapabilities, setSchematicEditorCapabilities),
      addSchematicComponent: (kind: string) => {
        setSchematicEditLoading(true);
        setSchematicEditError(null);
        backend
          .addSchematicComponent({
            component_kind: kind,
            component_definition_id: null,
            instance_id: null,
            x: 100 + Math.random() * 200,
            y: 100 + Math.random() * 200,
            rotation_deg: 0,
          })
          .then((result) => {
            setProject(result.project);
            setValidationReport({
              valid: result.validation_warnings.length === 0,
              warnings: result.validation_warnings,
              errors: [],
            });
          })
          .catch((err) => setSchematicEditError(err instanceof Error ? err.message : String(err)))
          .finally(() => setSchematicEditLoading(false));
      },
      moveSchematicComponent: (instanceId: string, x: number, y: number) => {
        setSchematicEditLoading(true);
        setSchematicEditError(null);
        backend
          .moveSchematicComponent({ instance_id: instanceId, x, y })
          .then((result) => {
            setProject(result.project);
            setValidationReport({
              valid: result.validation_warnings.length === 0,
              warnings: result.validation_warnings,
              errors: [],
            });
          })
          .catch((err) => setSchematicEditError(err instanceof Error ? err.message : String(err)))
          .finally(() => setSchematicEditLoading(false));
      },
      deleteSchematicComponent: (instanceId: string) => {
        setSchematicEditLoading(true);
        setSchematicEditError(null);
        backend
          .deleteSchematicComponent({ instance_id: instanceId })
          .then((result) => {
            setProject(result.project);
            setSelectedComponentId(null);
            setSelectedComponent(null);
            setValidationReport({
              valid: result.validation_warnings.length === 0,
              warnings: result.validation_warnings,
              errors: [],
            });
          })
          .catch((err) => setSchematicEditError(err instanceof Error ? err.message : String(err)))
          .finally(() => setSchematicEditLoading(false));
      },
      connectSchematicPins: (request: {
        from_component_id: string;
        from_pin_id: string;
        to_component_id: string;
        to_pin_id: string;
        net_name?: string | null;
      }) => {
        setSchematicEditLoading(true);
        setSchematicEditError(null);
        backend
          .connectSchematicPins(request)
          .then((result) => {
            setProject(result.project);
            setPendingConnectionStart(null);
            setValidationReport({
              valid: result.validation_warnings.length === 0,
              warnings: result.validation_warnings,
              errors: [],
            });
          })
          .catch((err) => setSchematicEditError(err instanceof Error ? err.message : String(err)))
          .finally(() => setSchematicEditLoading(false));
      },
      renameSchematicNet: (netId: string, newName: string) => {
        setSchematicEditLoading(true);
        setSchematicEditError(null);
        backend
          .renameSchematicNet({ net_id: netId, new_name: newName })
          .then((result) => {
            setProject(result.project);
            setValidationReport({
              valid: result.validation_warnings.length === 0,
              warnings: result.validation_warnings,
              errors: [],
            });
          })
          .catch((err) => setSchematicEditError(err instanceof Error ? err.message : String(err)))
          .finally(() => setSchematicEditLoading(false));
      },
      loadProjectSessionState: () => run(backend.getProjectSessionState, setProjectSessionState),
      saveCurrentProject: () =>
        run(backend.saveCurrentProject, (result) => {
          setLastProjectSaveResult(result);
          setProjectSessionState((prev: ProjectSessionStateDto | null) =>
            prev ? { ...prev, dirty: false, last_saved_at: result.saved_at } : null,
          );
        }),
      saveProjectAs: (path: string) =>
        run(
          () => backend.saveProjectAs(path),
          (result) => {
            setLastProjectSaveResult(result);
            setProjectSessionState((prev: ProjectSessionStateDto | null) =>
              prev
                ? {
                    ...prev,
                    dirty: false,
                    last_saved_at: result.saved_at,
                    current_project_path: result.path,
                  }
                : null,
            );
          },
        ),
      openProjectPackage: (path: string) => {
        const confirm = projectSessionState?.dirty
          ? window.confirm("Unsaved changes will be lost. Continue?")
          : true;
        if (!confirm) return;
        run(
          () => backend.openProjectPackage({ path, confirm_discard_unsaved: true }),
          (result) => {
            setProject(result.project);
            setLastProjectOpenResult(result);
            setSelectedComponentId(null);
            setSelectedComponent(null);
            setValidationReport(null);
          },
        );
      },
      listRecentProjects: () => run(backend.listRecentProjects, setRecentProjects),
      removeRecentProject: (path: string) =>
        run(
          () => backend.removeRecentProject(path),
          () => actions.listRecentProjects(),
        ),
      clearMissingRecentProjects: () =>
        run(backend.clearMissingRecentProjects, () => actions.listRecentProjects()),
    }),
    [
      savePath,
      packagePath,
      selectedSimulationEngine,
      simulationHistory,
      selectedComponentId,
      projectSessionState,
    ],
  );
  const hasProject = Boolean(project);

  return (
    <div className="workbench">
      <ProjectToolbar
        session={projectSessionState}
        onNewDemo={actions.createDemoProject}
        onOpen={actions.openProjectPackage}
        onSave={actions.saveCurrentProject}
        onSaveAs={actions.saveProjectAs}
        loading={projectPersistenceLoading}
      />
      <UnsavedChangesBanner
        session={projectSessionState}
        onSave={actions.saveCurrentProject}
        onSaveAs={actions.saveProjectAs}
      />
      <header className="toolbar">
        <Group gap="xs">
          <Button
            leftSection={<CircuitBoard size={16} />}
            onClick={actions.createDemoProject}
            loading={busy}
          >
            New RC Demo
          </Button>
          <Button
            variant="light"
            leftSection={<Calculator size={16} />}
            onClick={actions.calculateCutoff}
            disabled={!hasProject}
          >
            Calculate fc
          </Button>
          <Button
            variant="light"
            leftSection={<Sigma size={16} />}
            onClick={actions.selectNearestE24}
            disabled={!hasProject}
          >
            Nearest E24
          </Button>
          <Button
            variant="light"
            leftSection={<FileText size={16} />}
            onClick={actions.generateNetlist}
            disabled={!hasProject}
          >
            Netlist
          </Button>
          <Button
            variant="light"
            leftSection={<Play size={16} />}
            onClick={actions.runSimulation}
            disabled={!hasProject}
          >
            Mock AC
          </Button>
          <Button
            variant="light"
            leftSection={<Save size={16} />}
            onClick={actions.saveProject}
            disabled={!hasProject}
          >
            Save JSON
          </Button>
          <Button
            variant="light"
            leftSection={<Package size={16} />}
            onClick={actions.saveProjectPackage}
            disabled={!hasProject}
          >
            Save .circuit
          </Button>
          <Button
            variant="light"
            leftSection={<FolderOpen size={16} />}
            onClick={actions.loadProjectPackage}
          >
            Load .circuit
          </Button>
        </Group>
        <Group gap="xs">
          <TextInput
            className="save-path"
            value={savePath}
            onChange={(event) => setSavePath(event.currentTarget.value)}
            aria-label="Project save path"
          />
          <TextInput
            className="save-path"
            value={packagePath}
            onChange={(event) => setPackagePath(event.currentTarget.value)}
            aria-label="Package path"
          />
        </Group>
        {packageResult && (
          <Text size="xs" c="dimmed">
            {packageResult}
          </Text>
        )}
      </header>

      {error && (
        <Paper className="error-panel">
          <Text size="sm">{error}</Text>
        </Paper>
      )}

      {renderScreen(activeScreen, {
        project,
        formulaResult,
        preferredValue,
        netlist,
        simulation,
        markdownReport,
        htmlReport,
        busy,
        hasProject,
        selectedComponent,
        validationReport,
        exportCapabilities,
        lastExportResult,
        ngspiceAvailability,
        selectedSimulationEngine,
        isSimulationRunning: busy,
        appDiagnostics,
        readinessSelfCheckResult,
        diagnosticsLoading,
        diagnosticsError,
        productWorkflowStatus,
        productWorkflowLoading,
        productWorkflowError,
        reportSectionCapabilities,
        lastAdvancedReport,
        advancedReportPreview,
        advancedReportExportResult,
        advancedReportLoading,
        advancedReportError,
        schematicEditorCapabilities,
        schematicEditLoading,
        schematicEditError,
        pendingConnectionStart,
        recentProjects,
        actions,
      })}
    </div>
  );
}

function renderScreen(
  activeScreen: ScreenId,
  context: {
    project: ReturnType<typeof useHotSasStore.getState>["project"];
    formulaResult: ReturnType<typeof useHotSasStore.getState>["formulaResult"];
    preferredValue: ReturnType<typeof useHotSasStore.getState>["preferredValue"];
    netlist: string;
    simulation: ReturnType<typeof useHotSasStore.getState>["simulation"];
    markdownReport: string;
    htmlReport: string;
    busy: boolean;
    hasProject: boolean;
    selectedComponent: ReturnType<typeof useHotSasStore.getState>["selectedComponent"];
    validationReport: ReturnType<typeof useHotSasStore.getState>["validationReport"];
    exportCapabilities: ReturnType<typeof useHotSasStore.getState>["exportCapabilities"];
    lastExportResult: ReturnType<typeof useHotSasStore.getState>["lastExportResult"];
    ngspiceAvailability: ReturnType<typeof useHotSasStore.getState>["ngspiceAvailability"];
    schematicEditorCapabilities: ReturnType<
      typeof useHotSasStore.getState
    >["schematicEditorCapabilities"];
    schematicEditLoading: ReturnType<typeof useHotSasStore.getState>["schematicEditLoading"];
    schematicEditError: ReturnType<typeof useHotSasStore.getState>["schematicEditError"];
    pendingConnectionStart: ReturnType<typeof useHotSasStore.getState>["pendingConnectionStart"];
    recentProjects: ReturnType<typeof useHotSasStore.getState>["recentProjects"];
    appDiagnostics: ReturnType<typeof useHotSasStore.getState>["appDiagnostics"];
    readinessSelfCheckResult: ReturnType<
      typeof useHotSasStore.getState
    >["readinessSelfCheckResult"];
    diagnosticsLoading: ReturnType<typeof useHotSasStore.getState>["diagnosticsLoading"];
    diagnosticsError: ReturnType<typeof useHotSasStore.getState>["diagnosticsError"];
    productWorkflowStatus: ReturnType<typeof useHotSasStore.getState>["productWorkflowStatus"];
    productWorkflowLoading: ReturnType<typeof useHotSasStore.getState>["productWorkflowLoading"];
    productWorkflowError: ReturnType<typeof useHotSasStore.getState>["productWorkflowError"];
    reportSectionCapabilities: ReturnType<
      typeof useHotSasStore.getState
    >["reportSectionCapabilities"];
    lastAdvancedReport: ReturnType<typeof useHotSasStore.getState>["lastAdvancedReport"];
    advancedReportPreview: ReturnType<typeof useHotSasStore.getState>["advancedReportPreview"];
    advancedReportExportResult: ReturnType<
      typeof useHotSasStore.getState
    >["advancedReportExportResult"];
    advancedReportLoading: ReturnType<typeof useHotSasStore.getState>["advancedReportLoading"];
    advancedReportError: ReturnType<typeof useHotSasStore.getState>["advancedReportError"];
    selectedSimulationEngine: ReturnType<
      typeof useHotSasStore.getState
    >["selectedSimulationEngine"];
    isSimulationRunning: ReturnType<typeof useHotSasStore.getState>["isSimulationRunning"];
    actions: {
      createDemoProject: () => void;
      calculateCutoff: () => void;
      selectNearestE24: () => void;
      generateNetlist: () => void;
      runSimulation: () => void;
      checkNgspice: () => void;
      runSimulationWithEngine: (analysis: string) => void;
      setSimulationEngine: (engine: string) => void;
      exportMarkdown: () => void;
      exportHtml: () => void;
      saveProjectPackage: () => void;
      loadProjectPackage: () => void;
      validateProjectPackage: () => void;
      selectComponent: (instanceId: string) => void;
      validateCircuit: () => void;
      loadExportCapabilities: () => void;
      loadAppDiagnostics: () => void;
      runReadinessSelfCheck: () => void;
      loadProductWorkflowStatus: () => void;
      runProductBetaSelfCheck: () => void;
      createIntegratedDemoProject: () => void;
      exportFile: (format: string, writeToFile: boolean, outputDir: string) => void;
      importSpiceModel: (content: string) => void;
      importTouchstoneModel: (content: string) => void;
      listImportedModels: () => void;
      getImportedModel: (modelId: string) => void;
      refreshSelectedComponent: () => void;
      loadReportSectionCapabilities: () => void;
      generateAdvancedReport: (
        reportType: string,
        includedSections: string[],
        title: string,
      ) => void;
      exportAdvancedReport: (reportId: string, format: string, outputPath: string | null) => void;
      setPendingConnectionStart: (start: { componentId: string; pinId: string } | null) => void;
      loadSchematicCapabilities: () => void;
      addSchematicComponent: (kind: string) => void;
      moveSchematicComponent: (instanceId: string, x: number, y: number) => void;
      deleteSchematicComponent: (instanceId: string) => void;
      connectSchematicPins: (request: {
        from_component_id: string;
        from_pin_id: string;
        to_component_id: string;
        to_pin_id: string;
        net_name?: string | null;
      }) => void;
      renameSchematicNet: (netId: string, newName: string) => void;
      loadProjectSessionState: () => void;
      saveCurrentProject: () => void;
      saveProjectAs: (path: string) => void;
      openProjectPackage: (path: string) => void;
      listRecentProjects: () => void;
      removeRecentProject: (path: string) => void;
      clearMissingRecentProjects: () => void;
    };
  },
) {
  if (activeScreen === "start") {
    return (
      <StartScreen
        project={context.project}
        formulaResult={context.formulaResult}
        preferredValue={context.preferredValue}
        simulation={context.simulation}
        busy={context.busy}
        hasProject={context.hasProject}
        onCreateDemo={context.actions.createDemoProject}
        onCalculate={context.actions.calculateCutoff}
        onNearestE24={context.actions.selectNearestE24}
        recentProjects={context.recentProjects}
        onOpenRecent={context.actions.openProjectPackage}
        onRemoveRecent={context.actions.removeRecentProject}
        onClearMissingRecent={context.actions.clearMissingRecentProjects}
      />
    );
  }

  if (activeScreen === "schematic") {
    return (
      <SchematicScreen
        project={context.project}
        formulaResult={context.formulaResult}
        preferredValue={context.preferredValue}
        simulation={context.simulation}
        netlist={context.netlist}
        markdownReport={context.markdownReport}
        htmlReport={context.htmlReport}
        onMarkdown={context.actions.exportMarkdown}
        onHtml={context.actions.exportHtml}
        hasProject={context.hasProject}
        selectedComponent={context.selectedComponent}
        validationReport={context.validationReport}
        onSelectComponent={context.actions.selectComponent}
        onValidate={context.actions.validateCircuit}
        onPropertyUpdate={context.actions.refreshSelectedComponent}
        schematicCapabilities={context.schematicEditorCapabilities}
        schematicEditLoading={context.schematicEditLoading}
        schematicEditError={context.schematicEditError}
        pendingConnectionStart={context.pendingConnectionStart}
        onLoadSchematicCapabilities={context.actions.loadSchematicCapabilities}
        onAddComponent={context.actions.addSchematicComponent}
        onMoveComponent={context.actions.moveSchematicComponent}
        onDeleteComponent={context.actions.deleteSchematicComponent}
        onConnectPins={context.actions.connectSchematicPins}
        onRenameNet={context.actions.renameSchematicNet}
        onSetPendingConnectionStart={context.actions.setPendingConnectionStart}
      />
    );
  }

  if (activeScreen === "notebook") {
    return <CalculatorScreen />;
  }

  if (activeScreen === "formulas") {
    return <FormulaLibraryScreen />;
  }

  if (activeScreen === "components") {
    return <ComponentLibraryScreen />;
  }

  if (activeScreen === "simulation") {
    return (
      <SimulationScreen
        simulation={context.simulation}
        hasProject={context.hasProject}
        ngspiceAvailability={context.ngspiceAvailability}
        selectedEngine={context.selectedSimulationEngine}
        isRunning={context.isSimulationRunning}
        onCheckNgspice={context.actions.checkNgspice}
        onRunSimulation={context.actions.runSimulationWithEngine}
        onSetEngine={context.actions.setSimulationEngine}
      />
    );
  }

  if (activeScreen === "import") {
    return <ImportModelsScreen />;
  }

  if (activeScreen === "diagnostics") {
    return (
      <DiagnosticsScreen
        diagnostics={context.appDiagnostics}
        readinessResult={context.readinessSelfCheckResult}
        loading={context.diagnosticsLoading}
        error={context.diagnosticsError}
        onRefreshDiagnostics={context.actions.loadAppDiagnostics}
        onRunSelfCheck={context.actions.runReadinessSelfCheck}
      />
    );
  }

  if (activeScreen === "product-beta") {
    return (
      <ProductBetaScreen
        status={context.productWorkflowStatus}
        loading={context.productWorkflowLoading}
        error={context.productWorkflowError}
        onRefresh={context.actions.loadProductWorkflowStatus}
        onSelfCheck={context.actions.runProductBetaSelfCheck}
        onCreateDemo={context.actions.createIntegratedDemoProject}
        onNavigate={(screenId) => {
          const event = new CustomEvent("navigate", { detail: screenId });
          window.dispatchEvent(event);
        }}
      />
    );
  }

  if (activeScreen === "dcdc") {
    return <DcdcCalculatorScreen />;
  }

  if (activeScreen === "reports") {
    return (
      <AdvancedReportsScreen
        hasProject={context.hasProject}
        capabilities={context.reportSectionCapabilities}
        lastReport={context.lastAdvancedReport}
        previewReport={context.advancedReportPreview}
        exportResult={context.advancedReportExportResult}
        loading={context.advancedReportLoading}
        error={context.advancedReportError}
        onLoadCapabilities={context.actions.loadReportSectionCapabilities}
        onGenerateReport={context.actions.generateAdvancedReport}
        onExportReport={context.actions.exportAdvancedReport}
      />
    );
  }

  return (
    <ExportScreen
      hasProject={context.hasProject}
      capabilities={context.exportCapabilities}
      lastResult={context.lastExportResult}
      onLoadCapabilities={context.actions.loadExportCapabilities}
      onExport={context.actions.exportFile}
    />
  );
}
