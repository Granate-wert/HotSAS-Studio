import {
  AppShell,
  Badge,
  Button,
  Code,
  Divider,
  Group,
  MantineProvider,
  NavLink,
  Paper,
  ScrollArea,
  Stack,
  Tabs,
  Text,
  TextInput,
  Title,
} from '@mantine/core';
import {
  Activity,
  Calculator,
  CircuitBoard,
  FileText,
  Play,
  Save,
  Sigma,
  TableProperties,
} from 'lucide-react';
import { Background, Controls, MiniMap, ReactFlow, type Edge, type Node } from '@xyflow/react';
import * as echarts from 'echarts';
import { useEffect, useMemo, useRef, useState, type ReactNode } from 'react';
import { backend } from './api';
import { useHotSasStore } from './store';
import type { FormulaResultDto, GraphSeriesDto, PreferredValueDto, ProjectDto, SimulationResultDto } from './types';

type ScreenId =
  | 'start'
  | 'schematic'
  | 'notebook'
  | 'formulas'
  | 'components'
  | 'simulation'
  | 'export';

const navigationItems: Array<{ id: ScreenId; label: string; icon: ReactNode }> = [
  { id: 'start', label: 'Start', icon: <CircuitBoard size={18} /> },
  { id: 'schematic', label: 'Schematic', icon: <CircuitBoard size={18} /> },
  { id: 'notebook', label: 'Engineering Notebook', icon: <Calculator size={18} /> },
  { id: 'formulas', label: 'Formula Library', icon: <Sigma size={18} /> },
  { id: 'components', label: 'E Component Library', icon: <TableProperties size={18} /> },
  { id: 'simulation', label: 'Simulation Results', icon: <Activity size={18} /> },
  { id: 'export', label: 'Export Center', icon: <FileText size={18} /> },
];

export default function App() {
  const [activeScreen, setActiveScreen] = useState<ScreenId>('start');

  return (
    <MantineProvider defaultColorScheme="dark">
      <AppShell navbar={{ width: 250, breakpoint: 'sm' }} padding={0}>
        <AppShell.Navbar className="navbar">
          <Stack gap="xs" p="md">
            <Title order={3}>HotSAS Studio</Title>
            <Text size="xs" c="dimmed">
              Hardware-Oriented Schematic Analysis & Simulation Studio
            </Text>
          </Stack>
          <Divider />
          {navigationItems.map((item) => (
            <NavLink
              key={item.id}
              label={item.label}
              leftSection={item.icon}
              active={activeScreen === item.id}
              onClick={() => setActiveScreen(item.id)}
            />
          ))}
        </AppShell.Navbar>
        <AppShell.Main className="main">
          <Workbench activeScreen={activeScreen} />
        </AppShell.Main>
      </AppShell>
    </MantineProvider>
  );
}

