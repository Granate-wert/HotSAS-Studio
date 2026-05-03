import { Alert, Button, Group, Text } from "@mantine/core";
import type { ComponentDetailsDto, SelectedComponentDto } from "../../types";

type Props = {
  selectedComponent: SelectedComponentDto | null;
  libraryComponent: ComponentDetailsDto | null;
  onAssign: () => void;
};

export function AssignComponentPanel({ selectedComponent, libraryComponent, onAssign }: Props) {
  if (!selectedComponent) {
    return (
      <Alert color="blue">Select a component in the Schematic Editor to enable assignment.</Alert>
    );
  }

  return (
    <Alert color="green">
      <Group justify="space-between">
        <div>
          <Text size="sm">
            Selected schematic instance: <strong>{selectedComponent.instance_id}</strong>
          </Text>
          {libraryComponent && (
            <Text size="sm">
              Library component: <strong>{libraryComponent.name}</strong>
            </Text>
          )}
        </div>
        <Button onClick={onAssign} disabled={!libraryComponent}>
          Assign to selected instance
        </Button>
      </Group>
    </Alert>
  );
}
