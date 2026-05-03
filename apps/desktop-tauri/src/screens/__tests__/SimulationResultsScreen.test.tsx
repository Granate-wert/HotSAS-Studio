import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { SimulationScreen } from "../SimulationScreen";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

function mockSimulation(): {
  id: string;
  profile_id: string;
  status: string;
  engine: string;
  graph_series: [];
  warnings: string[];
  errors: string[];
} {
  return {
    id: "sim-1",
    profile_id: "profile-1",
    status: "Completed",
    engine: "mock",
    graph_series: [],
    warnings: ["mock warning"],
    errors: [],
  };
}

describe("SimulationScreen", () => {
  it("renders Simulation Results screen", () => {
    renderWithMantine(
      <SimulationScreen
        simulation={null}
        hasProject={false}
        ngspiceAvailability={null}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={vi.fn()}
        onRunSimulation={vi.fn()}
        onSetEngine={vi.fn()}
      />,
    );
    expect(screen.getByText("Simulation Results")).toBeInTheDocument();
    expect(screen.getByText("Check ngspice")).toBeInTheDocument();
  });

  it("Check ngspice button calls backend", () => {
    const onCheck = vi.fn();
    renderWithMantine(
      <SimulationScreen
        simulation={null}
        hasProject={true}
        ngspiceAvailability={null}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={onCheck}
        onRunSimulation={vi.fn()}
        onSetEngine={vi.fn()}
      />,
    );
    fireEvent.click(screen.getByText("Check ngspice"));
    expect(onCheck).toHaveBeenCalled();
  });

  it("unavailable ngspice message displayed", () => {
    renderWithMantine(
      <SimulationScreen
        simulation={null}
        hasProject={false}
        ngspiceAvailability={{
          available: false,
          message: "ngspice not found",
          warnings: [],
        }}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={vi.fn()}
        onRunSimulation={vi.fn()}
        onSetEngine={vi.fn()}
      />,
    );
    expect(screen.getByText(/not found \/ unavailable/i)).toBeInTheDocument();
  });

  it("engine selector works", () => {
    const onSetEngine = vi.fn();
    renderWithMantine(
      <SimulationScreen
        simulation={null}
        hasProject={false}
        ngspiceAvailability={null}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={vi.fn()}
        onRunSimulation={vi.fn()}
        onSetEngine={onSetEngine}
      />,
    );
    const mockBtn = screen.getByText("Mock");
    fireEvent.click(mockBtn);
    expect(onSetEngine).toHaveBeenCalledWith("mock");
  });

  it("Run AC Sweep button calls backend.runSimulation", () => {
    const onRun = vi.fn();
    renderWithMantine(
      <SimulationScreen
        simulation={null}
        hasProject={true}
        ngspiceAvailability={null}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={vi.fn()}
        onRunSimulation={onRun}
        onSetEngine={vi.fn()}
      />,
    );
    fireEvent.click(screen.getByText("Run AC Sweep"));
    expect(onRun).toHaveBeenCalledWith("ac_sweep");
  });

  it("result card displays status and warnings", () => {
    renderWithMantine(
      <SimulationScreen
        simulation={mockSimulation()}
        hasProject={true}
        ngspiceAvailability={null}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={vi.fn()}
        onRunSimulation={vi.fn()}
        onSetEngine={vi.fn()}
      />,
    );
    expect(screen.getByText(/Completed/)).toBeInTheDocument();
    expect(screen.getByText("mock warning")).toBeInTheDocument();
  });

  it("errors are shown without crash", () => {
    renderWithMantine(
      <SimulationScreen
        simulation={{
          ...mockSimulation(),
          status: "Failed",
          errors: ["simulation failed"],
        }}
        hasProject={true}
        ngspiceAvailability={null}
        selectedEngine="auto"
        isRunning={false}
        onCheckNgspice={vi.fn()}
        onRunSimulation={vi.fn()}
        onSetEngine={vi.fn()}
      />,
    );
    expect(screen.getByText("simulation failed")).toBeInTheDocument();
  });
});
