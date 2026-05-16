import { Button, Stack, Text, Tooltip } from "@mantine/core";
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

const PALETTE_GROUPS = [
  {
    label: "Passive",
    items: [
      { kind: "resistor", label: "Resistor", icon: <Minus size={16} /> },
      { kind: "capacitor", label: "Capacitor", icon: <FlipHorizontal size={16} /> },
      { kind: "inductor", label: "Inductor", icon: <Layers size={16} /> },
    ],
  },
  {
    label: "Sources",
    items: [
      { kind: "voltage_source", label: "Voltage Source", icon: <Zap size={16} /> },
      { kind: "ground", label: "Ground", icon: <CircleDot size={16} /> },
    ],
  },
  {
    label: "Semiconductors",
    items: [
      { kind: "diode", label: "Diode", icon: <Triangle size={16} /> },
      { kind: "mosfet", label: "MOSFET", icon: <Maximize size={16} /> },
    ],
  },
  {
    label: "Op-Amps",
    items: [{ kind: "op_amp", label: "OpAmp", icon: <Cpu size={16} /> }],
  },
];

export function ComponentPalette({
  onAdd,
  disabled,
}: {
  onAdd: (kind: string) => void;
  disabled?: boolean;
}) {
  return (
    <div className="component-palette">
      <Text size="sm" fw={600} mb={8}>
        Component Palette
      </Text>
      <Stack gap="sm">
        {PALETTE_GROUPS.map((group) => (
          <div key={group.label} className="palette-group">
            <Text size="xs" fw={700} c="dimmed" mb={4}>
              {group.label}
            </Text>
            <div className="palette-button-grid">
              {group.items.map((item) => (
                <Tooltip key={item.kind} label={item.label}>
                  <Button
                    size="xs"
                    variant="light"
                    leftSection={item.icon}
                    onClick={() => onAdd(item.kind)}
                    disabled={disabled}
                    data-testid={`add-${item.kind}`}
                    title={
                      disabled ? "Wait for the current schematic operation to finish" : item.label
                    }
                  >
                    {item.label}
                  </Button>
                </Tooltip>
              ))}
            </div>
          </div>
        ))}
      </Stack>
    </div>
  );
}
