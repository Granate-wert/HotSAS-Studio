import { useState } from "react";
import { Button, Group, Select, Text, TextInput } from "@mantine/core";
import type { NetDto } from "../../types";

export function NetLabelEditor({
  nets,
  onRename,
  onCancel,
}: {
  nets: NetDto[];
  onRename: (netId: string, newName: string) => void;
  onCancel: () => void;
}) {
  const [selectedNet, setSelectedNet] = useState<string | null>(null);
  const [newName, setNewName] = useState("");

  const canRename = selectedNet && newName.trim().length > 0;

  return (
    <div style={{ padding: 8 }}>
      <Text size="sm" fw={600} mb={8}>
        Rename Net
      </Text>
      <Select
        label="Net"
        data={nets.map((n) => ({ value: n.id, label: n.name }))}
        value={selectedNet}
        onChange={setSelectedNet}
        size="xs"
        mb={8}
        style={{ minWidth: 180 }}
      />
      <TextInput
        label="New Name"
        value={newName}
        onChange={(e) => setNewName(e.currentTarget.value)}
        size="xs"
        mb={8}
      />
      <Group gap="xs">
        <Button
          size="xs"
          disabled={!canRename}
          onClick={() => canRename && onRename(selectedNet, newName.trim())}
        >
          Rename
        </Button>
        <Button size="xs" variant="light" onClick={onCancel}>
          Cancel
        </Button>
      </Group>
    </div>
  );
}
