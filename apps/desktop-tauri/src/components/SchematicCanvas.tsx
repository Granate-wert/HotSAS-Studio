import {
  Background,
  Controls,
  MiniMap,
  ReactFlow,
  ReactFlowProvider,
  useReactFlow,
  type Edge,
  type Node,
} from "@xyflow/react";
import { useCallback, useMemo } from "react";
import type { PlaceableComponentDto, ProjectDto } from "../types";
import {
  CapacitorNode,
  GenericComponentNode,
  GroundNode,
  ResistorNode,
  VoltageSourceNode,
} from "./schematic/nodes";

const nodeTypes = {
  resistor: ResistorNode,
  capacitor: CapacitorNode,
  voltage_source: VoltageSourceNode,
  ground: GroundNode,
  generic: GenericComponentNode,
};

function mapComponentKindToNodeType(kind: string): string {
  switch (kind) {
    case "resistor":
      return "resistor";
    case "capacitor":
      return "capacitor";
    case "voltage_source":
      return "voltage_source";
    case "ground":
      return "ground";
    default:
      return "generic";
  }
}

type SchematicCanvasProps = {
  project: ProjectDto | null;
  toolMode?: "select" | "place" | "wire" | "delete";
  pendingPlaceComponent?: PlaceableComponentDto | null;
  onSelectComponent?: (instanceId: string) => void;
  onMoveComponent?: (instanceId: string, x: number, y: number) => void;
  onDeleteComponent?: (instanceId: string) => void;
  onSelectWire?: (wireId: string) => void;
  onDeleteWire?: (wireId: string) => void;
  onConnect?: (request: {
    from_component_id: string;
    from_pin_id: string;
    to_component_id: string;
    to_pin_id: string;
  }) => void;
  onPlaceSchematicComponent?: (request: {
    component_definition_id: string;
    x: number;
    y: number;
    rotation_deg: number;
  }) => void;
  disabled?: boolean;
};

export function SchematicCanvas(props: SchematicCanvasProps) {
  return (
    <ReactFlowProvider>
      <SchematicCanvasInner {...props} />
    </ReactFlowProvider>
  );
}

function SchematicCanvasInner({
  project,
  toolMode = "select",
  pendingPlaceComponent,
  onSelectComponent,
  onMoveComponent,
  onDeleteComponent,
  onSelectWire,
  onDeleteWire,
  onConnect,
  onPlaceSchematicComponent,
  disabled,
}: SchematicCanvasProps) {
  const { screenToFlowPosition } = useReactFlow();
  const netNameMap = useMemo(() => {
    const map = new Map<string, string>();
    project?.schematic.nets.forEach((net) => map.set(net.id, net.name));
    return map;
  }, [project]);

  const { nodes, edges } = useMemo(() => {
    if (!project) {
      return { nodes: [], edges: [] };
    }

    const nodes: Node[] = project.schematic.components.map((component) => ({
      id: component.instance_id,
      type: mapComponentKindToNodeType(component.component_kind),
      position: { x: component.x, y: component.y },
      data: {
        component,
        onSelect: onSelectComponent,
      },
    }));

    const edges: Edge[] = project.schematic.wires
      .filter((wire) => wire.from_component_id && wire.to_component_id)
      .map((wire) => ({
        id: wire.id,
        source: wire.from_component_id as string,
        target: wire.to_component_id as string,
        label: netNameMap.get(wire.net_id) || wire.net_id,
        type: "smoothstep",
      }));

    return { nodes, edges };
  }, [project, onSelectComponent, netNameMap]);

  const handleNodeClick = useCallback(
    (_event: React.MouseEvent, node: Node) => {
      if (toolMode === "delete") {
        onDeleteComponent?.(node.id);
        return;
      }
      onSelectComponent?.(node.id);
    },
    [toolMode, onSelectComponent, onDeleteComponent],
  );

  const handleNodeDragStop = useCallback(
    (_event: React.MouseEvent, node: Node) => {
      if (!disabled && onMoveComponent) {
        onMoveComponent(node.id, node.position.x, node.position.y);
      }
    },
    [disabled, onMoveComponent],
  );

  const handleEdgeClick = useCallback(
    (_event: React.MouseEvent, edge: Edge) => {
      if (toolMode === "delete") {
        onDeleteWire?.(edge.id);
        return;
      }
      if (!disabled && onSelectWire) {
        onSelectWire(edge.id);
      }
    },
    [toolMode, disabled, onSelectWire, onDeleteWire],
  );

  const handleConnect = useCallback(
    (params: {
      source: string | null;
      sourceHandle: string | null;
      target: string | null;
      targetHandle: string | null;
    }) => {
      if (
        !disabled &&
        onConnect &&
        params.source &&
        params.sourceHandle &&
        params.target &&
        params.targetHandle
      ) {
        onConnect({
          from_component_id: params.source,
          from_pin_id: params.sourceHandle,
          to_component_id: params.target,
          to_pin_id: params.targetHandle,
        });
      }
    },
    [disabled, onConnect],
  );

  const handlePaneClick = useCallback(
    (_event: React.MouseEvent) => {
      if (toolMode === "place" && pendingPlaceComponent && onPlaceSchematicComponent) {
        const nativeEvent = _event.nativeEvent as MouseEvent;
        const { x, y } = screenToFlowPosition({
          x: nativeEvent.clientX,
          y: nativeEvent.clientY,
        });
        onPlaceSchematicComponent({
          component_definition_id: pendingPlaceComponent.definition_id,
          x,
          y,
          rotation_deg: 0,
        });
      }
    },
    [toolMode, pendingPlaceComponent, onPlaceSchematicComponent, screenToFlowPosition],
  );

  const cursorClass =
    toolMode === "place" ? "cursor-crosshair" : toolMode === "delete" ? "cursor-not-allowed" : "";

  return (
    <div className={`canvas ${cursorClass}`}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodeClick={handleNodeClick}
        onNodeDragStop={handleNodeDragStop}
        onEdgeClick={handleEdgeClick}
        onConnect={handleConnect}
        onPaneClick={handlePaneClick}
        fitView
        fitViewOptions={{ maxZoom: 1.5, minZoom: 0.5 }}
      >
        <Background />
        <Controls />
        <MiniMap pannable zoomable />
      </ReactFlow>
    </div>
  );
}
