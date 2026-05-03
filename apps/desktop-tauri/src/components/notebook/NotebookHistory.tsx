import { Badge, Card, Table, Text } from "@mantine/core";
import type { NotebookStateDto } from "../../types";

export type NotebookHistoryProps = {
  state: NotebookStateDto;
};

export function NotebookHistory({ state }: NotebookHistoryProps) {
  if (!state.history.length) return null;

  return (
    <Card withBorder>
      <Text fw={500}>History</Text>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Input</Table.Th>
            <Table.Th>Result</Table.Th>
            <Table.Th>Status</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {state.history.map((entry) => (
            <Table.Tr key={entry.id}>
              <Table.Td>{entry.input}</Table.Td>
              <Table.Td>{entry.result_summary}</Table.Td>
              <Table.Td>
                <Badge
                  size="xs"
                  color={
                    entry.status === "success"
                      ? "green"
                      : entry.status === "error"
                        ? "red"
                        : "yellow"
                  }
                >
                  {entry.status}
                </Badge>
              </Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
    </Card>
  );
}