function Workbench({ activeScreen }: { activeScreen: ScreenId }) {
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

  const createDemoProject = () => run(backend.createRcLowPassDemoProject, setProject);
  const calculateCutoff = () => run(backend.calculateRcLowPass, setFormulaResult);
  const selectNearestE24 = () => run(backend.nearestE24ForResistor, setPreferredValue);
  const generateNetlist = () => run(backend.generateSpiceNetlist, setNetlist);
  const runSimulation = () => run(backend.runMockAcSimulation, setSimulation);
  const saveProject = () => run(() => backend.saveProjectJson(savePath), () => undefined);
  const exportMarkdown = () => run(backend.exportMarkdownReport, setMarkdownReport);
  const exportHtml = () => run(backend.exportHtmlReport, setHtmlReport);

  return (
    <div className="workbench">
      <header className="toolbar">
        <Group gap="xs">
          <Button
            leftSection={<CircuitBoard size={16} />}
            onClick={createDemoProject}
            loading={busy}
          >
            New RC Demo
          </Button>
          <Button
            variant="light"
            leftSection={<Calculator size={16} />}
            onClick={calculateCutoff}
            disabled={!project}
          >
            Calculate fc
          </Button>
          <Button
            variant="light"
            leftSection={<Sigma size={16} />}
            onClick={selectNearestE24}
            disabled={!project}
          >
            Nearest E24
          </Button>
          <Button
            variant="light"
            leftSection={<FileText size={16} />}
            onClick={generateNetlist}
            disabled={!project}
          >
            Netlist
          </Button>
          <Button
            variant="light"
            leftSection={<Play size={16} />}
            onClick={runSimulation}
            disabled={!project}
          >
            Mock AC
          </Button>
          <Button
            variant="light"
            leftSection={<Save size={16} />}
            onClick={saveProject}
            disabled={!project}
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

      {activeScreen === 'schematic' ? (
        <div className="grid">
          <section className="schematic-panel">
            <SchematicCanvas project={project} />
          </section>

          <aside className="side-panel">
            <ProjectMetrics project={project} formulaResult={formulaResult} preferredValue={preferredValue} simulation={simulation} />
          </aside>

          <section className="bottom-panel">
            <Tabs defaultValue="netlist" className="tabs">
              <Tabs.List>
                <Tabs.Tab value="netlist">Netlist</Tabs.Tab>
                <Tabs.Tab value="graph">Graph</Tabs.Tab>
                <Tabs.Tab value="formula">Formula</Tabs.Tab>
                <Tabs.Tab value="report">Report</Tabs.Tab>
                <Tabs.Tab value="library">Libraries</Tabs.Tab>
              </Tabs.List>
              <Tabs.Panel value="netlist">
                <PreBlock text={netlist || 'Generate SPICE netlist'} />
              </Tabs.Panel>
              <Tabs.Panel value="graph">
                <SimulationChart simulation={simulation} />
              </Tabs.Panel>
              <Tabs.Panel value="formula">
                <FormulaPanel />
              </Tabs.Panel>
              <Tabs.Panel value="report">
                <ReportPanel
                  markdownReport={markdownReport}
                  htmlReport={htmlReport}
                  onMarkdown={exportMarkdown}
                  onHtml={exportHtml}
                  disabled={!project}
                />
              </Tabs.Panel>
              <Tabs.Panel value="library">
                <LibraryPanel />
              </Tabs.Panel>
            </Tabs>
          </section>
        </div>
      ) : (
        <ScreenWorkspace
          activeScreen={activeScreen}
          project={project}
          formulaResult={formulaResult}
          preferredValue={preferredValue}
          simulation={simulation}
          markdownReport={markdownReport}
          htmlReport={htmlReport}
          onCreateDemo={createDemoProject}
          onCalculate={calculateCutoff}
          onNearestE24={selectNearestE24}
          onNetlist={generateNetlist}
          onSimulation={runSimulation}
          onMarkdown={exportMarkdown}
          onHtml={exportHtml}
          busy={busy}
          hasProject={Boolean(project)}
        />
      )}
    </div>
  );
}

