import { Table, Badge } from "@mantine/core";
import type { SParameterMetric } from "../../types";

interface Props {
  metrics: SParameterMetric[];
}

function confidenceColor(confidence: string): string {
  switch (confidence) {
    case "high":
      return "green";
    case "medium":
      return "yellow";
    case "low":
      return "red";
    default:
      return "gray";
  }
}

export function SParameterMetricsTable({ metrics }: Props) {
  if (metrics.length === 0) return null;

  return (
    <Table striped highlightOnHover>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Metric</Table.Th>
          <Table.Th>Value</Table.Th>
          <Table.Th>Unit</Table.Th>
          <Table.Th>Frequency</Table.Th>
          <Table.Th>Confidence</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {metrics.map((m) => (
          <Table.Tr key={m.id}>
            <Table.Td>{m.label}</Table.Td>
            <Table.Td>{m.value.toFixed(4)}</Table.Td>
            <Table.Td>{m.unit}</Table.Td>
            <Table.Td>{m.frequency_hz ? `${m.frequency_hz.toExponential(2)} Hz` : "—"}</Table.Td>
            <Table.Td>
              <Badge color={confidenceColor(m.confidence)} size="sm">
                {m.confidence}
              </Badge>
            </Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );
}
