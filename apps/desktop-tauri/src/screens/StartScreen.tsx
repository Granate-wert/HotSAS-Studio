import { Button, Group, Stack, Text, Title } from '@mantine/core';
import { Calculator, CircuitBoard, Sigma } from 'lucide-react';
import { ProjectMetrics } from '../components/ProjectMetrics';
import type { ProjectMetricsData } from './screenTypes';

type StartScreenProps = ProjectMetricsData & {
  busy: boolean;
  hasProject: boolean;
  onCreateDemo: () => void;
  onCalculate: () => void;
  onNearestE24: () => void;
};

export function StartScreen({
  project,
  formulaResult,
  preferredValue,
  simulation,
  busy,
  hasProject,
  onCreateDemo,
  onCalculate,
  onNearestE24,
}: StartScreenProps) {
  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Stack gap="xs">
          <Title order={1}>HotSAS Studio</Title>
          <Text c="dimmed">Hardware-Oriented Schematic Analysis & Simulation Studio</Text>
        </Stack>
        <Group gap="xs">
          <Button leftSection={<CircuitBoard size={16} />} onClick={onCreateDemo} loading={busy}>
            New RC Demo
          </Button>
          <Button
            variant="light"
            leftSection={<Calculator size={16} />}
            onClick={onCalculate}
            disabled={!hasProject}
          >
            Calculate fc
          </Button>
          <Button
            variant="light"
            leftSection={<Sigma size={16} />}
            onClick={onNearestE24}
            disabled={!hasProject}
          >
            Nearest E24
          </Button>
        </Group>
        <div className="metric-grid">
          <ProjectMetrics
            project={project}
            formulaResult={formulaResult}
            preferredValue={preferredValue}
            simulation={simulation}
          />
        </div>
      </div>
    </section>
  );
}
