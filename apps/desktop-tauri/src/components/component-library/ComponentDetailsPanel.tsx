import { Card, Group, Stack, Table, Text, Title } from "@mantine/core";
import type { ComponentDetailsDto, TypedComponentParametersDto, ValueDto } from "../../types";
import { ComponentFootprintPreview } from "./ComponentFootprintPreview";
import { ComponentSymbolPreview } from "./ComponentSymbolPreview";

type Props = {
  component: ComponentDetailsDto;
  typedParams?: TypedComponentParametersDto | null;
};

export function ComponentDetailsPanel({ component, typedParams }: Props) {
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

      {typedParams && typedParams.bundle.kind !== "generic" && (
        <Card withBorder>
          <Text fw={500} mb="xs">
            Typed Parameters
          </Text>
          <TypedParameterView bundle={typedParams.bundle} />
        </Card>
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

function TypedParameterView({ bundle }: { bundle: TypedComponentParametersDto["bundle"] }) {
  const rows: Array<{ label: string; value: ValueDto | null }> = [];

  switch (bundle.kind) {
    case "resistor":
      rows.push({ label: "Resistance", value: bundle.resistance });
      if (bundle.power_rating) rows.push({ label: "Power Rating", value: bundle.power_rating });
      break;
    case "capacitor":
      rows.push({ label: "Capacitance", value: bundle.capacitance });
      if (bundle.voltage_rating)
        rows.push({ label: "Voltage Rating", value: bundle.voltage_rating });
      break;
    case "inductor":
      rows.push({ label: "Inductance", value: bundle.inductance });
      if (bundle.current_rating)
        rows.push({ label: "Current Rating", value: bundle.current_rating });
      break;
    case "diode":
      if (bundle.forward_voltage)
        rows.push({ label: "Forward Voltage", value: bundle.forward_voltage });
      if (bundle.reverse_voltage)
        rows.push({ label: "Reverse Voltage", value: bundle.reverse_voltage });
      break;
    case "bjt":
      if (bundle.vce_max) rows.push({ label: "VCE Max", value: bundle.vce_max });
      if (bundle.ic_max) rows.push({ label: "IC Max", value: bundle.ic_max });
      break;
    case "mosfet":
      if (bundle.vds_max) rows.push({ label: "VDS Max", value: bundle.vds_max });
      if (bundle.id_max) rows.push({ label: "ID Max", value: bundle.id_max });
      if (bundle.rds_on) rows.push({ label: "RDS(on)", value: bundle.rds_on });
      break;
    case "op_amp":
      if (bundle.gbw) rows.push({ label: "Gain-Bandwidth", value: bundle.gbw });
      if (bundle.input_offset_voltage)
        rows.push({ label: "Input Offset Voltage", value: bundle.input_offset_voltage });
      break;
    case "regulator":
      if (bundle.output_voltage)
        rows.push({ label: "Output Voltage", value: bundle.output_voltage });
      if (bundle.max_current) rows.push({ label: "Max Current", value: bundle.max_current });
      break;
    default:
      return null;
  }

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          <Table.Th>Parameter</Table.Th>
          <Table.Th>Value</Table.Th>
          <Table.Th>Unit</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {rows.map((r, i) => (
          <Table.Tr key={i}>
            <Table.Td>{r.label}</Table.Td>
            <Table.Td>{r.value?.display ?? r.value?.original ?? "—"}</Table.Td>
            <Table.Td>{r.value?.unit ?? "—"}</Table.Td>
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );
}
