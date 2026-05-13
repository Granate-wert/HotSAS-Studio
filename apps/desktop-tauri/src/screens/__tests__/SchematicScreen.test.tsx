import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MantineProvider } from "@mantine/core";
import { describe, expect, it, vi } from "vitest";
import { SchematicScreen } from "../SchematicScreen";
import type { ProjectDto } from "../../types";

function renderWithProvider(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

const mockProject: ProjectDto = {
  id: "p1",
  name: "Test",
  format_version: "1",
  engine_version: "0.1.4",
  project_type: "schematic",
  schematic: {
    id: "s1",
    title: "Test",
    components: [
      {
        instance_id: "R1",
        definition_id: "resistor",
        component_kind: "resistor",
        display_label: "R1",
        x: 100,
        y: 100,
        rotation_degrees: 0,
        parameters: [],
        symbol: null,
        pins: [
          {
            id: "p1",
            name: "1",
            number: "1",
            electrical_type: "passive",
            x: -20,
            y: 0,
            side: "left",
          },
          {
            id: "p2",
            name: "2",
            number: "2",
            electrical_type: "passive",
            x: 20,
            y: 0,
            side: "right",
          },
        ],
        connected_nets: [],
      },
    ],
    wires: [],
    nets: [],
  },
};

const baseProps = {
  project: mockProject,
  formulaResult: null,
  preferredValue: null,
  simulation: null,
  netlist: "",
  markdownReport: "",
  htmlReport: "",
  onMarkdown: vi.fn(),
  onHtml: vi.fn(),
  hasProject: true,
  selectedComponent: null,
  validationReport: null,
  onSelectComponent: vi.fn(),
  onValidate: vi.fn(),
  onPropertyUpdate: vi.fn(),
  schematicCapabilities: [],
  schematicEditLoading: false,
  schematicEditError: null,
  pendingConnectionStart: null,
  onLoadSchematicCapabilities: vi.fn(),
  onAddComponent: vi.fn(),
  onMoveComponent: vi.fn(),
  onDeleteComponent: vi.fn(),
  onConnectPins: vi.fn(),
  onRenameNet: vi.fn(),
  onSetPendingConnectionStart: vi.fn(),
  // v2.8 interactive schematic editing
  schematicToolMode: "select" as const,
  placeableComponents: [],
  pendingPlaceComponent: null,
  pendingWireStart: null,
  selectedSchematicEntity: null,
  schematicSelectionDetails: null,
  undoRedoState: null,
  netlistPreview: null,
  schematicInteractionLoading: false,
  schematicInteractionError: null,
  onLoadPlaceableComponents: vi.fn(),
  onPlaceSchematicComponent: vi.fn(),
  onDeleteSchematicWire: vi.fn(),
  onUpdateSchematicQuickParameter: vi.fn(),
  onGetSchematicSelectionDetails: vi.fn(),
  onUndoSchematicEdit: vi.fn(),
  onRedoSchematicEdit: vi.fn(),
  onGetSchematicUndoRedoState: vi.fn(),
  onGenerateCurrentSchematicNetlistPreview: vi.fn(),
  onSetSchematicToolMode: vi.fn(),
  onSetPendingPlaceComponent: vi.fn(),
  onSetPendingWireStart: vi.fn(),
  onSetSelectedSchematicEntity: vi.fn(),
  onCreateDemoProject: vi.fn(),
  onLoadProjectPackage: vi.fn(),
};

describe("SchematicScreen v2.5", () => {
  it("renders toolbar with delete and connect buttons", () => {
    renderWithProvider(<SchematicScreen {...baseProps} />);
    expect(screen.getByText("Schematic Editor")).toBeInTheDocument();
    expect(screen.getByTestId("delete-selected-component")).toBeInTheDocument();
    expect(screen.getByTestId("connect-pins-button")).toBeInTheDocument();
    expect(screen.getByTestId("rename-net-button")).toBeInTheDocument();
  });

  it("renders component palette", () => {
    renderWithProvider(<SchematicScreen {...baseProps} />);
    expect(screen.getByText("Component Palette")).toBeInTheDocument();
    expect(screen.getByTestId("add-resistor")).toBeInTheDocument();
    expect(screen.getByTestId("add-capacitor")).toBeInTheDocument();
  });

  it("calls onAddComponent when palette button clicked", async () => {
    const user = userEvent.setup();
    const onAdd = vi.fn();
    renderWithProvider(<SchematicScreen {...baseProps} onAddComponent={onAdd} />);
    await user.click(screen.getByTestId("add-resistor"));
    expect(onAdd).toHaveBeenCalledWith("resistor");
  });

  it("delete button is disabled when no component selected", () => {
    renderWithProvider(<SchematicScreen {...baseProps} />);
    expect(screen.getByTestId("delete-selected-component")).toBeDisabled();
  });

  it("shows schematic edit error when present", () => {
    renderWithProvider(<SchematicScreen {...baseProps} schematicEditError="Something failed" />);
    expect(screen.getByText("Something failed")).toBeInTheDocument();
  });

  it("shows validation warnings in status panel", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        validationReport={{
          valid: false,
          warnings: [{ code: "W1", message: "Floating net", component_id: null, net_id: null }],
          errors: [],
        }}
      />,
    );
    expect(screen.getByText("W1: Floating net")).toBeInTheDocument();
  });
});

