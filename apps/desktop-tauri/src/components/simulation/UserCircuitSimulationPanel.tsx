import { Stack, Text } from "@mantine/core";
import { useCallback, useEffect, useState } from "react";
import { backend } from "../../api";
import { useHotSasStore } from "../../store";
import { SimulationPreflightCard } from "./SimulationPreflightCard";
import { SimulationProfileSelector } from "./SimulationProfileSelector";
import { SimulationProbeSelector } from "./SimulationProbeSelector";
import { SimulationRunControls } from "./SimulationRunControls";
import { UserCircuitSimulationResults } from "./UserCircuitSimulationResults";

export function UserCircuitSimulationPanel() {
  const store = useHotSasStore();

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
      store.setSelectedSimulationProbes(probes.slice(0, 3));
    } catch (e) {
      // non-critical
    }
  }, []);

  useEffect(() => {
    loadProfiles();
    loadProbes();
  }, [loadProfiles, loadProbes]);

  const handlePreflight = async () => {
    if (!store.selectedSimulationProfile) return;
    try {
      store.setSimulationWorkflowLoading(true);
      const preflight = await backend.validateCurrentCircuitForSimulation(
        store.selectedSimulationProfile,
      );
      store.setSimulationPreflight(preflight);
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

  return (
    <Stack gap="md">
      <Text size="sm" fw={600}>
        Simulation Setup
      </Text>
      <SimulationProfileSelector
        profiles={store.simulationProfiles}
        selected={store.selectedSimulationProfile}
        onSelect={store.setSelectedSimulationProfile}
        loading={store.simulationWorkflowLoading}
      />
      <SimulationProbeSelector
        probes={store.simulationProbes}
        selected={store.selectedSimulationProbes}
        onChange={store.setSelectedSimulationProbes}
        loading={store.simulationWorkflowLoading}
      />
      <SimulationRunControls
        canRun={store.simulationPreflight?.can_run ?? false}
        loading={store.simulationWorkflowLoading}
        onRun={handleRun}
        onPreflight={handlePreflight}
      />
      <SimulationPreflightCard preflight={store.simulationPreflight} />
      <UserCircuitSimulationResults
        run={store.currentSimulationRun}
        viewMode={store.simulationResultViewMode}
        onChangeViewMode={store.setSimulationResultViewMode}
        onAddToReport={handleAddToReport}
        loading={store.simulationWorkflowLoading}
      />
    </Stack>
  );
}
