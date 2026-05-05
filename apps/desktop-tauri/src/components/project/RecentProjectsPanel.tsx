import { Button, Group, Paper, Stack, Text, Badge, ActionIcon } from "@mantine/core";
import { FolderOpen, Trash2, RefreshCw } from "lucide-react";
import type { RecentProjectEntryDto } from "../../types";

type RecentProjectsPanelProps = {
  projects: RecentProjectEntryDto[];
  onOpen: (path: string) => void;
  onRemove: (path: string) => void;
  onClearMissing: () => void;
  loading?: boolean;
};

export function RecentProjectsPanel({
  projects,
  onOpen,
  onRemove,
  onClearMissing,
  loading,
}: RecentProjectsPanelProps) {
  return (
    <Paper withBorder p="sm" radius="md">
      <Stack gap="xs">
        <Group justify="space-between">
          <Text size="sm" fw={600}>
            Recent Projects ({projects.length})
          </Text>
          <Button
            size="compact-xs"
            variant="light"
            leftSection={<RefreshCw size={12} />}
            onClick={onClearMissing}
            disabled={loading}
          >
            Clear missing
          </Button>
        </Group>
        {projects.length === 0 && (
          <Text size="xs" c="dimmed">
            No recent projects.
          </Text>
        )}
        {projects.map((project) => (
          <Group key={project.path} gap="xs" justify="space-between" wrap="nowrap">
            <Stack gap={2} style={{ flex: 1, minWidth: 0 }}>
              <Group gap="xs" wrap="nowrap">
                <Text size="sm" truncate="end" style={{ maxWidth: 200 }}>
                  {project.display_name}
                </Text>
                {!project.exists && (
                  <Badge size="xs" color="red" variant="light">
                    Missing
                  </Badge>
                )}
              </Group>
              <Text size="xs" c="dimmed" truncate="end">
                {project.path}
              </Text>
            </Stack>
            <Group gap="xs" wrap="nowrap">
              <ActionIcon
                size="sm"
                variant="light"
                disabled={!project.exists || loading}
                onClick={() => onOpen(project.path)}
                title="Open"
              >
                <FolderOpen size={14} />
              </ActionIcon>
              <ActionIcon
                size="sm"
                variant="light"
                color="red"
                disabled={loading}
                onClick={() => onRemove(project.path)}
                title="Remove"
              >
                <Trash2 size={14} />
              </ActionIcon>
            </Group>
          </Group>
        ))}
      </Stack>
    </Paper>
  );
}
