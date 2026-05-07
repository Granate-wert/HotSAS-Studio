import { Table, Text } from "@mantine/core";
import type { SimulationMeasurementDto } from "../../types";

interface Props {
  measurements: SimulationMeasurementDto[];
}

export function SimulationMeasurementsTable({ measurements }: Props) {
  if (measurements.length === 0) {
    return (
      <Text size="xs" c="dimmed">
        No measurements
      </Text>
    );
  }

  return (
    <Table size="xs" striped>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Value</Table.Th>
          <Table.Th>Unit</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {measurements.map((m) => (
          <Table.Tr key={m.name}>
            <Table.Td>{m.name}</Table.Td>
            <Table.Td>{m.display}</Table.Td>
            <Table.Td>{m.unit}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );
}
