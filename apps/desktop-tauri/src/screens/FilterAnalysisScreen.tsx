import { Alert, ScrollArea, Stack, Title } from "@mantine/core";
import { AlertCircle, Filter } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { backend } from "../api";
import { FilterAnalysisExportActions } from "../components/filter-analysis/FilterAnalysisExportActions";
import { FilterAnalysisSummaryCard } from "../components/filter-analysis/FilterAnalysisSummaryCard";
import { FilterBodeChart } from "../components/filter-analysis/FilterBodeChart";
import { FilterDiagnosticsPanel } from "../components/filter-analysis/FilterDiagnosticsPanel";
import { FilterImpedanceChart } from "../components/filter-analysis/FilterImpedanceChart";
import { FilterMetricsTable } from "../components/filter-analysis/FilterMetricsTable";
import { FilterPortConfigurationCard } from "../components/filter-analysis/FilterPortConfigurationCard";
import { FilterRunControls } from "../components/filter-analysis/FilterRunControls";
import { FilterSweepControls } from "../components/filter-analysis/FilterSweepControls";
import { useHotSasStore } from "../store";
import type {
  CircuitAnalysisPort,
  FilterAnalysisMethod,
  FilterNetworkAnalysisRequest,
  FrequencySweepSettings,
} from "../types";

function defaultSweep(): FrequencySweepSettings {
  return {
    start_hz: 1,
    stop_hz: 1e9,
    points: 200,
    points_per_decade: null,
    scale: "logarithmic",
  };
}

