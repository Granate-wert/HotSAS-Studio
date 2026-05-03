import { Button, Group, Text } from "@mantine/core";
import type { NotebookEvaluationResultDto, SelectedComponentDto } from "../../types";

export type ApplyNotebookOutputPanelProps = {
  result: NotebookEvaluationResultDto;
  selectedComponent: SelectedComponentDto | null;
  onApply: (outputName: string) => void;
};

export function ApplyNotebookOutputPanel({
  result,
  selectedComponent,
  onApply,
}: ApplyNotebookOutputPanelProps) {
  if (!selectedComponent || result.outputs.length === 0) return null;

  return (
    <Group>
      <Text size="sm" c="dimmed">
        Apply to {selectedComponent.instance_id}:
      </Text>
      {result.outputs.map((output) => (
        <Button
          key={output.name}
          size="compact-xs"
          variant="light"
          onClick={() => onApply(output.name)}
        >
          {output.name}
        </Button>
      ))}
    </Group>
  );
}
