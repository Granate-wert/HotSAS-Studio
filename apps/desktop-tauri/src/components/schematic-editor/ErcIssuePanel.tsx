import { Stack, Text } from "@mantine/core";
import type { CircuitValidationIssueDto } from "../../types";

type Props = {
  errors: CircuitValidationIssueDto[];
  warnings: CircuitValidationIssueDto[];
};

export function ErcIssuePanel({ errors, warnings }: Props) {
  if (errors.length === 0 && warnings.length === 0) {
    return (
      <Text size="xs" c="green" p="xs">
        No ERC issues
      </Text>
    );
  }

  return (
    <Stack gap="xs" p="xs">
      {errors.map((err, i) => (
        <Text key={`err-${i}`} size="xs" c="red">
          [E] {err.code}: {err.message}
          {err.component_id && ` (comp: ${err.component_id})`}
          {err.net_id && ` (net: ${err.net_id})`}
        </Text>
      ))}
      {warnings.map((warn, i) => (
        <Text key={`warn-${i}`} size="xs" c="orange">
          [W] {warn.code}: {warn.message}
          {warn.component_id && ` (comp: ${warn.component_id})`}
          {warn.net_id && ` (net: ${warn.net_id})`}
        </Text>
      ))}
    </Stack>
  );
}
