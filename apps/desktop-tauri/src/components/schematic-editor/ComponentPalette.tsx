import { Button, Group, Text, Tooltip } from "@mantine/core";
import {
  CircleDot,
  Cpu,
  FlipHorizontal,
  Layers,
  Maximize,
  Minus,
  Triangle,
  Zap,
} from "lucide-react";

const PALETTE_ITEMS = [
  { kind: "resistor", label: "Resistor", icon: <Minus size={16} /> },
  { kind: "capacitor", label: "Capacitor", icon: <FlipHorizontal size={16} /> },
  { kind: "inductor", label: "Inductor", icon: <Layers size={16} /> },
  { kind: "diode", label: "Diode", icon: <Triangle size={16} /> },
  { kind: "op_amp", label: "OpAmp", icon: <Cpu size={16} /> },
  { kind: "mosfet", label: "MOSFET", icon: <Maximize size={16} /> },
  { kind: "voltage_source", label: "Voltage Source", icon: <Zap size={16} /> },
  { kind: "ground", label: "Ground", icon: <CircleDot size={16} /> },
];

export function ComponentPalette({
  onAdd,
  disabled,
}: {
  onAdd: (kind: string) => void;
  disabled?: boolean;
}) {
  return (
    <div className="component-palette" style={{ padding: 8 }}>
      <Text size="sm" fw={600} mb={8}>
        Component Palette
      </Text>
      <Group gap="xs" wrap="wrap">
        {PALETTE_ITEMS.map((item) => (
          <Tooltip key={item.kind} label={item.label}>
            <Button
              size="xs"
              variant="light"
              leftSection={item.icon}
              onClick={() => onAdd(item.kind)}
              disabled={disabled}
              data-testid={`add-${item.kind}`}
            >
              {item.label}
            </Button>
          </Tooltip>
        ))}
      </Group>
    </div>
  );
}
