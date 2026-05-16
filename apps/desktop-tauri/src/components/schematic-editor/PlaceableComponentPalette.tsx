import { Button, ScrollArea, Stack, Text } from "@mantine/core";
import { Box } from "lucide-react";
import type { PlaceableComponentDto } from "../../types";

type Props = {
  components: PlaceableComponentDto[];
  onSelect: (component: PlaceableComponentDto | null) => void;
  selected: PlaceableComponentDto | null;
  disabled?: boolean;
};

export function PlaceableComponentPalette({ components, onSelect, selected, disabled }: Props) {
  const grouped = [
    {
      label: "Passive",
      components: components.filter((c) =>
        ["passive", "resistor", "capacitor", "inductor"].some((token) =>
          `${c.category} ${c.component_kind}`.toLowerCase().includes(token),
        ),
      ),
    },
    {
      label: "Sources",
      components: components.filter((c) =>
        ["source", "ground"].some((token) =>
          `${c.category} ${c.component_kind}`.toLowerCase().includes(token),
        ),
      ),
    },
    {
      label: "Semiconductors",
      components: components.filter((c) =>
        ["diode", "mosfet", "bjt"].some((token) =>
          `${c.category} ${c.component_kind}`.toLowerCase().includes(token),
        ),
      ),
    },
    {
      label: "Op-Amps",
      components: components.filter((c) =>
        `${c.category} ${c.component_kind}`.toLowerCase().includes("op"),
      ),
    },
  ].filter((group) => group.components.length > 0);

  return (
    <div className="component-palette">
      <Text size="xs" fw={700} c="dimmed" mb={4}>
        Placeable Components
      </Text>
      <ScrollArea h={240}>
        <Stack gap={4}>
          {components.length === 0 && (
            <Text size="xs" c="dimmed">
              No placeable components found
            </Text>
          )}
          {(grouped.length > 0 ? grouped : [{ label: "Other", components }]).map((group) => (
            <div key={group.label} className="palette-group">
              <Text size="xs" fw={700} c="dimmed" mb={4}>
                {group.label}
              </Text>
              <Stack gap={4}>
                {group.components.map((c) => (
                  <Button
                    key={c.definition_id}
                    size="xs"
                    variant={selected?.definition_id === c.definition_id ? "filled" : "subtle"}
                    leftSection={<Box size={14} />}
                    onClick={() => onSelect(selected?.definition_id === c.definition_id ? null : c)}
                    disabled={disabled}
                    fullWidth
                    title={disabled ? "Wait for the current schematic operation to finish" : c.name}
                    styles={{ inner: { justifyContent: "flex-start" } }}
                  >
                    {c.name}{" "}
                    <Text span size="xs" c="dimmed" ml={4}>
                      ({c.category})
                    </Text>
                  </Button>
                ))}
              </Stack>
            </div>
          ))}
        </Stack>
      </ScrollArea>
    </div>
  );
}
