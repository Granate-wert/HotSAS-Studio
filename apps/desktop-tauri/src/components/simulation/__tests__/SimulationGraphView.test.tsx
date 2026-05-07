import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationGraphView } from "../SimulationGraphView";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

describe("SimulationGraphView", () => {
  it("renders empty state", () => {
    renderWithMantine(<SimulationGraphView graphView={null} visibleSeries={{}} />);
    expect(screen.getByText(/No graph view available/i)).toBeInTheDocument();
  });

  it("renders loading state", () => {
    renderWithMantine(<SimulationGraphView graphView={null} visibleSeries={{}} loading />);
    expect(screen.getByText(/Loading graph view/i)).toBeInTheDocument();
  });

  it("renders error state", () => {
    renderWithMantine(
      <SimulationGraphView graphView={null} visibleSeries={{}} error="Graph failed" />,
    );
    expect(screen.getByText(/Graph Error/)).toBeInTheDocument();
    expect(screen.getByText(/Graph failed/)).toBeInTheDocument();
  });

  it("renders graph view with title and axes", () => {
    renderWithMantine(
      <SimulationGraphView
        graphView={{
          run_id: "run-1",
          title: "AC Sweep — mock",
          x_axis: { label: "Hz", unit: "Hz", scale: "Log" },
          y_axis: { label: "Value", unit: "V", scale: "Linear" },
          series: [{ id: "s1", label: "V(out)", visible_by_default: true, points_count: 100 }],
        }}
        visibleSeries={{ s1: true }}
      />,
    );
    expect(screen.getByText("AC Sweep — mock")).toBeInTheDocument();
    expect(screen.getByText(/X: Hz \(Log\)/)).toBeInTheDocument();
    expect(screen.getByText(/Y: Value \(Linear\)/)).toBeInTheDocument();
  });

  it("shows all hidden message when no series visible", () => {
    renderWithMantine(
      <SimulationGraphView
        graphView={{
          run_id: "run-1",
          title: "AC Sweep",
          x_axis: { label: "Hz", unit: null, scale: "Log" },
          y_axis: { label: "Value", unit: null, scale: "Linear" },
          series: [{ id: "s1", label: "V(out)", visible_by_default: true, points_count: 100 }],
        }}
        visibleSeries={{ s1: false }}
      />,
    );
    expect(screen.getByText(/All series are hidden/i)).toBeInTheDocument();
  });
});
