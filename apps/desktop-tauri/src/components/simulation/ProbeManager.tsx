import { Button, Checkbox, Group, Stack, Text } from "@mantine/core";
import { RotateCcw } from "lucide-react";
import type { SimulationProbeDto } from "../../types";

interface Props {
  probes: SimulationProbeDto[];
  selected: SimulationProbeDto[];
  onChange: (probes: SimulationProbeDto[]) => void;
  onSetDefaults?: () => void;
  loading?: boolean;
}

export function ProbeManager({ probes, selected, onChange, onSetDefaults, loading }: Props) {
  const selectedIds = new Set(selected.map((p) => p.id));

  const toggleProbe = (probe: SimulationProbeDto, checked: boolean) => {
    let next: SimulationProbeDto[];
    if (checked) {
      next = [...selected, probe];
    } else {
      next = selected.filter((p) => p.id !== probe.id);
    }
    onChange(next);
  };

  return (
    <Stack gap="xs">
      <Group justify="space-between" align="center">
        <Text size="sm" fw={500}>
          Probes
        </Text>
        {onSetDefaults && (
          <Button
            variant="light"
            size="xs"
            leftSection={<RotateCcw size={14} />}
            onClick={onSetDefaults}
            loading={loading}
          >
            Defaults
          </Button>
        )}
      </Group>

      {probes.length === 0 && (
        <Text size="xs" c="dimmed">
          No probes suggested
        </Text>
      )}

      {probes.map((probe) => {
        const unsupported =
          probe.kind === "DifferentialVoltage" || probe.kind === "ComponentCurrent";
        return (
          <Checkbox
            key={probe.id}
            label={
              <Stack gap={0}>
                <Text size="xs">{probe.label}</Text>
                {unsupported && (
                  <Text size="xs" c="dimmed">
                    Unsupported: {probe.kind}
                  </Text>
                )}
              </Stack>
            }
            checked={selectedIds.has(probe.id)}
            onChange={(event) => toggleProbe(probe, event.currentTarget.checked)}
            disabled={loading || unsupported}
          />
        );
      })}
    </Stack>
  );
}
