import { Card, Text, Group, Badge } from "@mantine/core";
import type { SParameterDataset } from "../../types";

interface Props {
  dataset: SParameterDataset;
}

export function SParameterSummaryCard({ dataset }: Props) {
  return (
    <Card withBorder shadow="sm" p="md">
      <Group gap="sm" mb="xs">
        <Text fw={700}>{dataset.name}</Text>
        <Badge size="sm">{dataset.port_count}-port</Badge>
        <Badge size="sm" color="gray">
          {dataset.reference_impedance_ohm} Ω
        </Badge>
      </Group>
      <Group gap="sm">
        <Text size="sm" c="dimmed">
          Points: {dataset.points.length}
        </Text>
        <Text size="sm" c="dimmed">
          Frequency unit: {dataset.frequency_unit}
        </Text>
        <Text size="sm" c="dimmed">
          Format: {dataset.parameter_format}
        </Text>
      </Group>
    </Card>
  );
}
