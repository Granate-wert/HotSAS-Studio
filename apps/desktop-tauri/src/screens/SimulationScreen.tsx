import { Button, Group, Title } from "@mantine/core";
import { Play } from "lucide-react";
import { SimulationChart } from "../components/SimulationChart";
import type { SimulationResultDto } from "../types";

export function SimulationScreen({
  simulation,
  hasProject,
  onSimulation,
}: {
  simulation: SimulationResultDto | null;
  hasProject: boolean;
  onSimulation: () => void;
}) {
  return (
    <section className="screen-panel">
      <div className="screen-content wide">
        <Group justify="space-between" align="center">
          <Title order={2}>Simulation Results</Title>
          <Button
            variant="light"
            leftSection={<Play size={16} />}
            onClick={onSimulation}
            disabled={!hasProject}
          >
            Mock AC
          </Button>
        </Group>
        <SimulationChart simulation={simulation} />
      </div>
    </section>
  );
}
