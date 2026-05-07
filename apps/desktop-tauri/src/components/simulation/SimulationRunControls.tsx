import { Button, Group, Stack, Text } from "@mantine/core";

interface Props {
  canRun: boolean;
  loading: boolean;
  onRun: () => void;
  onPreflight: () => void;
}

export function SimulationRunControls({ canRun, loading, onRun, onPreflight }: Props) {
  return (
    <Group gap="sm">
      <Button variant="light" onClick={onPreflight} loading={loading} disabled={loading}>
        Preflight
      </Button>
      <Button onClick={onRun} loading={loading} disabled={loading || !canRun}>
        Run Simulation
      </Button>
    </Group>
  );
}
