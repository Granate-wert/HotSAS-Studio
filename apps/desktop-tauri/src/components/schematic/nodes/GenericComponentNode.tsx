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

function ComponentSymbol({ kind }: { kind: string }) {
  const stroke = "#7db2ff";
  const strokeWidth = 1.5;
  const size = 32;

  switch (kind) {
    case "resistor":
      return (
        <svg width={size} height={size} viewBox="0 0 32 32" style={{ margin: "0 auto" }}>
          <path
            d="M4 16 L8 16 L10 10 L14 22 L18 10 L22 22 L24 16 L28 16"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
        </svg>
      );
    case "capacitor":
      return (
        <svg width={size} height={size} viewBox="0 0 32 32" style={{ margin: "0 auto" }}>
          <line x1="4" y1="16" x2="13" y2="16" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="13" y1="8" x2="13" y2="24" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="19" y1="8" x2="19" y2="24" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="19" y1="16" x2="28" y2="16" stroke={stroke} strokeWidth={strokeWidth} />
        </svg>
      );
    case "inductor":
      return (
        <svg width={size} height={size} viewBox="0 0 32 32" style={{ margin: "0 auto" }}>
          <path
            d="M4 16 Q8 8 12 16 Q16 8 20 16 Q24 8 28 16"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
        </svg>
      );
    case "voltage_source":
      return (
        <svg width={size} height={size} viewBox="0 0 32 32" style={{ margin: "0 auto" }}>
          <circle cx="16" cy="16" r="10" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="16" y1="10" x2="16" y2="14" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="14" y1="12" x2="18" y2="12" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="16" y1="18" x2="16" y2="22" stroke={stroke} strokeWidth={strokeWidth} />
        </svg>
      );
    case "ground":
      return (
        <svg width={size} height={size} viewBox="0 0 32 32" style={{ margin: "0 auto" }}>
          <line x1="16" y1="4" x2="16" y2="16" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="8" y1="16" x2="24" y2="16" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="10" y1="20" x2="22" y2="20" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="12" y1="24" x2="20" y2="24" stroke={stroke} strokeWidth={strokeWidth} />
        </svg>
      );
    case "diode":
      return (
        <svg width={size} height={size} viewBox="0 0 32 32" style={{ margin: "0 auto" }}>
          <line x1="4" y1="16" x2="10" y2="16" stroke={stroke} strokeWidth={strokeWidth} />
          <polygon points="10,8 10,24 22,16" fill={stroke} />
          <line x1="22" y1="8" x2="22" y2="24" stroke={stroke} strokeWidth={strokeWidth} />
          <line x1="22" y1="16" x2="28" y2="16" stroke={stroke} strokeWidth={strokeWidth} />
        </svg>
      );
    default:
      return null;
  }
}

export function GenericComponentNode({ data, selected }: NodeProps) {
  const { component } = data as {
    component: ComponentDto;
    onSelect?: (instanceId: string) => void;
    onPinClick?: (componentId: string, pinId: string) => void;
  };
  const { onPinClick } = data as {
    onPinClick?: (componentId: string, pinId: string) => void;
  };
  const primaryParam = component.parameters[0];
  const label = primaryParam ? `${primaryParam.value.display}` : component.component_kind;

  return (
    <div
      data-testid={`generic-component-card-${component.instance_id}`}
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
          isConnectableStart={true}
          isConnectableEnd={true}
          style={{
            width: 8,
            height: 8,
            background: "#7db2ff",
            border: "none",
          }}
          onClick={(event) => {
            event.stopPropagation();
            onPinClick?.(component.instance_id, pin.id);
          }}
        />
      ))}
      <ComponentSymbol kind={component.component_kind} />
      <Text fw={700} size="sm">
        {component.display_label}
      </Text>
      <Text size="xs" c="dimmed">
        {label}
      </Text>
    </div>
  );
}
