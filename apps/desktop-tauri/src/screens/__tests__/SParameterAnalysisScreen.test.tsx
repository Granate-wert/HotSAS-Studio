import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SParameterAnalysisScreen } from "../SParameterAnalysisScreen";
import { useHotSasStore } from "../../store";
import type { SParameterAnalysisResult } from "../../types";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

const mockResult: SParameterAnalysisResult = {
  id: "spa-1",
  dataset: {
    id: "ds-1",
    name: "test.s2p",
    source: "imported_touchstone",
    port_count: 2,
    reference_impedance_ohm: 50,
    frequency_unit: "Hz",
    parameter_format: "RI",
    points: [
      {
        frequency_hz: 1e6,
        s11: { re: 0.5, im: 0 },
        s21: { re: 0.9, im: 0.1 },
        s12: { re: 0.9, im: 0.1 },
        s22: { re: 0.4, im: 0 },
      },
    ],
    warnings: [],
  },
  curve_points: [
    {
      frequency_hz: 1e6,
      s11_db: -6.02,
      s21_db: -0.83,
      s12_db: -0.83,
      s22_db: -7.96,
      s11_phase_deg: 0,
      s21_phase_deg: 6.34,
      s12_phase_deg: 6.34,
      s22_phase_deg: 0,
      return_loss_s11_db: 6.02,
      return_loss_s22_db: 7.96,
      insertion_loss_s21_db: 0.83,
      vswr_s11: 3.0,
      vswr_s22: 2.33,
    },
  ],
  metrics: [
    {
      id: "s21_peak",
      label: "S21 Peak",
      value: -0.83,
      unit: "dB",
      frequency_hz: 1e6,
      confidence: "high",
      notes: ["Maximum S21 magnitude"],
    },
  ],
  diagnostics: [],
  can_plot_s11: true,
  can_plot_s21: true,
  can_plot_s12: true,
  can_plot_s22: true,
  summary: "2-port S-parameter dataset with 1 points, reference 50 Ω",
};

vi.mock("../../api", () => ({
  backend: {
    analyzeTouchstoneSParameters: vi.fn(() => Promise.resolve(mockResult)),
    exportSParameterCsv: vi.fn(() => Promise.resolve("freq,s11_db\n1000000,-6.02\n")),
    addSParameterAnalysisToAdvancedReport: vi.fn(() =>
      Promise.resolve({ sections: [], title: "", generated_at: "" }),
    ),
  },
}));

describe("SParameterAnalysisScreen", () => {
  beforeEach(() => {
    useHotSasStore.setState({
      sParameterAnalysisResult: null,
      sParameterAnalysisDiagnostics: [],
      sParameterAnalysisLoading: false,
      sParameterAnalysisError: null,
      sParameterAnalysisCsvExport: null,
    });
  });

  it("renders title and input fields", () => {
    renderWithMantine(<SParameterAnalysisScreen />);
    expect(screen.getByRole("heading", { name: /S-Parameter Analysis/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/Touchstone content/i)).toBeInTheDocument();
  });

  it("shows error when analyzing empty content", async () => {
    renderWithMantine(<SParameterAnalysisScreen />);
    const analyzeBtn = screen.getByRole("button", { name: /Analyze/i });
    fireEvent.click(analyzeBtn);
    await waitFor(() => {
      expect(screen.getByText(/Paste Touchstone content to analyze/i)).toBeInTheDocument();
    });
  });

  it("calls backend analyze API when content is provided", async () => {
    const { backend } = await import("../../api");
    renderWithMantine(<SParameterAnalysisScreen />);
    const textarea = screen.getByLabelText(/Touchstone content/i);
    fireEvent.change(textarea, { target: { value: "# test\n1e6 0.5 0 0.9 0.1 0.9 0.1 0.4 0" } });
    const analyzeBtn = screen.getByRole("button", { name: /Analyze/i });
    fireEvent.click(analyzeBtn);
    await waitFor(() => {
      expect(backend.analyzeTouchstoneSParameters).toHaveBeenCalled();
    });
  });

  it("displays result after analysis", async () => {
    renderWithMantine(<SParameterAnalysisScreen />);
    const textarea = screen.getByLabelText(/Touchstone content/i);
    fireEvent.change(textarea, { target: { value: "# test" } });
    const analyzeBtn = screen.getByRole("button", { name: /Analyze/i });
    fireEvent.click(analyzeBtn);
    await waitFor(() => {
      expect(screen.getByText(/test.s2p/)).toBeInTheDocument();
    });
  });

  it("clear button resets state", async () => {
    renderWithMantine(<SParameterAnalysisScreen />);
    const textarea = screen.getByLabelText(/Touchstone content/i);
    fireEvent.change(textarea, { target: { value: "# test" } });
    const clearBtn = screen.getByRole("button", { name: /Clear/i });
    fireEvent.click(clearBtn);
    expect(textarea).toHaveValue("");
  });
});
