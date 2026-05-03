import { Alert, Badge, Card, Group, Stack, Table, Text } from "@mantine/core";
import type { ReactNode } from "react";
import type { NotebookEvaluationResultDto } from "../../types";

export type NotebookResultCardProps = {
  result: NotebookEvaluationResultDto;
  children?: ReactNode;
};

export function NotebookResultCard({ result, children }: NotebookResultCardProps) {
  const badgeColor =
    result.status === "success" ? "green" : result.status === "error" ? "red" : "yellow";

  return (
    <Card withBorder>
      <Stack gap="xs">
        <Group>
          <Text fw={500}>Input:</Text>
          <Text>{result.input}</Text>
        </Group>
        <Group>
          <Text fw={500}>Status:</Text>
          <Badge color={badgeColor}>{result.status}</Badge>
        </Group>
        {result.message && (
          <Alert color={result.status === "error" ? "red" : "blue"} variant="light">
            {result.message}
          </Alert>
        )}
        {result.status === "unsupported" && (
          <Alert color="yellow" variant="light">
            v1.4 supports assignments, formula calls and nearestE/lowerE/higherE commands. Free math
            expressions like sin(...) are planned later.
          </Alert>
        )}
        {result.outputs.length > 0 && (
          <div>
            <Text fw={500}>Outputs:</Text>
            <Table>
              <Table.Thead>
                <Table.Tr>
                  <Table.Th>Name</Table.Th>
                  <Table.Th>Value</Table.Th>
                  <Table.Th>Unit</Table.Th>
                </Table.Tr>
              </Table.Thead>
              <Table.Tbody>
                {result.outputs.map((output) => (
                  <Table.Tr key={output.name}>
                    <Table.Td>{output.name}</Table.Td>
                    <Table.Td>{output.value.original}</Table.Td>
                    <Table.Td>{output.value.unit}</Table.Td>
                  </Table.Tr>
                ))}
              </Table.Tbody>
            </Table>
          </div>
        )}
        {result.warnings.length > 0 && (
          <Alert color="yellow" variant="light">
            {result.warnings.join("; ")}
          </Alert>
        )}
        {children}
      </Stack>
    </Card>
  );
}