function ScreenWorkspace({
  activeScreen,
  project,
  formulaResult,
  preferredValue,
  simulation,
  markdownReport,
  htmlReport,
  onCreateDemo,
  onCalculate,
  onNearestE24,
  onNetlist,
  onSimulation,
  onMarkdown,
  onHtml,
  busy,
  hasProject,
}: {
  activeScreen: ScreenId;
  project: ProjectDto | null;
  formulaResult: FormulaResultDto | null;
  preferredValue: PreferredValueDto | null;
  simulation: SimulationResultDto | null;
  markdownReport: string;
  htmlReport: string;
  onCreateDemo: () => void;
  onCalculate: () => void;
  onNearestE24: () => void;
  onNetlist: () => void;
  onSimulation: () => void;
  onMarkdown: () => void;
  onHtml: () => void;
  busy: boolean;
  hasProject: boolean;
}) {
  if (activeScreen === 'start') {
    return (
      <section className="screen-panel">
        <div className="screen-content">
          <Stack gap="xs">
            <Title order={1}>HotSAS Studio</Title>
            <Text c="dimmed">Hardware-Oriented Schematic Analysis & Simulation Studio</Text>
          </Stack>
          <Group gap="xs">
            <Button leftSection={<CircuitBoard size={16} />} onClick={onCreateDemo} loading={busy}>
              New RC Demo
            </Button>
            <Button variant="light" leftSection={<Calculator size={16} />} onClick={onCalculate} disabled={!hasProject}>
              Calculate fc
            </Button>
            <Button variant="light" leftSection={<Sigma size={16} />} onClick={onNearestE24} disabled={!hasProject}>
              Nearest E24
            </Button>
          </Group>
          <div className="metric-grid">
            <ProjectMetrics project={project} formulaResult={formulaResult} preferredValue={preferredValue} simulation={simulation} />
          </div>
        </div>
      </section>
    );
  }

  if (activeScreen === 'notebook' || activeScreen === 'formulas') {
    return (
      <section className="screen-panel">
        <div className="screen-content">
          <Title order={2}>{activeScreen === 'notebook' ? 'Engineering Notebook' : 'Formula Library'}</Title>
          <FormulaPanel />
        </div>
      </section>
    );
  }

  if (activeScreen === 'components') {
    return (
      <section className="screen-panel">
        <div className="screen-content">
          <Title order={2}>E Component Library</Title>
          <LibraryPanel />
        </div>
      </section>
    );
  }

  if (activeScreen === 'simulation') {
    return (
      <section className="screen-panel">
        <div className="screen-content wide">
          <Group justify="space-between" align="center">
            <Title order={2}>Simulation Results</Title>
            <Button variant="light" leftSection={<Play size={16} />} onClick={onSimulation} disabled={!hasProject}>
              Mock AC
            </Button>
          </Group>
          <SimulationChart simulation={simulation} />
        </div>
      </section>
    );
  }

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>Export Center</Title>
        <Group gap="xs">
          <Button variant="light" leftSection={<FileText size={16} />} onClick={onNetlist} disabled={!hasProject}>
            Netlist
          </Button>
        </Group>
        <ReportPanel
          markdownReport={markdownReport}
          htmlReport={htmlReport}
          onMarkdown={onMarkdown}
          onHtml={onHtml}
          disabled={!hasProject}
        />
      </div>
    </section>
  );
}

function ProjectMetrics({
  project,
  formulaResult,
  preferredValue,
  simulation,
}: {
  project: ProjectDto | null;
  formulaResult: FormulaResultDto | null;
  preferredValue: PreferredValueDto | null;
  simulation: SimulationResultDto | null;
}) {
  return (
    <Stack gap="md">
      <Metric label="Project" value={project?.name ?? '-'} />
      <Metric label="Cutoff" value={formulaResult?.value.display ?? '-'} />
      <Metric label="Nearest E24" value={preferredValue?.nearest.display ?? '-'} />
      <Metric label="Simulation" value={simulation ? `${simulation.status} / ${simulation.graph_series.length} series` : '-'} />
    </Stack>
  );
}

function SchematicCanvas({ project }: { project: ProjectDto | null }) {
  const { nodes, edges } = useMemo(() => {
    if (!project) {
      return { nodes: [], edges: [] };
    }

    const nodes: Node[] = project.schematic.components.map((component) => ({
      id: component.instance_id,
      type: 'default',
      position: { x: component.x, y: component.y },
      data: {
        label: (
          <Stack gap={2}>
            <Text fw={700} size="sm">
              {component.instance_id}
            </Text>
            <Text size="xs" c="dimmed">
              {component.definition_id}
            </Text>
          </Stack>
        ),
      },
    }));

    const edges: Edge[] = project.schematic.wires
      .filter((wire) => wire.from_component_id && wire.to_component_id)
      .map((wire) => ({
        id: wire.id,
        source: wire.from_component_id as string,
        target: wire.to_component_id as string,
        label: wire.net_id,
        type: 'smoothstep',
      }));

    return { nodes, edges };
  }, [project]);

  return (
    <div className="canvas">
      <ReactFlow nodes={nodes} edges={edges} fitView>
        <Background />
        <Controls />
        <MiniMap pannable zoomable />
      </ReactFlow>
    </div>
  );
}

