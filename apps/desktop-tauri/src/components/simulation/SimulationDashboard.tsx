import { Divider, Stack, Tabs, Text } from "@mantine/core";
import {
  Activity,
  BarChart3,
  ClipboardCheck,
  History,
  Radio,
  Share2,
  Stethoscope,
} from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { backend } from "../../api";
import { useHotSasStore } from "../../store";
import type { SimulationProbeDto, UserCircuitSimulationProfileDto } from "../../types";
import { NgspiceDiagnosticsCard } from "./NgspiceDiagnosticsCard";
import { ProbeManager } from "./ProbeManager";
import { SimulationDiagnosticsPanel } from "./SimulationDiagnosticsPanel";
import { SimulationGraphControls } from "./SimulationGraphControls";
import { SimulationGraphView } from "./SimulationGraphView";
import { SimulationPreflightCard } from "./SimulationPreflightCard";
import { SimulationProfileSelector } from "./SimulationProfileSelector";
import { SimulationRunControls } from "./SimulationRunControls";
import { SimulationRunHistoryPanel } from "./SimulationRunHistoryPanel";
import { SimulationSeriesExportPanel } from "./SimulationSeriesExportPanel";
import { UserCircuitSimulationResults } from "./UserCircuitSimulationResults";

export function SimulationDashboard() {
  const store = useHotSasStore();
  const [activeTab, setActiveTab] = useState<string>("setup");
  const [exportLoading, setExportLoading] = useState(false);
  const [lastExportContent, setLastExportContent] = useState<string | null>(null);
  const [lastExportFormat, setLastExportFormat] = useState<"csv" | "json" | null>(null);

  const loadProfiles = useCallback(async () => {
    try {
      store.setSimulationWorkflowLoading(true);
      const profiles = await backend.listUserCircuitSimulationProfiles();
      store.setSimulationProfiles(profiles);
      if (profiles.length > 0 && !store.selectedSimulationProfile) {
        store.setSelectedSimulationProfile(profiles[0]);
      }
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    } finally {
      store.setSimulationWorkflowLoading(false);
    }
  }, []);

  const loadProbes = useCallback(async () => {
    try {
      const probes = await backend.suggestUserCircuitSimulationProbes();
      store.setSimulationProbes(probes);
      if (store.selectedSimulationProbes.length === 0) {
        store.setSelectedSimulationProbes(probes.slice(0, 3));
      }
    } catch (e) {
      // non-critical
    }
  }, []);

  const loadNgspiceDiagnostics = useCallback(async () => {
    try {
      store.setSimulationDiagnosticsLoading(true);
      const diag = await backend.checkNgspiceDiagnostics();
      store.setNgspiceDiagnostics(diag);
    } catch (e) {
      store.setSimulationDiagnosticsError(String(e));
    } finally {
      store.setSimulationDiagnosticsLoading(false);
    }
  }, []);

  const loadHistory = useCallback(async () => {
    try {
      const history = await backend.listSimulationHistory();
      store.setSimulationRunHistory(history);
    } catch (e) {
      // non-critical
    }
  }, []);

  useEffect(() => {
    loadProfiles();
    loadProbes();
    loadNgspiceDiagnostics();
    loadHistory();
  }, [loadProfiles, loadProbes, loadNgspiceDiagnostics, loadHistory]);

  const handlePreflight = async () => {
    if (!store.selectedSimulationProfile) return;
    try {
      store.setSimulationWorkflowLoading(true);
      const preflight = await backend.validateCurrentCircuitForSimulation(
        store.selectedSimulationProfile,
      );
      store.setSimulationPreflight(preflight);

      const diagnostics = await backend.diagnoseSimulationPreflight(
        store.selectedSimulationProfile,
      );
      store.setSimulationDiagnostics(diagnostics);
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    } finally {
      store.setSimulationWorkflowLoading(false);
    }
  };

  const handleRun = async () => {
    if (!store.selectedSimulationProfile) return;
    try {
      store.setSimulationWorkflowLoading(true);
      store.setCurrentSimulationRun(null);
      const run = await backend.runCurrentCircuitSimulation(store.selectedSimulationProfile);
      store.setCurrentSimulationRun(run);
      store.setLastSimulationRun(run);

      await backend.addRunToHistory();
      const history = await backend.listSimulationHistory();
      store.setSimulationRunHistory(history);

      const graphView = await backend.buildSimulationGraphView();
      store.setSimulationGraphView(graphView);
      if (graphView) {
        const visibility: Record<string, boolean> = {};
        graphView.series.forEach((s) => {
          visibility[s.id] = s.visible_by_default;
        });
        store.setSimulationGraphVisibleSeries(visibility);
      }

      const lastRunDiagnostics = await backend.diagnoseLastSimulationRun();
      store.setSimulationDiagnostics(lastRunDiagnostics);

      setActiveTab("results");
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    } finally {
      store.setSimulationWorkflowLoading(false);
    }
  };

  const handleAddToReport = async () => {
    try {
      store.setSimulationWorkflowLoading(true);
      await backend.addLastSimulationToAdvancedReport();
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    } finally {
      store.setSimulationWorkflowLoading(false);
    }
  };

  const handleDeleteRun = async (runId: string) => {
    try {
      await backend.deleteSimulationHistoryRun(runId);
      const history = await backend.listSimulationHistory();
      store.setSimulationRunHistory(history);
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    }
  };

  const handleClearHistory = async () => {
    try {
      await backend.clearSimulationHistory();
      store.setSimulationRunHistory([]);
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    }
  };

  const handleExportCsv = async () => {
    try {
      setExportLoading(true);
      const csv = await backend.exportRunSeriesCsv();
      setLastExportContent(csv);
      setLastExportFormat("csv");
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    } finally {
      setExportLoading(false);
    }
  };

  const handleExportJson = async () => {
    try {
      setExportLoading(true);
      const json = await backend.exportRunSeriesJson();
      setLastExportContent(json);
      setLastExportFormat("json");
    } catch (e) {
      store.setSimulationWorkflowError(String(e));
    } finally {
      setExportLoading(false);
    }
  };

  const handleToggleSeries = (seriesId: string, visible: boolean) => {
    store.setSimulationGraphVisibleSeries({
      ...store.simulationGraphVisibleSeries,
      [seriesId]: visible,
    });
  };

  const handleSetDefaultProbes = () => {
    const defaults = store.simulationProbes.slice(0, 3);
    store.setSelectedSimulationProbes(defaults);
  };

  const handleSelectProfile = (profile: UserCircuitSimulationProfileDto | null) => {
    store.setSelectedSimulationProfile(profile);
  };

  const handleSelectProbes = (probes: SimulationProbeDto[]) => {
    store.setSelectedSimulationProbes(probes);
  };

  return (
    <Stack gap="md">
      <Text size="lg" fw={700}>
        Simulation Dashboard
      </Text>

      <NgspiceDiagnosticsCard
        diagnostics={store.ngspiceDiagnostics}
        loading={store.simulationDiagnosticsLoading}
        onRefresh={loadNgspiceDiagnostics}
      />

      <Divider />

      <Tabs value={activeTab} onChange={(v) => v && setActiveTab(v)}>
        <Tabs.List>
          <Tabs.Tab value="setup" leftSection={<ClipboardCheck size={16} />}>
            Setup
          </Tabs.Tab>
          <Tabs.Tab value="diagnostics" leftSection={<Stethoscope size={16} />}>
            Diagnostics
          </Tabs.Tab>
          <Tabs.Tab value="results" leftSection={<Activity size={16} />}>
            Results
          </Tabs.Tab>
          <Tabs.Tab value="graph" leftSection={<BarChart3 size={16} />}>
            Graph
          </Tabs.Tab>
          <Tabs.Tab value="history" leftSection={<History size={16} />}>
            History
          </Tabs.Tab>
          <Tabs.Tab value="export" leftSection={<Share2 size={16} />}>
            Export
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="setup" pt="md">
          <Stack gap="md">
            <SimulationProfileSelector
              profiles={store.simulationProfiles}
              selected={store.selectedSimulationProfile}
              onSelect={handleSelectProfile}
              loading={store.simulationWorkflowLoading}
            />
            <ProbeManager
              probes={store.simulationProbes}
              selected={store.selectedSimulationProbes}
              onChange={handleSelectProbes}
              onSetDefaults={handleSetDefaultProbes}
              loading={store.simulationWorkflowLoading}
            />
            <SimulationRunControls
              canRun={store.simulationPreflight?.can_run ?? false}
              loading={store.simulationWorkflowLoading}
              onRun={handleRun}
              onPreflight={handlePreflight}
            />
            <SimulationPreflightCard preflight={store.simulationPreflight} />
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="diagnostics" pt="md">
          <Stack gap="md">
            <Text size="sm" fw={600}>
              Preflight & Run Diagnostics
            </Text>
            <SimulationDiagnosticsPanel
              diagnostics={store.simulationDiagnostics}
              loading={store.simulationDiagnosticsLoading}
            />
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="results" pt="md">
          <UserCircuitSimulationResults
            run={store.currentSimulationRun}
            viewMode={store.simulationResultViewMode}
            onChangeViewMode={store.setSimulationResultViewMode}
            onAddToReport={handleAddToReport}
            loading={store.simulationWorkflowLoading}
          />
        </Tabs.Panel>

        <Tabs.Panel value="graph" pt="md">
          <Stack gap="md">
            <SimulationGraphControls
              series={store.simulationGraphView?.series ?? []}
              visibleSeries={store.simulationGraphVisibleSeries}
              onToggleSeries={handleToggleSeries}
            />
            <SimulationGraphView
              graphView={store.simulationGraphView}
              visibleSeries={store.simulationGraphVisibleSeries}
              loading={store.simulationWorkflowLoading}
              error={store.simulationWorkflowError}
            />
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="history" pt="md">
          <SimulationRunHistoryPanel
            history={store.simulationRunHistory}
            loading={store.simulationWorkflowLoading}
            onDeleteRun={handleDeleteRun}
            onClearHistory={handleClearHistory}
          />
        </Tabs.Panel>

        <Tabs.Panel value="export" pt="md">
          <SimulationSeriesExportPanel
            onExportCsv={handleExportCsv}
            onExportJson={handleExportJson}
            loading={exportLoading}
            lastExportContent={lastExportContent}
            lastExportFormat={lastExportFormat}
          />
        </Tabs.Panel>
      </Tabs>
    </Stack>
  );
}
