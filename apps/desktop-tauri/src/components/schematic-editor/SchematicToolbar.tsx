import { Button, Group, Text } from "@mantine/core";
import { Trash2, Move, Plug, Tag, AlertTriangle } from "lucide-react";

export function SchematicToolbar({
  selectedComponentId,
  onDelete,
  onConnect,
  onRenameNet,
  disabled,
  editError,
}: {
  selectedComponentId: string | null;
  onDelete: (id: string) => void;
  onConnect: () => void;
  onRenameNet: () => void;
  disabled?: boolean;
  editError: string | null;
}) {
  return (
    <div className="schematic-toolbar" style={{ padding: 8 }}>
      <Group gap="xs" justify="space-between">
        <Group gap="xs">
          <Text size="sm" fw={600}>
            Schematic Editor
          </Text>
          <Button
            size="xs"
            variant="light"
            color="red"
            leftSection={<Trash2 size={14} />}
            disabled={!selectedComponentId || disabled}
            onClick={() => selectedComponentId && onDelete(selectedComponentId)}
            data-testid="delete-selected-component"
          >
            Delete
          </Button>
          <Button
            size="xs"
            variant="light"
            leftSection={<Plug size={14} />}
            disabled={disabled}
            onClick={onConnect}
            data-testid="connect-pins-button"
          >
            Connect
          </Button>
          <Button
            size="xs"
            variant="light"
            leftSection={<Tag size={14} />}
            disabled={disabled}
            onClick={onRenameNet}
            data-testid="rename-net-button"
          >
            Rename Net
          </Button>
        </Group>
        {editError && (
          <Text size="xs" c="red">
            <AlertTriangle size={12} style={{ marginRight: 4 }} />
            {editError}
          </Text>
        )}
      </Group>
    </div>
  );
}
