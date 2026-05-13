import { Text } from "@mantine/core";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import type { CSSProperties } from "react";
import type { ComponentDto } from "../../../types";

type SymbolKind =
  | "resistor"
  | "capacitor"
  | "inductor"
  | "voltage_source"
  | "ground"
  | "diode"
  | "op_amp"
  | "mosfet";

type NodeData = {
  component: ComponentDto;
  onPinClick?: (componentId: string, pinId: string) => void;
};

const SYMBOL_WIDTH = 120;
const SYMBOL_HEIGHT = 84;

function mapSide(side: string): Position {
  switch (side) {
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

function normalizeKind(kind: string): SymbolKind | "generic" {
  const base = kind.startsWith("generic_") ? kind.slice(8) : kind;
  if (base === "mosfet_n" || base === "mosfet_p" || base === "mosfet") {
    return "mosfet";
  }
  if (
    base === "resistor" ||
    base === "capacitor" ||
    base === "inductor" ||
    base === "voltage_source" ||
    base === "ground" ||
    base === "diode" ||
    base === "op_amp"
  ) {
    return base;
  }
  return "generic";
}

function valueLabel(component: ComponentDto) {
  return component.parameters[0]?.value.display ?? component.component_kind;
}

function pinStyle(pin: ComponentDto["pins"][number]): CSSProperties {
  const left = SYMBOL_WIDTH / 2 + pin.x;
  const top = SYMBOL_HEIGHT / 2 + pin.y;
  return {
    width: 9,
    height: 9,
    left,
    top,
    background: "#f1c663",
    border: "1px solid #0f1520",
    boxShadow: "0 0 0 2px rgba(241, 198, 99, 0.2)",
  };
}

function SymbolSvg({ kind }: { kind: SymbolKind }) {
  const stroke = "#d7e0ee";
  const accent = "#f1c663";
  const strokeWidth = 2;

  switch (kind) {
    case "resistor":
      return (
        <svg data-testid="schematic-symbol-resistor" viewBox="0 0 120 84">
          <path
            d="M8 42 H24 V28 H96 V56 H24 V42 M96 42 H112"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
        </svg>
      );
    case "capacitor":
      return (
        <svg data-testid="schematic-symbol-capacitor" viewBox="0 0 120 84">
          <path d="M60 8 V27 M60 57 V76" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
          <path d="M38 34 H82 M38 50 H82" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
        </svg>
      );
    case "inductor":
      return (
        <svg data-testid="schematic-symbol-inductor" viewBox="0 0 120 84">
          <path
            d="M8 42 H22 C22 26 42 26 42 42 C42 26 62 26 62 42 C62 26 82 26 82 42 C82 26 102 26 102 42 H112"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
        </svg>
      );
    case "voltage_source":
      return (
        <svg data-testid="schematic-symbol-voltage-source" viewBox="0 0 120 84">
          <path d="M60 4 V18 M60 66 V80" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
          <circle cx="60" cy="42" r="24" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
          <path d="M60 27 V37 M55 32 H65 M55 54 H65" stroke={accent} strokeWidth={strokeWidth} />
        </svg>
      );
    case "ground":
      return (
        <svg data-testid="schematic-symbol-ground" viewBox="0 0 120 84">
          <path
            d="M60 8 V32 M34 32 H86 M42 44 H78 M50 56 H70"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
        </svg>
      );
    case "diode":
      return (
        <svg data-testid="schematic-symbol-diode" viewBox="0 0 120 84">
          <path
            d="M8 42 H38 M82 42 H112 M82 22 V62"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
          <path d="M38 22 L82 42 L38 62 Z" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
        </svg>
      );
    case "op_amp":
      return (
        <svg data-testid="schematic-symbol-op-amp" viewBox="0 0 120 84">
          <path d="M20 12 L20 72 L96 42 Z" fill="none" stroke={stroke} strokeWidth={strokeWidth} />
          <path
            d="M4 32 H20 M4 52 H20 M96 42 H116 M60 8 V24 M60 60 V76"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
          <text x="26" y="36" fill={accent} fontSize="14">
            -
          </text>
          <text x="26" y="58" fill={accent} fontSize="14">
            +
          </text>
        </svg>
      );
    case "mosfet":
      return (
        <svg data-testid="schematic-symbol-mosfet" viewBox="0 0 120 84">
          <path
            d="M78 8 V28 M78 56 V76 M78 28 V56 M54 24 V60 M8 42 H44 M54 30 H70 M54 54 H70"
            fill="none"
            stroke={stroke}
            strokeWidth={strokeWidth}
          />
          <path d="M68 54 L78 48 L68 42" fill="none" stroke={accent} strokeWidth={strokeWidth} />
        </svg>
      );
  }
}

export function SchematicSymbolNode({ data, selected }: NodeProps) {
  const { component, onPinClick } = data as NodeData;
  const kind = normalizeKind(component.component_kind);

  if (kind === "generic") {
    return null;
  }

  return (
    <div
      className="schematic-symbol-node"
      data-testid={`schematic-node-${component.instance_id}`}
      data-symbol-kind={kind}
      style={{
        width: SYMBOL_WIDTH,
        minHeight: 118,
        position: "relative",
        color: "#e6edf6",
        cursor: "pointer",
        outline: selected ? "2px solid #f1c663" : "1px solid transparent",
      }}
    >
      {component.pins.map((pin) => (
        <Handle
          key={pin.id}
          type="source"
          id={pin.id}
          position={mapSide(pin.side)}
          isConnectableStart
          isConnectableEnd
          style={pinStyle(pin)}
          onClick={(event) => {
            event.stopPropagation();
            onPinClick?.(component.instance_id, pin.id);
          }}
        />
      ))}
      <div className="schematic-symbol-body">
        <SymbolSvg kind={kind} />
      </div>
      <div className="schematic-symbol-labels">
        <Text fw={700} size="sm" lh={1.1}>
          {component.display_label}
        </Text>
        <Text size="xs" c="dimmed" lh={1.1}>
          {valueLabel(component)}
        </Text>
      </div>
    </div>
  );
}
