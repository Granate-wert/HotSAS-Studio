import { useState } from "react";
import { Button, Group, Text, TextInput, Badge, Tooltip } from "@mantine/core";
import { FolderOpen, Save, SaveAll, FilePlus } from "lucide-react";
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
    <div className="project-toolbar">
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
          <Button
            size="xs"
            leftSection={<Save size={14} />}
            onClick={onSave}
            disabled={!hasProject || !dirty || loading}
            loading={loading}
            title={
              !hasProject
                ? "Open or create a project first"
                : !dirty
                  ? "Project has no unsaved changes"
                  : loading
                    ? "Wait for the current project operation to finish"
                    : "Save current project"
            }
          >
            Save
          </Button>
          <details className="project-path-tools">
            <summary>Advanced paths</summary>
            <Group gap="xs" wrap="wrap" mt={6}>
              <TextInput
                size="xs"
                placeholder="Path to open .circuit"
                value={openPath}
                onChange={(e) => setOpenPath(e.currentTarget.value)}
                className="project-path-input"
                rightSection={
                  <Button
                    size="compact-xs"
                    leftSection={<FolderOpen size={14} />}
                    onClick={() => {
                      if (openPath) onOpen(openPath);
                    }}
                    disabled={!openPath || loading}
                    title={
                      !openPath
                        ? "Enter a .circuit path first"
                        : loading
                          ? "Wait for the current project operation to finish"
                          : "Open project package"
                    }
                  >
                    Open
                  </Button>
                }
              />
              <TextInput
                size="xs"
                placeholder="Path to save .circuit"
                value={saveAsPath}
                onChange={(e) => setSaveAsPath(e.currentTarget.value)}
                className="project-path-input"
                rightSection={
                  <Button
                    size="compact-xs"
                    leftSection={<SaveAll size={14} />}
                    onClick={() => {
                      if (saveAsPath) onSaveAs(saveAsPath);
                    }}
                    disabled={!hasProject || !saveAsPath || loading}
                    title={
                      !hasProject
                        ? "Open or create a project first"
                        : !saveAsPath
                          ? "Enter a .circuit path first"
                          : loading
                            ? "Wait for the current project operation to finish"
                            : "Save project package to this path"
                    }
                  >
                    Save As
                  </Button>
                }
              />
            </Group>
          </details>
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
