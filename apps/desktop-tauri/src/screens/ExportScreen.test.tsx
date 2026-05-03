import { describe, it, expect, vi } from "vitest";
import { screen, fireEvent, waitFor } from "@testing-library/react";
import { render } from "../test-utils";
import { ExportScreen } from "./ExportScreen";
import type { ExportCapabilityDto, ExportResultDto } from "../types";

const mockCapabilities: ExportCapabilityDto[] = [
  {
    format: "markdown_report",
    label: "Markdown Report",
    description: "Human-readable report",
    file_extension: "md",
    available: true,
  },
  {
    format: "spice_netlist",
    label: "SPICE Netlist",
    description: "Circuit simulation",
    file_extension: "cir",
    available: true,
  },
  {
    format: "bom_csv",
    label: "BOM (CSV)",
    description: "Bill of materials",
    file_extension: "csv",
    available: true,
  },
];

function renderScreen(props: Partial<React.ComponentProps<typeof ExportScreen>> = {}) {
  return render(
    <ExportScreen
      hasProject={false}
      capabilities={mockCapabilities}
      lastResult={null}
      onLoadCapabilities={vi.fn()}
      onExport={vi.fn()}
      {...props}
    />,
  );
}

describe("ExportScreen", () => {
  it("renders export center title and description", () => {
    renderScreen();
    expect(screen.getByText("Export Center")).toBeInTheDocument();
    expect(screen.getByText(/Generate and download design artifacts/)).toBeInTheDocument();
  });

  it("disables export buttons when no project is loaded", () => {
    renderScreen({ hasProject: false });
    const buttons = screen
      .getAllByRole("button")
      .filter((b) => b.textContent?.includes("Markdown") || b.textContent?.includes("SPICE"));
    for (const button of buttons) {
      expect(button).toBeDisabled();
    }
  });

  it("enables export buttons when project exists", () => {
    renderScreen({ hasProject: true });
    const buttons = screen
      .getAllByRole("button")
      .filter((b) => b.textContent?.includes("Markdown") || b.textContent?.includes("SPICE"));
    for (const button of buttons) {
      expect(button).toBeEnabled();
    }
  });

  it("calls onLoadCapabilities on mount when capabilities are empty", () => {
    const onLoad = vi.fn();
    renderScreen({ capabilities: [], onLoadCapabilities: onLoad });
    expect(onLoad).toHaveBeenCalledTimes(1);
  });

  it("calls onExport with correct format when button clicked", async () => {
    const onExport = vi.fn();
    renderScreen({ hasProject: true, onExport });
    const mdButton = screen.getByRole("button", { name: /Markdown Report/i });
    fireEvent.click(mdButton);
    await waitFor(() => {
      expect(onExport).toHaveBeenCalledWith("markdown_report", false, "./exports");
    });
  });

  it("toggles write-to-file switch and shows output directory input", () => {
    renderScreen();
    const toggle = screen.getByRole("switch");
    expect(toggle).not.toBeChecked();
    fireEvent.click(toggle);
    expect(toggle).toBeChecked();
    expect(screen.getByPlaceholderText("Output directory")).toBeInTheDocument();
  });

  it("displays last export result when provided", () => {
    const result: ExportResultDto = {
      format: "markdown_report",
      content: "# Hello",
      file_path: "/tmp/out.md",
      success: true,
      message: "Saved to /tmp/out.md",
    };
    renderScreen({ lastResult: result });
    expect(screen.getByText("Saved to /tmp/out.md")).toBeInTheDocument();
    expect(screen.getByText("# Hello")).toBeInTheDocument();
  });
});
