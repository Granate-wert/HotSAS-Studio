import { Card, Table, Text } from "@mantine/core";
import type { NotebookStateDto } from "../../types";

export type NotebookVariableTableProps = {
  state: NotebookStateDto;
};

export function NotebookVariableTable({ state }: NotebookVariableTableProps) {
  if (!state.variables.length) return null;

  return (
    <Card withBorder>
      <Text fw={500}>Variables</Text>
      <Table>
        <Table.Thead>
          <Table.Tr>
            <Table.Th>Name</Table.Th>
            <Table.Th>Value</Table.Th>
            <Table.Th>Unit</Table.Th>
          </Table.Tr>
        </Table.Thead>
        <Table.Tbody>
          {state.variables.map((variable) => (
            <Table.Tr key={variable.name}>
              <Table.Td>{variable.name}</Table.Td>
              <Table.Td>{variable.value.original}</Table.Td>
              <Table.Td>{variable.value.unit}</Table.Td>
            </Table.Tr>
          ))}
        </Table.Tbody>
      </Table>
    </Card>
  );
}
