import { Button, Group, Tooltip } from "@mantine/core";
import { MousePointer, Plus, Link2, Trash2 } from "lucide-react";

type Props = {
  toolMode: "select" | "place" | "wire" | "delete";
  onSetToolMode: (mode: "select" | "place" | "wire" | "delete") => void;
  disabled?: boolean;
  disabledReason?: string;
};

export function InteractiveSchematicToolbar({
  toolMode,
  onSetToolMode,
  disabled,
  disabledReason,
}: Props) {
  const wrap = (child: React.ReactNode, label: string) =>
    disabled && disabledReason ? (
      <Tooltip label={disabledReason}>
        <span>{child}</span>
      </Tooltip>
    ) : (
      child
    );

  return (
    <Group gap="xs" p="xs">
      {wrap(
        <Button
          size="xs"
          variant={toolMode === "select" ? "filled" : "light"}
          leftSection={<MousePointer size={14} />}
          onClick={() => onSetToolMode("select")}
          disabled={disabled}
        >
          Select
        </Button>,
        disabledReason || "Select mode unavailable",
      )}
      {wrap(
        <Button
          size="xs"
          variant={toolMode === "place" ? "filled" : "light"}
          leftSection={<Plus size={14} />}
          onClick={() => onSetToolMode("place")}
          disabled={disabled}
        >
          Place
        </Button>,
        disabledReason || "Place mode unavailable",
      )}
      {wrap(
        <Button
          size="xs"
          variant={toolMode === "wire" ? "filled" : "light"}
          leftSection={<Link2 size={14} />}
          onClick={() => onSetToolMode("wire")}
          disabled={disabled}
        >
          Wire
        </Button>,
        disabledReason || "Wire mode unavailable",
      )}
      {wrap(
        <Button
          size="xs"
          variant={toolMode === "delete" ? "filled" : "light"}
          leftSection={<Trash2 size={14} />}
          onClick={() => onSetToolMode("delete")}
          disabled={disabled}
        >
          Delete
        </Button>,
        disabledReason || "Delete mode unavailable",
      )}
    </Group>
  );
}
