import { Alert, Code, Stack, Text } from "@mantine/core";
import type { SimulationPreflightResultDto } from "../../types";

interface Props {
  preflight: SimulationPreflightResultDto | null;
}

export function SimulationPreflightCard({ preflight }: Props) {
  if (!preflight) {
    return (
      <Text size="xs" c="dimmed">
        Run preflight to validate the circuit
      </Text>
    );
  }

  return (
    <Stack gap="xs">
      {preflight.can_run ? (
        <Alert color="green" title="Preflight OK" variant="light">
          Circuit is ready for simulation
        </Alert>
      ) : (
        <Alert color="red" title="Preflight Failed" variant="light">
          Fix blocking errors before running
        </Alert>
      )}
      {preflight.blocking_errors.map((e) => (
        <Alert key={e.code} color="red" variant="light" title={e.code}>
          {e.message}
        </Alert>
      ))}
      {preflight.warnings.map((w) => (
        <Alert key={w.code} color="yellow" variant="light" title={w.code}>
          {w.message}
        </Alert>
      ))}
      {preflight.generated_netlist_preview && (
        <Stack gap="xs">
          <Text size="xs" fw={500}>
            Netlist preview
          </Text>
          <Code block>{preflight.generated_netlist_preview}</Code>
        </Stack>
      )}
    </Stack>
  );
}
