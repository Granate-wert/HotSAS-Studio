import { Button, Group, Text } from "@mantine/core";
import { Undo2, Redo2 } from "lucide-react";

type Props = {
  canUndo: boolean;
  canRedo: boolean;
  lastActionLabel: string | null;
  nextRedoLabel: string | null;
  onUndo: () => void;
  onRedo: () => void;
  disabled?: boolean;
};

export function UndoRedoToolbar({
  canUndo,
  canRedo,
  lastActionLabel,
  nextRedoLabel,
  onUndo,
  onRedo,
  disabled,
}: Props) {
  return (
    <Group gap="xs" p="xs" justify="space-between">
      <Group gap="xs">
        <Button
          size="xs"
          variant="light"
          leftSection={<Undo2 size={14} />}
          onClick={onUndo}
          disabled={!canUndo || disabled}
        >
          Undo
        </Button>
        <Button
          size="xs"
          variant="light"
          leftSection={<Redo2 size={14} />}
          onClick={onRedo}
          disabled={!canRedo || disabled}
        >
          Redo
        </Button>
      </Group>
      {(lastActionLabel || nextRedoLabel) && (
        <Text size="xs" c="dimmed">
          {lastActionLabel ? `Last: ${lastActionLabel}` : ""}
          {lastActionLabel && nextRedoLabel ? " | " : ""}
          {nextRedoLabel ? `Next: ${nextRedoLabel}` : ""}
        </Text>
      )}
    </Group>
  );
}
