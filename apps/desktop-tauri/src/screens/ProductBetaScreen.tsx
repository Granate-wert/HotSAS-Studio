import { Button, Group, Stack, Text, Title, Badge, Card, Alert } from "@mantine/core";
import {
  Activity,
  AlertTriangle,
  Calculator,
  CheckCircle2,
  CircuitBoard,
  FileText,
  HeartPulse,
  Play,
  RefreshCw,
  Rocket,
  Sigma,
  TableProperties,
  Upload,
  XCircle,
} from "lucide-react";
import type { ProductWorkflowStatusDto } from "../types";

const statusColor: Record<string, string> = {
  ready: "green",
  limited: "yellow",
  unavailable: "red",
  not_configured: "gray",
  error: "red",
};

const statusIcon: Record<string, React.ReactNode> = {
  ready: <CheckCircle2 size={14} />,
  limited: <AlertTriangle size={14} />,
  unavailable: <XCircle size={14} />,
  not_configured: <XCircle size={14} />,
  error: <XCircle size={14} />,
};

const stepScreenMap: Record<string, string> = {
  project: "start",
  schematic: "schematic",
  formula_library: "formulas",
  engineering_notebook: "notebook",
  component_library: "components",
  model_import: "import",
  simulation: "simulation",
  selected_region: "schematic",
  export_center: "export",
  diagnostics: "diagnostics",
};

const stepIconMap: Record<string, React.ReactNode> = {
  project: <CircuitBoard size={16} />,
  schematic: <CircuitBoard size={16} />,
  formula_library: <Sigma size={16} />,
  engineering_notebook: <Calculator size={16} />,
  component_library: <TableProperties size={16} />,
  model_import: <Upload size={16} />,
  simulation: <Activity size={16} />,
  selected_region: <CircuitBoard size={16} />,
  export_center: <FileText size={16} />,
  diagnostics: <HeartPulse size={16} />,
};

type ProductBetaScreenProps = {
  status: ProductWorkflowStatusDto | null;
  loading: boolean;
  error: string | null;
  onRefresh: () => void;
  onSelfCheck: () => void;
  onCreateDemo: () => void;
  onNavigate: (screenId: string) => void;
};

export function ProductBetaScreen({
  status,
  loading,
  error,
  onRefresh,
  onSelfCheck,
  onCreateDemo,
  onNavigate,
}: ProductBetaScreenProps) {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Stack gap="md">
          <Group gap="xs">
            <Rocket size={24} />
            <Title order={1}>Product Beta</Title>
          </Group>

          <Text c="dimmed">HotSAS Studio v2.0 — integrated engineering workflow status.</Text>

          {status && (
            <Group gap="xs">
              <Badge variant="light" color="blue">
                {status.app_version}
              </Badge>
              <Badge variant="light" color="gray">
                {status.roadmap_stage}
              </Badge>
            </Group>
          )}

          {error && (
            <Alert color="red" icon={<AlertTriangle size={16} />}>
              {error}
            </Alert>
          )}

          {status?.current_project && (
            <Card withBorder shadow="sm" padding="sm">
              <Text fw={600}>Current Project</Text>
              <Text size="sm">
                {status.current_project.project_name} ({status.current_project.component_count}{" "}
                components, {status.current_project.net_count} nets)
              </Text>
            </Card>
          )}

          <Group gap="xs">
            <Button
              leftSection={<RefreshCw size={16} />}
              onClick={onRefresh}
              loading={loading}
              variant="light"
            >
              Refresh workflow status
            </Button>
            <Button
              leftSection={<Play size={16} />}
              onClick={onSelfCheck}
              loading={loading}
              variant="light"
            >
              Run product beta self-check
            </Button>
            <Button
              leftSection={<CircuitBoard size={16} />}
              onClick={onCreateDemo}
              loading={loading}
            >
              Create integrated demo project
            </Button>
          </Group>

          <Title order={3}>Guided Workflow</Title>
          <Stack gap="xs">
            {status?.workflow_steps.map((step) => (
              <Card key={step.id} withBorder shadow="sm" padding="sm">
                <Group justify="space-between">
                  <Group gap="xs">
                    {stepIconMap[step.id] ?? <CircuitBoard size={16} />}
                    <Text fw={600}>{step.title}</Text>
                    <Badge
                      color={statusColor[step.status] ?? "gray"}
                      leftSection={statusIcon[step.status]}
                    >
                      {step.status}
                    </Badge>
                  </Group>
                  <Button
                    size="xs"
                    variant="subtle"
                    onClick={() => onNavigate(stepScreenMap[step.id] ?? step.screen_id)}
                  >
                    Open
                  </Button>
                </Group>
                <Text size="sm" c="dimmed">
                  {step.description}
                </Text>
                {step.warnings.length > 0 && (
                  <Alert color="yellow" icon={<AlertTriangle size={14} />} mt="xs" p="xs">
                    {step.warnings.join("; ")}
                  </Alert>
                )}
              </Card>
            ))}
          </Stack>

          <Title order={3}>Module Readiness</Title>
          <Stack gap="xs">
            {status?.module_statuses.map((module) => (
              <Card key={module.id} withBorder shadow="sm" padding="sm">
                <Group gap="xs">
                  <Badge
                    color={statusColor[module.status] ?? "gray"}
                    leftSection={statusIcon[module.status]}
                  >
                    {module.status}
                  </Badge>
                  <Text fw={600}>{module.title}</Text>
                </Group>
                {module.details.length > 0 && (
                  <Text size="sm" c="dimmed" mt="xs">
                    {module.details.map((d) => `${d.key}: ${d.value}`).join(" | ")}
                  </Text>
                )}
              </Card>
            ))}
          </Stack>

          {status?.blockers.length ? (
            <Alert color="red" icon={<XCircle size={16} />}>
              <Text fw={600}>Blockers</Text>
              <ul>
                {status.blockers.map((b, i) => (
                  <li key={i}>{b}</li>
                ))}
              </ul>
            </Alert>
          ) : null}

          {status?.warnings.length ? (
            <Alert color="yellow" icon={<AlertTriangle size={16} />}>
              <Text fw={600}>Warnings</Text>
              <ul>
                {status.warnings.map((w, i) => (
                  <li key={i}>{w}</li>
                ))}
              </ul>
            </Alert>
          ) : null}
        </Stack>
      </div>
    </section>
  );
}
