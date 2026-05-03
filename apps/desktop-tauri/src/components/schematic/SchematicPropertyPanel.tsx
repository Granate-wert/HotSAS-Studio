import { Button, Stack, Text, TextInput } from "@mantine/core";
import { useState } from "react";
import { backend } from "../../api";
import { useHotSasStore } from "../../store";
import type { SelectedComponentDto } from "../../types";

type SchematicPropertyPanelProps = {
  component: SelectedComponentDto | null;
  onUpdate: () => void;
};

export function SchematicPropertyPanel({ component, onUpdate }: SchematicPropertyPanelProps) {
  const [edits, setEdits] = useState<Record<string, string>>({});
  const [busy, setBusy] = useState(false);

  if (!component) {
    return (
      <Stack gap="sm">
        <Text c="dimmed" size="sm">
          Select a component on the schematic to view its properties.
        </Text>
      </Stack>
    );
  }

  const handleApply = async (paramName: string) => {
    const value = edits[paramName];
    if (value === undefined) return;
    setBusy(true);
    try {
      const updatedProject = await backend.updateComponentParameter(
        component.instance_id,
        paramName,
        value,
        null,
      );
      useHotSasStore.getState().setProject(updatedProject);
      onUpdate();
    } catch (error) {
      useHotSasStore.getState().setError(String(error));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Stack gap="sm">
      <Text fw={700} size="sm">
        {component.title}
      </Text>
      <Text size="xs" c="dimmed">
        {component.instance_id} ({component.component_kind})
      </Text>
      {component.parameters.map((param) => (
        <Stack gap={4} key={param.name}>
          <Text size="xs">{param.name}</Text>
          <TextInput
            size="xs"
            defaultValue={param.value}
            disabled={busy}
            onChange={(e) => setEdits((prev) => ({ ...prev, [param.name]: e.target.value }))}
            rightSection={
              <Button
                size="compact-xs"
                onClick={() => handleApply(param.name)}
                disabled={busy || edits[param.name] === undefined}
              >
                Apply
              </Button>
            }
          />
          {param.unit && (
            <Text size="xs" c="dimmed">
              Unit: {param.unit}
            </Text>
          )}
        </Stack>
      ))}
    </Stack>
  );
}
