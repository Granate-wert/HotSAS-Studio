import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { MantineProvider } from "@mantine/core";
import { FilterAnalysisScreen } from "../FilterAnalysisScreen";
import { useHotSasStore } from "../../store";
import type { CircuitAnalysisPort, FilterNetworkAnalysisResult } from "../../types";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

const mockPorts: CircuitAnalysisPort[] = [
  {
    label: "IN",
    positive_net_id: "net_in",
    negative_net_id: null,
    reference_node_id: null,
    nominal_impedance_ohm: 50,
  },
  {
    label: "OUT",
    positive_net_id: "net_out",
    negative_net_id: null,
    reference_node_id: null,
    nominal_impedance_ohm: 50,
  },
];

const mockResult: FilterNetworkAnalysisResult = {
  analysis_id: "fa-1",
  project_id: "proj-1",
  request: {
    project_id: "proj-1",
    scope: "whole_circuit",
    selected_component_ids: [],
    input_port: mockPorts[0],
    output_port: mockPorts[1],
    sweep: {
      start_hz: 1,
      stop_hz: 1e9,
      points: 200,
      points_per_decade: null,
      scale: "logarithmic",
    },
    method: "mock",
    source_amplitude_v: 1,
    requested_metrics: ["cutoff_frequency", "peak_gain"],
  },
  method_used: "mock",
  detected_filter_kind: "low_pass",
  can_trust_as_engineering_estimate: false,
  points: [
    {
      frequency_hz: 100,
      vin_magnitude: 1,
      vout_magnitude: 0.99,
      transfer_magnitude: 0.99,
      gain_db: -0.09,
      attenuation_db: 0.09,
      phase_deg: -5.7,
      zin_magnitude_ohm: null,
      zin_phase_deg: null,
      zout_magnitude_ohm: null,
      zout_phase_deg: null,
    },
    {
      frequency_hz: 1000,
      vin_magnitude: 1,
      vout_magnitude: 0.707,
      transfer_magnitude: 0.707,
      gain_db: -3.01,
      attenuation_db: 3.01,
      phase_deg: -45,
      zin_magnitude_ohm: null,
      zin_phase_deg: null,
      zout_magnitude_ohm: null,
      zout_phase_deg: null,
    },
  ],
  metrics: [
    {
      kind: "cutoff_frequency",
      label: "Cutoff Frequency",
      value: 159.15,
      unit: "Hz",
      frequency_hz: null,
      confidence: "estimated",
      note: null,
    },
    {
      kind: "peak_gain",
      label: "Peak Gain",
      value: 0,
      unit: "dB",
      frequency_hz: null,
      confidence: "exact",
      note: null,
    },
  ],
  diagnostics: [],
  generated_netlist_preview: null,
  created_at: "2026-05-11T12:00:00Z",
};

vi.mock("../../api", () => ({
  backend: {
    suggestFilterAnalysisPorts: vi.fn(() => Promise.resolve(mockPorts)),
    validateFilterNetworkAnalysisRequest: vi.fn(() => Promise.resolve([])),
    runFilterNetworkAnalysis: vi.fn(() => Promise.resolve(mockResult)),
    exportFilterNetworkAnalysisCsv: vi.fn(() => Promise.resolve("freq,gain\n100,-0.09\n")),
    addFilterNetworkAnalysisToAdvancedReport: vi.fn(() =>
      Promise.resolve({ sections: [], title: "", generated_at: "" }),
    ),
  },
}));

describe("FilterAnalysisScreen", () => {
  beforeEach(() => {
    useHotSasStore.setState({
      project: null,
      filterAnalysisPorts: [],
      filterAnalysisResult: null,
      filterAnalysisDiagnostics: [],
      filterAnalysisLoading: false,
      filterAnalysisError: null,
      filterAnalysisCsvExport: null,
    });
  });

  it("renders no-project empty state", () => {
    renderWithMantine(<FilterAnalysisScreen />);
    expect(screen.getByRole("heading", { name: /Filter Analysis/i })).toBeInTheDocument();
    expect(screen.getByText(/Open or create a project/i)).toBeInTheDocument();
  });

  it("loads and suggests ports when project exists", async () => {
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
    });
    renderWithMantine(<FilterAnalysisScreen />);
    await waitFor(() => {
      expect(screen.getByText(/Port Configuration/i)).toBeInTheDocument();
    });
  });

  it("shows input and output port selectors", async () => {
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
      filterAnalysisPorts: mockPorts,
    });
    renderWithMantine(<FilterAnalysisScreen />);
    await waitFor(() => {
      expect(screen.getByText(/Input Port/i)).toBeInTheDocument();
      expect(screen.getByText(/Output Port/i)).toBeInTheDocument();
    });
  });

  it("run analysis button is disabled without ports", () => {
    renderWithMantine(<FilterAnalysisScreen />);
    const runBtn = screen.getByRole("button", { name: /Run Analysis/i });
    expect(runBtn).toBeDisabled();
  });

  it("calls backend run API when run is clicked", async () => {
    const { backend } = await import("../../api");
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
      filterAnalysisPorts: mockPorts,
    });
    renderWithMantine(<FilterAnalysisScreen />);

    const runBtn = await screen.findByRole("button", { name: /Run Analysis/i });
    expect(runBtn).toBeEnabled();
    fireEvent.click(runBtn);

    await waitFor(() => {
      expect(backend.runFilterNetworkAnalysis).toHaveBeenCalledTimes(1);
    });
  });

  it("shows result summary after analysis", async () => {
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
      filterAnalysisPorts: mockPorts,
      filterAnalysisResult: mockResult,
    });
    renderWithMantine(<FilterAnalysisScreen />);

    expect(await screen.findByText(/Result Summary/i)).toBeInTheDocument();
    expect(screen.getByText(/low_pass/i)).toBeInTheDocument();
  });

  it("shows metrics table with cutoff frequency", async () => {
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
      filterAnalysisPorts: mockPorts,
      filterAnalysisResult: mockResult,
    });
    renderWithMantine(<FilterAnalysisScreen />);

    expect(await screen.findByText(/Cutoff Frequency/i)).toBeInTheDocument();
    expect(screen.getByText(/Peak Gain/i)).toBeInTheDocument();
  });

  it("shows backend error state", () => {
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
      filterAnalysisError: "Backend error",
    });
    renderWithMantine(<FilterAnalysisScreen />);
    expect(screen.getByText(/Backend error/i)).toBeInTheDocument();
  });

  it("does not show S-parameter UI", () => {
    useHotSasStore.setState({
      project: {
        id: "proj-1",
        name: "Test",
        schematic: { id: "c1", title: "Test", components: [], nets: [], wires: [] },
        format_version: "1",
        engine_version: "1",
        project_type: "circuit",
      },
      filterAnalysisResult: mockResult,
    });
    renderWithMantine(<FilterAnalysisScreen />);
    expect(screen.queryByText(/S11/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/S21/i)).not.toBeInTheDocument();
  });
});
