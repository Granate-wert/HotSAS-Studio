import { useState } from "react";
import { Button, Group, Text, TextInput, Badge, Tooltip } from "@mantine/core";
import { CircuitBoard, FolderOpen, Save, SaveAll, FilePlus } from "lucide-react";
import type { ProjectSessionStateDto } from "../../types";

type ProjectToolbarProps = {
  session: ProjectSessionStateDto | null;
  onNewDemo: () => void;
  onOpen: (path: string) => void;
  onSave: () => void;
  onSaveAs: (path: string) => void;
  loading?: boolean;
};

export function ProjectToolbar({
  session,
  onNewDemo,
  onOpen,
  onSave,
  onSaveAs,
  loading,
}: ProjectToolbarProps) {
  const [openPath, setOpenPath] = useState("");
  const [saveAsPath, setSaveAsPath] = useState("");

  const hasProject = Boolean(session?.current_project_id);
  const dirty = session?.dirty ?? false;
  const projectName = session?.current_project_name ?? "No project";
  const projectPath = session?.current_project_path ?? "-";

  return (
    <div className="project-toolbar" style={{ padding: 8 }}>
      <Group gap="xs" justify="space-between" wrap="wrap">
        <Group gap="xs" wrap="wrap">
          <Button
            size="xs"
            leftSection={<FilePlus size={14} />}
            onClick={onNewDemo}
            loading={loading}
          >
            New Demo
          </Button>
          <TextInput
            size="xs"
            placeholder="Path to open .circuit"
            value={openPath}
            onChange={(e) => setOpenPath(e.currentTarget.value)}
            style={{ width: 200 }}
            rightSection={
              <Button
                size="compact-xs"
                leftSection={<FolderOpen size={14} />}
                onClick={() => {
                  if (openPath) onOpen(openPath);
                }}
                disabled={!openPath || loading}
              >
                Open
              </Button>
            }
          />
          <Button
            size="xs"
            leftSection={<Save size={14} />}
            onClick={onSave}
            disabled={!hasProject || !dirty || loading}
            loading={loading}
          >
            Save
          </Button>
          <TextInput
            size="xs"
            placeholder="Path to save .circuit"
            value={saveAsPath}
            onChange={(e) => setSaveAsPath(e.currentTarget.value)}
            style={{ width: 200 }}
            rightSection={
              <Button
                size="compact-xs"
                leftSection={<SaveAll size={14} />}
                onClick={() => {
                  if (saveAsPath) onSaveAs(saveAsPath);
                }}
                disabled={!hasProject || !saveAsPath || loading}
              >
                Save As
              </Button>
            }
          />
        </Group>
        <Group gap="xs" wrap="wrap">
          <Text size="sm" fw={600}>
            {projectName}
          </Text>
          {dirty && (
            <Badge size="xs" color="orange" variant="light">
              Unsaved changes
            </Badge>
          )}
          {!dirty && hasProject && (
            <Badge size="xs" color="green" variant="light">
              Saved
            </Badge>
          )}
          <Tooltip label={projectPath}>
            <Text size="xs" c="dimmed" style={{ maxWidth: 300 }} truncate="end">
              {projectPath}
            </Text>
          </Tooltip>
        </Group>
      </Group>
    </div>
  );
}
