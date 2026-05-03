import { Button, Checkbox, Group, Paper, Stack, Text, Title } from "@mantine/core";
import { useState } from "react";
import { backend } from "../../api";
import { useHotSasStore } from "../../store";
import type { ProjectDto, SelectedRegionPreviewDto } from "../../types";
import { SelectedRegionPreviewCard } from "./SelectedRegionPreviewCard";
import { SelectedRegionResultCard } from "./SelectedRegionResultCard";

export function SelectedRegionPanel({ project }: { project: ProjectDto | null }) {
  const {
    selectedRegionComponentIds,
    selectedRegionPreview,
    selectedRegionAnalysisResult,
    busy,
    setBusy,
    setError,
    setSelectedRegionComponentIds,
    setSelectedRegionPreview,
    setSelectedRegionAnalysisResult,
  } = useHotSasStore();

  const [localSelection, setLocalSelection] = useState<Set<string>>(
    new Set(selectedRegionComponentIds),
  );

  const components = project?.schematic.components ?? [];

  const toggleComponent = (id: string) => {
    const next = new Set(localSelection);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    setLocalSelection(next);
  };

  const run = async <T,>(operation: () => Promise<T>, onResult: (result: T) => void) => {
    setBusy(true);
    setError(null);
    try {
      const result = await operation();
      onResult(result);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      setBusy(false);
    }
  };

  const handlePreview = () => {
    const ids = Array.from(localSelection);
    setSelectedRegionComponentIds(ids);
    run(() => backend.previewSelectedRegion(ids), setSelectedRegionPreview);
  };

  const handleAnalyze = () => {
    const ids = Array.from(localSelection);
    setSelectedRegionComponentIds(ids);
    const preview = selectedRegionPreview;
    const request = {
      component_ids: ids,
      input_port: preview?.region.input_port ?? null,
      output_port: preview?.region.output_port ?? null,
      reference_node: preview?.region.reference_node ?? null,
      analysis_direction: "AllAvailable",
      analysis_mode: "AllAvailable",
    };
    run(() => backend.analyzeSelectedRegion(request), setSelectedRegionAnalysisResult);
  };

  const handleClear = () => {
    setLocalSelection(new Set());
    setSelectedRegionComponentIds([]);
    setSelectedRegionPreview(null);
    setSelectedRegionAnalysisResult(null);
  };

  const selectionCount = localSelection.size;

  return (
    <Stack gap="sm">
      <Group justify="space-between">
        <Title order={5}>Selected Region</Title>
        <Text size="xs" c="dimmed">
          {selectionCount} selected
        </Text>
      </Group>

      <Paper withBorder p="xs">
        <Stack gap="xs">
          {components.length === 0 && (
            <Text size="sm" c="dimmed">
              No components in circuit.
            </Text>
          )}
          {components.map((c) => (
            <Checkbox
              key={c.instance_id}
              label={`${c.display_label} (${c.component_kind})`}
              checked={localSelection.has(c.instance_id)}
              onChange={() => toggleComponent(c.instance_id)}
              size="xs"
            />
          ))}
        </Stack>
      </Paper>

      <Group gap="xs">
        <Button
          size="xs"
          onClick={handlePreview}
          disabled={selectionCount === 0 || busy}
          loading={busy}
        >
          Preview
        </Button>
        <Button
          size="xs"
          variant="light"
          onClick={handleAnalyze}
          disabled={selectionCount === 0 || busy}
          loading={busy}
        >
          Analyze
        </Button>
        <Button size="xs" variant="subtle" color="gray" onClick={handleClear} disabled={busy}>
          Clear
        </Button>
      </Group>

      {selectedRegionPreview && <SelectedRegionPreviewCard preview={selectedRegionPreview} />}

      {selectedRegionAnalysisResult && (
        <SelectedRegionResultCard result={selectedRegionAnalysisResult} />
      )}
    </Stack>
  );
}
