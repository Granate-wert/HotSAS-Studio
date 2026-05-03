import { Text } from "@mantine/core";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import type { ComponentDto } from "../../../types";

function mapSide(side: string): Position {
  switch (side) {
    case "left":
      return Position.Left;
    case "right":
      return Position.Right;
    case "top":
      return Position.Top;
    case "bottom":
      return Position.Bottom;
    default:
      return Position.Left;
  }
}

export function GenericComponentNode({ data, selected }: NodeProps) {
  const { component, onSelect } = data as {
    component: ComponentDto;
    onSelect?: (instanceId: string) => void;
  };
  const primaryParam = component.parameters[0];
  const label = primaryParam ? `${primaryParam.value.display}` : component.component_kind;

  return (
    <div
      onClick={() => onSelect?.(component.instance_id)}
      style={{
        padding: "6px 10px",
        borderRadius: 6,
        border: selected ? "2px solid #7db2ff" : "1px solid #56657a",
        background: "#171f2d",
        color: "#e6edf6",
        minWidth: 90,
        textAlign: "center",
        cursor: "pointer",
        position: "relative",
      }}
    >
      {component.pins.map((pin) => (
        <Handle
          key={pin.id}
          type="source"
          id={pin.id}
          position={mapSide(pin.side)}
          style={{
            width: 8,
            height: 8,
            background: "#7db2ff",
            border: "none",
          }}
        />
      ))}
      <Text fw={700} size="sm">
        {component.display_label}
      </Text>
      <Text size="xs" c="dimmed">
        {label}
      </Text>
    </div>
  );
}
