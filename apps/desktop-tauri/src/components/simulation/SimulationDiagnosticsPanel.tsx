import { Alert, Badge, Group, Stack, Text } from "@mantine/core";
import type { SimulationDiagnosticMessageDto } from "../../types";

interface Props {
  diagnostics: SimulationDiagnosticMessageDto[];
  loading?: boolean;
}

function severityColor(severity: string): string {
  switch (severity) {
    case "Blocking":
      return "red";
    case "Error":
      return "red";
    case "Warning":
      return "yellow";
    case "Info":
      return "blue";
    default:
      return "gray";
  }
}

function severityLabel(severity: string): string {
  switch (severity) {
    case "Blocking":
      return "Blocking";
    case "Error":
      return "Error";
    case "Warning":
      return "Warning";
    case "Info":
      return "Info";
    default:
      return severity;
  }
}

export function SimulationDiagnosticsPanel({ diagnostics, loading }: Props) {
  if (loading) {
    return (
      <Text size="xs" c="dimmed">
        Checking diagnostics...
      </Text>
    );
  }

  if (diagnostics.length === 0) {
    return (
      <Alert color="green" variant="light" title="Diagnostics OK">
        No simulation diagnostics issues found
      </Alert>
    );
  }

  const blocking = diagnostics.filter((d) => d.severity === "Blocking");
  const errors = diagnostics.filter((d) => d.severity === "Error");
  const warnings = diagnostics.filter((d) => d.severity === "Warning");
  const infos = diagnostics.filter((d) => d.severity === "Info");

  return (
    <Stack gap="xs">
      <Group gap="xs">
        {blocking.length > 0 && (
          <Badge color="red" size="sm">
            {blocking.length} Blocking
          </Badge>
        )}
        {errors.length > 0 && (
          <Badge color="red" size="sm">
            {errors.length} Error{errors.length > 1 ? "s" : ""}
          </Badge>
        )}
        {warnings.length > 0 && (
          <Badge color="yellow" size="sm">
            {warnings.length} Warning{warnings.length > 1 ? "s" : ""}
          </Badge>
        )}
        {infos.length > 0 && (
          <Badge color="blue" size="sm">
            {infos.length} Info
          </Badge>
        )}
      </Group>

      {diagnostics.map((d, i) => (
        <Alert
          key={`${d.code}-${i}`}
          color={severityColor(d.severity)}
          variant="light"
          title={`[${severityLabel(d.severity)}] ${d.code}: ${d.title}`}
        >
          <Text size="xs">{d.message}</Text>
          {d.suggested_fix && (
            <Text size="xs" c="dimmed" mt={4}>
              <strong>Suggested fix:</strong> {d.suggested_fix}
            </Text>
          )}
          {d.related_entity && (
            <Text size="xs" c="dimmed" mt={4}>
              Related: {d.related_entity.kind} ({d.related_entity.id})
            </Text>
          )}
        </Alert>
      ))}
    </Stack>
  );
}
