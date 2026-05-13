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

  // v3.6-pre-fix2: newly placed palette components must have editable parameters

  it("renders Capacitance editor for newly placed generic_capacitor", () => {
    const capacitor = {
      instance_id: "capacitor-4",
      component_kind: "generic_capacitor",
      title: "Capacitor",
      parameters: [{ name: "capacitance", value: "100n", unit: "F" }],
      symbol: null,
    };
    render(<SchematicPropertyPanel component={capacitor} onUpdate={() => {}} />);
    expect(screen.getByText("Capacitor")).toBeInTheDocument();
    expect(screen.getByText(/capacitor-4 \(generic_capacitor\)/)).toBeInTheDocument();
    expect(screen.getByText("capacitance")).toBeInTheDocument();
    expect(screen.getByDisplayValue("100n")).toBeInTheDocument();
    expect(screen.getByText("Unit: F")).toBeInTheDocument();
  });

  it("renders Resistance editor for newly placed generic_resistor", () => {
    const resistor = {
      instance_id: "resistor-2",
      component_kind: "generic_resistor",
      title: "Resistor",
      parameters: [{ name: "resistance", value: "10k", unit: "Ohm" }],
      symbol: null,
    };
    render(<SchematicPropertyPanel component={resistor} onUpdate={() => {}} />);
    expect(screen.getByText("Resistor")).toBeInTheDocument();
    expect(screen.getByText("resistance")).toBeInTheDocument();
    expect(screen.getByDisplayValue("10k")).toBeInTheDocument();
    expect(screen.getByText("Unit: Ohm")).toBeInTheDocument();
  });

  it("renders Voltage editor for newly placed generic_voltage_source", () => {
    const source = {
      instance_id: "voltage_source-1",
      component_kind: "generic_voltage_source",
      title: "Voltage Source",
      parameters: [{ name: "voltage", value: "5", unit: "V" }],
      symbol: null,
    };
    render(<SchematicPropertyPanel component={source} onUpdate={() => {}} />);
    expect(screen.getByText("Voltage Source")).toBeInTheDocument();
    expect(screen.getByText("voltage")).toBeInTheDocument();
    expect(screen.getByDisplayValue("5")).toBeInTheDocument();
    expect(screen.getByText("Unit: V")).toBeInTheDocument();
  });

  it("does not fall back to id-only display when parameters exist", () => {
    const component = {
      instance_id: "capacitor-4",
      component_kind: "generic_capacitor",
      title: "Capacitor",
      parameters: [{ name: "capacitance", value: "100n", unit: "F" }],
      symbol: null,
    };
    render(<SchematicPropertyPanel component={component} onUpdate={() => {}} />);
    // Should show the parameter input, not just title/id
    expect(screen.getByDisplayValue("100n")).toBeInTheDocument();
    // Should NOT show a "no parameters" message
    expect(screen.queryByText(/no editable parameters/i)).not.toBeInTheDocument();
  });
});
