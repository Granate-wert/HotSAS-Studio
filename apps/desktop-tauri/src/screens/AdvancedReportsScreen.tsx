import { useEffect, useState } from "react";
import {
  Alert,
  Badge,
  Button,
  Card,
  Checkbox,
  Group,
  ScrollArea,
  Select,
  Stack,
  Switch,
  Table,
  Text,
  TextInput,
  Title,
  Tooltip,
} from "@mantine/core";
import {
  AlertCircle,
  CheckCircle,
  FileText,
  FileJson,
  FileSpreadsheet,
  FileCode,
  Loader2,
  RefreshCw,
  Eye,
  Download,
  BarChart3,
} from "lucide-react";
import { PreBlock } from "../components/PreBlock";
import type { AdvancedReportDto, ReportSectionCapabilityDto } from "../types";

const REPORT_TYPE_OPTIONS = [
  { value: "ProjectSummary", label: "Project Summary" },
  { value: "CalculationReport", label: "Calculation Report" },
  { value: "SimulationReport", label: "Simulation Report" },
  { value: "SelectedRegionReport", label: "Selected Region Report" },
  { value: "DcdcDesignReport", label: "DC-DC Design Report" },
  { value: "FullProjectReport", label: "Full Project Report" },
];

function statusColor(status: string): string {
  switch (status.toLowerCase()) {
    case "included":
      return "green";
    case "empty":
      return "yellow";
    case "unavailable":
      return "gray";
    case "error":
      return "red";
    default:
      return "gray";
  }
}

function SectionCard({ section }: { section: AdvancedReportDto["sections"][number] }) {
  return (
    <Card withBorder shadow="sm" padding="sm" radius="md">
      <Group justify="space-between" wrap="nowrap">
        <Group gap="xs">
          <Badge color={statusColor(section.status)} variant="light" size="sm">
            {section.status}
          </Badge>
          <Text fw={600} size="sm">
            {section.title}
          </Text>
        </Group>
        <Text size="xs" c="dimmed">
          {section.kind}
        </Text>
      </Group>

      {section.warnings.length > 0 && (
        <Stack gap={2} mt={6}>
          {section.warnings.map((w, i) => (
            <Group key={i} gap="xs">
              <AlertCircle size={12} color="#f08c00" />
              <Text size="xs" c="orange">
                [{w.severity}] {w.message}
              </Text>
            </Group>
          ))}
        </Stack>
      )}

      {section.blocks.map((block, bi) => (
        <div key={bi} className="report-block" style={{ marginTop: 8 }}>
          {block.block_type === "Paragraph" && block.text && <Text size="sm">{block.text}</Text>}
          {block.block_type === "KeyValueTable" && block.rows && block.rows.length > 0 && (
            <Table withTableBorder>
              <Table.Tbody>
                {block.rows.map((row, ri) => (
                  <Table.Tr key={ri}>
                    <Table.Td fw={600} style={{ width: "40%" }}>
                      {row.key}
                    </Table.Td>
                    <Table.Td>
                      {row.value} {row.unit ?? ""}
                    </Table.Td>
                  </Table.Tr>
                ))}
              </Table.Tbody>
            </Table>
          )}
          {block.block_type === "DataTable" && block.data_rows && (
            <Table withTableBorder>
              <Table.Thead>
                <Table.Tr>
                  {block.columns?.map((col, ci) => (
                    <Table.Th key={ci}>{col}</Table.Th>
                  ))}
                </Table.Tr>
              </Table.Thead>
              <Table.Tbody>
                {block.data_rows.map((dr, dri) => (
                  <Table.Tr key={dri}>
                    {dr.map((cell, ci) => (
                      <Table.Td key={ci}>{cell}</Table.Td>
                    ))}
                  </Table.Tr>
                ))}
              </Table.Tbody>
            </Table>
          )}
          {block.block_type === "FormulaBlock" && (
            <Stack gap={2}>
              {block.equation && (
                <Text size="xs" c="dimmed" fs="italic">
                  {block.equation}
                </Text>
              )}
              {block.substituted_values && block.substituted_values.length > 0 && (
                <Table withTableBorder>
                  <Table.Tbody>
                    {block.substituted_values.map((row, ri) => (
                      <Table.Tr key={ri}>
                        <Table.Td fw={600}>{row.key}</Table.Td>
                        <Table.Td>
                          {row.value} {row.unit ?? ""}
                        </Table.Td>
                      </Table.Tr>
                    ))}
                  </Table.Tbody>
                </Table>
              )}
              {block.result && (
                <Text size="sm" fw={600}>
                  Result: {block.result}
                </Text>
              )}
            </Stack>
          )}
          {block.block_type === "CodeBlock" && block.content && <PreBlock text={block.content} />}
          {block.block_type === "GraphReference" && (
            <Group gap="xs">
              <BarChart3 size={16} color="#868e96" />
              <Text size="xs" c="dimmed">
                Graph: {block.title ?? "Untitled"}
                {block.series_names && ` (${block.series_names.join(", ")})`}
              </Text>
            </Group>
          )}
          {block.block_type === "WarningList" && block.items && (
            <Stack gap={2}>
              {block.items.map((w, wi) => (
                <Group key={wi} gap="xs">
                  <AlertCircle size={12} color="#f08c00" />
                  <Text size="xs" c="orange">
                    [{w.severity}] {w.message}
                  </Text>
                </Group>
              ))}
            </Stack>
          )}
        </div>
      ))}
    </Card>
  );
}

