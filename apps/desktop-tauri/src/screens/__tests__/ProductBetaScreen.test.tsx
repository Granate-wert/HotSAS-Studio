import { describe, it, expect, vi } from "vitest";
import { screen, fireEvent } from "@testing-library/react";
import { render } from "../../test-utils";
import { ProductBetaScreen } from "../ProductBetaScreen";

const mockStatus = {
  app_name: "HotSAS Studio",
  app_version: "2.0.0",
  roadmap_stage: "v2.0 Product Beta",
  build_profile: "release",
  current_project: {
    project_id: "demo-1",
    project_name: "RC Low-Pass Demo",
    format_version: "1.0",
    component_count: 3,
    net_count: 3,
    simulation_profile_count: 1,
  },
  workflow_steps: [
    {
      id: "project",
      title: "Project",
      status: "ready",
      screen_id: "start",
      description: "Create or open a .circuit project.",
      warnings: [],
    },
    {
      id: "schematic",
      title: "Schematic",
      status: "ready",
      screen_id: "schematic",
      description: "View and edit the schematic diagram.",
      warnings: [],
    },
    {
      id: "formula_library",
      title: "Formula Library",
      status: "ready",
      screen_id: "formulas",
      description: "Calculate formulas from the registry.",
      warnings: [],
    },
  ],
  module_statuses: [
    {
      id: "formula_registry",
      title: "Formula Registry",
      status: "ready",
      details: [{ key: "supported", value: "rc_low_pass_cutoff" }],
    },
    {
      id: "simulation",
      title: "Simulation Engine",
      status: "limited",
      details: [{ key: "ngspice", value: "unavailable" }],
    },
  ],
  blockers: [],
  warnings: ["ngspice not available. Simulation will use mock engine."],
};

function setup(props = {}) {
  const onRefresh = vi.fn();
  const onSelfCheck = vi.fn();
  const onCreateDemo = vi.fn();
  const onNavigate = vi.fn();

  render(
    <ProductBetaScreen
      status={mockStatus as unknown as import("../../types").ProductWorkflowStatusDto}
      loading={false}
      error={null}
      onRefresh={onRefresh}
      onSelfCheck={onSelfCheck}
      onCreateDemo={onCreateDemo}
      onNavigate={onNavigate}
      {...props}
    />,
  );

  return { onRefresh, onSelfCheck, onCreateDemo, onNavigate };
}

describe("ProductBetaScreen", () => {
  it("renders product beta title", () => {
    setup();
    expect(screen.getByText("Product Beta")).toBeInTheDocument();
  });

  it("renders guided workflow steps", () => {
    setup();
    expect(screen.getByText("Project")).toBeInTheDocument();
    expect(screen.getByText("Schematic")).toBeInTheDocument();
    expect(screen.getByText("Formula Library")).toBeInTheDocument();
  });

  it("refresh workflow calls backend", () => {
    const { onRefresh } = setup();
    fireEvent.click(screen.getByText("Refresh workflow status"));
    expect(onRefresh).toHaveBeenCalled();
  });

  it("run self check calls backend", () => {
    const { onSelfCheck } = setup();
    fireEvent.click(screen.getByText("Run product beta self-check"));
    expect(onSelfCheck).toHaveBeenCalled();
  });

  it("create integrated demo project calls backend", () => {
    const { onCreateDemo } = setup();
    fireEvent.click(screen.getByText("Create integrated demo project"));
    expect(onCreateDemo).toHaveBeenCalled();
  });

  it("shows ready and limited badges", () => {
    setup();
    expect(screen.getAllByText("ready").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("limited").length).toBeGreaterThanOrEqual(1);
  });

  it("shows backend error message", () => {
    setup({ error: "Network error" });
    expect(screen.getByText("Network error")).toBeInTheDocument();
  });

  it("navigation buttons are rendered", () => {
    setup();
    const openButtons = screen.getAllByText("Open");
    expect(openButtons.length).toBeGreaterThanOrEqual(1);
  });
});
