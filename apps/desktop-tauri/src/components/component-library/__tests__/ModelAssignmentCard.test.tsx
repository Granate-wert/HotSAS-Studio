import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { render } from "../../../test-utils";
import { ModelAssignmentCard } from "../ModelAssignmentCard";
import type { ComponentModelAssignmentDto, SpiceModelReferenceDto } from "../../../types";

const assignedModel: SpiceModelReferenceDto = {
  id: "builtin_resistor_primitive",
  display_name: "Builtin resistor primitive",
  model_kind: "primitive_model",
  source: "builtin",
  status: "assigned_builtin",
  limitations: [],
  warnings: [],
};

const assignment: ComponentModelAssignmentDto = {
  component_definition_id: "generic_resistor",
  component_instance_id: "R1",
  model_ref: assignedModel,
  pin_mappings: [
    {
      component_pin_id: "1",
      model_pin_name: "positive",
      model_pin_index: 0,
      role: "positive",
      required: true,
    },
  ],
  parameter_bindings: [
    {
      model_parameter_name: "R",
      component_parameter_id: "resistance",
      value_expression: null,
      required: true,
    },
  ],
  status: "assigned_builtin",
  readiness: {
    can_simulate: true,
    can_export_netlist: true,
    uses_placeholder: false,
    blocking_count: 0,
    warning_count: 0,
    status_label: "Simulation ready",
  },
  diagnostics: [],
};

describe("ModelAssignmentCard", () => {
  it("renders assigned model status, readiness, pins and bindings", () => {
    render(
      <ModelAssignmentCard
        assignment={assignment}
        availableModels={[assignedModel]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("Model Assignment")).toBeInTheDocument();
    expect(screen.getByText("assigned builtin")).toBeInTheDocument();
    expect(screen.getByText("Simulation ready")).toBeInTheDocument();
    expect(screen.getByText("Builtin resistor primitive")).toBeInTheDocument();
    expect(screen.getByText("1 -> positive")).toBeInTheDocument();
    expect(screen.getByText("R -> resistance")).toBeInTheDocument();
  });

  it("calls assign callback with selected model id", async () => {
    const onAssignModel = vi.fn();

    render(
      <ModelAssignmentCard
        assignment={{ ...assignment, model_ref: null, status: "missing" }}
        availableModels={[assignedModel]}
        onAssignModel={onAssignModel}
      />,
    );

    await userEvent.click(screen.getByRole("button", { name: /Assign model/i }));

    expect(onAssignModel).toHaveBeenCalledWith("builtin_resistor_primitive");
  });

  it("shows diagnostics for placeholder or missing models", () => {
    render(
      <ModelAssignmentCard
        assignment={{
          ...assignment,
          status: "placeholder",
          diagnostics: [
            {
              code: "PLACEHOLDER_MODEL",
              severity: "warning",
              title: "Placeholder model assigned",
              message: "Results are not production-accurate.",
              suggested_fix: "Replace with a real SPICE model.",
              related_component_id: "U1",
              related_model_id: "generic_op_amp_model",
            },
          ],
        }}
        availableModels={[assignedModel]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("PLACEHOLDER_MODEL")).toBeInTheDocument();
    expect(screen.getByText("Replace with a real SPICE model.")).toBeInTheDocument();
  });

  it("shows missing model blocking warning", () => {
    render(
      <ModelAssignmentCard
        assignment={{
          ...assignment,
          model_ref: null,
          status: "missing",
          readiness: {
            can_simulate: false,
            can_export_netlist: true,
            uses_placeholder: false,
            blocking_count: 1,
            warning_count: 0,
            status_label: "No SPICE model",
          },
          diagnostics: [
            {
              code: "MISSING_MODEL",
              severity: "blocking",
              title: "No SPICE model assigned",
              message: "Assign a builtin or imported SPICE model.",
              suggested_fix: "Assign a builtin or imported SPICE model.",
              related_component_id: "X1",
              related_model_id: null,
            },
          ],
        }}
        availableModels={[assignedModel]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("missing")).toBeInTheDocument();
    expect(screen.getByText("No SPICE model")).toBeInTheDocument();
    expect(screen.getByText("1 blocking")).toBeInTheDocument();
    expect(screen.getByText("MISSING_MODEL")).toBeInTheDocument();
  });

  it("shows persisted status for imported model", () => {
    render(
      <ModelAssignmentCard
        assignment={{
          ...assignment,
          model_ref: { ...assignedModel, source: "imported", status: "available" },
          status: "mapped",
        }}
        availableModels={[{ ...assignedModel, source: "imported", status: "available" }]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("Persisted")).toBeInTheDocument();
  });

  it("shows derived builtin status for builtin model", () => {
    render(
      <ModelAssignmentCard
        assignment={assignment}
        availableModels={[assignedModel]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("Derived builtin")).toBeInTheDocument();
  });

  it("shows missing asset warning when model ref status is missing", () => {
    render(
      <ModelAssignmentCard
        assignment={{
          ...assignment,
          model_ref: { ...assignedModel, source: "imported", status: "missing" },
          status: "invalid",
          diagnostics: [
            {
              code: "MISSING_ASSET",
              severity: "warning",
              title: "Model asset missing",
              message: "The referenced model asset is no longer available.",
              suggested_fix: "Reimport the model.",
              related_component_id: "R1",
              related_model_id: "missing_model",
            },
          ],
        }}
        availableModels={[{ ...assignedModel, source: "imported", status: "missing" }]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("Missing asset")).toBeInTheDocument();
    expect(screen.getByText(/missing or stale model asset references/i)).toBeInTheDocument();
  });

  it("shows stale reference warning when model ref status is stale", () => {
    render(
      <ModelAssignmentCard
        assignment={{
          ...assignment,
          model_ref: { ...assignedModel, source: "imported", status: "stale" },
          status: "invalid",
          diagnostics: [
            {
              code: "STALE_ASSIGNMENT",
              severity: "warning",
              title: "Stale assignment",
              message: "Assignment references a model that has changed.",
              suggested_fix: "Reassign the model.",
              related_component_id: "R1",
              related_model_id: "stale_model",
            },
          ],
        }}
        availableModels={[{ ...assignedModel, source: "imported", status: "stale" }]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("Missing asset")).toBeInTheDocument();
  });

  it("does not crash when persistence data is absent/legacy", () => {
    render(
      <ModelAssignmentCard
        assignment={{
          ...assignment,
          model_ref: null,
          status: "unknown",
          diagnostics: [],
        }}
        availableModels={[]}
        onAssignModel={vi.fn()}
      />,
    );

    expect(screen.getByText("Model Assignment")).toBeInTheDocument();
    expect(screen.getByText("No model assigned")).toBeInTheDocument();
    expect(screen.getByText("unknown")).toBeInTheDocument();
  });
});