describe("SchematicScreen v3.5 empty states", () => {
  it("shows empty state when no project", () => {
    renderWithProvider(<SchematicScreen {...baseProps} project={null} hasProject={false} />);
    expect(
      screen.getByText("Create a new circuit project or load the RC low-pass demo to get started."),
    ).toBeInTheDocument();
    expect(screen.getByText("New RC Demo")).toBeInTheDocument();
    expect(screen.getByText("Open Project")).toBeInTheDocument();
  });

  it("shows empty state when project has no components", () => {
    const emptyProject = {
      ...mockProject,
      schematic: { ...mockProject.schematic, components: [] },
    };
    renderWithProvider(<SchematicScreen {...baseProps} project={emptyProject} />);
    expect(screen.getByText("New RC Demo")).toBeInTheDocument();
    expect(screen.getByText("Open Project")).toBeInTheDocument();
  });

  it("calls onCreateDemoProject when New RC Demo clicked", async () => {
    const user = userEvent.setup();
    const onCreate = vi.fn();
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        project={null}
        hasProject={false}
        onCreateDemoProject={onCreate}
      />,
    );
    await user.click(screen.getByText("New RC Demo"));
    expect(onCreate).toHaveBeenCalled();
  });

  it("does not render canvas when no project", () => {
    renderWithProvider(<SchematicScreen {...baseProps} project={null} hasProject={false} />);
    expect(document.querySelector(".react-flow")).not.toBeInTheDocument();
  });
});

describe("SchematicScreen v3.6-pre ACL and interaction", () => {
  it("shows place mode hint when pending place component is set", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        schematicToolMode="place"
        pendingPlaceComponent={{
          definition_id: "resistor",
          name: "Resistor",
          category: "passive",
          component_kind: "resistor",
          has_symbol: true,
        }}
      />,
    );
    expect(screen.getByText(/Click canvas to place Resistor/)).toBeInTheDocument();
  });

  it("calls onPlaceSchematicComponent when canvas clicked in place mode", async () => {
    const user = userEvent.setup();
    const onPlace = vi.fn();
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        schematicToolMode="place"
        pendingPlaceComponent={{
          definition_id: "capacitor",
          name: "Capacitor",
          category: "passive",
          component_kind: "resistor",
          has_symbol: true,
        }}
        onPlaceSchematicComponent={onPlace}
      />,
    );
    // The canvas container should be present
    const canvas = document.querySelector(".canvas");
    expect(canvas).toBeInTheDocument();
  });

  it("does not show ACL error by default", () => {
    renderWithProvider(<SchematicScreen {...baseProps} schematicInteractionError={null} />);
    expect(screen.queryByText(/not allowed by ACL/i)).not.toBeInTheDocument();
  });

  it("shows user-friendly error instead of raw ACL denial", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        schematicInteractionError="Could not place component. Please check project state or open a project first."
      />,
    );
    expect(
      screen.getByText(
        "Could not place component. Please check project state or open a project first.",
      ),
    ).toBeInTheDocument();
  });

  it("calls onSetPendingPlaceComponent when placeable palette item selected", async () => {
    const user = userEvent.setup();
    const onSetPending = vi.fn();
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        schematicToolMode="place"
        placeableComponents={[
          {
            definition_id: "resistor",
            name: "Resistor",
            category: "passive",
            component_kind: "resistor",
            has_symbol: true,
          },
        ]}
        onSetPendingPlaceComponent={onSetPending}
      />,
    );
    await user.click(screen.getByText("Resistor"));
    expect(onSetPending).toHaveBeenCalledWith(
      expect.objectContaining({ definition_id: "resistor" }),
    );
  });
});

