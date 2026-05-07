import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationGraphControls } from "../SimulationGraphControls";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

const mockSeries = [
  { id: "s1", label: "V(out)", visible_by_default: true, points_count: 100 },
  { id: "s2", label: "V(in)", visible_by_default: true, points_count: 100 },
  { id: "s3", label: "I(R1)", visible_by_default: false, points_count: 100 },
];

describe("SimulationGraphControls", () => {
  it("renders series checkboxes", () => {
    renderWithMantine(
      <SimulationGraphControls
        series={mockSeries}
        visibleSeries={{ s1: true, s2: true, s3: false }}
        onToggleSeries={vi.fn()}
      />,
    );
    expect(screen.getByText(/V\(out\)/)).toBeInTheDocument();
    expect(screen.getByText(/V\(in\)/)).toBeInTheDocument();
    expect(screen.getByText(/I\(R1\)/)).toBeInTheDocument();
    expect(screen.getByText(/2 \/ 3 visible/)).toBeInTheDocument();
  });

  it("calls onToggleSeries when checkbox clicked", () => {
    const onToggleSeries = vi.fn();
    renderWithMantine(
      <SimulationGraphControls
        series={mockSeries}
        visibleSeries={{ s1: true, s2: true, s3: false }}
        onToggleSeries={onToggleSeries}
      />,
    );
    fireEvent.click(screen.getByLabelText(/V\(out\)/));
    expect(onToggleSeries).toHaveBeenCalledWith("s1", false);
  });
});
