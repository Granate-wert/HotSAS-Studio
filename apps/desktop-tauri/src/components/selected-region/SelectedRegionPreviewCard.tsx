import { Badge, Paper, Stack, Text, Title } from "@mantine/core";
import type { SelectedRegionPreviewDto } from "../../types";

export function SelectedRegionPreviewCard({ preview }: { preview: SelectedRegionPreviewDto }) {
  return (
    <Paper withBorder p="sm">
      <Stack gap="xs">
        <Title order={6}>Preview</Title>
        <Text size="xs">
          Components: {preview.selected_components.length} | Internal nets: {preview.detected_internal_nets.length} | Boundary nets: {preview.detected_boundary_nets.length}
        </Text>

        {preview.suggested_input_nets.length > 0 && (
          <Text size="xs">
            Suggested inputs: {preview.suggested_input_nets.join(", ")}
          </Text>
        )}
        {preview.suggested_output_nets.length > 0 && (
          <Text size="xs">
            Suggested outputs: {preview.suggested_output_nets.join(", ")}
          </Text>
        )}
        {preview.suggested_reference_nodes.length > 0 && (
          <Text size="xs">
            Suggested references: {preview.suggested_reference_nodes.join(", ")}
          </Text>
        )}

        {preview.warnings.length > 0 && (
          <Stack gap={2}>
            {preview.warnings.map((w, i) => (
              <Badge key={i} color="yellow" size="xs" variant="light">
                {w.code}: {w.message}
              </Badge>
            ))}
          </Stack>
        )}

        {preview.errors.length > 0 && (
          <Stack gap={2}>
            {preview.errors.map((e, i) => (
              <Badge key={i} color="red" size="xs" variant="light">
                {e.code}: {e.message}
              </Badge>
            ))}
          </Stack>
        )}
      </Stack>
    </Paper>
  );
}
