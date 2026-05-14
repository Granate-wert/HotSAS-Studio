import { render, screen, fireEvent } from "../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { SchematicCanvas } from "../SchematicCanvas";
import type { ProjectDto } from "../../types";

vi.mock("@xyflow/react", async () => {
  const React = await import("react");
  return {
    Background: () => <div data-testid="rf-background" />,
    BaseEdge: ({ id, path }: { id: string; path: string }) => (
      <path data-testid={`base-edge-${id}`} d={path} />
    ),
    ConnectionMode: { Loose: "loose" },
    Controls: () => <div data-testid="rf-controls" />,
    Handle: ({
      id,
      onClick,
      style,
    }: {
      id: string;
      onClick?: (event: React.MouseEvent) => void;
      style?: React.CSSProperties;
    }) => (
      <button
        type="button"
        data-testid={`pin-handle-${id}`}
        style={style}
        onClick={(event) => onClick?.(event)}
      />
    ),
    MiniMap: () => <div data-testid="rf-minimap" />,
    Position: {
      Bottom: "bottom",
      Left: "left",
      Right: "right",
      Top: "top",
    },
    ReactFlow: ({
      nodes,
      edges,
      nodeTypes,
      edgeTypes,
      onNodesChange,
      onPaneClick,
      onPaneMouseMove,
      children,
    }: {
      nodes: Array<{ id: string; type?: string; data: unknown; selected?: boolean }>;
      edges: Array<{
        id: string;
        type?: string;
        data?: { routePoints?: Array<{ x: number; y: number }> };
      }>;
      nodeTypes: Record<string, React.ComponentType<any>>;
      edgeTypes?: Record<string, React.ComponentType<any>>;
      onNodesChange?: () => void;
      onPaneClick?: (event: React.MouseEvent) => void;
      onPaneMouseMove?: (event: React.MouseEvent) => void;
      children: React.ReactNode;
    }) => (
      <div
        className="react-flow"
        data-testid="mock-flow"
        data-has-node-change-handler={String(Boolean(onNodesChange))}
        onClick={onPaneClick}
        onMouseMove={onPaneMouseMove}
      >
        {nodes.map((node) => {
          const NodeComponent = nodeTypes[node.type ?? "generic"];
          return (
            <div key={node.id} data-testid={`rf-node-${node.id}`}>
              <NodeComponent id={node.id} data={node.data} selected={node.selected} />
            </div>
          );
        })}
        {edges.map((edge) => {
          const EdgeComponent = edge.type ? edgeTypes?.[edge.type] : null;
          return (
            <div
              key={edge.id}
              data-testid={`rf-edge-${edge.id}`}
              data-route-points={JSON.stringify(edge.data?.routePoints ?? [])}
            >
              {EdgeComponent ? (
                <svg>
                  <EdgeComponent
                    id={edge.id}
                    sourceX={0}
                    sourceY={0}
                    targetX={80}
                    targetY={40}
                    data={edge.data}
                  />
                </svg>
              ) : null}
            </div>
          );
        })}
        {children}
      </div>
    ),
    ReactFlowProvider: ({ children }: { children: React.ReactNode }) => <>{children}</>,
    useReactFlow: () => ({
      screenToFlowPosition: ({ x, y }: { x: number; y: number }) => ({ x, y }),
    }),
    useNodesState: (initialNodes: unknown[]) => {
      const [nodes, setNodes] = React.useState(initialNodes);
      return [nodes, setNodes, vi.fn()];
    },
    useViewport: () => ({ x: 0, y: 0, zoom: 1 }),
  };
});

