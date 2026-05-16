import { Alert, Badge, Button, Group, Stack, Text } from "@mantine/core";
import { Copy, RotateCw, Trash2 } from "lucide-react";
import type {
  CircuitValidationReportDto,
  ComponentDto,
  NetDto,
  SchematicSelectionDetailsDto,
  SpiceModelReferenceDto,
} from "../../types";
import { SimulationReadinessBadge } from "../component-library/SimulationReadinessBadge";
import { QuickParameterEditor } from "./QuickParameterEditor";

type Props = {
  entity: { kind: "component" | "wire" | "net"; id: string } | null;
  details: SchematicSelectionDetailsDto | null;
  selectedComponent?: ComponentDto | null;
  nets?: NetDto[];
  validationReport?: CircuitValidationReportDto | null;
  onDeleteComponent?: (componentId: string) => void;
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
  selectedComponent,
  nets = [],
  validationReport,
  onDeleteComponent,
  onDeleteWire,
  onUpdateParameter,
  loading,
}: Props) {
  if (!entity) {
    return (
      <Stack gap="xs" p="md" data-testid="engineering-inspector-empty">
        <Text size="sm" fw={700}>
          Engineering Inspector
        </Text>
        <Text size="sm" c="dimmed">
          Select a component or place one from the palette.
        </Text>
        <Text size="xs" c="dimmed">
          Component values, pins, connected nets, model readiness, and diagnostics appear here.
        </Text>
      </Stack>
    );
  }

  const assignment = details?.model_assignment;
  const persistence = getPersistenceStatusLabel(assignment?.model_ref);
  const netNameById = new Map(nets.map((net) => [net.id, net.name]));
  const componentIssues = [
    ...(validationReport?.errors ?? []),
    ...(validationReport?.warnings ?? []),
  ].filter((issue) => issue.component_id === entity.id);
  const primaryValue = selectedComponent?.parameters[0];
  const hasMissingOrStale = assignment?.diagnostics.some(
    (d) => d.severity === "blocking" || d.severity === "error" || d.severity === "warning",
  );

  return (
    <Stack gap="md" p="md" data-testid="engineering-inspector">
      <Stack gap={4}>
        <Text size="xs" fw={700} c="dimmed">
          Component identity
        </Text>
        <Group justify="space-between" align="flex-start">
          <Stack gap={2}>
            <Text size="lg" fw={700}>
              {details?.display_name ?? selectedComponent?.display_label ?? entity.id}
            </Text>
            <Text size="xs" c="dimmed">
              {selectedComponent?.definition_id ?? entity.kind}
            </Text>
          </Stack>
          <Badge variant="light">{entity.kind}</Badge>
        </Group>
      </Stack>

      {entity.kind === "component" && selectedComponent && (
        <>
          <Stack gap={4}>
            <Text size="xs" fw={700} c="dimmed">
              Type and value
            </Text>
            <Text size="sm">{selectedComponent.component_kind}</Text>
            <Text size="sm">
              {primaryValue
                ? `${primaryValue.name}: ${primaryValue.value.display}`
                : "No editable value on this component"}
            </Text>
          </Stack>

          <Stack gap={4}>
            <Text size="xs" fw={700} c="dimmed">
              Pins and connected nets
            </Text>
            {selectedComponent.pins.map((pin) => {
              const connected = selectedComponent.connected_nets.find(
                (net) => net.pin_id === pin.id,
              );
              const netLabel = connected
                ? (netNameById.get(connected.net_id) ?? connected.net_id)
                : "unconnected";
              return (
                <Text key={pin.id} size="xs" c="dimmed">
                  {`${pin.id} -> ${netLabel}`}
                </Text>
              );
            })}
          </Stack>
        </>
      )}

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
        <Stack gap={4}>
          <Text size="xs" fw={700} c="dimmed">
            Editable value
          </Text>
          <QuickParameterEditor
            fields={details.editable_fields}
            componentId={entity.id}
            onUpdate={onUpdateParameter}
            loading={loading}
          />
        </Stack>
      )}

      {entity.kind === "component" && (
        <Stack gap={4}>
          <Group justify="space-between">
            <Text size="xs" fw={700} c="dimmed">
              Model/readiness status
            </Text>
            <Badge color={persistence.color} variant="filled" size="sm">
              {persistence.label}
            </Badge>
          </Group>
          {!assignment && (
            <Text size="xs" c="dimmed">
              No model assignment details loaded.
            </Text>
          )}
        </Stack>
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
                  {`${mapping.component_pin_id} -> ${mapping.model_pin_name}`}
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
                  {`${binding.model_parameter_name} -> ${binding.component_parameter_id}`}
                </Text>
              ))}
            </Stack>
          )}
        </Stack>
      )}

      {entity.kind === "component" && (
        <Stack gap={4}>
          <Text size="xs" fw={700} c="dimmed">
            Warnings/diagnostics
          </Text>
          {componentIssues.length === 0 ? (
            <Text size="xs" c="dimmed">
              No component diagnostics.
            </Text>
          ) : (
            componentIssues.map((issue) => (
              <Text key={`${issue.code}-${issue.message}`} size="xs" c="yellow">
                {issue.code}: {issue.message}
              </Text>
            ))
          )}
        </Stack>
      )}

      {entity.kind === "component" && (
        <Stack gap={4}>
          <Text size="xs" fw={700} c="dimmed">
            Actions
          </Text>
          <Group gap="xs">
            <Button
              size="xs"
              color="red"
              leftSection={<Trash2 size={14} />}
              onClick={() => onDeleteComponent?.(entity.id)}
              loading={loading}
              title={onDeleteComponent ? "Delete selected component" : "Select component first"}
            >
              Delete
            </Button>
            <Button
              size="xs"
              variant="light"
              leftSection={<Copy size={14} />}
              disabled
              title="Duplicate is not implemented yet"
            >
              Duplicate
            </Button>
            <Button
              size="xs"
              variant="light"
              leftSection={<RotateCw size={14} />}
              disabled
              title="Rotation is not implemented yet"
            >
              Rotate
            </Button>
          </Group>
          <Text size="xs" c="dimmed">
            Rotation is not implemented yet
          </Text>
        </Stack>
      )}
    </Stack>
  );
}
