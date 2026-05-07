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
  return (
    <div style={{ padding: 8, borderBottom: "1px solid #333" }}>
      <Text size="xs" fw={700} c="dimmed" mb={4}>
        Placeable Components
      </Text>
      <ScrollArea h={120}>
        <Stack gap={4}>
          {components.length === 0 && (
            <Text size="xs" c="dimmed">
              No placeable components found
            </Text>
          )}
          {components.map((c) => (
            <Button
              key={c.definition_id}
              size="xs"
              variant={selected?.definition_id === c.definition_id ? "filled" : "subtle"}
              leftSection={<Box size={14} />}
              onClick={() => onSelect(selected?.definition_id === c.definition_id ? null : c)}
              disabled={disabled}
              fullWidth
              styles={{ inner: { justifyContent: "flex-start" } }}
            >
              {c.name}{" "}
              <Text span size="xs" c="dimmed" ml={4}>
                ({c.category})
              </Text>
            </Button>
          ))}
        </Stack>
      </ScrollArea>
    </div>
  );
}
