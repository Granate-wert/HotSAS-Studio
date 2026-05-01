import { Button, Group, Title } from '@mantine/core';
import { FileText } from 'lucide-react';
import { ReportPanel } from '../components/ReportPanel';

export function ExportScreen({
  markdownReport,
  htmlReport,
  hasProject,
  onNetlist,
  onMarkdown,
  onHtml,
}: {
  markdownReport: string;
  htmlReport: string;
  hasProject: boolean;
  onNetlist: () => void;
  onMarkdown: () => void;
  onHtml: () => void;
}) {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>Export Center</Title>
        <Group gap="xs">
          <Button
            variant="light"
            leftSection={<FileText size={16} />}
            onClick={onNetlist}
            disabled={!hasProject}
          >
            Netlist
          </Button>
        </Group>
        <ReportPanel
          markdownReport={markdownReport}
          htmlReport={htmlReport}
          onMarkdown={onMarkdown}
          onHtml={onHtml}
          disabled={!hasProject}
        />
      </div>
    </section>
  );
}
