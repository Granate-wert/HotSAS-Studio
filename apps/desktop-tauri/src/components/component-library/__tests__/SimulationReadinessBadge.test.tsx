import { screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { render } from "../../../test-utils";
import { SimulationReadinessBadge } from "../SimulationReadinessBadge";

describe("SimulationReadinessBadge", () => {
  it("shows ready state", () => {
    render(
      <SimulationReadinessBadge
        readiness={{
          can_simulate: true,
          can_export_netlist: true,
          uses_placeholder: false,
          blocking_count: 0,
          warning_count: 0,
          status_label: "Simulation ready",
        }}
      />,
    );

    expect(screen.getByText("Simulation ready")).toBeInTheDocument();
  });

  it("shows warning counts for placeholder state", () => {
    render(
      <SimulationReadinessBadge
        readiness={{
          can_simulate: true,
          can_export_netlist: true,
          uses_placeholder: true,
          blocking_count: 0,
          warning_count: 1,
          status_label: "Placeholder model",
        }}
      />,
    );

    expect(screen.getByText("Placeholder model")).toBeInTheDocument();
    expect(screen.getByText("1 warning")).toBeInTheDocument();
  });

  it("shows blocking counts for missing or invalid state", () => {
    render(
      <SimulationReadinessBadge
        readiness={{
          can_simulate: false,
          can_export_netlist: false,
          uses_placeholder: false,
          blocking_count: 1,
          warning_count: 0,
          status_label: "Invalid model assignment",
        }}
      />,
    );

    expect(screen.getByText("Invalid model assignment")).toBeInTheDocument();
    expect(screen.getByText("1 blocking")).toBeInTheDocument();
  });
});
