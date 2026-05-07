import { Select, Stack, Text } from "@mantine/core";
import type { UserCircuitSimulationProfileDto } from "../../types";

interface Props {
  profiles: UserCircuitSimulationProfileDto[];
  selected: UserCircuitSimulationProfileDto | null;
  onSelect: (profile: UserCircuitSimulationProfileDto | null) => void;
  loading?: boolean;
}

export function SimulationProfileSelector({ profiles, selected, onSelect, loading }: Props) {
  const value = selected?.id ?? "";
  const data = profiles.map((p) => ({
    value: p.id,
    label: `${p.name} (${p.engine})`,
  }));

  return (
    <Stack gap="xs">
      <Text size="sm" fw={500}>
        Simulation Profile
      </Text>
      <Select
        data={data}
        value={value}
        onChange={(id) => {
          const profile = profiles.find((p) => p.id === id) ?? null;
          onSelect(profile);
        }}
        placeholder="Select a profile"
        disabled={loading || profiles.length === 0}
      />
    </Stack>
  );
}
