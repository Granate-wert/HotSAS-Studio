import { Button, Group } from "@mantine/core";
import { Download, FileText } from "lucide-react";

interface Props {
  onExportCsv: () => void;
  onAddToReport: () => void;
  loading: boolean;
}

export function FilterAnalysisExportActions({ onExportCsv, onAddToReport, loading }: Props) {
  return (
    <Group gap="sm">
      <Button
        leftSection={<Download size={16} />}
        variant="light"
        size="sm"
        onClick={onExportCsv}
        disabled={loading}
      >
        Export CSV
      </Button>
      <Button
        leftSection={<FileText size={16} />}
        variant="light"
        size="sm"
        onClick={onAddToReport}
        disabled={loading}
      >
        Add to Report
      </Button>
    </Group>
  );
}
