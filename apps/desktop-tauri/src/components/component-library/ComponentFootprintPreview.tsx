import { Card, Group, Table, Text } from "@mantine/core";
import type { FootprintDto } from "../../types";

type Props = {
  footprints: FootprintDto[];
};

export function ComponentFootprintPreview({ footprints }: Props) {
  return (
    <Card withBorder>
      <Text fw={500} mb="xs">
        Footprint Previews
      </Text>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>ID</Table.Th>
            <Table.Th>Name</Table.Th>
            <Table.Th>Package</Table.Th>
            <Table.Th>Pads</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {footprints.map((fp) => (
            <Table.Tr key={fp.id}>
              <Table.Td>{fp.id}</Table.Td>
              <Table.Td>{fp.name}</Table.Td>
              <Table.Td>{fp.package_name}</Table.Td>
              <Table.Td>{fp.pad_count}</Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
      {footprints.map((fp) =>
        fp.metadata.length > 0 ? (
          <Group key={`meta-${fp.id}`} mt="xs">
            <Text size="xs" c="dimmed">
              {fp.name} metadata:
            </Text>
            <Text size="xs">{fp.metadata.map((m) => `${m.key}=${m.value}`).join(", ")}</Text>
          </Group>
        ) : null,
      )}
    </Card>
  );
}
