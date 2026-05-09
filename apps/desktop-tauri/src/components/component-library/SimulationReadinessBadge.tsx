import { Badge, Group, Text } from "@mantine/core";
import type { SimulationReadinessDto } from "../../types";

type Props = {
  readiness: SimulationReadinessDto;
};

export function SimulationReadinessBadge({ readiness }: Props) {
  const color =
    readiness.blocking_count > 0 ? "red" : readiness.uses_placeholder ? "yellow" : "green";

  return (
    <Group gap="xs">
      <Badge color={color} variant="light">
        {readiness.status_label}
      </Badge>
      {readiness.blocking_count > 0 && (
        <Text size="xs" c="red">
          {readiness.blocking_count} blocking
        </Text>
      )}
      {readiness.warning_count > 0 && (
        <Text size="xs" c="yellow.8">
          {readiness.warning_count} warning
        </Text>
      )}
    </Group>
  );
}
