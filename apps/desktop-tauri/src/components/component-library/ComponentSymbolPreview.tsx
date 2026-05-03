import { Card, Group, Table, Text } from "@mantine/core";
import type { SymbolDto } from "../../types";

type Props = {
  symbol: SymbolDto;
};

export function ComponentSymbolPreview({ symbol }: Props) {
  return (
    <Card withBorder>
      <Text fw={500} mb="xs">
        Symbol Preview
      </Text>
      <Group>
        <Text size="sm" c="dimmed">
          ID:
        </Text>
        <Text size="sm">{symbol.id}</Text>
      </Group>
      <Group>
        <Text size="sm" c="dimmed">
          Kind:
        </Text>
        <Text size="sm">{symbol.component_kind}</Text>
      </Group>
      <Group>
        <Text size="sm" c="dimmed">
          Size:
        </Text>
        <Text size="sm">
          {symbol.width} × {symbol.height}
        </Text>
      </Group>
      {symbol.pins.length > 0 && (
        <Table mt="xs">
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Pin</Table.Th>
              <Table.Th>Name</Table.Th>
              <Table.Th>Type</Table.Th>
              <Table.Th>Side</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {symbol.pins.map((pin) => (
              <Table.Tr key={pin.id}>
                <Table.Td>{pin.number}</Table.Td>
                <Table.Td>{pin.name}</Table.Td>
                <Table.Td>{pin.electrical_type}</Table.Td>
                <Table.Td>{pin.side}</Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      )}
    </Card>
  );
}
