import { Alert, Text } from "@mantine/core";
import type { CircuitValidationIssueDto } from "../../types";

export function SchematicEditStatusPanel({ warnings }: { warnings: CircuitValidationIssueDto[] }) {
  if (warnings.length === 0) {
    return (
      <Text size="xs" c="green">
        No validation warnings
      </Text>
    );
  }

  return (
    <div>
      {warnings.map((w, i) => (
        <Alert key={i} color="yellow" mb={4}>
          <Text size="xs">
            {w.code}: {w.message}
          </Text>
        </Alert>
      ))}
    </div>
  );
}
