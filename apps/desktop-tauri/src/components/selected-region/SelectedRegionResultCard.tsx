import { Badge, Code, Group, Paper, Stack, Text, Title } from "@mantine/core";
import type { SelectedRegionAnalysisResultDto } from "../../types";

export function SelectedRegionResultCard({ result }: { result: SelectedRegionAnalysisResultDto }) {
  return (
    <Paper withBorder p="sm">
      <Stack gap="xs">
        <Group justify="space-between">
          <Title order={6}>Analysis Result</Title>
          <Badge
            size="xs"
            color={
              result.status === "Success" ? "green" : result.status === "Partial" ? "yellow" : "red"
            }
          >
            {result.status}
          </Badge>
        </Group>
        <Text size="xs">{result.summary}</Text>

        {result.matched_template && (
          <Paper withBorder p="xs">
            <Text size="xs" fw={700}>
              Template: {result.matched_template.title}
            </Text>
            <Text size="xs">
              Confidence: {(result.matched_template.confidence * 100).toFixed(0)}%
            </Text>
            <Text size="xs">{result.matched_template.explanation}</Text>
          </Paper>
        )}

        {result.transfer_function && (
          <Paper withBorder p="xs">
            <Text size="xs" fw={700}>
              Transfer Function
            </Text>
            <Code block>{result.transfer_function.expression}</Code>
            {result.transfer_function.availability_note && (
              <Text size="xs" c="dimmed">
                {result.transfer_function.availability_note}
              </Text>
            )}
          </Paper>
        )}

        {result.netlist_fragment && (
          <Paper withBorder p="xs">
            <Text size="xs" fw={700}>
              Netlist Fragment ({result.netlist_fragment.format})
            </Text>
            <Code block>{result.netlist_fragment.content}</Code>
          </Paper>
        )}

        {result.measurements.length > 0 && (
          <Stack gap={2}>
            <Text size="xs" fw={700}>
              Measurements
            </Text>
            {result.measurements.map((m, i) => (
              <Text key={i} size="xs">
                {m.name}: {m.value ? m.value.display : "—"} — {m.description}
              </Text>
            ))}
          </Stack>
        )}

        {result.warnings.length > 0 && (
          <Stack gap={2}>
            {result.warnings.map((w, i) => (
              <Badge key={i} color="yellow" size="xs" variant="light">
                {w.code}: {w.message}
              </Badge>
            ))}
          </Stack>
        )}

        {result.errors.length > 0 && (
          <Stack gap={2}>
            {result.errors.map((e, i) => (
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
