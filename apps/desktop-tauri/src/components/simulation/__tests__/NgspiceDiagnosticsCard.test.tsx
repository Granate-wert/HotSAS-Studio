import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { NgspiceDiagnosticsCard } from "../NgspiceDiagnosticsCard";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

describe("NgspiceDiagnosticsCard", () => {
  it("renders null state when no diagnostics", () => {
    renderWithMantine(<NgspiceDiagnosticsCard diagnostics={null} />);
    expect(screen.getByText(/ngspice diagnostics not checked yet/i)).toBeInTheDocument();
  });

  it("renders available ngspice status", () => {
    renderWithMantine(
      <NgspiceDiagnosticsCard
        diagnostics={{
          availability: {
            available: true,
            executablePath: "/usr/bin/ngspice",
            version: "42",
            message: null,
            warnings: [],
          },
          executable_path: "/usr/bin/ngspice",
          version: "42",
          checked_at: "2024-01-01T00:00:00Z",
          warnings: [],
          errors: [],
        }}
      />,
    );
    expect(screen.getByText(/Available/)).toBeInTheDocument();
    expect(screen.getByText(/Path: \/usr\/bin\/ngspice/)).toBeInTheDocument();
    expect(screen.getByText(/Version: 42/)).toBeInTheDocument();
  });

  it("renders unavailable ngspice with fallback explanation", () => {
    renderWithMantine(
      <NgspiceDiagnosticsCard
        diagnostics={{
          availability: {
            available: false,
            executablePath: null,
            version: null,
            message: "not found",
            warnings: [],
          },
          executable_path: null,
          version: null,
          checked_at: "2024-01-01T00:00:00Z",
          warnings: [],
          errors: [
            {
              code: "NGSPICE_UNAVAILABLE",
              severity: "Warning",
              title: "ngspice not available",
              message: "ngspice not found",
              related_entity: { kind: "Engine", id: "ngspice" },
              suggested_fix: "Install ngspice",
            },
          ],
        }}
      />,
    );
    expect(screen.getByText(/Unavailable/)).toBeInTheDocument();
    expect(screen.getByText(/Mock fallback available/)).toBeInTheDocument();
    expect(screen.getByText(/NGSPICE_UNAVAILABLE/)).toBeInTheDocument();
    expect(screen.getByText(/Suggested fix: Install ngspice/)).toBeInTheDocument();
  });

  it("calls onRefresh when refresh button clicked", () => {
    const onRefresh = vi.fn();
    renderWithMantine(<NgspiceDiagnosticsCard diagnostics={null} onRefresh={onRefresh} />);
    const btn = screen.getByRole("button", { name: /refresh/i });
    btn.click();
    expect(onRefresh).toHaveBeenCalled();
  });
});
