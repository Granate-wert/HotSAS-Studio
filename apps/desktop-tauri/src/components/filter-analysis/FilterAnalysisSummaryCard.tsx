import { Badge, Card, Group, Stack, Text } from "@mantine/core";
import { Activity, CheckCircle, XCircle } from "lucide-react";
import type { FilterNetworkAnalysisResult } from "../../types";

interface Props {
  result: FilterNetworkAnalysisResult | null;
}

export function FilterAnalysisSummaryCard({ result }: Props) {
  if (!result) {
    return (
      <Card withBorder shadow="sm" padding="sm" radius="md">
        <Text c="dimmed" size="sm">
          Run analysis to see results.
        </Text>
      </Card>
    );
  }

  return (
    <Card withBorder shadow="sm" padding="sm" radius="md">
      <Stack gap="xs">
        <Group justify="space-between">
          <Text fw={600} size="sm">
            Result Summary
          </Text>
          <Badge
            color={result.can_trust_as_engineering_estimate ? "green" : "yellow"}
            size="sm"
            variant="light"
          >
            {result.can_trust_as_engineering_estimate ? "Trusted" : "Estimate"}
          </Badge>
        </Group>
        <Group gap="xs">
          <Badge size="sm" variant="outline">
            {result.detected_filter_kind}
          </Badge>
          <Badge size="sm" variant="outline" color="gray">
            {result.method_used}
          </Badge>
          <Text size="xs" c="dimmed">
            {result.points.length} points
          </Text>
        </Group>
        {result.diagnostics.length > 0 && (
          <Group gap="xs">
            {result.diagnostics.some((d) => d.severity === "error" || d.severity === "blocking") ? (
              <XCircle size={14} color="#e03131" />
            ) : (
              <CheckCircle size={14} color="#2f9e44" />
            )}
            <Text size="xs" c="dimmed">
              {result.diagnostics.length} diagnostic(s)
            </Text>
          </Group>
        )}
      </Stack>
    </Card>
  );
}
