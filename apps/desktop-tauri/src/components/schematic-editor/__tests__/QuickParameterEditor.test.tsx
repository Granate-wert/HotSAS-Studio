import { render, screen } from "../../../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { QuickParameterEditor } from "../QuickParameterEditor";
import type { SchematicEditableFieldDto } from "../../../types";

const mockFields: SchematicEditableFieldDto[] = [
  { field_id: "resistance", label: "Resistance", current_value: "10k", editable: true },
  { field_id: "instance_id", label: "Instance ID", current_value: "R1", editable: false },
];

describe("QuickParameterEditor", () => {
  it("renders fields and labels", () => {
    render(<QuickParameterEditor fields={mockFields} componentId="R1" onUpdate={vi.fn()} />);
    expect(screen.getByText("Resistance")).toBeInTheDocument();
    expect(screen.getByDisplayValue("10k")).toBeInTheDocument();
    expect(screen.getByText("Instance ID")).toBeInTheDocument();
    expect(screen.getByDisplayValue("R1")).toBeInTheDocument();
  });

  it("shows no editable parameters message when empty", () => {
    render(<QuickParameterEditor fields={[]} componentId="R1" onUpdate={vi.fn()} />);
    expect(screen.getByText("No editable parameters")).toBeInTheDocument();
  });

  it("calls onUpdate when update button clicked", async () => {
    const user = userEvent.setup();
    const onUpdate = vi.fn();
    render(<QuickParameterEditor fields={mockFields} componentId="R1" onUpdate={onUpdate} />);
    const input = screen.getByDisplayValue("10k");
    await user.clear(input);
    await user.type(input, "4.7k");
    await user.click(screen.getByText("Update"));
    expect(onUpdate).toHaveBeenCalledWith("R1", "resistance", "4.7k");
  });

  it("does not show update button for non-editable fields", () => {
    render(<QuickParameterEditor fields={mockFields} componentId="R1" onUpdate={vi.fn()} />);
    const updateButtons = screen.getAllByText("Update");
    expect(updateButtons.length).toBe(1);
  });

  it("disables inputs when loading", () => {
    render(
      <QuickParameterEditor fields={mockFields} componentId="R1" onUpdate={vi.fn()} loading />,
    );
    expect(screen.getByDisplayValue("10k")).toBeDisabled();
    expect(screen.getByDisplayValue("R1")).toBeDisabled();
  });
});
