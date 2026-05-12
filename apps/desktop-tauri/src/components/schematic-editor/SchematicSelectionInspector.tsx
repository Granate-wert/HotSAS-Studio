import { Alert, Badge, Button, Group, Stack, Text } from "@mantine/core";
import { Trash2 } from "lucide-react";
import type { SchematicSelectionDetailsDto, SpiceModelReferenceDto } from "../../types";
import { SimulationReadinessBadge } from "../component-library/SimulationReadinessBadge";
import { QuickParameterEditor } from "./QuickParameterEditor";

type Props = {
  entity: { kind: "component" | "wire" | "net"; id: string } | null;
  details: SchematicSelectionDetailsDto | null;
  onDeleteWire: (wireId: string) => void;
  onUpdateParameter: (componentId: string, parameterId: string, value: string) => void;
  loading?: boolean;
};

function getPersistenceStatusLabel(modelRef: SpiceModelReferenceDto | null | undefined): {
  label: string;
  color: string;
} {
  if (!modelRef) return { label: "No model", color: "gray" };
  const status = modelRef.status?.toLowerCase() ?? "";
  const source = modelRef.source?.toLowerCase() ?? "";
  if (status === "missing" || status === "stale") return { label: "Missing asset", color: "red" };
  if (source === "builtin" || source === "derived_builtin")
    return { label: "Derived builtin", color: "blue" };
  if (source === "imported" || source === "user_assigned")
    return { label: "Persisted", color: "green" };
  if (status === "available" || status === "present")
    return { label: "Package backed", color: "teal" };
  if (source === "generated" || source === "generated_fallback")
    return { label: "Session only", color: "orange" };
  return { label: "Unknown", color: "gray" };
}

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

  const assignment = details?.model_assignment;
  const persistence = getPersistenceStatusLabel(assignment?.model_ref);
  const hasMissingOrStale = assignment?.diagnostics.some(
    (d) => d.severity === "blocking" || d.severity === "error" || d.severity === "warning",
  );

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

      {entity.kind === "component" && assignment && (
        <Stack gap={4}>
          <Group justify="space-between">
            <Text size="sm" fw={600}>
              Model assignment
            </Text>
            <Group gap="xs">
              <Badge variant="light">{details.model_assignment_origin ?? "inherited"}</Badge>
              <Badge color={persistence.color} variant="filled" size="sm">
                {persistence.label}
              </Badge>
            </Group>
          </Group>
          <Text size="xs" c="dimmed">
            {assignment.model_ref?.display_name ?? "No model assigned"}
          </Text>
          {assignment.model_ref?.source && (
            <Text size="xs" c="dimmed">
              Source: {assignment.model_ref.source.replace(/_/g, " ")}
            </Text>
          )}
          <Group gap="xs">
            <Badge variant="outline">{assignment.status.replace(/_/g, " ")}</Badge>
            <SimulationReadinessBadge readiness={assignment.readiness} />
          </Group>

          {hasMissingOrStale && (
            <Alert color="yellow" title="Persistence warning" p="xs">
              <Text size="xs">Missing or stale model asset references detected.</Text>
            </Alert>
          )}

          {assignment.diagnostics.length > 0 && (
            <Stack gap={2}>
              <Text size="xs" fw={500} c="dimmed">
                Diagnostics
              </Text>
              {assignment.diagnostics.map((diagnostic) => (
                <Text key={diagnostic.code} size="xs" c="dimmed">
                  {diagnostic.code}: {diagnostic.message}
                </Text>
              ))}
            </Stack>
          )}

          {assignment.pin_mappings.length > 0 && (
            <Stack gap={2}>
              <Text size="xs" fw={500} c="dimmed">
                Pin mappings ({assignment.pin_mappings.length})
              </Text>
              {assignment.pin_mappings.map((mapping) => (
                <Text key={mapping.component_pin_id} size="xs" c="dimmed">
                  {mapping.component_pin_id} → {mapping.model_pin_name}
                </Text>
              ))}
            </Stack>
          )}

          {assignment.parameter_bindings.length > 0 && (
            <Stack gap={2}>
              <Text size="xs" fw={500} c="dimmed">
                Parameter bindings ({assignment.parameter_bindings.length})
              </Text>
              {assignment.parameter_bindings.map((binding) => (
                <Text key={binding.model_parameter_name} size="xs" c="dimmed">
                  {binding.model_parameter_name} → {binding.component_parameter_id}
                </Text>
              ))}
            </Stack>
          )}
        </Stack>
      )}
    </Stack>
  );
}
