import { Button, Group, Stack, Text, TextInput } from "@mantine/core";
import { Save } from "lucide-react";
import { useState } from "react";
import type { SchematicEditableFieldDto } from "../../types";

type Props = {
  fields: SchematicEditableFieldDto[];
  componentId: string;
  onUpdate: (componentId: string, fieldId: string, value: string) => void;
  loading?: boolean;
};

export function QuickParameterEditor({ fields, componentId, onUpdate, loading }: Props) {
  const [values, setValues] = useState<Record<string, string>>({});

  return (
    <Stack gap="xs">
      {fields.length === 0 && (
        <Text size="sm" c="dimmed">
          No editable parameters
        </Text>
      )}
      {fields.map((field) => (
        <Group key={field.field_id} gap="xs" align="flex-end">
          <Text size="xs" style={{ minWidth: 100 }}>
            {field.label}
          </Text>
          <TextInput
            size="xs"
            defaultValue={field.current_value}
            onChange={(e) => setValues((prev) => ({ ...prev, [field.field_id]: e.target.value }))}
            disabled={loading || !field.editable}
            style={{ flex: 1 }}
          />
          {field.editable && (
            <Button
              size="xs"
              leftSection={<Save size={14} />}
              onClick={() =>
                onUpdate(componentId, field.field_id, values[field.field_id] ?? field.current_value)
              }
              loading={loading}
            >
              Update
            </Button>
          )}
        </Group>
      ))}
    </Stack>
  );
}
