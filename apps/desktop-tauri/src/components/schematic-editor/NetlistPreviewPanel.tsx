import { Code, ScrollArea, Stack, Text } from "@mantine/core";
import type { NetlistPreviewDto } from "../../types";

type Props = {
  preview: NetlistPreviewDto | null;
  loading?: boolean;
};

export function NetlistPreviewPanel({ preview, loading }: Props) {
  if (loading) {
    return (
      <Text size="sm" c="dimmed" p="md">
        Generating netlist preview...
      </Text>
    );
  }

  if (!preview) {
    return (
      <Text size="sm" c="dimmed" p="md">
        Click the "Netlist Preview" tab to generate a current schematic netlist preview.
      </Text>
    );
  }

  return (
    <Stack gap="xs" p="md">
      {preview.errors.length > 0 && (
        <div style={{ background: "#ff444433", padding: 8, borderRadius: 4 }}>
          <Text size="xs" fw={700} c="red">
            Errors ({preview.errors.length})
          </Text>
          {preview.errors.map((err, i) => (
            <Text key={i} size="xs" c="red">
              {err}
            </Text>
          ))}
        </div>
      )}
      {preview.warnings.length > 0 && (
        <div style={{ background: "#ffaa0033", padding: 8, borderRadius: 4 }}>
          <Text size="xs" fw={700} c="orange">
            Warnings ({preview.warnings.length})
          </Text>
          {preview.warnings.map((warn, i) => (
            <Text key={i} size="xs" c="orange">
              {warn}
            </Text>
          ))}
        </div>
      )}
      <ScrollArea h={200}>
        <Code block style={{ whiteSpace: "pre-wrap" }}>
          {preview.netlist}
        </Code>
      </ScrollArea>
    </Stack>
  );
}
