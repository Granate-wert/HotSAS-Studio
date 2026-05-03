import { Alert, Button, Card, Group, SegmentedControl, Stack, Text, Title } from "@mantine/core";
import { Activity, Play } from "lucide-react";
import { SimulationChart } from "../components/SimulationChart";
import type { NgspiceAvailabilityDto, SimulationResultDto } from "../types";

export function SimulationScreen({
  simulation,
  hasProject,
  ngspiceAvailability,
  selectedEngine,
  isRunning,
  onCheckNgspice,
  onRunSimulation,
  onSetEngine,
}: {
  simulation: SimulationResultDto | null;
  hasProject: boolean;
  ngspiceAvailability: NgspiceAvailabilityDto | null;
  selectedEngine: string;
  isRunning: boolean;
  onCheckNgspice: () => void;
  onRunSimulation: (analysis: string) => void;
  onSetEngine: (engine: string) => void;
}) {
  const engineStatus = ngspiceAvailability ?? {
    available: false,
    message: "Not checked",
    warnings: [],
  };

  return (
    <section className="screen-panel">
      <div className="screen-content wide">
        <Stack gap="md">
          <Group justify="space-between" align="center">
            <Title order={2}>Simulation Results</Title>
            <Button
              variant="light"
              leftSection={<Activity size={16} />}
              onClick={onCheckNgspice}
              loading={isRunning}
            >
              Check ngspice
            </Button>
          </Group>

          <Card withBorder>
            <Stack gap="xs">
              <Text fw={600}>Simulation Engine</Text>
              <Group>
                <SegmentedControl
                  value={selectedEngine}
                  onChange={onSetEngine}
                  data={[
                    { label: "Auto", value: "auto" },
                    { label: "Mock", value: "mock" },
                    { label: "ngspice", value: "ngspice" },
                  ]}
                />
              </Group>
              <Text size="sm" c={engineStatus.available ? "green" : "orange"}>
                ngspice: {engineStatus.available ? "Available" : "Not found / unavailable"}
              </Text>
              {engineStatus.executablePath && (
                <Text size="xs" c="dimmed">
                  Path: {engineStatus.executablePath}
                </Text>
              )}
              {engineStatus.version && (
                <Text size="xs" c="dimmed">
                  Version: {engineStatus.version}
                </Text>
              )}
              {engineStatus.message && (
                <Text size="xs" c="dimmed">
                  {engineStatus.message}
                </Text>
              )}
              {engineStatus.warnings.map((w, i) => (
                <Text key={i} size="xs" c="orange">
                  Warning: {w}
                </Text>
              ))}
            </Stack>
          </Card>

          <Group>
            <Button
              variant="light"
              leftSection={<Play size={16} />}
              onClick={() => onRunSimulation("operating_point")}
              disabled={!hasProject || isRunning}
              loading={isRunning}
            >
              Run Operating Point
            </Button>
            <Button
              variant="light"
              leftSection={<Play size={16} />}
              onClick={() => onRunSimulation("ac_sweep")}
              disabled={!hasProject || isRunning}
              loading={isRunning}
            >
              Run AC Sweep
            </Button>
            <Button
              variant="light"
              leftSection={<Play size={16} />}
              onClick={() => onRunSimulation("transient")}
              disabled={!hasProject || isRunning}
              loading={isRunning}
            >
              Run Transient
            </Button>
          </Group>

          {simulation && (
            <Card withBorder>
              <Stack gap="xs">
                <Group justify="space-between">
                  <Text fw={600}>Result</Text>
                  <Text size="sm" c={simulation.status === "Completed" ? "green" : "red"}>
                    {simulation.status} ({simulation.engine})
                  </Text>
                </Group>
                {simulation.warnings.length > 0 && (
                  <Alert color="yellow" title="Warnings">
                    {simulation.warnings.map((w, i) => (
                      <Text key={i} size="sm">
                        {w}
                      </Text>
                    ))}
                  </Alert>
                )}
                {simulation.errors.length > 0 && (
                  <Alert color="red" title="Errors">
                    {simulation.errors.map((e, i) => (
                      <Text key={i} size="sm">
                        {e}
                      </Text>
                    ))}
                  </Alert>
                )}
              </Stack>
            </Card>
          )}

          <SimulationChart simulation={simulation} />
        </Stack>
      </div>
    </section>
  );
}
