import { Button, Group, Stack, Text } from "@mantine/core";
import { FileText, Info } from "lucide-react";
import { PreBlock } from "./PreBlock";

export function ReportPanel({
  markdownReport,
  htmlReport,
  disabled,
  onMarkdown,
  onHtml,
}: {
  markdownReport: string;
  htmlReport: string;
  disabled: boolean;
  onMarkdown: () => void;
  onHtml: () => void;
}) {
  const hasReport = markdownReport || htmlReport;

  return (
    <Stack gap="sm" p="md">
      <Group>
        <Button leftSection={<FileText size={16} />} onClick={onMarkdown} disabled={disabled}>
          Markdown
        </Button>
        <Button
          variant="light"
          leftSection={<FileText size={16} />}
          onClick={onHtml}
          disabled={disabled}
        >
          HTML
        </Button>
      </Group>
      {disabled && (
        <Text size="xs" c="dimmed">
          <Info size={12} style={{ display: "inline", marginRight: 4 }} />
          Open or create a project to generate a report.
        </Text>
      )}
      {hasReport ? (
        <PreBlock text={markdownReport || htmlReport} />
      ) : (
        <Stack align="center" gap="xs" py="xl">
          <FileText size={32} color="#56657a" />
          <Text size="sm" c="dimmed" ta="center">
            No report generated yet.
          </Text>
          <Text size="xs" c="dimmed" ta="center">
            Generate a report from the Export screen or after running a simulation.
          </Text>
        </Stack>
      )}
    </Stack>
  );
}
