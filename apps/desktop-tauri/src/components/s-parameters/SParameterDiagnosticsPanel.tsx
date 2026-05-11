import { Alert, Stack, Text } from "@mantine/core";
import { AlertCircle, Info, CheckCircle } from "lucide-react";
import type { SParameterDiagnostic } from "../../types";

interface Props {
  diagnostics: SParameterDiagnostic[];
}

function severityColor(severity: string): string {
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

function severityIcon(severity: string) {
  switch (severity) {
    case "blocking":
    case "error":
      return <AlertCircle size={16} />;
    case "warning":
      return <AlertCircle size={16} />;
    case "info":
      return <Info size={16} />;
    default:
      return <CheckCircle size={16} />;
  }
}

export function SParameterDiagnosticsPanel({ diagnostics }: Props) {
  if (diagnostics.length === 0) return null;

  return (
    <Stack gap="xs">
      {diagnostics.map((d) => (
        <Alert key={d.code} color={severityColor(d.severity)} icon={severityIcon(d.severity)}>
          <Text fw={600}>{d.title}</Text>
          <Text size="sm">{d.message}</Text>
          {d.suggested_fix && (
            <Text size="xs" c="dimmed">
              Suggested fix: {d.suggested_fix}
            </Text>
          )}
        </Alert>
      ))}
    </Stack>
  );
}
