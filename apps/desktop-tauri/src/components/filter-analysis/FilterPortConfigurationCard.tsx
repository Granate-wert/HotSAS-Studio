import { Card, Group, Select, Stack, Text } from "@mantine/core";
import type { CircuitAnalysisPort } from "../../types";

interface Props {
  ports: CircuitAnalysisPort[];
  inputPort: CircuitAnalysisPort | null;
  outputPort: CircuitAnalysisPort | null;
  onInputChange: (port: CircuitAnalysisPort | null) => void;
  onOutputChange: (port: CircuitAnalysisPort | null) => void;
}

export function FilterPortConfigurationCard({
  ports,
  inputPort,
  outputPort,
  onInputChange,
  onOutputChange,
}: Props) {
  const portOptions = ports.map((p) => ({
    value: p.label,
    label: `${p.label}${p.nominal_impedance_ohm ? ` (${p.nominal_impedance_ohm} Ω)` : ""}`,
  }));

  return (
    <Card withBorder shadow="sm" padding="sm" radius="md">
      <Stack gap="sm">
        <Text fw={600} size="sm">
          Port Configuration
        </Text>
        <Group gap="sm" align="flex-end">
          <Select
            label="Input Port"
            placeholder="Select input port"
            data={portOptions}
            value={inputPort?.label ?? null}
            onChange={(value) => {
              const port = ports.find((p) => p.label === value) ?? null;
              onInputChange(port);
            }}
            style={{ flex: 1 }}
            clearable
          />
          <Select
            label="Output Port"
            placeholder="Select output port"
            data={portOptions}
            value={outputPort?.label ?? null}
            onChange={(value) => {
              const port = ports.find((p) => p.label === value) ?? null;
              onOutputChange(port);
            }}
            style={{ flex: 1 }}
            clearable
          />
        </Group>
        {ports.length === 0 && (
          <Text c="dimmed" size="xs">
            No ports suggested. Create or open a project first.
          </Text>
        )}
      </Stack>
    </Card>
  );
}
