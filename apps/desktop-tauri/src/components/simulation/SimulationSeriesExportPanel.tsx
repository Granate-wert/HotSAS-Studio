import { Button, Group, Stack, Text } from "@mantine/core";
import { Download, FileJson } from "lucide-react";

interface Props {
  onExportCsv?: () => void;
  onExportJson?: () => void;
  loading?: boolean;
  lastExportContent?: string | null;
  lastExportFormat?: "csv" | "json" | null;
}

export function SimulationSeriesExportPanel({
  onExportCsv,
  onExportJson,
  loading,
  lastExportContent,
  lastExportFormat,
}: Props) {
  return (
    <Stack gap="xs">
      <Text size="sm" fw={500}>
        Export Series
      </Text>

      <Group gap="xs">
        {onExportCsv && (
          <Button
            variant="light"
            size="xs"
            leftSection={<Download size={14} />}
            onClick={onExportCsv}
            loading={loading}
          >
            Export CSV
          </Button>
        )}
        {onExportJson && (
          <Button
            variant="light"
            size="xs"
            leftSection={<FileJson size={14} />}
            onClick={onExportJson}
            loading={loading}
          >
            Export JSON
          </Button>
        )}
      </Group>

      {lastExportContent && lastExportFormat && (
        <Stack gap="xs">
          <Text size="xs" c="green">
            Exported {lastExportFormat.toUpperCase()} ready
          </Text>
          <Text size="xs" c="dimmed" lineClamp={4} style={{ whiteSpace: "pre-wrap" }}>
            {lastExportContent}
          </Text>
        </Stack>
      )}
    </Stack>
  );
}
