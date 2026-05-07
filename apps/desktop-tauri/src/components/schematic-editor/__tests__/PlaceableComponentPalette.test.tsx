import { render, screen } from "../../../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { PlaceableComponentPalette } from "../PlaceableComponentPalette";
import type { PlaceableComponentDto } from "../../../types";

const mockComponents: PlaceableComponentDto[] = [
  {
    definition_id: "resistor",
    name: "Resistor",
    category: "resistor",
    component_kind: "resistor",
    has_symbol: true,
  },
  {
    definition_id: "capacitor",
    name: "Capacitor",
    category: "capacitor",
    component_kind: "capacitor",
    has_symbol: true,
  },
];

describe("PlaceableComponentPalette", () => {
  it("renders component list", () => {
    render(
      <PlaceableComponentPalette components={mockComponents} onSelect={vi.fn()} selected={null} />,
    );
    expect(screen.getByText("Resistor")).toBeInTheDocument();
    expect(screen.getByText("Capacitor")).toBeInTheDocument();
  });

  it("shows empty message when no components", () => {
    render(<PlaceableComponentPalette components={[]} onSelect={vi.fn()} selected={null} />);
    expect(screen.getByText("No placeable components found")).toBeInTheDocument();
  });

  it("calls onSelect when a component is clicked", async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(
      <PlaceableComponentPalette components={mockComponents} onSelect={onSelect} selected={null} />,
    );
    await user.click(screen.getByText("Resistor"));
    expect(onSelect).toHaveBeenCalledWith(mockComponents[0]);
  });

  it("calls onSelect with null when clicking already selected component", async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(
      <PlaceableComponentPalette
        components={mockComponents}
        onSelect={onSelect}
        selected={mockComponents[0]}
      />,
    );
    await user.click(screen.getByText("Resistor"));
    expect(onSelect).toHaveBeenCalledWith(null);
  });
});
