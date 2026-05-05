import { useState } from "react";
import { Button, Group, Select, Text, TextInput } from "@mantine/core";
import type { ComponentDto } from "../../types";

export function ConnectionPanel({
  components,
  onConnect,
  onCancel,
}: {
  components: ComponentDto[];
  onConnect: (request: {
    from_component_id: string;
    from_pin_id: string;
    to_component_id: string;
    to_pin_id: string;
    net_name?: string | null;
  }) => void;
  onCancel: () => void;
}) {
  const [fromComponent, setFromComponent] = useState<string | null>(null);
  const [fromPin, setFromPin] = useState<string | null>(null);
  const [toComponent, setToComponent] = useState<string | null>(null);
  const [toPin, setToPin] = useState<string | null>(null);
  const [netName, setNetName] = useState("");

  const fromPins = components.find((c) => c.instance_id === fromComponent)?.pins ?? [];
  const toPins = components.find((c) => c.instance_id === toComponent)?.pins ?? [];

  const canConnect = fromComponent && fromPin && toComponent && toPin;

  return (
    <div style={{ padding: 8 }}>
      <Text size="sm" fw={600} mb={8}>
        Connect Pins
      </Text>
      <Group gap="xs" mb={8}>
        <Select
          label="From Component"
          data={components.map((c) => ({
            value: c.instance_id,
            label: c.display_label || c.instance_id,
          }))}
          value={fromComponent}
          onChange={setFromComponent}
          size="xs"
          style={{ minWidth: 140 }}
        />
        <Select
          label="From Pin"
          data={fromPins.map((p) => ({ value: p.id, label: p.name }))}
          value={fromPin}
          onChange={setFromPin}
          size="xs"
          style={{ minWidth: 100 }}
          disabled={!fromComponent}
        />
      </Group>
      <Group gap="xs" mb={8}>
        <Select
          label="To Component"
          data={components.map((c) => ({
            value: c.instance_id,
            label: c.display_label || c.instance_id,
          }))}
          value={toComponent}
          onChange={setToComponent}
          size="xs"
          style={{ minWidth: 140 }}
        />
        <Select
          label="To Pin"
          data={toPins.map((p) => ({ value: p.id, label: p.name }))}
          value={toPin}
          onChange={setToPin}
          size="xs"
          style={{ minWidth: 100 }}
          disabled={!toComponent}
        />
      </Group>
      <TextInput
        label="Net Name (optional)"
        value={netName}
        onChange={(e) => setNetName(e.currentTarget.value)}
        size="xs"
        mb={8}
      />
      <Group gap="xs">
        <Button
          size="xs"
          disabled={!canConnect}
          onClick={() =>
            canConnect &&
            onConnect({
              from_component_id: fromComponent,
              from_pin_id: fromPin,
              to_component_id: toComponent,
              to_pin_id: toPin,
              net_name: netName || null,
            })
          }
        >
          Connect
        </Button>
        <Button size="xs" variant="light" onClick={onCancel}>
          Cancel
        </Button>
      </Group>
    </div>
  );
}
