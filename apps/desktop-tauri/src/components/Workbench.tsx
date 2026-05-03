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
import { backend } from "../api";
import { CalculatorScreen } from "../screens/CalculatorScreen";
import { ComponentLibraryScreen } from "../screens/ComponentLibraryScreen";
import { ExportScreen } from "../screens/ExportScreen";
import { FormulaLibraryScreen } from "../screens/FormulaLibraryScreen";
import { SchematicScreen } from "../screens/SchematicScreen";
import { SimulationScreen } from "../screens/SimulationScreen";
import { StartScreen } from "../screens/StartScreen";
import { useHotSasStore } from "../store";
import type { ScreenId } from "../screens/navigation";

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

  const actions = {
    createDemoProject: () => run(backend.createRcLowPassDemoProject, setProject),
    calculateCutoff: () => run(backend.calculateRcLowPass, setFormulaResult),
    selectNearestE24: () => run(backend.nearestE24ForResistor, setPreferredValue),
    generateNetlist: () => run(backend.generateSpiceNetlist, setNetlist),
    runSimulation: () => run(backend.runMockAcSimulation, setSimulation),
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
  };
  const hasProject = Boolean(project);

  return (
    <div className="workbench">
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
    actions: {
      createDemoProject: () => void;
      calculateCutoff: () => void;
      selectNearestE24: () => void;
      generateNetlist: () => void;
      runSimulation: () => void;
      exportMarkdown: () => void;
      exportHtml: () => void;
      saveProjectPackage: () => void;
      loadProjectPackage: () => void;
      validateProjectPackage: () => void;
      selectComponent: (instanceId: string) => void;
      validateCircuit: () => void;
      loadExportCapabilities: () => void;
      exportFile: (format: string, writeToFile: boolean, outputDir: string) => void;
      refreshSelectedComponent: () => void;
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
        onSimulation={context.actions.runSimulation}
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
