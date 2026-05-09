import { Badge, Button, Group, Stack, Text } from "@mantine/core";
import { Trash2 } from "lucide-react";
import type { SchematicSelectionDetailsDto } from "../../types";
import { SimulationReadinessBadge } from "../component-library/SimulationReadinessBadge";
import { QuickParameterEditor } from "./QuickParameterEditor";

type Props = {
  entity: { kind: "component" | "wire" | "net"; id: string } | null;
  details: SchematicSelectionDetailsDto | null;
  onDeleteWire: (wireId: string) => void;
  onUpdateParameter: (componentId: string, parameterId: string, value: string) => void;
  loading?: boolean;
};

export function SchematicSelectionInspector({
  entity,
  details,
  onDeleteWire,
  onUpdateParameter,
  loading,
}: Props) {
  if (!entity) {
    return (
      <Text size="sm" c="dimmed" p="md">
        Select a component, wire, or net on the canvas to view details.
      </Text>
    );
  }

  return (
    <Stack gap="xs" p="md">
      <Group justify="space-between">
        <Text size="sm" fw={700}>
          {details?.display_name ?? entity.id}
        </Text>
        <Text size="xs" c="dimmed">
          {entity.kind}
        </Text>
      </Group>

      {entity.kind === "wire" && (
        <Button
          size="xs"
          color="red"
          leftSection={<Trash2 size={14} />}
          onClick={() => onDeleteWire(entity.id)}
          loading={loading}
        >
          Delete Wire
        </Button>
      )}

      {entity.kind === "component" && details?.editable_fields && (
        <QuickParameterEditor
          fields={details.editable_fields}
          componentId={entity.id}
          onUpdate={onUpdateParameter}
          loading={loading}
        />
      )}

      {entity.kind === "component" && details?.model_assignment && (
        <Stack gap={4}>
          <Group justify="space-between">
            <Text size="sm" fw={600}>
              Model assignment
            </Text>
            <Badge variant="light">{details.model_assignment_origin ?? "inherited"}</Badge>
          </Group>
          <Text size="xs" c="dimmed">
            {details.model_assignment.model_ref?.display_name ?? "No model assigned"}
          </Text>
          <Group gap="xs">
            <Badge variant="outline">{details.model_assignment.status.replace(/_/g, " ")}</Badge>
            <SimulationReadinessBadge readiness={details.model_assignment.readiness} />
          </Group>
          {details.model_assignment.diagnostics.length > 0 && (
            <Text size="xs" c="dimmed">
              {details.model_assignment.diagnostics.map((diagnostic) => diagnostic.code).join(", ")}
            </Text>
          )}
        </Stack>
      )}
    </Stack>
  );
}
