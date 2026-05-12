import { Button, Stack, Text } from "@mantine/core";
import { BookOpen, ExternalLink } from "lucide-react";

export function LibraryPanel() {
  return (
    <Stack gap="xs" p="md">
      <Text size="sm" fw={600}>
        Quick References
      </Text>
      <Text size="sm" c="dimmed">
        Formula packs: basic electronics, filters, op-amp, SMPS placeholders
      </Text>
      <Text size="sm" c="dimmed">
        Component model: symbol, footprint, simulation model, datasheet, BOM fields
      </Text>
      <Text size="sm" c="dimmed">
        Export placeholders: PDF, KiCad, Altium workflow package
      </Text>
      <Button
        variant="light"
        leftSection={<BookOpen size={16} />}
        onClick={() => {
          const event = new CustomEvent("navigate", { detail: "components" });
          window.dispatchEvent(event);
        }}
        size="sm"
      >
        Open Component Library
      </Button>
    </Stack>
  );
}
