import { Badge, Code, Group, Stack, Text } from '@mantine/core';
import { useHotSasStore } from '../store';

export function FormulaPanel() {
  const { formulaResult, preferredValue } = useHotSasStore();

  return (
    <Stack gap="sm" p="md">
      <Group gap="xs">
        <Badge variant="light">rc_low_pass_cutoff</Badge>
        <Code>{formulaResult?.expression ?? 'fc = 1 / (2*pi*R*C)'}</Code>
      </Group>
      <Text size="sm">fc: {formulaResult?.value.display ?? '-'}</Text>
      <Text size="sm">
        E24:{' '}
        {preferredValue
          ? `${preferredValue.requested_value.display} -> ${preferredValue.nearest.display}`
          : '-'}
      </Text>
      <Text size="sm">
        Error: {preferredValue ? `${preferredValue.error_percent.toFixed(4)}%` : '-'}
      </Text>
    </Stack>
  );
}
