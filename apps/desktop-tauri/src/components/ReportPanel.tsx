import { Button, Group, Stack } from "@mantine/core";
import { FileText } from "lucide-react";
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
      <PreBlock text={markdownReport || htmlReport || "Export report"} />
    </Stack>
  );
}