function SimulationChart({ simulation }: { simulation: SimulationResultDto | null }) {
  const ref = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }
    const chart = echarts.init(ref.current);
    const series = simulation?.graph_series ?? [];
    chart.setOption({
      backgroundColor: 'transparent',
      tooltip: { trigger: 'axis' },
      legend: { textStyle: { color: '#c9d2df' } },
      grid: { left: 58, right: 24, top: 36, bottom: 42 },
      xAxis: {
        type: 'log',
        name: 'Hz',
        axisLabel: { color: '#9aa8ba' },
        nameTextStyle: { color: '#9aa8ba' },
        splitLine: { lineStyle: { color: '#263244' } },
      },
      yAxis: {
        type: 'value',
        axisLabel: { color: '#9aa8ba' },
        splitLine: { lineStyle: { color: '#263244' } },
      },
      series: series.map((item: GraphSeriesDto) => ({
        name: `${item.name} (${item.y_unit})`,
        type: 'line',
        showSymbol: false,
        data: item.points,
      })),
    });
    const resize = () => chart.resize();
    window.addEventListener('resize', resize);
    return () => {
      window.removeEventListener('resize', resize);
      chart.dispose();
    };
  }, [simulation]);

  return <div ref={ref} className="chart" />;
}

function FormulaPanel() {
  const { formulaResult, preferredValue } = useHotSasStore();
  return (
    <Stack gap="sm" p="md">
      <Group gap="xs">
        <Badge variant="light">rc_low_pass_cutoff</Badge>
        <Code>{formulaResult?.expression ?? 'fc = 1 / (2*pi*R*C)'}</Code>
      </Group>
      <Text size="sm">fc: {formulaResult?.value.display ?? '-'}</Text>
      <Text size="sm">
        E24: {preferredValue ? `${preferredValue.requested_value.display} -> ${preferredValue.nearest.display}` : '-'}
      </Text>
      <Text size="sm">Error: {preferredValue ? `${preferredValue.error_percent.toFixed(4)}%` : '-'}</Text>
    </Stack>
  );
}

function ReportPanel({
  markdownReport,
  htmlReport,
  disabled,
  onMarkdown,
  onHtml,
}: {
  markdownReport: string;
  htmlReport: string;
  disabled: boolean;
  onMarkdown: () => void;
  onHtml: () => void;
}) {
  return (
    <Stack gap="sm" p="md">
      <Group>
        <Button leftSection={<FileText size={16} />} onClick={onMarkdown} disabled={disabled}>
          Markdown
        </Button>
        <Button variant="light" leftSection={<FileText size={16} />} onClick={onHtml} disabled={disabled}>
          HTML
        </Button>
      </Group>
      <PreBlock text={markdownReport || htmlReport || 'Export report'} />
    </Stack>
  );
}

function LibraryPanel() {
  return (
    <Stack gap="xs" p="md">
      <Text size="sm">Formula packs: basic electronics, filters, op-amp, SMPS placeholders</Text>
      <Text size="sm">Component model: symbol, footprint, simulation model, datasheet, BOM fields</Text>
      <Text size="sm">Export placeholders: PDF, KiCad, Altium workflow package</Text>
    </Stack>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="metric">
      <Text size="xs" c="dimmed">
        {label}
      </Text>
      <Text size="sm" fw={700}>
        {value}
      </Text>
    </div>
  );
}

function PreBlock({ text }: { text: string }) {
  return (
    <ScrollArea className="pre-scroll">
      <pre>{text}</pre>
    </ScrollArea>
  );
}
