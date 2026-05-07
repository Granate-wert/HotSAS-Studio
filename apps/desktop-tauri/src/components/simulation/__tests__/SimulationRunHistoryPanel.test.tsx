import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationRunHistoryPanel } from "../SimulationRunHistoryPanel";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

const mockHistory = [
  {
    run_id: "run-1",
    profile_id: "mock-ac",
    profile_name: "AC Sweep",
    analysis_type: "AcSweep",
    engine_used: "mock",
    status: "Succeeded",
    created_at: "2024-01-01T00:00:00Z",
    warnings_count: 0,
    errors_count: 0,
    series_count: 2,
    measurements_count: 3,
  },
  {
    run_id: "run-2",
    profile_id: "mock-op",
    profile_name: "Operating Point",
    analysis_type: "OperatingPoint",
    engine_used: "mock",
    status: "Failed",
    created_at: "2024-01-01T01:00:00Z",
    warnings_count: 1,
    errors_count: 1,
    series_count: 0,
    measurements_count: 0,
  },
];

describe("SimulationRunHistoryPanel", () => {
  it("renders empty state", () => {
    renderWithMantine(<SimulationRunHistoryPanel history={[]} />);
    expect(screen.getByText(/No simulation runs in history/i)).toBeInTheDocument();
  });

  it("renders history entries", () => {
    renderWithMantine(<SimulationRunHistoryPanel history={mockHistory} />);
    expect(screen.getByText("AC Sweep")).toBeInTheDocument();
    expect(screen.getByText("Operating Point")).toBeInTheDocument();
    expect(screen.getByText("Succeeded")).toBeInTheDocument();
    expect(screen.getByText("Failed")).toBeInTheDocument();
  });

  it("calls onDeleteRun when delete clicked", () => {
    const onDeleteRun = vi.fn();
    renderWithMantine(
      <SimulationRunHistoryPanel history={mockHistory} onDeleteRun={onDeleteRun} />,
    );
    const deleteButtons = screen.getAllByRole("button").filter((b) => b.querySelector("svg"));
    fireEvent.click(deleteButtons[0]);
    expect(onDeleteRun).toHaveBeenCalledWith("run-1");
  });

  it("calls onClearHistory when clear clicked", () => {
    const onClearHistory = vi.fn();
    renderWithMantine(
      <SimulationRunHistoryPanel history={mockHistory} onClearHistory={onClearHistory} />,
    );
    fireEvent.click(screen.getByRole("button", { name: /clear/i }));
    expect(onClearHistory).toHaveBeenCalled();
  });
});