const project: ProjectDto = {
  id: "p1",
  name: "Manual Wire Test",
  format_version: "1",
  engine_version: "0.1.4",
  project_type: "schematic",
  schematic: {
    id: "s1",
    title: "Manual Wire Test",
    components: [
      {
        instance_id: "R1",
        definition_id: "generic_resistor",
        component_kind: "generic_resistor",
        display_label: "R1",
        x: 100,
        y: 100,
        rotation_degrees: 0,
        parameters: [
          {
            name: "resistance",
            value: { original: "10k", si_value: 10000, unit: "Ohm", display: "10 kOhm" },
          },
        ],
        symbol: null,
        pins: [
          {
            id: "1",
            name: "1",
            number: "1",
            electrical_type: "passive",
            x: -40,
            y: 0,
            side: "left",
          },
          {
            id: "2",
            name: "2",
            number: "2",
            electrical_type: "passive",
            x: 40,
            y: 0,
            side: "right",
          },
        ],
        connected_nets: [],
      },
      {
        instance_id: "C1",
        definition_id: "generic_capacitor",
        component_kind: "generic_capacitor",
        display_label: "C1",
        x: 240,
        y: 160,
        rotation_degrees: 0,
        parameters: [
          {
            name: "capacitance",
            value: { original: "100n", si_value: 1e-7, unit: "F", display: "100 nF" },
          },
        ],
        symbol: null,
        pins: [
          {
            id: "1",
            name: "1",
            number: "1",
            electrical_type: "passive",
            x: -20,
            y: -30,
            side: "top",
          },
          {
            id: "2",
            name: "2",
            number: "2",
            electrical_type: "passive",
            x: -20,
            y: 30,
            side: "bottom",
          },
        ],
        connected_nets: [],
      },
    ],
    wires: [
      {
        id: "wire-routed",
        from_component_id: "R1",
        from_pin_id: "2",
        to_component_id: "C1",
        to_pin_id: "1",
        net_id: "net_out",
        route_points: [
          { x: 140, y: 100 },
          { x: 200, y: 100 },
          { x: 200, y: 160 },
        ],
        routing_style: "manual",
      },
    ],
    nets: [{ id: "net_out", name: "Vout" }],
  },
};

describe("SchematicCanvas CAD symbol rendering", () => {
  it("renders resistor and capacitor as schematic symbols instead of generic cards", () => {
    render(<SchematicCanvas project={project} />);

    expect(screen.getByTestId("schematic-symbol-resistor")).toBeInTheDocument();
    expect(screen.getByTestId("schematic-symbol-capacitor")).toBeInTheDocument();
    expect(screen.queryByTestId("generic-component-card-R1")).not.toBeInTheDocument();
  });

  it("renders source, ground, diode, op-amp, and MOSFET schematic symbols", () => {
    const symbolProject: ProjectDto = {
      ...project,
      schematic: {
        ...project.schematic,
        components: [
          {
            ...project.schematic.components[0],
            instance_id: "V1",
            definition_id: "generic_voltage_source",
            component_kind: "generic_voltage_source",
            display_label: "V1",
            pins: [
              {
                id: "p",
                name: "p",
                number: "p",
                electrical_type: "power",
                x: 0,
                y: -40,
                side: "top",
              },
              {
                id: "n",
                name: "n",
                number: "n",
                electrical_type: "ground",
                x: 0,
                y: 40,
                side: "bottom",
              },
            ],
          },
          {
            ...project.schematic.components[0],
            instance_id: "GND",
            definition_id: "generic_ground",
            component_kind: "generic_ground",
            display_label: "GND",
            pins: [
              {
                id: "gnd",
                name: "gnd",
                number: "gnd",
                electrical_type: "ground",
                x: 0,
                y: -20,
                side: "top",
              },
            ],
          },
          {
            ...project.schematic.components[0],
            instance_id: "D1",
            definition_id: "generic_diode",
            component_kind: "generic_diode",
            display_label: "D1",
            pins: [
              {
                id: "anode",
                name: "A",
                number: "1",
                electrical_type: "passive",
                x: -40,
                y: 0,
                side: "left",
              },
              {
                id: "cathode",
                name: "K",
                number: "2",
                electrical_type: "passive",
                x: 40,
                y: 0,
                side: "right",
              },
            ],
          },
          {
            ...project.schematic.components[0],
            instance_id: "U1",
            definition_id: "generic_op_amp",
            component_kind: "generic_op_amp",
            display_label: "U1",
            pins: [
              {
                id: "inverting",
                name: "-",
                number: "2",
                electrical_type: "input",
                x: -40,
                y: -10,
                side: "left",
              },
              {
                id: "non_inverting",
                name: "+",
                number: "3",
                electrical_type: "input",
                x: -40,
                y: 10,
                side: "left",
              },
              {
                id: "output",
                name: "OUT",
                number: "1",
                electrical_type: "output",
                x: 40,
                y: 0,
                side: "right",
              },
            ],
          },
          {
            ...project.schematic.components[0],
            instance_id: "M1",
            definition_id: "generic_mosfet_n",
            component_kind: "generic_mosfet_n",
            display_label: "M1",
            pins: [
              {
                id: "drain",
                name: "D",
                number: "1",
                electrical_type: "passive",
                x: 30,
                y: -40,
                side: "top",
              },
              {
                id: "gate",
                name: "G",
                number: "2",
                electrical_type: "passive",
                x: -40,
                y: 0,
                side: "left",
              },
              {
                id: "source",
                name: "S",
                number: "3",
                electrical_type: "passive",
                x: 30,
                y: 40,
                side: "bottom",
              },
            ],
          },
        ],
        wires: [],
      },
    };

    render(<SchematicCanvas project={symbolProject} />);

    expect(screen.getByTestId("schematic-symbol-voltage-source")).toBeInTheDocument();
    expect(screen.getByTestId("schematic-symbol-ground")).toBeInTheDocument();
    expect(screen.getByTestId("schematic-symbol-diode")).toBeInTheDocument();
    expect(screen.getByTestId("schematic-symbol-op-amp")).toBeInTheDocument();
    expect(screen.getByTestId("schematic-symbol-mosfet")).toBeInTheDocument();
  });

  it("keeps visible pin handles aligned to symbol pins", () => {
    render(<SchematicCanvas project={project} />);

    expect(screen.getAllByTestId("pin-handle-1").length).toBeGreaterThan(0);
    expect(screen.getAllByTestId("pin-handle-2").length).toBeGreaterThan(0);
  });

  it("hydrates persisted route points into a manual wire edge", () => {
    render(<SchematicCanvas project={project} />);

    const edge = screen.getByTestId("rf-edge-wire-routed");
    expect(edge).toHaveAttribute(
      "data-route-points",
      JSON.stringify(project.schematic.wires[0].route_points),
    );
    expect(screen.getByTestId("base-edge-wire-routed")).toHaveAttribute(
      "d",
      "M 0 0 L 140 100 L 200 100 L 200 160 L 80 40",
    );
  });
});

