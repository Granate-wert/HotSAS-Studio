import { Text } from '@mantine/core';

export function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="metric">
      <Text size="xs" c="dimmed">
        {label}
      </Text>
      <Text size="sm" fw={700}>
        {value}
      </Text>
    </div>
  );
}
