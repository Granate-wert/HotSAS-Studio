import { Badge, Code, Group, Stack, Text } from "@mantine/core";
import { FunctionSquare } from "lucide-react";
import { useHotSasStore } from "../store";

export function FormulaPanel() {
  const { formulaResult, preferredValue } = useHotSasStore();

  if (!formulaResult && !preferredValue) {
    return (
      <Stack align="center" justify="center" gap="sm" style={{ height: "100%", padding: 24 }}>
        <FunctionSquare size={32} color="#56657a" />
        <Text size="sm" c="dimmed" ta="center">
          No formula results yet.
        </Text>
        <Text size="xs" c="dimmed" ta="center">
          Use the Formula Library or RC Demo to calculate values.
        </Text>
      </Stack>
    );
  }

  return (
    <Stack gap="sm" p="md">
      <Group gap="xs">
        <Badge variant="light">rc_low_pass_cutoff</Badge>
        <Code>{formulaResult?.expression ?? "fc = 1 / (2*pi*R*C)"}</Code>
      </Group>
      <Text size="sm">fc: {formulaResult?.value.display ?? "-"}</Text>
      <Text size="sm">
        E24:{" "}
        {preferredValue
          ? `${preferredValue.requested_value.display} -> ${preferredValue.nearest.display}`
          : "-"}
      </Text>
      <Text size="sm">
        Error: {preferredValue ? `${preferredValue.error_percent.toFixed(4)}%` : "-"}
      </Text>
    </Stack>
  );
}
