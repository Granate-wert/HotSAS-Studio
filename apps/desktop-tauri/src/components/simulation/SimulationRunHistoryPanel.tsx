import { ActionIcon, Button, Group, Stack, Table, Text } from "@mantine/core";
import { Trash2, X } from "lucide-react";
import type { SimulationRunHistoryEntryDto } from "../../types";

interface Props {
  history: SimulationRunHistoryEntryDto[];
  loading?: boolean;
  onOpenRun?: (runId: string) => void;
  onDeleteRun?: (runId: string) => void;
  onClearHistory?: () => void;
}

export function SimulationRunHistoryPanel({
  history,
  loading,
  onOpenRun,
  onDeleteRun,
  onClearHistory,
}: Props) {
  if (history.length === 0) {
    return (
      <Stack gap="xs">
        <Text size="sm" fw={500}>
          Run History
        </Text>
        <Text size="xs" c="dimmed">
          No simulation runs in history
        </Text>
      </Stack>
    );
  }

  return (
    <Stack gap="xs">
      <Group justify="space-between" align="center">
        <Text size="sm" fw={500}>
          Run History
        </Text>
        {onClearHistory && (
          <Button
            variant="light"
            size="xs"
            color="red"
            leftSection={<X size={14} />}
            onClick={onClearHistory}
            loading={loading}
          >
            Clear
          </Button>
        )}
      </Group>

      <Table striped>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Profile</Table.Th>
            <Table.Th>Engine</Table.Th>
            <Table.Th>Status</Table.Th>
            <Table.Th>Warnings</Table.Th>
            <Table.Th>Errors</Table.Th>
            <Table.Th>Series</Table.Th>
            <Table.Th></Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {history.map((entry) => (
            <Table.Tr key={entry.run_id}>
              <Table.Td>
                <Text size="xs">{entry.profile_name}</Text>
              </Table.Td>
              <Table.Td>
                <Text size="xs">{entry.engine_used}</Text>
              </Table.Td>
              <Table.Td>
                <Text
                  size="xs"
                  c={
                    entry.status === "Succeeded"
                      ? "green"
                      : entry.status === "Failed"
                        ? "red"
                        : "dimmed"
                  }
                >
                  {entry.status}
                </Text>
              </Table.Td>
              <Table.Td>
                <Text size="xs">{entry.warnings_count}</Text>
              </Table.Td>
              <Table.Td>
                <Text size="xs">{entry.errors_count}</Text>
              </Table.Td>
              <Table.Td>
                <Text size="xs">{entry.series_count}</Text>
              </Table.Td>
              <Table.Td>
                <Group gap="xs">
                  {onOpenRun && (
                    <Button
                      variant="light"
                      size="xs"
                      onClick={() => onOpenRun(entry.run_id)}
                      loading={loading}
                    >
                      Open
                    </Button>
                  )}
                  {onDeleteRun && (
                    <ActionIcon
                      variant="light"
                      color="red"
                      size="sm"
                      onClick={() => onDeleteRun(entry.run_id)}
                      loading={loading}
                    >
                      <Trash2 size={14} />
                    </ActionIcon>
                  )}
                </Group>
              </Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
    </Stack>
  );
}