describe("SchematicCanvas manual wire routing", () => {
  it("starts the wire preview from the selected pin instead of the canvas origin", () => {
    render(<SchematicCanvas project={project} toolMode="wire" onConnect={vi.fn()} />);

    fireEvent.click(screen.getAllByTestId("pin-handle-2")[0]);
    fireEvent.click(screen.getByTestId("mock-flow"), { clientX: 151, clientY: 161 });

    const previewPath = screen.getByTestId("wire-route-preview").querySelector("path");
    expect(previewPath).toHaveAttribute("d", "M 200 142 L 160 160");
  });

  it("passes node changes back into React Flow so dragged components remain visible", () => {
    render(<SchematicCanvas project={project} />);

    expect(screen.getByTestId("mock-flow")).toHaveAttribute("data-has-node-change-handler", "true");
  });

  it("starts a wire from a pin, adds a grid-snapped bend, and completes on a target pin", () => {
    const onConnect = vi.fn();
    render(<SchematicCanvas project={project} toolMode="wire" onConnect={onConnect} />);

    fireEvent.click(screen.getAllByTestId("pin-handle-2")[0]);
    expect(screen.getByText(/Wire R1\.2 started/i)).toBeInTheDocument();

    fireEvent.click(screen.getByTestId("mock-flow"), { clientX: 151, clientY: 161 });
    expect(screen.getByText(/1 bend point/i)).toBeInTheDocument();

    fireEvent.click(screen.getAllByTestId("pin-handle-1")[1]);

    expect(onConnect).toHaveBeenCalledWith({
      from_component_id: "R1",
      from_pin_id: "2",
      to_component_id: "C1",
      to_pin_id: "1",
      route_points: [{ x: 160, y: 160 }],
    });
  });

  it("cancels an in-progress manual wire with Escape", () => {
    const onConnect = vi.fn();
    render(<SchematicCanvas project={project} toolMode="wire" onConnect={onConnect} />);

    fireEvent.click(screen.getAllByTestId("pin-handle-2")[0]);
    fireEvent.keyDown(window, { key: "Escape" });

    expect(screen.queryByText(/Wire R1\.2 started/i)).not.toBeInTheDocument();
    expect(onConnect).not.toHaveBeenCalled();
  });
});
