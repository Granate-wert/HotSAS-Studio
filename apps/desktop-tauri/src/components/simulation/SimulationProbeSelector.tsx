import { Checkbox, Stack, Text } from "@mantine/core";
import type { SimulationProbeDto } from "../../types";

interface Props {
  probes: SimulationProbeDto[];
  selected: SimulationProbeDto[];
  onChange: (probes: SimulationProbeDto[]) => void;
  loading?: boolean;
}

export function SimulationProbeSelector({ probes, selected, onChange, loading }: Props) {
  const selectedIds = new Set(selected.map((p) => p.id));

  return (
    <Stack gap="xs">
      <Text size="sm" fw={500}>
        Probes
      </Text>
      {probes.length === 0 && (
        <Text size="xs" c="dimmed">
          No probes suggested
        </Text>
      )}
      {probes.map((probe) => (
        <Checkbox
          key={probe.id}
          label={probe.label}
          checked={selectedIds.has(probe.id)}
          onChange={(event) => {
            const checked = event.currentTarget.checked;
            let next: SimulationProbeDto[];
            if (checked) {
              next = [...selected, probe];
            } else {
              next = selected.filter((p) => p.id !== probe.id);
            }
            onChange(next);
          }}
          disabled={loading}
        />
      ))}
    </Stack>
  );
}
