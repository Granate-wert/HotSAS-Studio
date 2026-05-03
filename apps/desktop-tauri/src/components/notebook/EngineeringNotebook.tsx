import {
  Alert,
  Button,
  Card,
  Group,
  Stack,
  Table,
  Text,
  Textarea,
  Title,
  Badge,
} from "@mantine/core";
import { useState } from "react";
import { backend } from "../../api";
import { useHotSasStore } from "../../store";
import { logger } from "../../utils/logger";
import type { NotebookEvaluationResultDto, NotebookStateDto } from "../../types";

export function EngineeringNotebook() {
  const [input, setInput] = useState("");
  const [result, setResult] = useState<NotebookEvaluationResultDto | null>(null);
  const [loading, setLoading] = useState(false);
  const notebookState = useHotSasStore((s) => s.notebookState);
  const setNotebookState = useHotSasStore((s) => s.setNotebookState);
  const setLastNotebookResult = useHotSasStore((s) => s.setLastNotebookResult);
  const selectedComponent = useHotSasStore((s) => s.selectedComponent);

  async function evaluate() {
    if (!input.trim()) return;
    setLoading(true);
    try {
      const res = await backend.evaluateNotebookInput({ input });
      setResult(res);
      setLastNotebookResult(res);
      const state = await backend.getNotebookState();
      setNotebookState(state);
      logger.info(`Notebook evaluated: ${input} -> ${res.status}`);
    } catch (err) {
      logger.error(`Notebook evaluation failed: ${String(err)}`);
      setResult({
        input,
        status: "error",
        kind: "Text",
        outputs: [],
        variables: [],
        message: String(err),
        warnings: [],
      });
    } finally {
      setLoading(false);
    }
  }

  async function clear() {
    try {
      const state = await backend.clearNotebook();
      setNotebookState(state);
      setResult(null);
      setLastNotebookResult(null);
      logger.info("Notebook cleared");
    } catch (err) {
      logger.error(`Clear notebook failed: ${String(err)}`);
    }
  }

  async function applyToComponent(outputName: string) {
    if (!selectedComponent) return;
    try {
      await backend.applyNotebookOutputToComponent({
        instance_id: selectedComponent.instance_id,
        parameter_name: outputName,
        output_name: outputName,
      });
      logger.info(`Applied ${outputName} to ${selectedComponent.instance_id}`);
    } catch (err) {
      logger.error(`Apply failed: ${String(err)}`);
    }
  }

  return (
    <Stack gap="md">
      <Title order={3}>Engineering Notebook</Title>

      <Textarea
        label="Input"
        placeholder="R = 10k&#10;C = 100n&#10;rc_low_pass_cutoff(R=10k, C=100n)&#10;nearestE(15.93k, E96, Ohm)"
        value={input}
        onChange={(e) => setInput(e.currentTarget.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            void evaluate();
          }
        }}
        minRows={3}
      />

      <Group>
        <Button onClick={() => void evaluate()} loading={loading} size="compact-sm">
          Evaluate
        </Button>
        <Button onClick={() => void clear()} variant="light" size="compact-sm" color="red">
          Clear
        </Button>
      </Group>

      {result && (
        <Card withBorder>
          <Stack gap="xs">
            <Group>
              <Text fw={500}>Input:</Text>
              <Text>{result.input}</Text>
            </Group>
            <Group>
              <Text fw={500}>Status:</Text>
              <Badge
                color={
                  result.status === "success"
                    ? "green"
                    : result.status === "error"
                      ? "red"
                      : "yellow"
                }
              >
                {result.status}
              </Badge>
            </Group>
            {result.message && (
              <Alert color={result.status === "error" ? "red" : "blue"} variant="light">
                {result.message}
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
            {selectedComponent && result.outputs.length > 0 && (
              <Group>
                <Text size="sm" c="dimmed">
                  Apply to {selectedComponent.instance_id}:
                </Text>
                {result.outputs.map((output) => (
                  <Button
                    key={output.name}
                    size="compact-xs"
                    variant="light"
                    onClick={() => void applyToComponent(output.name)}
                  >
                    {output.name}
                  </Button>
                ))}
              </Group>
            )}
          </Stack>
        </Card>
      )}

      {notebookState && notebookState.variables.length > 0 && (
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
              {notebookState.variables.map((variable) => (
                <Table.Tr key={variable.name}>
                  <Table.Td>{variable.name}</Table.Td>
                  <Table.Td>{variable.value.original}</Table.Td>
                  <Table.Td>{variable.value.unit}</Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Card>
      )}

      {notebookState && notebookState.history.length > 0 && (
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
              {notebookState.history.map((entry) => (
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
      )}
    </Stack>
  );
}
