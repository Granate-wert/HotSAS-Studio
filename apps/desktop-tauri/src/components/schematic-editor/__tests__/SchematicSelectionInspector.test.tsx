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
});
