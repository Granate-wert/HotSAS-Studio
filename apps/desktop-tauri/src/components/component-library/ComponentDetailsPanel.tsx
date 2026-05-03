import { Card, Group, Stack, Table, Text, Title } from "@mantine/core";
import type { ComponentDetailsDto } from "../../types";
import { ComponentFootprintPreview } from "./ComponentFootprintPreview";
import { ComponentSymbolPreview } from "./ComponentSymbolPreview";

type Props = {
  component: ComponentDetailsDto;
};

export function ComponentDetailsPanel({ component }: Props) {
  return (
    <Stack gap="md">
      <Title order={4}>{component.name}</Title>
      <Group>
        <Text size="sm" c="dimmed">
          ID:
        </Text>
        <Text size="sm">{component.id}</Text>
      </Group>
      <Group>
        <Text size="sm" c="dimmed">
          Category:
        </Text>
        <Text size="sm">{component.category}</Text>
      </Group>
      {component.manufacturer && (
        <Group>
          <Text size="sm" c="dimmed">
            Manufacturer:
          </Text>
          <Text size="sm">{component.manufacturer}</Text>
        </Group>
      )}
      {component.part_number && (
        <Group>
          <Text size="sm" c="dimmed">
            Part Number:
          </Text>
          <Text size="sm">{component.part_number}</Text>
        </Group>
      )}
      {component.description && <Text size="sm">{component.description}</Text>}

      {component.parameters.length > 0 && (
        <Card withBorder>
          <Text fw={500} mb="xs">
            Parameters
          </Text>
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Name</Table.Th>
                <Table.Th>Value</Table.Th>
                <Table.Th>Unit</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {component.parameters.map((p) => (
                <Table.Tr key={p.name}>
                  <Table.Td>{p.name}</Table.Td>
                  <Table.Td>{p.value}</Table.Td>
                  <Table.Td>{p.unit ?? "—"}</Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Card>
      )}

      {component.ratings.length > 0 && (
        <Card withBorder>
          <Text fw={500} mb="xs">
            Ratings
          </Text>
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Name</Table.Th>
                <Table.Th>Value</Table.Th>
                <Table.Th>Unit</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {component.ratings.map((r) => (
                <Table.Tr key={r.name}>
                  <Table.Td>{r.name}</Table.Td>
                  <Table.Td>{r.value}</Table.Td>
                  <Table.Td>{r.unit ?? "—"}</Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Card>
      )}

      {component.symbol_preview && <ComponentSymbolPreview symbol={component.symbol_preview} />}

      {component.footprint_previews.length > 0 && (
        <ComponentFootprintPreview footprints={component.footprint_previews} />
      )}

      {component.tags.length > 0 && (
        <Group>
          <Text size="sm" c="dimmed">
            Tags:
          </Text>
          <Text size="sm">{component.tags.join(", ")}</Text>
        </Group>
      )}

      {component.metadata.length > 0 && (
        <Group>
          <Text size="sm" c="dimmed">
            Metadata:
          </Text>
          <Text size="sm">{component.metadata.map((m) => `${m.key}=${m.value}`).join(", ")}</Text>
        </Group>
      )}
    </Stack>
  );
}
