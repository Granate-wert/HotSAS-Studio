import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SParameterMetricsTable } from "../SParameterMetricsTable";
import { SParameterDiagnosticsPanel } from "../SParameterDiagnosticsPanel";
import { SParameterSummaryCard } from "../SParameterSummaryCard";
import { SParameterExportActions } from "../SParameterExportActions";
import type { SParameterMetric, SParameterDiagnostic, SParameterDataset } from "../../../types";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

describe("SParameterMetricsTable", () => {
  const metrics: SParameterMetric[] = [
    {
      id: "m1",
      label: "S21 Peak",
      value: -0.83,
      unit: "dB",
      frequency_hz: 1e6,
      confidence: "high",
      notes: ["note"],
    },
    {
      id: "m2",
      label: "Max Insertion Loss",
      value: 3.5,
      unit: "dB",
      frequency_hz: null,
      confidence: "medium",
      notes: [],
    },
  ];

  it("renders metric rows", () => {
    renderWithMantine(<SParameterMetricsTable metrics={metrics} />);
    expect(screen.getByText("S21 Peak")).toBeInTheDocument();
    expect(screen.getByText("Max Insertion Loss")).toBeInTheDocument();
    expect(screen.getByText("high")).toBeInTheDocument();
    expect(screen.getByText("medium")).toBeInTheDocument();
  });
});

describe("SParameterDiagnosticsPanel", () => {
  const diagnostics: SParameterDiagnostic[] = [
    {
      code: "d1",
      severity: "warning",
      title: "Non-50 ohm ref",
      message: "Ref is 75 ohm",
      suggested_fix: "Check setup",
    },
    {
      code: "d2",
      severity: "info",
      title: "Parsed OK",
      message: "No issues",
      suggested_fix: null,
    },
  ];

  it("renders diagnostics", () => {
    renderWithMantine(<SParameterDiagnosticsPanel diagnostics={diagnostics} />);
    expect(screen.getByText("Non-50 ohm ref")).toBeInTheDocument();
    expect(screen.getByText("Parsed OK")).toBeInTheDocument();
    expect(screen.getByText(/Check setup/)).toBeInTheDocument();
  });
});

describe("SParameterSummaryCard", () => {
  const dataset: SParameterDataset = {
    id: "ds-1",
    name: "sample.s2p",
    source: "imported_touchstone",
    port_count: 2,
    reference_impedance_ohm: 50,
    frequency_unit: "Hz",
    parameter_format: "RI",
    points: [],
    warnings: [],
  };

  it("renders dataset summary", () => {
    renderWithMantine(<SParameterSummaryCard dataset={dataset} />);
    expect(screen.getByText("sample.s2p")).toBeInTheDocument();
    expect(screen.getByText("2-port")).toBeInTheDocument();
    expect(screen.getByText(/50/)).toBeInTheDocument();
  });
});

describe("SParameterExportActions", () => {
  it("renders export and report buttons", () => {
    renderWithMantine(
      <SParameterExportActions onExportCsv={() => {}} onAddToReport={() => {}} loading={false} />,
    );
    expect(screen.getByRole("button", { name: /Export CSV/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Add to Report/i })).toBeInTheDocument();
  });
});
