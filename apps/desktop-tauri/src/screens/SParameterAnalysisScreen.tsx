import { Alert, Button, Checkbox, Group, ScrollArea, Stack, Textarea, TextInput, Title } from "@mantine/core";
import { AlertCircle, Radio } from "lucide-react";
import { useState } from "react";
import { backend } from "../api";
import { SParameterDiagnosticsPanel } from "../components/s-parameters/SParameterDiagnosticsPanel";
import { SParameterExportActions } from "../components/s-parameters/SParameterExportActions";
import { SParameterMagnitudeChart } from "../components/s-parameters/SParameterMagnitudeChart";
import { SParameterMetricsTable } from "../components/s-parameters/SParameterMetricsTable";
import { SParameterPhaseChart } from "../components/s-parameters/SParameterPhaseChart";
import { SParameterSummaryCard } from "../components/s-parameters/SParameterSummaryCard";
import { useHotSasStore } from "../store";

export function SParameterAnalysisScreen() {
  const storeResult = useHotSasStore((s) => s.sParameterAnalysisResult);
  const storeLoading = useHotSasStore((s) => s.sParameterAnalysisLoading);
  const storeError = useHotSasStore((s) => s.sParameterAnalysisError);
  const setSParameterAnalysisResult = useHotSasStore((s) => s.setSParameterAnalysisResult);
  const setSParameterAnalysisDiagnostics = useHotSasStore((s) => s.setSParameterAnalysisDiagnostics);
  const setSParameterAnalysisLoading = useHotSasStore((s) => s.setSParameterAnalysisLoading);
  const setSParameterAnalysisError = useHotSasStore((s) => s.setSParameterAnalysisError);
  const setSParameterAnalysisCsvExport = useHotSasStore((s) => s.setSParameterAnalysisCsvExport);

  const [content, setContent] = useState("");
  const [sourceName, setSourceName] = useState("");
  const [localError, setLocalError] = useState<string | null>(null);
  const [showS11, setShowS11] = useState(true);
  const [showS21, setShowS21] = useState(true);
  const [showS12, setShowS12] = useState(true);
  const [showS22, setShowS22] = useState(true);

  const combinedError = localError || storeError;

  const runAnalysis = async () => {
    if (!content.trim()) {
      setLocalError("Paste Touchstone content to analyze.");
      return;
    }
    setLocalError(null);
    setSParameterAnalysisLoading(true);
    setSParameterAnalysisError(null);
    setSParameterAnalysisResult(null);
    setSParameterAnalysisDiagnostics([]);

    try {
      const result = await backend.analyzeTouchstoneSParameters({
        source_name: sourceName.trim() || null,
        content: content.trim(),
      });
      setSParameterAnalysisResult(result);
      setSParameterAnalysisDiagnostics(result.diagnostics);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setSParameterAnalysisError(msg);
    } finally {
      setSParameterAnalysisLoading(false);
    }
  };

  const clearAnalysis = () => {
    setSParameterAnalysisResult(null);
    setSParameterAnalysisDiagnostics([]);
    setSParameterAnalysisError(null);
    setLocalError(null);
    setContent("");
    setSourceName("");
  };

  const exportCsv = async () => {
    setSParameterAnalysisLoading(true);
    try {
      const csv = await backend.exportSParameterCsv();
      setSParameterAnalysisCsvExport(csv);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setSParameterAnalysisError(msg);
    } finally {
      setSParameterAnalysisLoading(false);
    }
  };

  const addToReport = async () => {
    setSParameterAnalysisLoading(true);
    try {
      await backend.addSParameterAnalysisToAdvancedReport();
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setSParameterAnalysisError(msg);
    } finally {
      setSParameterAnalysisLoading(false);
    }
  };

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <ScrollArea className="screen-container">
          <Stack gap="md" p="md">
            <Title order={2}>
              <Radio size={24} style={{ marginRight: 8, verticalAlign: "middle" }} />
              S-Parameter Analysis
            </Title>

            {combinedError && (
              <Alert color="red" icon={<AlertCircle size={16} />}>
                {combinedError}
              </Alert>
            )}

            <TextInput
              label="Source name (optional)"
              placeholder="e.g. sample.s2p"
              value={sourceName}
              onChange={(e) => setSourceName(e.currentTarget.value)}
            />

            <Textarea
              label="Touchstone content"
              placeholder={`# Hz S RI R 50.0\n1000000 0.5 0.0 0.9 0.1 0.9 0.1 0.4 0.0\n...`}
              minRows={6}
              value={content}
              onChange={(e) => setContent(e.currentTarget.value)}
            />

            <Group gap="sm">
              <Button onClick={runAnalysis} loading={storeLoading}>
                Analyze
              </Button>
              <Button variant="light" color="gray" onClick={clearAnalysis}>
                Clear
              </Button>
            </Group>

            {storeResult && (
              <>
                <SParameterSummaryCard dataset={storeResult.dataset} />
                <SParameterExportActions
                  onExportCsv={exportCsv}
                  onAddToReport={addToReport}
                  loading={storeLoading}
                />

                <Group gap="sm">
                  {storeResult.can_plot_s11 && (
                    <Checkbox label="S11" checked={showS11} onChange={(e) => setShowS11(e.currentTarget.checked)} />
                  )}
                  {storeResult.can_plot_s21 && (
                    <Checkbox label="S21" checked={showS21} onChange={(e) => setShowS21(e.currentTarget.checked)} />
                  )}
                  {storeResult.can_plot_s12 && (
                    <Checkbox label="S12" checked={showS12} onChange={(e) => setShowS12(e.currentTarget.checked)} />
                  )}
                  {storeResult.can_plot_s22 && (
                    <Checkbox label="S22" checked={showS22} onChange={(e) => setShowS22(e.currentTarget.checked)} />
                  )}
                </Group>

                <SParameterMagnitudeChart
                  points={storeResult.curve_points}
                  showS11={showS11}
                  showS21={showS21}
                  showS12={showS12}
                  showS22={showS22}
                />
                <SParameterPhaseChart
                  points={storeResult.curve_points}
                  showS11={showS11}
                  showS21={showS21}
                  showS12={showS12}
                  showS22={showS22}
                />
                <SParameterMetricsTable metrics={storeResult.metrics} />
                <SParameterDiagnosticsPanel diagnostics={storeResult.diagnostics} />
              </>
            )}

            {!storeResult && !storeLoading && !combinedError && (
              <Alert color="gray" icon={<AlertCircle size={16} />}>
                Paste Touchstone data and click Analyze to view S-parameter curves and metrics.
              </Alert>
            )}
          </Stack>
        </ScrollArea>
      </div>
    </section>
  );
}
