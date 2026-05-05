import { useEffect, useState } from "react";
import {
  Button,
  Group,
  Stack,
  Title,
  Switch,
  TextInput,
  Text,
  Badge,
  ScrollArea,
  Divider,
} from "@mantine/core";
import { FileText, Download, History, RefreshCw, CheckCircle, AlertCircle } from "lucide-react";
import { backend } from "../api";
import { PreBlock } from "../components/PreBlock";
import type { ExportCapabilityDto, ExportResultDto } from "../types";

export function ExportScreen({
  hasProject,
  capabilities,
  lastResult,
  onLoadCapabilities,
  onExport,
}: {
  hasProject: boolean;
  capabilities: ExportCapabilityDto[];
  lastResult: ExportResultDto | null;
  onLoadCapabilities: () => void;
  onExport: (format: string, writeToFile: boolean, outputDir: string) => void;
}) {
  const [writeToFile, setWriteToFile] = useState(false);
  const [outputDir, setOutputDir] = useState("./exports");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [history, setHistory] = useState<ExportResultDto[]>([]);
  const [hasAttemptedLoad, setHasAttemptedLoad] = useState(false);

  useEffect(() => {
    if (!hasAttemptedLoad && capabilities.length === 0) {
      setHasAttemptedLoad(true);
      onLoadCapabilities();
    }
  }, [hasAttemptedLoad, capabilities.length, onLoadCapabilities]);

  const handleExport = async (format: string) => {
    setBusy(true);
    setError(null);
    try {
      onExport(format, writeToFile, outputDir);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  const loadHistory = async () => {
    setBusy(true);
    try {
      const entries = await backend.exportHistory();
      setHistory(
        entries.map((h) => ({
          format: h.format,
          content: "",
          file_path: h.file_path,
          success: h.success,
          message: h.message,
        })),
      );
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <Title order={2}>Export Center</Title>
        <Text size="sm" c="dimmed">
          Generate and download design artifacts in multiple formats.
        </Text>

        <Group gap="xs" align="center">
          <Switch
            label="Write to file"
            checked={writeToFile}
            onChange={(event) => setWriteToFile(event.currentTarget.checked)}
          />
          {writeToFile && (
            <TextInput
              placeholder="Output directory"
              value={outputDir}
              onChange={(event) => setOutputDir(event.currentTarget.value)}
              style={{ minWidth: 200 }}
            />
          )}
        </Group>

        <Divider />

        <Stack gap="sm">
          <Text size="sm" fw={500}>
            Available Formats
          </Text>
          <Group gap="xs">
            {capabilities.map((cap) => (
              <Button
                key={cap.format}
                variant="light"
                size="xs"
                leftSection={<FileText size={14} />}
                disabled={!hasProject || !cap.available || busy}
                onClick={() => handleExport(cap.format)}
                title={cap.description}
              >
                {cap.label}
                <Badge size="xs" ml={4} variant="outline">
                  .{cap.file_extension}
                </Badge>
                {cap.format === "altium_workflow_package" && (
                  <Badge size="xs" ml={4} color="orange">
                    Placeholder
                  </Badge>
                )}
              </Button>
            ))}
            <Button
              variant="default"
              size="xs"
              leftSection={<RefreshCw size={14} />}
              onClick={onLoadCapabilities}
              disabled={busy}
            >
              Refresh
            </Button>
          </Group>
        </Stack>

        <Divider />

        {error && (
          <Group gap="xs" c="red">
            <AlertCircle size={16} />
            <Text size="sm">{error}</Text>
          </Group>
        )}

        {lastResult && (
          <Stack gap="xs">
            <Group gap="xs">
              {lastResult.success ? (
                <CheckCircle size={16} color="green" />
              ) : (
                <AlertCircle size={16} color="red" />
              )}
              <Text size="sm" fw={500}>
                {lastResult.message}
              </Text>
              {lastResult.file_path && (
                <Badge size="sm" variant="light" color="blue">
                  {lastResult.file_path}
                </Badge>
              )}
            </Group>
            <ScrollArea h={300}>
              <PreBlock text={lastResult.content || "No content"} />
            </ScrollArea>
          </Stack>
        )}

        <Divider />

        <Group gap="xs">
          <Button
            variant="default"
            size="xs"
            leftSection={<History size={14} />}
            onClick={loadHistory}
            disabled={busy}
          >
            Load History
          </Button>
        </Group>

        {history.length > 0 && (
          <Stack gap="xs">
            <Text size="sm" fw={500}>
              Recent Exports
            </Text>
            {history.map((h, i) => (
              <Group key={i} gap="xs">
                <Badge size="xs">{h.format}</Badge>
                <Text size="xs" c={h.success ? "green" : "red"}>
                  {h.message}
                </Text>
                {h.file_path && (
                  <Text size="xs" c="dimmed">
                    {h.file_path}
                  </Text>
                )}
              </Group>
            ))}
          </Stack>
        )}
      </div>
    </section>
  );
}
