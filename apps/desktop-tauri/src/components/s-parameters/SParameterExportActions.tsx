import { Button, Group } from "@mantine/core";
import { Download, FilePlus } from "lucide-react";

interface Props {
  onExportCsv: () => void;
  onAddToReport: () => void;
  loading: boolean;
}

export function SParameterExportActions({ onExportCsv, onAddToReport, loading }: Props) {
  return (
    <Group gap="sm">
      <Button leftSection={<Download size={16} />} onClick={onExportCsv} loading={loading} variant="light">
        Export CSV
      </Button>
      <Button leftSection={<FilePlus size={16} />} onClick={onAddToReport} loading={loading} variant="light">
        Add to Report
      </Button>
    </Group>
  );
}
