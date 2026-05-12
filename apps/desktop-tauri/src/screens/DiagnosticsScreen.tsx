import {
  Alert,
  Badge,
  Button,
  Card,
  Group,
  ScrollArea,
  Stack,
  Table,
  Text,
  Title,
} from "@mantine/core";
import { AlertCircle, CheckCircle, Loader2, RefreshCw, ShieldAlert } from "lucide-react";
import type { AppDiagnosticsReportDto } from "../types";

function statusColor(status: string): string {
  switch (status.toLowerCase()) {
    case "ready":
    case "pass":
      return "green";
    case "limited":
    case "warn":
      return "yellow";
    case "unavailable":
    case "fail":
      return "red";
    default:
      return "gray";
  }
}

function statusIcon(status: string) {
  const color = statusColor(status);
  if (color === "green") return <CheckCircle size={16} color="#2f9e44" />;
  if (color === "yellow") return <AlertCircle size={16} color="#f08c00" />;
  if (color === "red") return <ShieldAlert size={16} color="#e03131" />;
  return <AlertCircle size={16} color="#868e96" />;
}

function ModuleCard({ module }: { module: AppDiagnosticsReportDto["modules"][number] }) {
  return (
    <Card withBorder shadow="sm" padding="sm" radius="md">
      <Group justify="space-between" wrap="nowrap">
        <Group gap="xs">
          {statusIcon(module.status)}
          <Text fw={600} size="sm">
            {module.title}
          </Text>
        </Group>
        <Badge color={statusColor(module.status)} variant="light" size="sm">
          {module.status}
        </Badge>
      </Group>
      <Text size="xs" c="dimmed" mt={4}>
        {module.summary}
      </Text>
      {Object.entries(module.details).length > 0 && (
        <Stack gap={2} mt={6}>
          {Object.entries(module.details).map(([k, v]) => (
            <Text key={k} size="xs" c="dimmed">
              <strong>{k}:</strong> {v}
            </Text>
          ))}
        </Stack>
      )}
    </Card>
  );
}

export function DiagnosticsScreen({
  diagnostics,
  readinessResult,
  loading,
  error,
  onRefreshDiagnostics,
  onRunSelfCheck,
}: {
  diagnostics: AppDiagnosticsReportDto | null;
  readinessResult: AppDiagnosticsReportDto | null;
  loading: boolean;
  error: string | null;
  onRefreshDiagnostics: () => void;
  onRunSelfCheck: () => void;
}) {
  const activeReport = readinessResult ?? diagnostics;

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <ScrollArea className="screen-container">
          <Stack gap="md" p="md">
            <Title order={3}>Internal Alpha / Diagnostics</Title>

            <Alert color="blue" icon={<CheckCircle size={16} />}>
              <Text size="sm" fw={600}>
                v2.0 Product Beta Readiness
              </Text>
              <Text size="xs" c="dimmed">
                Diagnostics shows module readiness for the integrated workflow. ngspice unavailable
                is a controlled warning, not a failure. Public release has not been created.
              </Text>
            </Alert>

            {diagnostics && (
              <Group gap="xs">
                <Text size="sm" c="dimmed">
                  {diagnostics.app_name} v{diagnostics.app_version}
                </Text>
                <Badge size="sm" variant="outline">
                  {diagnostics.roadmap_stage}
                </Badge>
                <Badge size="sm" variant="outline" color="gray">
                  {diagnostics.build_profile}
                </Badge>
              </Group>
            )}

            <Group gap="sm">
              <Button
                leftSection={
                  loading ? <Loader2 size={16} className="spin" /> : <RefreshCw size={16} />
                }
                onClick={onRefreshDiagnostics}
                disabled={loading}
                size="sm"
              >
                Refresh diagnostics
              </Button>
              <Button
                leftSection={
                  loading ? <Loader2 size={16} className="spin" /> : <ShieldAlert size={16} />
                }
                onClick={onRunSelfCheck}
                disabled={loading}
                variant="light"
                size="sm"
              >
                Run readiness self-check
              </Button>
            </Group>

            {error && (
              <Alert color="red" icon={<AlertCircle size={16} />}>
                {error}
              </Alert>
            )}

            {activeReport && (
              <>
                {activeReport.warnings.length > 0 && (
                  <Stack gap="xs">
                    {activeReport.warnings.map((w, i) => (
                      <Alert key={i} color="yellow" icon={<AlertCircle size={16} />}>
                        {w}
                      </Alert>
                    ))}
                  </Stack>
                )}

                <Title order={5}>Modules</Title>
                <Stack gap="sm">
                  {activeReport.modules.map((m) => (
                    <ModuleCard key={m.id} module={m} />
                  ))}
                </Stack>

                {activeReport.checks.length > 0 && (
                  <>
                    <Title order={5}>Readiness Checks</Title>
                    <Table striped highlightOnHover withTableBorder>
                      <Table.Thead>
                        <Table.Tr>
                          <Table.Th>Check</Table.Th>
                          <Table.Th>Status</Table.Th>
                          <Table.Th>Message</Table.Th>
                        </Table.Tr>
                      </Table.Thead>
                      <Table.Tbody>
                        {activeReport.checks.map((c) => (
                          <Table.Tr key={c.id}>
                            <Table.Td>
                              <Text size="sm" fw={500}>
                                {c.title}
                              </Text>
                            </Table.Td>
                            <Table.Td>
                              <Badge color={statusColor(c.status)} size="sm">
                                {c.status}
                              </Badge>
                            </Table.Td>
                            <Table.Td>
                              <Text size="sm">{c.message}</Text>
                            </Table.Td>
                          </Table.Tr>
                        ))}
                      </Table.Tbody>
                    </Table>
                  </>
                )}
              </>
            )}

            {!activeReport && !loading && !error && (
              <Text c="dimmed" size="sm">
                No diagnostics loaded yet. Click Refresh diagnostics to begin.
              </Text>
            )}
          </Stack>
        </ScrollArea>
      </div>
    </section>
  );
}
