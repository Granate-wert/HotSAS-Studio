import { Stack } from '@mantine/core';
import type { FormulaResultDto, PreferredValueDto, ProjectDto, SimulationResultDto } from '../types';
import { Metric } from './Metric';

export function ProjectMetrics({
  project,
  formulaResult,
  preferredValue,
  simulation,
}: {
  project: ProjectDto | null;
  formulaResult: FormulaResultDto | null;
  preferredValue: PreferredValueDto | null;
  simulation: SimulationResultDto | null;
}) {
  return (
    <Stack gap="md">
      <Metric label="Project" value={project?.name ?? '-'} />
      <Metric label="Cutoff" value={formulaResult?.value.display ?? '-'} />
      <Metric label="Nearest E24" value={preferredValue?.nearest.display ?? '-'} />
      <Metric
        label="Simulation"
        value={simulation ? `${simulation.status} / ${simulation.graph_series.length} series` : '-'}
      />
    </Stack>
  );
}
