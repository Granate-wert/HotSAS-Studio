import { Alert, Button, Group, Text } from "@mantine/core";
import { AlertTriangle, Save, SaveAll } from "lucide-react";
import type { ProjectSessionStateDto } from "../../types";

type UnsavedChangesBannerProps = {
  session: ProjectSessionStateDto | null;
  onSave: () => void;
  onSaveAs: (path: string) => void;
};

export function UnsavedChangesBanner({ session, onSave, onSaveAs }: UnsavedChangesBannerProps) {
  if (!session?.dirty) {
    return null;
  }

  return (
    <Alert
      color="orange"
      icon={<AlertTriangle size={16} />}
      title="Unsaved changes"
      variant="light"
      style={{ marginBottom: 8 }}
    >
      <Group gap="xs" justify="space-between">
        <Text size="sm">
          Project "{session.current_project_name ?? "Untitled"}" has unsaved changes.
        </Text>
        <Group gap="xs">
          {session.current_project_path && (
            <Button size="compact-xs" leftSection={<Save size={12} />} onClick={onSave}>
              Save
            </Button>
          )}
          <Button
            size="compact-xs"
            variant="light"
            leftSection={<SaveAll size={12} />}
            onClick={() => {
              const path = window.prompt("Save As path:", session.current_project_path ?? "");
              if (path) onSaveAs(path);
            }}
          >
            Save As
          </Button>
        </Group>
      </Group>
    </Alert>
  );
}
