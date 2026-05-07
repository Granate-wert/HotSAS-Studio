import { Checkbox, Group, Stack, Text } from "@mantine/core";
import type { SimulationGraphSeriesDto } from "../../types";

interface Props {
  series: SimulationGraphSeriesDto[];
  visibleSeries: Record<string, boolean>;
  onToggleSeries: (seriesId: string, visible: boolean) => void;
}

export function SimulationGraphControls({ series, visibleSeries, onToggleSeries }: Props) {
  const visibleCount = series.filter((s) => visibleSeries[s.id] !== false).length;

  return (
    <Stack gap="xs">
      <Group justify="space-between" align="center">
        <Text size="sm" fw={500}>
          Series Visibility
        </Text>
        <Text size="xs" c="dimmed">
          {visibleCount} / {series.length} visible
        </Text>
      </Group>

      {series.length === 0 && (
        <Text size="xs" c="dimmed">
          No series available
        </Text>
      )}

      <Group gap="sm">
        {series.map((s) => (
          <Checkbox
            key={s.id}
            label={
              <Text size="xs">
                {s.label} ({s.points_count} pts)
              </Text>
            }
            checked={visibleSeries[s.id] !== false}
            onChange={(event) => onToggleSeries(s.id, event.currentTarget.checked)}
          />
        ))}
      </Group>
    </Stack>
  );
}
