import { Alert, Card, Group, Stack, Text } from "@mantine/core";
import type { SimulationGraphViewDto } from "../../types";
import { SimulationSeriesChart } from "./SimulationSeriesChart";

interface Props {
  graphView: SimulationGraphViewDto | null;
  visibleSeries: Record<string, boolean>;
  loading?: boolean;
  error?: string | null;
}

export function SimulationGraphView({ graphView, visibleSeries, loading, error }: Props) {
  if (loading) {
    return (
      <Text size="xs" c="dimmed">
        Loading graph view...
      </Text>
    );
  }

  if (error) {
    return (
      <Alert color="red" variant="light" title="Graph Error">
        {error}
      </Alert>
    );
  }

  if (!graphView) {
    return (
      <Text size="xs" c="dimmed">
        No graph view available. Run a simulation first.
      </Text>
    );
  }

  const filteredSeries = graphView.series
    .filter((s) => visibleSeries[s.id] !== false)
    .map((s) => ({
      id: s.id,
      label: s.label,
      x_unit: graphView.x_axis.unit,
      y_unit: graphView.y_axis.unit,
      points: [],
    }));

  return (
    <Card withBorder>
      <Stack gap="xs">
        <Group justify="space-between" align="center">
          <Text fw={600}>{graphView.title}</Text>
          <Text size="xs" c="dimmed">
            Run: {graphView.run_id}
          </Text>
        </Group>

        <Group gap="xs">
          <Text size="xs" c="dimmed">
            X: {graphView.x_axis.label} ({graphView.x_axis.scale})
            {graphView.x_axis.unit ? ` — ${graphView.x_axis.unit}` : ""}
          </Text>
          <Text size="xs" c="dimmed">
            Y: {graphView.y_axis.label} ({graphView.y_axis.scale})
            {graphView.y_axis.unit ? ` — ${graphView.y_axis.unit}` : ""}
          </Text>
        </Group>

        {filteredSeries.length > 0 ? (
          <SimulationSeriesChart series={filteredSeries} />
        ) : (
          <Text size="xs" c="dimmed">
            All series are hidden. Use the controls above to show at least one series.
          </Text>
        )}
      </Stack>
    </Card>
  );
}
