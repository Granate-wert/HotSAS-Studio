import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationSeriesExportPanel } from "../SimulationSeriesExportPanel";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

describe("SimulationSeriesExportPanel", () => {
  it("renders export buttons", () => {
    renderWithMantine(<SimulationSeriesExportPanel onExportCsv={vi.fn()} onExportJson={vi.fn()} />);
    expect(screen.getByRole("button", { name: /export csv/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /export json/i })).toBeInTheDocument();
  });

  it("calls onExportCsv when CSV button clicked", () => {
    const onExportCsv = vi.fn();
    renderWithMantine(<SimulationSeriesExportPanel onExportCsv={onExportCsv} />);
    fireEvent.click(screen.getByRole("button", { name: /export csv/i }));
    expect(onExportCsv).toHaveBeenCalled();
  });

  it("calls onExportJson when JSON button clicked", () => {
    const onExportJson = vi.fn();
    renderWithMantine(<SimulationSeriesExportPanel onExportJson={onExportJson} />);
    fireEvent.click(screen.getByRole("button", { name: /export json/i }));
    expect(onExportJson).toHaveBeenCalled();
  });

  it("shows last export content", () => {
    renderWithMantine(
      <SimulationSeriesExportPanel
        onExportCsv={vi.fn()}
        lastExportContent="a,b,c"
        lastExportFormat="csv"
      />,
    );
    expect(screen.getByText(/Exported CSV ready/)).toBeInTheDocument();
    expect(screen.getByText("a,b,c")).toBeInTheDocument();
  });
});
