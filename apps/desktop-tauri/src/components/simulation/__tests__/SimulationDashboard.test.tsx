import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationDashboard } from "../SimulationDashboard";
import { useHotSasStore } from "../../../store";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

vi.mock("../../../api", () => ({
  backend: {
    listUserCircuitSimulationProfiles: vi.fn(() =>
      Promise.resolve([
        {
          id: "mock-ac",
          name: "AC Sweep",
          analysis_type: "AcSweep",
          engine: "Mock",
          probes: [],
          ac: { start_hz: 10, stop_hz: 1000000, points_per_decade: 100 },
          transient: null,
          op: null,
        },
      ]),
    ),
    suggestUserCircuitSimulationProbes: vi.fn(() =>
      Promise.resolve([
        { id: "p1", label: "V(in)", kind: "NodeVoltage", target: { net_id: "in" } },
        { id: "p2", label: "V(out)", kind: "NodeVoltage", target: { net_id: "out" } },
      ]),
    ),
    checkNgspiceDiagnostics: vi.fn(() =>
      Promise.resolve({
        availability: {
          available: false,
          executablePath: null,
          version: null,
          message: "not installed",
          warnings: [],
        },
        executable_path: null,
        version: null,
        checked_at: "now",
        warnings: [],
        errors: [
          {
            code: "NGSPICE_UNAVAILABLE",
            severity: "Warning",
            title: "ngspice not available",
            message: "not installed",
            related_entity: { kind: "Engine", id: "ngspice" },
            suggested_fix: "Install ngspice",
          },
        ],
      }),
    ),
    listSimulationHistory: vi.fn(() => Promise.resolve([])),
    validateCurrentCircuitForSimulation: vi.fn(() =>
      Promise.resolve({
        can_run: true,
        blocking_errors: [],
        warnings: [],
        generated_netlist_preview: "V1 in 0 AC 1",
      }),
    ),
    diagnoseSimulationPreflight: vi.fn(() => Promise.resolve([])),
    runCurrentCircuitSimulation: vi.fn(() =>
      Promise.resolve({
        id: "run-1",
        project_id: "proj",
        profile: {
          id: "mock-ac",
          name: "AC Sweep",
          analysis_type: "AcSweep",
          engine: "Mock",
          probes: [],
          ac: null,
          transient: null,
          op: null,
        },
        generated_netlist: "V1 in 0 AC 1",
        status: "Succeeded",
        engine_used: "mock",
        warnings: [],
        errors: [],
        result: {
          summary: [],
          series: [],
        },
        created_at: "now",
      }),
    ),
    addRunToHistory: vi.fn(() => Promise.resolve()),
    buildSimulationGraphView: vi.fn(() =>
      Promise.resolve({
        run_id: "run-1",
        title: "AC Sweep — mock",
        x_axis: { label: "Hz", unit: "Hz", scale: "Log" },
        y_axis: { label: "Value", unit: "V", scale: "Linear" },
        series: [{ id: "s1", label: "V(out)", visible_by_default: true, points_count: 100 }],
      }),
    ),
    diagnoseLastSimulationRun: vi.fn(() => Promise.resolve([])),
    addLastSimulationToAdvancedReport: vi.fn(() => Promise.resolve()),
    deleteSimulationHistoryRun: vi.fn(() => Promise.resolve()),
    clearSimulationHistory: vi.fn(() => Promise.resolve()),
    exportRunSeriesCsv: vi.fn(() => Promise.resolve("series_id,series_label,x,y")),
    exportRunSeriesJson: vi.fn(() => Promise.resolve('{"run_id":"run-1"}')),
  },
}));

describe("SimulationDashboard", () => {
  beforeEach(() => {
    useHotSasStore.setState({
      simulationProfiles: [],
      simulationProbes: [],
      selectedSimulationProfile: null,
      selectedSimulationProbes: [],
      simulationPreflight: null,
      currentSimulationRun: null,
      lastSimulationRun: null,
      simulationWorkflowLoading: false,
      simulationWorkflowError: null,
      simulationResultViewMode: "graph",
      ngspiceDiagnostics: null,
      simulationDiagnostics: [],
      simulationDiagnosticsLoading: false,
      simulationDiagnosticsError: null,
      simulationRunHistory: [],
      simulationGraphView: null,
      simulationGraphVisibleSeries: {},
    });
  });

  it("renders dashboard with tabs", async () => {
    renderWithMantine(<SimulationDashboard />);
    expect(screen.getByText(/Simulation Dashboard/)).toBeInTheDocument();
    await waitFor(() => {
      expect(screen.getByRole("tab", { name: /setup/i })).toBeInTheDocument();
    });
    expect(screen.getByRole("tab", { name: /diagnostics/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /results/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /graph/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /history/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /export/i })).toBeInTheDocument();
  });

  it("loads ngspice diagnostics on mount", async () => {
    renderWithMantine(<SimulationDashboard />);
    await waitFor(() => {
      expect(screen.getByText(/ngspice Diagnostics/)).toBeInTheDocument();
    });
    await waitFor(() => {
      expect(screen.getByText(/Unavailable/)).toBeInTheDocument();
    });
  });

  it("shows setup tab by default with profile selector", async () => {
    renderWithMantine(<SimulationDashboard />);
    await waitFor(() => {
      expect(screen.getByRole("tab", { name: /setup/i })).toBeInTheDocument();
    });
  });
});
