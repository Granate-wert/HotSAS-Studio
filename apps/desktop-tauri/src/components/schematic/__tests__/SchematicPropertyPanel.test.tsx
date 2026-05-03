import { screen } from "@testing-library/react";
import { render } from "../../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { SchematicPropertyPanel } from "../SchematicPropertyPanel";

const mockComponent = {
  instance_id: "R1",
  component_kind: "resistor",
  title: "Resistor",
  parameters: [{ name: "resistance", value: "10k", unit: "Ohm" }],
  symbol: null,
};

describe("SchematicPropertyPanel", () => {
  it("renders placeholder when no component selected", () => {
    render(<SchematicPropertyPanel component={null} onUpdate={() => {}} />);
    expect(screen.getByText(/select a component on the schematic/i)).toBeInTheDocument();
  });

  it("renders selected component parameters", () => {
    render(<SchematicPropertyPanel component={mockComponent} onUpdate={() => {}} />);
    expect(screen.getByText("Resistor")).toBeInTheDocument();
    expect(screen.getByText(/R1 \(resistor\)/)).toBeInTheDocument();
    expect(screen.getByText("resistance")).toBeInTheDocument();
    expect(screen.getByDisplayValue("10k")).toBeInTheDocument();
  });
});
