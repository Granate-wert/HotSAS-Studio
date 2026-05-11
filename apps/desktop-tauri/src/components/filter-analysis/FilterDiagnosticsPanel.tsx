import { Alert, Badge, Group, Stack, Text, Title } from "@mantine/core";
import { AlertCircle, Info, ShieldAlert, XCircle } from "lucide-react";
import type { FilterAnalysisDiagnostic, FilterAnalysisSeverity } from "../../types";

interface Props {
  diagnostics: FilterAnalysisDiagnostic[];
}

function severityColor(severity: FilterAnalysisSeverity): string {
  switch (severity) {
    case "blocking":
      return "red";
    case "error":
      return "red";
    case "warning":
      return "yellow";
    case "info":
      return "blue";
    default:
      return "gray";
  }
}

function severityIcon(severity: FilterAnalysisSeverity) {
  const size = 16;
  switch (severity) {
    case "blocking":
      return <XCircle size={size} />;
    case "error":
      return <ShieldAlert size={size} />;
    case "warning":
      return <AlertCircle size={size} />;
    case "info":
      return <Info size={size} />;
    default:
      return <Info size={size} />;
  }
}

export function FilterDiagnosticsPanel({ diagnostics }: Props) {
  if (diagnostics.length === 0) {
    return (
      <Text c="dimmed" size="sm">
        No diagnostics.
      </Text>
    );
  }

  return (
    <Stack gap="xs">
      <Title order={5}>Diagnostics</Title>
      {diagnostics.map((d, i) => (
        <Alert
          key={`${d.code}-${i}`}
          color={severityColor(d.severity)}
          icon={severityIcon(d.severity)}
          variant="light"
        >
          <Stack gap={2}>
            <Group justify="space-between" wrap="nowrap">
              <Text size="sm" fw={600}>
                {d.title}
              </Text>
              <Badge color={severityColor(d.severity)} size="sm" variant="outline">
                {d.severity}
              </Badge>
            </Group>
            <Text size="sm">{d.message}</Text>
            {d.suggested_fix && (
              <Text size="xs" c="dimmed">
                Suggested fix: {d.suggested_fix}
              </Text>
            )}
            {d.related_component_id && (
              <Text size="xs" c="dimmed">
                Related component: {d.related_component_id}
              </Text>
            )}
          </Stack>
        </Alert>
      ))}
    </Stack>
  );
}
