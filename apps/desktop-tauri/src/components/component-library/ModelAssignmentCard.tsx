import { Alert, Badge, Button, Card, Group, Select, Stack, Table, Text } from "@mantine/core";
import { useMemo, useState } from "react";
import type { ComponentModelAssignmentDto, SpiceModelReferenceDto } from "../../types";
import { SimulationReadinessBadge } from "./SimulationReadinessBadge";

type Props = {
  assignment: ComponentModelAssignmentDto | null;
  availableModels: SpiceModelReferenceDto[];
  loading?: boolean;
  onAssignModel: (modelId: string) => void;
};

export function ModelAssignmentCard({
  assignment,
  availableModels,
  loading = false,
  onAssignModel,
}: Props) {
  const modelOptions = useMemo(
    () =>
      availableModels.map((model) => ({
        value: model.id,
        label: `${model.id} (${formatLabel(model.status)})`,
      })),
    [availableModels],
  );
  const [selectedModelId, setSelectedModelId] = useState<string | null>(
    assignment?.model_ref?.id ?? modelOptions[0]?.value ?? null,
  );

  if (!assignment) {
    return (
      <Card withBorder radius="sm">
        <Text fw={500}>Model Assignment</Text>
        <Text size="sm" c="dimmed">
          No component instance selected.
        </Text>
      </Card>
    );
  }

  return (
    <Card withBorder radius="sm">
      <Stack gap="sm">
        <Group justify="space-between" align="center">
          <Text fw={500}>Model Assignment</Text>
          <Badge variant="light">{formatLabel(assignment.status)}</Badge>
        </Group>

        <SimulationReadinessBadge readiness={assignment.readiness} />

        <Stack gap={4}>
          <Text size="xs" c="dimmed">
            Current model
          </Text>
          <Text size="sm">{assignment.model_ref?.display_name ?? "No model assigned"}</Text>
        </Stack>

        {modelOptions.length > 0 && (
          <Group align="end">
            <Select
              label="Available models"
              data={modelOptions}
              value={selectedModelId}
              onChange={setSelectedModelId}
              disabled={loading}
            />
            <Button
              onClick={() => selectedModelId && onAssignModel(selectedModelId)}
              disabled={!selectedModelId || loading}
            >
              Assign model
            </Button>
          </Group>
        )}

        {assignment.pin_mappings.length > 0 && (
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Component pin</Table.Th>
                <Table.Th>Model pin</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {assignment.pin_mappings.map((mapping) => (
                <Table.Tr key={`${mapping.component_pin_id}-${mapping.model_pin_name}`}>
                  <Table.Td>{mapping.component_pin_id}</Table.Td>
                  <Table.Td>
                    {mapping.component_pin_id} -&gt; {mapping.model_pin_name}
                  </Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        )}

        {assignment.parameter_bindings.length > 0 && (
          <Stack gap={4}>
            <Text size="sm" fw={500}>
              Parameter bindings
            </Text>
            {assignment.parameter_bindings.map((binding) => (
              <Text
                key={`${binding.model_parameter_name}-${binding.component_parameter_id}`}
                size="sm"
              >
                {binding.model_parameter_name} -&gt; {binding.component_parameter_id}
              </Text>
            ))}
          </Stack>
        )}

        {assignment.diagnostics.map((diagnostic) => (
          <Alert
            key={`${diagnostic.code}-${diagnostic.related_component_id ?? ""}`}
            color={diagnostic.severity === "blocking" ? "red" : "yellow"}
            title={diagnostic.title}
          >
            <Stack gap={4}>
              <Badge variant="light">{diagnostic.code}</Badge>
              <Text size="sm">{diagnostic.message}</Text>
              {diagnostic.suggested_fix && <Text size="sm">{diagnostic.suggested_fix}</Text>}
            </Stack>
          </Alert>
        ))}
      </Stack>
    </Card>
  );
}

function formatLabel(value: string) {
  return value.replace(/_/g, " ");
}
