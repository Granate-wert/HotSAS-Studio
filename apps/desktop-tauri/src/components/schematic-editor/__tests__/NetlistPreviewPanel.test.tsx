import { render, screen } from "../../../test-utils";
import { describe, expect, it } from "vitest";
import { NetlistPreviewPanel } from "../NetlistPreviewPanel";
import type { NetlistPreviewDto } from "../../../types";

const mockPreview: NetlistPreviewDto = {
  netlist: "R1 1 2 10k\nC1 2 0 100n",
  warnings: ["Floating net detected"],
  errors: ["Missing ground"],
};

describe("NetlistPreviewPanel", () => {
  it("shows loading state", () => {
    render(<NetlistPreviewPanel preview={null} loading />);
    expect(screen.getByText("Generating netlist preview...")).toBeInTheDocument();
  });

  it("shows empty state when no preview", () => {
    render(<NetlistPreviewPanel preview={null} />);
    expect(screen.getByText(/Click the "Netlist Preview" tab/)).toBeInTheDocument();
  });

  it("renders netlist content", () => {
    render(<NetlistPreviewPanel preview={mockPreview} />);
    expect(screen.getByText((content) => content.includes("R1 1 2 10k"))).toBeInTheDocument();
    expect(screen.getByText((content) => content.includes("C1 2 0 100n"))).toBeInTheDocument();
  });

  it("renders warnings and errors", () => {
    render(<NetlistPreviewPanel preview={mockPreview} />);
    expect(screen.getByText("Errors (1)")).toBeInTheDocument();
    expect(screen.getByText("Missing ground")).toBeInTheDocument();
    expect(screen.getByText("Warnings (1)")).toBeInTheDocument();
    expect(screen.getByText("Floating net detected")).toBeInTheDocument();
  });
});