export function FilterAnalysisScreen() {
  const project = useHotSasStore((s) => s.project);
  const storePorts = useHotSasStore((s) => s.filterAnalysisPorts);
  const storeResult = useHotSasStore((s) => s.filterAnalysisResult);
  const storeDiagnostics = useHotSasStore((s) => s.filterAnalysisDiagnostics);
  const storeLoading = useHotSasStore((s) => s.filterAnalysisLoading);
  const storeError = useHotSasStore((s) => s.filterAnalysisError);
  const setFilterAnalysisPorts = useHotSasStore((s) => s.setFilterAnalysisPorts);
  const setFilterAnalysisResult = useHotSasStore((s) => s.setFilterAnalysisResult);
  const setFilterAnalysisDiagnostics = useHotSasStore((s) => s.setFilterAnalysisDiagnostics);
  const setFilterAnalysisLoading = useHotSasStore((s) => s.setFilterAnalysisLoading);
  const setFilterAnalysisError = useHotSasStore((s) => s.setFilterAnalysisError);
  const setFilterAnalysisCsvExport = useHotSasStore((s) => s.setFilterAnalysisCsvExport);

  const [inputPort, setInputPort] = useState<CircuitAnalysisPort | null>(null);
  const [outputPort, setOutputPort] = useState<CircuitAnalysisPort | null>(null);
  const [sweep, setSweep] = useState(defaultSweep());
  const [method, setMethod] = useState<FilterAnalysisMethod>("auto");
  const [localError, setLocalError] = useState<string | null>(null);

  const hasProject = !!project;

  useEffect(() => {
    if (!hasProject) {
      setFilterAnalysisPorts([]);
      setInputPort(null);
      setOutputPort(null);
      return;
    }
    let cancelled = false;
    backend
      .suggestFilterAnalysisPorts([])
      .then((ports) => {
        if (cancelled) return;
        setFilterAnalysisPorts(ports);
        if (ports.length > 0 && !inputPort) {
          setInputPort(ports[0]);
        }
        if (ports.length > 1 && !outputPort) {
          setOutputPort(ports[1]);
        }
      })
      .catch((e) => {
        if (cancelled) return;
        setFilterAnalysisError(e instanceof Error ? e.message : String(e));
      });
    return () => {
      cancelled = true;
    };
  }, [hasProject, project?.id]);

  const runAnalysis = async () => {
    if (!project) {
      setLocalError("No project open.");
      return;
    }
    if (!inputPort || !outputPort) {
      setLocalError("Please select both input and output ports.");
      return;
    }
    setLocalError(null);
    setFilterAnalysisLoading(true);
    setFilterAnalysisError(null);
    setFilterAnalysisResult(null);
    setFilterAnalysisDiagnostics([]);

    const request: FilterNetworkAnalysisRequest = {
      project_id: project.id,
      scope: "whole_circuit",
      selected_component_ids: [],
      input_port: inputPort,
      output_port: outputPort,
      sweep,
      method,
      source_amplitude_v: 1.0,
      requested_metrics: [
        "cutoff_frequency",
        "bandwidth",
        "peak_gain",
        "passband_ripple",
        "stopband_attenuation",
        "input_impedance",
        "output_impedance",
      ],
    };

    try {
      const validation = await backend.validateFilterNetworkAnalysisRequest(request);
      const blockers = validation.filter((d) => d.severity === "blocking");
      if (blockers.length > 0) {
        setFilterAnalysisDiagnostics(validation);
        setFilterAnalysisLoading(false);
        return;
      }
      const result = await backend.runFilterNetworkAnalysis(request);
      setFilterAnalysisResult(result);
      setFilterAnalysisDiagnostics(result.diagnostics);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setFilterAnalysisError(msg);
    } finally {
      setFilterAnalysisLoading(false);
    }
  };

  const clearAnalysis = () => {
    setFilterAnalysisResult(null);
    setFilterAnalysisDiagnostics([]);
    setFilterAnalysisError(null);
    setLocalError(null);
  };

  const exportCsv = async () => {
    setFilterAnalysisLoading(true);
    try {
      const csv = await backend.exportFilterNetworkAnalysisCsv();
      setFilterAnalysisCsvExport(csv);
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setFilterAnalysisError(msg);
    } finally {
      setFilterAnalysisLoading(false);
    }
  };

  const addToReport = async () => {
    setFilterAnalysisLoading(true);
    try {
      await backend.addFilterNetworkAnalysisToAdvancedReport();
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setFilterAnalysisError(msg);
    } finally {
      setFilterAnalysisLoading(false);
    }
  };

  const combinedError = localError || storeError;

  const allDiagnostics = useMemo(() => {
    const resultDiags = storeResult?.diagnostics ?? [];
    return [...storeDiagnostics, ...resultDiags].filter(
      (d, i, arr) => arr.findIndex((x) => x.code === d.code && x.message === d.message) === i,
    );
  }, [storeDiagnostics, storeResult]);

  return (
    <section className="screen-panel">
      <div className="screen-content">
        <ScrollArea className="screen-container">
          <Stack gap="md" p="md">
            <Title order={2}>
              <Filter size={24} style={{ marginRight: 8, verticalAlign: "middle" }} />
              Filter Analysis
            </Title>

            {!hasProject && (
              <Alert color="blue" icon={<AlertCircle size={16} />}>
                Open or create a project to run filter analysis.
              </Alert>
            )}

            {combinedError && (
              <Alert color="red" icon={<AlertCircle size={16} />}>
                {combinedError}
              </Alert>
            )}

            <FilterPortConfigurationCard
              ports={storePorts}
              inputPort={inputPort}
              outputPort={outputPort}
              onInputChange={setInputPort}
              onOutputChange={setOutputPort}
            />

            <FilterSweepControls settings={sweep} onChange={setSweep} />

            <FilterRunControls
              method={method}
              onMethodChange={setMethod}
              onRun={runAnalysis}
              onClear={clearAnalysis}
              loading={storeLoading}
              disabled={!hasProject || !inputPort || !outputPort}
            />

            {storeResult && (
              <>
                <FilterAnalysisSummaryCard result={storeResult} />
                <FilterAnalysisExportActions
                  onExportCsv={exportCsv}
                  onAddToReport={addToReport}
                  loading={storeLoading}
                />
                <FilterBodeChart points={storeResult.points} />
                <FilterImpedanceChart points={storeResult.points} />
                <FilterMetricsTable metrics={storeResult.metrics} />
                <FilterDiagnosticsPanel diagnostics={allDiagnostics} />
              </>
            )}

            {!storeResult && !storeLoading && !combinedError && hasProject && (
              <Alert color="gray" icon={<AlertCircle size={16} />}>
                Configure ports and sweep settings, then click Run Analysis.
              </Alert>
            )}
          </Stack>
        </ScrollArea>
      </div>
    </section>
  );
}
