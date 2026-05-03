import { Stack, Title } from "@mantine/core";
import { useState } from "react";
import { backend } from "../../api";
import { useHotSasStore } from "../../store";
import { logger } from "../../utils/logger";
import type { NotebookEvaluationResultDto } from "../../types";
import { NotebookInput } from "./NotebookInput";
import { NotebookResultCard } from "./NotebookResultCard";
import { NotebookVariableTable } from "./NotebookVariableTable";
import { NotebookHistory } from "./NotebookHistory";
import { PreferredValueQuickTools } from "./PreferredValueQuickTools";
import { ApplyNotebookOutputPanel } from "./ApplyNotebookOutputPanel";

export function EngineeringNotebook() {
  const [input, setInput] = useState("");
  const [result, setResult] = useState<NotebookEvaluationResultDto | null>(null);
  const [loading, setLoading] = useState(false);
  const notebookState = useHotSasStore((s) => s.notebookState);
  const setNotebookState = useHotSasStore((s) => s.setNotebookState);
  const setLastNotebookResult = useHotSasStore((s) => s.setLastNotebookResult);
  const selectedComponent = useHotSasStore((s) => s.selectedComponent);

  async function evaluate() {
    if (!input.trim()) return;
    setLoading(true);
    try {
      const res = await backend.evaluateNotebookInput({ input });
      setResult(res);
      setLastNotebookResult(res);
      const state = await backend.getNotebookState();
      setNotebookState(state);
      logger.info(`Notebook evaluated: ${input} -> ${res.status}`);
    } catch (err) {
      logger.error(`Notebook evaluation failed: ${String(err)}`);
      setResult({
        input,
        status: "error",
        kind: "Text",
        outputs: [],
        variables: [],
        message: String(err),
        warnings: [],
      });
    } finally {
      setLoading(false);
    }
  }

  async function clear() {
    try {
      const state = await backend.clearNotebook();
      setNotebookState(state);
      setResult(null);
      setLastNotebookResult(null);
      logger.info("Notebook cleared");
    } catch (err) {
      logger.error(`Clear notebook failed: ${String(err)}`);
    }
  }

  async function applyToComponent(outputName: string) {
    if (!selectedComponent) return;
    try {
      await backend.applyNotebookOutputToComponent({
        instance_id: selectedComponent.instance_id,
        parameter_name: outputName,
        output_name: outputName,
      });
      logger.info(`Applied ${outputName} to ${selectedComponent.instance_id}`);
    } catch (err) {
      logger.error(`Apply failed: ${String(err)}`);
    }
  }

  function insertTemplate(template: string) {
    setInput((prev) => (prev ? `${prev}\n${template}` : template));
  }

  return (
    <Stack gap="md">
      <Title order={3}>Engineering Notebook</Title>

      <NotebookInput
        input={input}
        onChange={setInput}
        onEvaluate={() => void evaluate()}
        onClear={() => void clear()}
        loading={loading}
      />

      <PreferredValueQuickTools onInsert={insertTemplate} />

      {result && (
        <NotebookResultCard result={result}>
          <ApplyNotebookOutputPanel
            result={result}
            selectedComponent={selectedComponent}
            onApply={(name) => void applyToComponent(name)}
          />
        </NotebookResultCard>
      )}

      {notebookState && <NotebookVariableTable state={notebookState} />}

      {notebookState && <NotebookHistory state={notebookState} />}
    </Stack>
  );
}
