import { Badge, Table, Text } from "@mantine/core";
import type { FilterMetricValue, FilterMetricConfidence } from "../../types";

interface Props {
  metrics: FilterMetricValue[];
}

function confidenceColor(confidence: FilterMetricConfidence): string {
  switch (confidence) {
    case "exact":
      return "green";
    case "estimated":
      return "blue";
    case "approximate":
      return "yellow";
    case "not_available":
      return "gray";
    default:
      return "gray";
  }
}

function formatMetricValue(value: number | null, unit: string): string {
  if (value === null) return "—";
  if (Math.abs(value) >= 1000) return `${value.toFixed(2)} ${unit}`;
  if (Math.abs(value) >= 1) return `${value.toFixed(3)} ${unit}`;
  return `${value.toExponential(3)} ${unit}`;
}

export function FilterMetricsTable({ metrics }: Props) {
  if (metrics.length === 0) {
    return (
      <Text c="dimmed" size="sm">
        No metrics available.
      </Text>
    );
  }

  return (
    <Table striped highlightOnHover withTableBorder>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Metric</Table.Th>
          <Table.Th>Value</Table.Th>
          <Table.Th>Confidence</Table.Th>
          <Table.Th>Note</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {metrics.map((m, i) => (
          <Table.Tr key={`${m.kind}-${i}`}>
            <Table.Td>
              <Text size="sm" fw={500}>
                {m.label}
              </Text>
            </Table.Td>
            <Table.Td>
              <Text size="sm">{formatMetricValue(m.value, m.unit)}</Text>
            </Table.Td>
            <Table.Td>
              <Badge color={confidenceColor(m.confidence)} size="sm" variant="light">
                {m.confidence}
              </Badge>
            </Table.Td>
            <Table.Td>
              <Text size="sm" c="dimmed">
                {m.note ?? "—"}
              </Text>
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );
}
