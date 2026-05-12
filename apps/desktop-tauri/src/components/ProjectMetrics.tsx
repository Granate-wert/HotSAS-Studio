import { Stack, Text } from "@mantine/core";
import type {
  FormulaResultDto,
  PreferredValueDto,
  ProjectDto,
  SimulationResultDto,
} from "../types";
import { Metric } from "./Metric";

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
  const componentCount = project?.schematic.components.length ?? 0;
  const netCount = project?.schematic.nets.length ?? 0;
  const wireCount = project?.schematic.wires.length ?? 0;

  return (
    <Stack gap="md">
      <Metric label="Project" value={project?.name ?? "-"} />
      <Metric label="Components" value={componentCount.toString()} />
      <Metric label="Nets" value={netCount.toString()} />
      <Metric label="Wires" value={wireCount.toString()} />
      <Metric label="Cutoff" value={formulaResult?.value.display ?? "-"} />
      <Metric label="Nearest E24" value={preferredValue?.nearest.display ?? "-"} />
      <Metric
        label="Simulation"
        value={simulation ? `${simulation.status} / ${simulation.graph_series.length} series` : "-"}
      />
      {project && componentCount === 0 && (
        <Text size="xs" c="dimmed">
          Project is empty. Add components from the palette or load a demo.
        </Text>
      )}
    </Stack>
  );
}