export function AdvancedReportsScreen({
  hasProject,
  capabilities,
  lastReport,
  previewReport,
  exportResult,
  loading,
  error,
  onLoadCapabilities,
  onGenerateReport,
  onExportReport,
}: {
  hasProject: boolean;
  capabilities: ReportSectionCapabilityDto[];
  lastReport: AdvancedReportDto | null;
  previewReport: AdvancedReportDto | null;
  exportResult: string | null;
  loading: boolean;
  error: string | null;
  onLoadCapabilities: () => void;
  onGenerateReport: (reportType: string, includedSections: string[], title: string) => void;
  onExportReport: (reportId: string, format: string, outputPath: string | null) => void;
}) {
  const [reportType, setReportType] = useState("ProjectSummary");
  const [includedSections, setIncludedSections] = useState<string[]>([]);
  const [reportTitle, setReportTitle] = useState("");
  const [exportFormat, setExportFormat] = useState("markdown");
  const [exportPath, setExportPath] = useState("");
  const [hasAttemptedLoad, setHasAttemptedLoad] = useState(false);

  const activeReport = previewReport ?? lastReport;

  useEffect(() => {
    if (!hasAttemptedLoad && capabilities.length === 0) {
      setHasAttemptedLoad(true);
      onLoadCapabilities();
    }
  }, [hasAttemptedLoad, capabilities.length, onLoadCapabilities]);

  useEffect(() => {
    if (capabilities.length > 0 && includedSections.length === 0) {
      const defaults = capabilities.filter((c) => c.default_enabled).map((c) => c.kind);
      setIncludedSections(defaults);
    }
  }, [capabilities, includedSections.length]);

  const handleToggleSection = (kind: string) => {
    setIncludedSections((prev) =>
      prev.includes(kind) ? prev.filter((k) => k !== kind) : [...prev, kind],
    );
  };

  const handleGenerate = () => {
    const title =
      reportTitle || REPORT_TYPE_OPTIONS.find((o) => o.value === reportType)?.label || "Report";
    onGenerateReport(reportType, includedSections, title);
  };

  const handleExport = () => {
    if (!activeReport) return;
    onExportReport(activeReport.id, exportFormat, exportPath || null);
  };

  return (
    <ScrollArea className="screen-container">
      <Stack gap="md" p="md">
        <Title order={2}>Advanced Reports</Title>
        <Text size="sm" c="dimmed">
          Generate structured, multi-section reports from project data, calculations, simulations,
          and design analyses.
        </Text>

        <Group gap="xs">
          <Button
            variant="default"
            size="xs"
            leftSection={loading ? <Loader2 size={14} className="spin" /> : <RefreshCw size={14} />}
            onClick={onLoadCapabilities}
            disabled={loading}
          >
            Refresh Capabilities
          </Button>
        </Group>

        {error && (
          <Alert color="red" icon={<AlertCircle size={16} />}>
            <Text size="sm">{error}</Text>
          </Alert>
        )}

        <Card withBorder shadow="sm" padding="md" radius="md">
          <Stack gap="sm">
            <Text size="sm" fw={500}>
              Report Configuration
            </Text>
            <Select
              label="Report Type"
              value={reportType}
              onChange={(v) => setReportType(v ?? "ProjectSummary")}
              data={REPORT_TYPE_OPTIONS}
              disabled={loading}
            />
            <TextInput
              label="Title (optional)"
              placeholder="Auto-generated from type if empty"
              value={reportTitle}
              onChange={(e) => setReportTitle(e.currentTarget.value)}
              disabled={loading}
            />
          </Stack>
        </Card>

        {capabilities.length > 0 && (
          <Card withBorder shadow="sm" padding="md" radius="md">
            <Stack gap="sm">
              <Text size="sm" fw={500}>
                Sections ({includedSections.length}/{capabilities.length} selected)
              </Text>
              <Group gap="xs">
                <Button
                  variant="default"
                  size="xs"
                  onClick={() => setIncludedSections(capabilities.map((c) => c.kind))}
                >
                  Select All
                </Button>
                <Button variant="default" size="xs" onClick={() => setIncludedSections([])}>
                  Clear All
                </Button>
              </Group>
              <Stack gap="xs">
                {capabilities.map((cap) => {
                  const selected = includedSections.includes(cap.kind);
                  return (
                    <Group key={cap.kind} gap="xs" wrap="nowrap">
                      <Checkbox
                        checked={selected}
                        onChange={() => handleToggleSection(cap.kind)}
                        disabled={loading}
                      />
                      <Tooltip label={cap.description}>
                        <Text size="sm" style={{ flex: 1 }}>
                          {cap.title}
                        </Text>
                      </Tooltip>
                      <Badge size="xs" variant="light" color="gray">
                        {cap.supported_report_types.slice(0, 3).join(", ")}
                        {cap.supported_report_types.length > 3 ? "…" : ""}
                      </Badge>
                    </Group>
                  );
                })}
              </Stack>
            </Stack>
          </Card>
        )}

        <Group gap="xs">
          <Button
            leftSection={<Eye size={16} />}
            onClick={handleGenerate}
            disabled={!hasProject || loading || includedSections.length === 0}
            loading={loading}
          >
            Generate Report
          </Button>
        </Group>

        {activeReport && (
          <>
            <Card withBorder shadow="sm" padding="md" radius="md">
              <Stack gap="sm">
                <Group justify="space-between">
                  <Text size="sm" fw={500}>
                    Export Report
                  </Text>
                  <Group gap="xs">
                    <Badge size="sm" variant="light">
                      {activeReport.report_type}
                    </Badge>
                    <Badge size="sm" variant="outline" color="green">
                      {activeReport.sections.length} sections
                    </Badge>
                  </Group>
                </Group>
                <Select
                  label="Format"
                  value={exportFormat}
                  onChange={(v) => setExportFormat(v ?? "markdown")}
                  data={[
                    { value: "markdown", label: "Markdown" },
                    { value: "html", label: "HTML" },
                    { value: "json", label: "JSON" },
                    { value: "csv_summary", label: "CSV Summary" },
                  ]}
                  disabled={loading}
                />
                <TextInput
                  label="Output Path (optional)"
                  placeholder="Leave empty for in-memory export"
                  value={exportPath}
                  onChange={(e) => setExportPath(e.currentTarget.value)}
                  disabled={loading}
                />
                <Button
                  leftSection={<Download size={16} />}
                  variant="light"
                  onClick={handleExport}
                  disabled={loading}
                  loading={loading}
                >
                  Export
                </Button>
                {exportResult && (
                  <Alert color="green" icon={<CheckCircle size={16} />}>
                    <Text size="sm">{exportResult}</Text>
                  </Alert>
                )}
              </Stack>
            </Card>

            <Card withBorder shadow="sm" padding="md" radius="md">
              <Stack gap="sm">
                <Group justify="space-between">
                  <Text size="sm" fw={500}>
                    Report Preview: {activeReport.title}
                  </Text>
                  {activeReport.generated_at && (
                    <Text size="xs" c="dimmed">
                      Generated: {activeReport.generated_at}
                    </Text>
                  )}
                </Group>

                {activeReport.warnings.length > 0 && (
                  <Alert color="yellow" icon={<AlertCircle size={16} />}>
                    <Stack gap={2}>
                      {activeReport.warnings.map((w, i) => (
                        <Text key={i} size="xs">
                          [{w.severity}] {w.message}
                        </Text>
                      ))}
                    </Stack>
                  </Alert>
                )}

                {activeReport.assumptions.length > 0 && (
                  <Stack gap={2}>
                    <Text size="xs" fw={500} c="dimmed">
                      Assumptions:
                    </Text>
                    {activeReport.assumptions.map((a, i) => (
                      <Text key={i} size="xs" c="dimmed">
                        • {a}
                      </Text>
                    ))}
                  </Stack>
                )}

                <Stack gap="sm">
                  {activeReport.sections.map((section, i) => (
                    <SectionCard key={i} section={section} />
                  ))}
                </Stack>
              </Stack>
            </Card>
          </>
        )}
      </Stack>
    </ScrollArea>
  );
}
