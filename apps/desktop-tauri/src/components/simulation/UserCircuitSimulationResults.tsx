import { Alert, Button, Group, SegmentedControl, Stack, Text } from "@mantine/core";
import type { UserCircuitSimulationRunDto } from "../../types";
import { SimulationMeasurementsTable } from "./SimulationMeasurementsTable";
import { SimulationRawOutputCard } from "./SimulationRawOutputCard";
import { SimulationSeriesChart } from "./SimulationSeriesChart";

interface Props {
  run: UserCircuitSimulationRunDto | null;
  viewMode: "graph" | "table" | "netlist";
  onChangeViewMode: (mode: "graph" | "table" | "netlist") => void;
  onAddToReport: () => void;
  loading?: boolean;
}

export function UserCircuitSimulationResults({
  run,
  viewMode,
  onChangeViewMode,
  onAddToReport,
  loading,
}: Props) {
  if (!run) {
    return (
      <Text size="xs" c="dimmed">
        No simulation run yet
      </Text>
    );
  }

  const result = run.result;
  const hasSeries = result && result.series.length > 0;
  const hasMeasurements = result && result.summary.length > 0;

  return (
    <Stack gap="sm">
      <Group justify="space-between">
        <Text size="sm" fw={500}>
          Results: {run.status}
        </Text>
        <Group gap="xs">
          <SegmentedControl
            size="xs"
            value={viewMode}
            onChange={(v) => onChangeViewMode(v as "graph" | "table" | "netlist")}
            data={[
              { label: "Graph", value: "graph" },
              { label: "Table", value: "table" },
              { label: "Netlist", value: "netlist" },
            ]}
          />
          <Button size="xs" onClick={onAddToReport} loading={loading} disabled={!result}>
            Add to Report
          </Button>
        </Group>
      </Group>

      <Text size="xs">
        Engine: {run.engine_used} | Profile: {run.profile.name}
      </Text>

      {run.errors.length > 0 &&
        run.errors.map((e) => (
          <Alert key={e.code} color="red" variant="light" title={e.code}>
            {e.message}
          </Alert>
        ))}
      {run.warnings.length > 0 &&
        run.warnings.map((w) => (
          <Alert key={w.code} color="yellow" variant="light" title={w.code}>
            {w.message}
          </Alert>
        ))}

      {viewMode === "graph" && hasSeries && <SimulationSeriesChart series={result!.series} />}
      {viewMode === "graph" && !hasSeries && (
        <Text size="xs" c="dimmed">
          No series data
        </Text>
      )}

      {viewMode === "table" && hasMeasurements && (
        <SimulationMeasurementsTable measurements={result!.summary} />
      )}
      {viewMode === "table" && !hasMeasurements && (
        <Text size="xs" c="dimmed">
          No measurements
        </Text>
      )}

      {viewMode === "netlist" && <SimulationRawOutputCard netlist={run.generated_netlist} />}
    </Stack>
  );
}
