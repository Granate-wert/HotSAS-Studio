import { Button, Group } from "@mantine/core";
import { MousePointer, Plus, Link2, Trash2 } from "lucide-react";

type Props = {
  toolMode: "select" | "place" | "wire" | "delete";
  onSetToolMode: (mode: "select" | "place" | "wire" | "delete") => void;
  disabled?: boolean;
};

export function InteractiveSchematicToolbar({ toolMode, onSetToolMode, disabled }: Props) {
  return (
    <Group gap="xs" p="xs">
      <Button
        size="xs"
        variant={toolMode === "select" ? "filled" : "light"}
        leftSection={<MousePointer size={14} />}
        onClick={() => onSetToolMode("select")}
        disabled={disabled}
      >
        Select
      </Button>
      <Button
        size="xs"
        variant={toolMode === "place" ? "filled" : "light"}
        leftSection={<Plus size={14} />}
        onClick={() => onSetToolMode("place")}
        disabled={disabled}
      >
        Place
      </Button>
      <Button
        size="xs"
        variant={toolMode === "wire" ? "filled" : "light"}
        leftSection={<Link2 size={14} />}
        onClick={() => onSetToolMode("wire")}
        disabled={disabled}
      >
        Wire
      </Button>
      <Button
        size="xs"
        variant={toolMode === "delete" ? "filled" : "light"}
        leftSection={<Trash2 size={14} />}
        onClick={() => onSetToolMode("delete")}
        disabled={disabled}
      >
        Delete
      </Button>
    </Group>
  );
}
