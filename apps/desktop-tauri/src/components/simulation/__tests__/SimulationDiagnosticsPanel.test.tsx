import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationDiagnosticsPanel } from "../SimulationDiagnosticsPanel";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

describe("SimulationDiagnosticsPanel", () => {
  it("renders OK when no diagnostics", () => {
    renderWithMantine(<SimulationDiagnosticsPanel diagnostics={[]} />);
    expect(screen.getByText(/Diagnostics OK/)).toBeInTheDocument();
  });

  it("renders loading state", () => {
    renderWithMantine(<SimulationDiagnosticsPanel diagnostics={[]} loading />);
    expect(screen.getByText(/Checking diagnostics/i)).toBeInTheDocument();
  });

  it("renders blocking error with suggested fix", () => {
    renderWithMantine(
      <SimulationDiagnosticsPanel
        diagnostics={[
          {
            code: "NO_COMPONENTS",
            severity: "Blocking",
            title: "No components",
            message: "Schematic is empty",
            related_entity: null,
            suggested_fix: "Add a component",
          },
        ]}
      />,
    );
    expect(screen.getByText(/1 Blocking/)).toBeInTheDocument();
    expect(screen.getByText(/NO_COMPONENTS/)).toBeInTheDocument();
    expect(screen.getByText(/Add a component/)).toBeInTheDocument();
  });

  it("renders warning and info diagnostics", () => {
    renderWithMantine(
      <SimulationDiagnosticsPanel
        diagnostics={[
          {
            code: "NO_GROUND",
            severity: "Warning",
            title: "No ground",
            message: "No ground reference",
            related_entity: null,
            suggested_fix: "Add ground",
          },
          {
            code: "NO_PROBES",
            severity: "Info",
            title: "No probes",
            message: "No probes selected",
            related_entity: null,
            suggested_fix: "Select probes",
          },
        ]}
      />,
    );
    expect(screen.getByText(/1 Warning/)).toBeInTheDocument();
    expect(screen.getByText(/1 Info/)).toBeInTheDocument();
    expect(screen.getByText(/NO_GROUND/)).toBeInTheDocument();
    expect(screen.getByText(/NO_PROBES/)).toBeInTheDocument();
  });
});
