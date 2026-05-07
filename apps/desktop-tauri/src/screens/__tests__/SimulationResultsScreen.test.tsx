import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationScreen } from "../SimulationScreen";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

vi.mock("../../../api", () => ({
  backend: {
    listUserCircuitSimulationProfiles: vi.fn(() => Promise.resolve([])),
    suggestUserCircuitSimulationProbes: vi.fn(() => Promise.resolve([])),
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
        errors: [],
      }),
    ),
    listSimulationHistory: vi.fn(() => Promise.resolve([])),
    validateCurrentCircuitForSimulation: vi.fn(() =>
      Promise.resolve({
        can_run: true,
        blocking_errors: [],
        warnings: [],
        generated_netlist_preview: "",
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
        generated_netlist: "",
        status: "Succeeded",
        engine_used: "mock",
        warnings: [],
        errors: [],
        result: { summary: [], series: [] },
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
        series: [],
      }),
    ),
    diagnoseLastSimulationRun: vi.fn(() => Promise.resolve([])),
    addLastSimulationToAdvancedReport: vi.fn(() => Promise.resolve()),
    deleteSimulationHistoryRun: vi.fn(() => Promise.resolve()),
    clearSimulationHistory: vi.fn(() => Promise.resolve()),
    exportRunSeriesCsv: vi.fn(() => Promise.resolve("")),
    exportRunSeriesJson: vi.fn(() => Promise.resolve("")),
  },
}));

describe("SimulationScreen", () => {
  it("renders Simulation Dashboard", async () => {
    renderWithMantine(<SimulationScreen />);
    await waitFor(() => {
      expect(screen.getByText(/Simulation Dashboard/)).toBeInTheDocument();
    });
  });

  it("renders dashboard tabs", async () => {
    renderWithMantine(<SimulationScreen />);
    await waitFor(() => {
      expect(screen.getByRole("tab", { name: /setup/i })).toBeInTheDocument();
    });
    expect(screen.getByRole("tab", { name: /diagnostics/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /results/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /graph/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /history/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /export/i })).toBeInTheDocument();
  });
});
