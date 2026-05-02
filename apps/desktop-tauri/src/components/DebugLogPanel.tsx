import {
  ActionIcon,
  Badge,
  Button,
  Code,
  Group,
  Modal,
  ScrollArea,
  Stack,
  Text,
  Title,
} from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import { Bug, Copy, Trash } from "lucide-react";
import { useEffect, useState } from "react";
import { logger, type LogEntry } from "../utils/logger";

const levelColors: Record<LogEntry["level"], string> = {
  trace: "gray",
  debug: "blue",
  info: "green",
  warn: "yellow",
  error: "red",
};

export function DebugLogPanel() {
  const [open, setOpen] = useState(false);
  const [entries, setEntries] = useState<readonly LogEntry[]>(logger.getEntries());
  const clipboard = useClipboard();

  useEffect(() => {
    const unsubscribe = logger.subscribe(() => setEntries(logger.getEntries()));
    return () => {
      unsubscribe();
    };
  }, []);

  return (
    <>
      <ActionIcon variant="light" color="gray" title="Debug logs" onClick={() => setOpen(true)}>
        <Bug size={16} />
      </ActionIcon>

      <Modal
        opened={open}
        onClose={() => setOpen(false)}
        title={<Title order={4}>Debug Logs</Title>}
        size="xl"
      >
        <Stack gap="sm">
          <Group gap="xs">
            <Button
              leftSection={<Copy size={14} />}
              variant="light"
              size="xs"
              onClick={() => clipboard.copy(logger.exportText())}
            >
              {clipboard.copied ? "Copied" : "Copy all"}
            </Button>
            <Button
              leftSection={<Trash size={14} />}
              variant="light"
              color="red"
              size="xs"
              onClick={() => logger.clear()}
            >
              Clear
            </Button>
            <Text size="xs" c="dimmed">
              {entries.length} entries
            </Text>
          </Group>

          <ScrollArea h={400}>
            <Stack gap={2}>
              {entries.map((entry) => (
                <Group key={entry.id} gap="xs" wrap="nowrap">
                  <Text size="xs" c="dimmed" w={60}>
                    {entry.timestamp}
                  </Text>
                  <Badge size="xs" color={levelColors[entry.level]} variant="light">
                    {entry.level}
                  </Badge>
                  <Badge size="xs" color="gray" variant="outline">
                    {entry.source}
                  </Badge>
                  <Code style={{ flex: 1, wordBreak: "break-all", fontSize: "0.75rem" }}>
                    {entry.message}
                  </Code>
                </Group>
              ))}
            </Stack>
          </ScrollArea>
        </Stack>
      </Modal>
    </>
  );
}
