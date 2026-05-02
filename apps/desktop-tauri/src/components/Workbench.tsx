import { Button, Group, Paper, Text, TextInput } from "@mantine/core";
import { Calculator, CircuitBoard, FileText, Play, Save, Sigma } from "lucide-react";
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
    setBusy,
    setError,
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
        </Group>
        <TextInput
          className="save-path"
          value={savePath}
          onChange={(event) => setSavePath(event.currentTarget.value)}
          aria-label="Project save path"
        />
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
    actions: {
      createDemoProject: () => void;
      calculateCutoff: () => void;
      selectNearestE24: () => void;
      generateNetlist: () => void;
      runSimulation: () => void;
      exportMarkdown: () => void;
      exportHtml: () => void;
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
      markdownReport={context.markdownReport}
      htmlReport={context.htmlReport}
      hasProject={context.hasProject}
      onNetlist={context.actions.generateNetlist}
      onMarkdown={context.actions.exportMarkdown}
      onHtml={context.actions.exportHtml}
    />
  );
}
