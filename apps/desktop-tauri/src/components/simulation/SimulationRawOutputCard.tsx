import { Code, Stack, Text } from "@mantine/core";

interface Props {
  netlist: string;
  rawOutput?: string | null;
}

export function SimulationRawOutputCard({ netlist, rawOutput }: Props) {
  return (
    <Stack gap="xs">
      <Text size="sm" fw={500}>
        Generated Netlist
      </Text>
      <Code block>{netlist || "No netlist available"}</Code>
      {rawOutput && (
        <>
          <Text size="sm" fw={500}>
            Raw Output
          </Text>
          <Code block>{rawOutput}</Code>
        </>
      )}
    </Stack>
  );
}