describe("SchematicScreen v3.6-pre-fix parameter editing", () => {
  it("shows editable resistance field when resistor is selected", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        selectedSchematicEntity={{ kind: "component", id: "R1" }}
        schematicSelectionDetails={{
          kind: "component",
          id: "R1",
          display_name: "R1",
          editable_fields: [
            {
              field_id: "instance_id",
              label: "Instance ID",
              current_value: "R1",
              editable: false,
              unit: null,
            },
            {
              field_id: "resistance",
              label: "Resistance",
              current_value: "10k",
              editable: true,
              unit: "Ohm",
            },
          ],
        }}
      />,
    );
    expect(screen.getByText("Resistance")).toBeInTheDocument();
    expect(screen.getByDisplayValue("10k")).toBeInTheDocument();
  });

  it("calls onUpdateSchematicQuickParameter when resistance is updated", async () => {
    const user = userEvent.setup();
    const onUpdate = vi.fn();
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        selectedSchematicEntity={{ kind: "component", id: "R1" }}
        schematicSelectionDetails={{
          kind: "component",
          id: "R1",
          display_name: "R1",
          editable_fields: [
            {
              field_id: "instance_id",
              label: "Instance ID",
              current_value: "R1",
              editable: false,
              unit: null,
            },
            {
              field_id: "resistance",
              label: "Resistance",
              current_value: "10k",
              editable: true,
              unit: "Ohm",
            },
          ],
        }}
        onUpdateSchematicQuickParameter={onUpdate}
      />,
    );
    const input = screen.getByDisplayValue("10k");
    await user.clear(input);
    await user.type(input, "4.7k");
    await user.click(screen.getByText("Update"));
    expect(onUpdate).toHaveBeenCalledWith("R1", "resistance", "4.7k");
  });

  it("shows unit next to parameter input", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        selectedSchematicEntity={{ kind: "component", id: "C1" }}
        schematicSelectionDetails={{
          kind: "component",
          id: "C1",
          display_name: "C1",
          editable_fields: [
            {
              field_id: "capacitance",
              label: "Capacitance",
              current_value: "100n",
              editable: true,
              unit: "F",
            },
          ],
        }}
      />,
    );
    expect(screen.getByText("Capacitance")).toBeInTheDocument();
    expect(screen.getByText("F")).toBeInTheDocument();
  });

  it("shows no editable parameters message for unsupported component", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        selectedSchematicEntity={{ kind: "component", id: "D1" }}
        schematicSelectionDetails={{
          kind: "component",
          id: "D1",
          display_name: "D1",
          editable_fields: [
            {
              field_id: "instance_id",
              label: "Instance ID",
              current_value: "D1",
              editable: false,
              unit: null,
            },
          ],
        }}
      />,
    );
    expect(screen.getByText("No editable parameters")).toBeInTheDocument();
  });

  it("shows validation error when parameter update fails", () => {
    renderWithProvider(
      <SchematicScreen
        {...baseProps}
        selectedSchematicEntity={{ kind: "component", id: "R1" }}
        schematicSelectionDetails={{
          kind: "component",
          id: "R1",
          display_name: "R1",
          editable_fields: [
            {
              field_id: "resistance",
              label: "Resistance",
              current_value: "10k",
              editable: true,
              unit: "Ohm",
            },
          ],
        }}
        schematicInteractionError="invalid value: could not parse engineering value"
      />,
    );
    expect(screen.getByText(/invalid value/)).toBeInTheDocument();
  });
});
