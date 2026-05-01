import { Stack, Text } from '@mantine/core';

export function LibraryPanel() {
  return (
    <Stack gap="xs" p="md">
      <Text size="sm">Formula packs: basic electronics, filters, op-amp, SMPS placeholders</Text>
      <Text size="sm">Component model: symbol, footprint, simulation model, datasheet, BOM fields</Text>
      <Text size="sm">Export placeholders: PDF, KiCad, Altium workflow package</Text>
    </Stack>
  );
}
