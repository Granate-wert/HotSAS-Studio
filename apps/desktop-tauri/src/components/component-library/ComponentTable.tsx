import { Badge, Table } from "@mantine/core";
import type { ComponentSummaryDto } from "../../types";

type Props = {
  components: ComponentSummaryDto[];
  selectedId: string | null;
  onSelect: (id: string) => void;
};

export function ComponentTable({ components, selectedId, onSelect }: Props) {
  if (components.length === 0) {
    return <div>No components found.</div>;
  }

  const rows = components.map((c) => (
    <Table.Tr
      key={c.id}
      style={{ cursor: "pointer" }}
      bg={selectedId === c.id ? "blue.0" : undefined}
      onClick={() => onSelect(c.id)}
    >
      <Table.Td>{c.name}</Table.Td>
      <Table.Td>{c.category}</Table.Td>
      <Table.Td>{c.manufacturer ?? "—"}</Table.Td>
      <Table.Td>{c.part_number ?? "—"}</Table.Td>
      <Table.Td>{c.has_symbol ? "Yes" : "No"}</Table.Td>
      <Table.Td>{c.has_footprint ? "Yes" : "No"}</Table.Td>
      <Table.Td>{c.has_simulation_model ? "Yes" : "No"}</Table.Td>
      <Table.Td>
        {c.tags.map((t) => (
          <Badge key={t} size="xs" mr={4}>
            {t}
          </Badge>
        ))}
      </Table.Td>
    </Table.Tr>
  ));

  return (
    <Table highlightOnHover withTableBorder>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Name</Table.Th>
          <Table.Th>Category</Table.Th>
          <Table.Th>Manufacturer</Table.Th>
          <Table.Th>Part Number</Table.Th>
          <Table.Th>Symbol</Table.Th>
          <Table.Th>Footprint</Table.Th>
          <Table.Th>Model</Table.Th>
          <Table.Th>Tags</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>{rows}</Table.Tbody>
    </Table>
  );
}
