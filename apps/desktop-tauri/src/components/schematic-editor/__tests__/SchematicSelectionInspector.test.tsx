import { render, screen } from "../../../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { SchematicSelectionInspector } from "../SchematicSelectionInspector";
import type { SchematicSelectionDetailsDto } from "../../../types";

const mockDetails: SchematicSelectionDetailsDto = {
  kind: "component",
  id: "R1",
  display_name: "R1",
  editable_fields: [
    { field_id: "resistance", label: "Resistance", current_value: "10k", editable: true },
    { field_id: "instance_id", label: "Instance ID", current_value: "R1", editable: false },
  ],
};

const mockDetailsWithModel: SchematicSelectionDetailsDto = {
  ...mockDetails,
  model_assignment_origin: "override",
  model_assignment: {
    component_definition_id: "generic_resistor",
    component_instance_id: "R1",
    model_ref: {
      id: "builtin_resistor_primitive",
      display_name: "Builtin resistor primitive",
      model_kind: "primitive_model",
      source: "builtin",
      status: "assigned_builtin",
      limitations: [],
      warnings: [],
    },
    pin_mappings: [],
    parameter_bindings: [],
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
  },
};

describe("SchematicSelectionInspector", () => {
  it("shows empty state when no entity selected", () => {
    render(
      <SchematicSelectionInspector
        entity={null}
        details={null}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );
    expect(screen.getByText(/Select a component, wire, or net/)).toBeInTheDocument();
  });

  it("renders component details", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={mockDetails}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );
    expect(screen.getByText("Resistance")).toBeInTheDocument();
    expect(screen.getByDisplayValue("10k")).toBeInTheDocument();
    expect(screen.getByText("Instance ID")).toBeInTheDocument();
  });

  it("shows selected component model assignment and readiness", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={mockDetailsWithModel}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );

    expect(screen.getByText("Model assignment")).toBeInTheDocument();
    expect(screen.getByText("override")).toBeInTheDocument();
    expect(screen.getByText("Builtin resistor primitive")).toBeInTheDocument();
    expect(screen.getByText("assigned builtin")).toBeInTheDocument();
    expect(screen.getByText("Simulation ready")).toBeInTheDocument();
  });

  it("shows delete wire button for wire entity", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "wire", id: "w1" }}
        details={{ kind: "wire", id: "w1", display_name: "Wire w1", editable_fields: [] }}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );
    expect(screen.getByText("Delete Wire")).toBeInTheDocument();
  });

  it("calls onDeleteWire when delete button clicked", async () => {
    const user = userEvent.setup();
    const onDeleteWire = vi.fn();
    render(
      <SchematicSelectionInspector
        entity={{ kind: "wire", id: "w1" }}
        details={{ kind: "wire", id: "w1", display_name: "Wire w1", editable_fields: [] }}
        onDeleteWire={onDeleteWire}
        onUpdateParameter={vi.fn()}
      />,
    );
    await user.click(screen.getByText("Delete Wire"));
    expect(onDeleteWire).toHaveBeenCalledWith("w1");
  });

  it("allows editing an editable field via QuickParameterEditor", async () => {
    const user = userEvent.setup();
    const onUpdateParameter = vi.fn();
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={mockDetails}
        onDeleteWire={vi.fn()}
        onUpdateParameter={onUpdateParameter}
      />,
    );
    const input = screen.getByDisplayValue("10k");
    await user.clear(input);
    await user.type(input, "4.7k");
    await user.click(screen.getByText("Update"));
    expect(onUpdateParameter).toHaveBeenCalledWith("R1", "resistance", "4.7k");
  });

  it("shows inherited assignment persistence status", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={{
          ...mockDetailsWithModel,
          model_assignment_origin: "inherited",
          model_assignment: {
            ...mockDetailsWithModel.model_assignment!,
            model_ref: {
              ...mockDetailsWithModel.model_assignment!.model_ref!,
              source: "imported",
              status: "available",
            },
          },
        }}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );

    expect(screen.getByText("inherited")).toBeInTheDocument();
    expect(screen.getByText("Persisted")).toBeInTheDocument();
  });

  it("shows instance override persistence status", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={mockDetailsWithModel}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );

    expect(screen.getByText("override")).toBeInTheDocument();
    expect(screen.getByText("Derived builtin")).toBeInTheDocument();
  });

  it("shows missing/stale diagnostics for selected instance", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={{
          ...mockDetailsWithModel,
          model_assignment: {
            ...mockDetailsWithModel.model_assignment!,
            model_ref: {
              ...mockDetailsWithModel.model_assignment!.model_ref!,
              source: "imported",
              status: "missing",
            },
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
          },
        }}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );

    expect(screen.getByText("Missing asset")).toBeInTheDocument();
    expect(
      screen.getByText(/missing or stale model asset references detected/i),
    ).toBeInTheDocument();
  });

  it("does not crash when persistence data is absent/legacy/unknown", () => {
    render(
      <SchematicSelectionInspector
        entity={{ kind: "component", id: "R1" }}
        details={mockDetails}
        onDeleteWire={vi.fn()}
        onUpdateParameter={vi.fn()}
      />,
    );

    expect(screen.getByText("R1")).toBeInTheDocument();
    expect(screen.queryByText("Model assignment")).not.toBeInTheDocument();
  });
});
