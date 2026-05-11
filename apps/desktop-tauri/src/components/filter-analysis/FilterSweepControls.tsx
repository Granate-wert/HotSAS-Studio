import { Card, Group, Select, Stack, Text, TextInput } from "@mantine/core";
import type { FrequencySweepSettings, FrequencySweepScale } from "../../types";

interface Props {
  settings: FrequencySweepSettings;
  onChange: (settings: FrequencySweepSettings) => void;
}

export function FilterSweepControls({ settings, onChange }: Props) {
  const update = (partial: Partial<FrequencySweepSettings>) => {
    onChange({ ...settings, ...partial });
  };

  return (
    <Card withBorder shadow="sm" padding="sm" radius="md">
      <Stack gap="sm">
        <Text fw={600} size="sm">
          Frequency Sweep
        </Text>
        <Group gap="sm" align="flex-end">
          <TextInput
            label="Start (Hz)"
            value={String(settings.start_hz)}
            onChange={(e) => {
              const v = parseFloat(e.currentTarget.value);
              if (!Number.isNaN(v)) update({ start_hz: v });
            }}
            style={{ flex: 1 }}
          />
          <TextInput
            label="Stop (Hz)"
            value={String(settings.stop_hz)}
            onChange={(e) => {
              const v = parseFloat(e.currentTarget.value);
              if (!Number.isNaN(v)) update({ stop_hz: v });
            }}
            style={{ flex: 1 }}
          />
          <TextInput
            label="Points"
            value={String(settings.points)}
            onChange={(e) => {
              const v = parseInt(e.currentTarget.value, 10);
              if (!Number.isNaN(v)) update({ points: v });
            }}
            style={{ flex: 1 }}
          />
          <Select
            label="Scale"
            data={[
              { value: "linear", label: "Linear" },
              { value: "logarithmic", label: "Logarithmic" },
            ]}
            value={settings.scale}
            onChange={(value) => {
              if (value === "linear" || value === "logarithmic") {
                update({ scale: value as FrequencySweepScale });
              }
            }}
            style={{ flex: 1 }}
          />
        </Group>
      </Stack>
    </Card>
  );
}
