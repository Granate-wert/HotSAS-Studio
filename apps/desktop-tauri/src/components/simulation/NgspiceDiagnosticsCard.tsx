import { Alert, Button, Card, Group, Stack, Text } from "@mantine/core";
import { Activity } from "lucide-react";
import type { NgspiceDiagnosticsDto } from "../../types";

interface Props {
  diagnostics: NgspiceDiagnosticsDto | null;
  loading?: boolean;
  onRefresh?: () => void;
}

export function NgspiceDiagnosticsCard({ diagnostics, loading, onRefresh }: Props) {
  const availability = diagnostics?.availability;

  return (
    <Card withBorder>
      <Stack gap="xs">
        <Group justify="space-between" align="center">
          <Text fw={600}>ngspice Diagnostics</Text>
          {onRefresh && (
            <Button
              variant="light"
              size="xs"
              leftSection={<Activity size={14} />}
              onClick={onRefresh}
              loading={loading}
            >
              Refresh
            </Button>
          )}
        </Group>

        {!diagnostics && (
          <Text size="xs" c="dimmed">
            ngspice diagnostics not checked yet
          </Text>
        )}

        {diagnostics && (
          <>
            <Text size="sm" c={availability?.available ? "green" : "orange"}>
              Status: {availability?.available ? "Available" : "Unavailable"}
            </Text>

            {availability?.executablePath && (
              <Text size="xs" c="dimmed">
                Path: {availability.executablePath}
              </Text>
            )}

            {availability?.version && (
              <Text size="xs" c="dimmed">
                Version: {availability.version}
              </Text>
            )}

            {diagnostics.executable_path && (
              <Text size="xs" c="dimmed">
                Detected path: {diagnostics.executable_path}
              </Text>
            )}

            {diagnostics.version && (
              <Text size="xs" c="dimmed">
                Detected version: {diagnostics.version}
              </Text>
            )}

            {diagnostics.checked_at && (
              <Text size="xs" c="dimmed">
                Checked at: {diagnostics.checked_at}
              </Text>
            )}

            {!availability?.available && (
              <Alert color="yellow" variant="light" title="Mock fallback available">
                ngspice is not installed or not on PATH. Simulation will automatically fall back to
                the mock engine in Auto mode.
              </Alert>
            )}

            {diagnostics.warnings.map((w, i) => (
              <Alert key={`w-${i}`} color="yellow" variant="light" title={w.code}>
                <Text size="xs">{w.message}</Text>
                {w.suggested_fix && (
                  <Text size="xs" c="dimmed" mt={4}>
                    Suggested fix: {w.suggested_fix}
                  </Text>
                )}
              </Alert>
            ))}

            {diagnostics.errors.map((e, i) => (
              <Alert key={`e-${i}`} color="red" variant="light" title={e.code}>
                <Text size="xs">{e.message}</Text>
                {e.suggested_fix && (
                  <Text size="xs" c="dimmed" mt={4}>
                    Suggested fix: {e.suggested_fix}
                  </Text>
                )}
              </Alert>
            ))}
          </>
        )}
      </Stack>
    </Card>
  );
}
