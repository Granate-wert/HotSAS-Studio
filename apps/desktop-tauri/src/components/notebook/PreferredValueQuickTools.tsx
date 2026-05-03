import { Button, Card, Group, Text } from "@mantine/core";

export type PreferredValueQuickToolsProps = {
  onInsert: (template: string) => void;
};

export function PreferredValueQuickTools({ onInsert }: PreferredValueQuickToolsProps) {
  const tools = [
    { label: "nearestE24", template: "nearestE(15.93k, E24, Ohm)" },
    { label: "nearestE96", template: "nearestE(15.93k, E96, Ohm)" },
    { label: "lowerE96", template: "lowerE(15.93k, E96, Ohm)" },
    { label: "higherE96", template: "higherE(15.93k, E96, Ohm)" },
  ];

  return (
    <Card withBorder>
      <Text fw={500} size="sm">
        Preferred Value Quick Tools
      </Text>
      <Group mt="xs">
        {tools.map((tool) => (
          <Button
            key={tool.label}
            size="compact-xs"
            variant="light"
            onClick={() => onInsert(tool.template)}
          >
            {tool.label}
          </Button>
        ))}
      </Group>
    </Card>
  );
}
