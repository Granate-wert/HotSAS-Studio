import {
  Background,
  BaseEdge,
  Controls,
  MiniMap,
  ReactFlow,
  ReactFlowProvider,
  useReactFlow,
  useNodesState,
  useViewport,
  ConnectionMode,
  type Edge,
  type EdgeProps,
  type Node,
} from "@xyflow/react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { PlaceableComponentDto, ProjectDto, WireRoutePointDto } from "../types";
import {
  CapacitorNode,
  DiodeNode,
  GenericComponentNode,
  GroundNode,
  InductorNode,
  MosfetNode,
  OpAmpNode,
  ResistorNode,
  VoltageSourceNode,
} from "./schematic/nodes";

const nodeTypes = {
  resistor: ResistorNode,
  capacitor: CapacitorNode,
  inductor: InductorNode,
  voltage_source: VoltageSourceNode,
  ground: GroundNode,
  diode: DiodeNode,
  op_amp: OpAmpNode,
  mosfet: MosfetNode,
  generic: GenericComponentNode,
};

const edgeTypes = {
  manualWire: ManualWireEdge,
};

const GRID_SIZE = 20;
const SYMBOL_PIN_ORIGIN_X = 60;
const SYMBOL_PIN_ORIGIN_Y = 42;

type WireDraft = {
  from_component_id: string;
  from_pin_id: string;
  source_point: WireRoutePointDto;
  route_points: WireRoutePointDto[];
  preview_point: WireRoutePointDto | null;
};

type ManualWireEdgeData = {
  routePoints?: WireRoutePointDto[];
};

function mapComponentKindToNodeType(kind: string): string {
  const base = kind.startsWith("generic_") ? kind.slice(8) : kind;
  switch (base) {
    case "resistor":
      return "resistor";
    case "capacitor":
      return "capacitor";
    case "inductor":
      return "inductor";
    case "voltage_source":
      return "voltage_source";
    case "ground":
      return "ground";
    case "diode":
      return "diode";
    case "op_amp":
      return "op_amp";
    case "mosfet":
    case "mosfet_n":
    case "mosfet_p":
      return "mosfet";
    default:
      return "generic";
  }
}

function snapToGrid(point: WireRoutePointDto): WireRoutePointDto {
  return {
    x: Math.round(point.x / GRID_SIZE) * GRID_SIZE,
    y: Math.round(point.y / GRID_SIZE) * GRID_SIZE,
  };
}

function manualWirePath({
  sourceX,
  sourceY,
  targetX,
  targetY,
  routePoints = [],
}: {
  sourceX: number;
  sourceY: number;
  targetX: number;
  targetY: number;
  routePoints?: WireRoutePointDto[];
}) {
  const points = [{ x: sourceX, y: sourceY }, ...routePoints, { x: targetX, y: targetY }];
  return points.map((point, index) => `${index === 0 ? "M" : "L"} ${point.x} ${point.y}`).join(" ");
}

function manualWirePathFromPoints(points: WireRoutePointDto[]) {
  return points.map((point, index) => `${index === 0 ? "M" : "L"} ${point.x} ${point.y}`).join(" ");
}

function findPinFlowPoint(
  project: ProjectDto | null,
  componentId: string,
  pinId: string,
): WireRoutePointDto | null {
  const component = project?.schematic.components.find((item) => item.instance_id === componentId);
  const pin = component?.pins.find((item) => item.id === pinId);

  if (!component || !pin) {
    return null;
  }

  const nodeType = mapComponentKindToNodeType(component.component_kind);
  if (nodeType === "generic") {
    return {
      x: component.x + pin.x,
      y: component.y + pin.y,
    };
  }

  return {
    x: component.x + SYMBOL_PIN_ORIGIN_X + pin.x,
    y: component.y + SYMBOL_PIN_ORIGIN_Y + pin.y,
  };
}

function ManualWireEdge({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  data,
  label,
}: EdgeProps<Edge<ManualWireEdgeData>>) {
  const routePoints = data?.routePoints ?? [];
  const path = manualWirePath({ sourceX, sourceY, targetX, targetY, routePoints });
  const labelPoint = routePoints[Math.floor(routePoints.length / 2)];

  return (
    <BaseEdge id={id} path={path} label={label} labelX={labelPoint?.x} labelY={labelPoint?.y} />
  );
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
    route_points?: WireRoutePointDto[];
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
  const viewport = useViewport();
  const [wireDraft, setWireDraft] = useState<WireDraft | null>(null);
  const wireDraftRef = useRef<WireDraft | null>(null);

  useEffect(() => {
    wireDraftRef.current = wireDraft;
  }, [wireDraft]);

  const handlePinClick = useCallback(
    (componentId: string, pinId: string) => {
      if (toolMode !== "wire" || disabled) {
        return;
      }

      const currentDraft = wireDraftRef.current;

      if (!currentDraft) {
        setWireDraft({
          from_component_id: componentId,
          from_pin_id: pinId,
          source_point: findPinFlowPoint(project, componentId, pinId) ?? { x: 0, y: 0 },
          route_points: [],
          preview_point: null,
        });
        return;
      }

      if (currentDraft.from_component_id === componentId && currentDraft.from_pin_id === pinId) {
        return;
      }

      onConnect?.({
        from_component_id: currentDraft.from_component_id,
        from_pin_id: currentDraft.from_pin_id,
        to_component_id: componentId,
        to_pin_id: pinId,
        route_points: currentDraft.route_points,
      });
      setWireDraft(null);
    },
    [toolMode, disabled, project, onConnect],
  );

  const netNameMap = useMemo(() => {
    const map = new Map<string, string>();
    project?.schematic.nets.forEach((net) => map.set(net.id, net.name));
    return map;
  }, [project]);

  const { nodes: projectNodes, edges } = useMemo(() => {
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
        onPinClick: handlePinClick,
      },
    }));

    const edges: Edge[] = project.schematic.wires
      .filter((wire) => wire.from_component_id && wire.to_component_id)
      .map((wire) => ({
        id: wire.id,
        source: wire.from_component_id as string,
        target: wire.to_component_id as string,
        sourceHandle: wire.from_pin_id ?? undefined,
        targetHandle: wire.to_pin_id ?? undefined,
        label: netNameMap.get(wire.net_id) || wire.net_id,
        type: wire.route_points.length > 0 ? "manualWire" : "smoothstep",
        data: {
          routePoints: wire.route_points,
        },
      }));

    return { nodes, edges };
  }, [project, onSelectComponent, netNameMap, handlePinClick]);

  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);

  useEffect(() => {
    setNodes(projectNodes);
  }, [projectNodes, setNodes]);

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
          route_points: [],
        });
      }
    },
    [disabled, onConnect],
  );

  const handlePaneClick = useCallback(
    (_event: React.MouseEvent) => {
      if (toolMode === "wire" && wireDraft) {
        const nativeEvent = _event.nativeEvent as MouseEvent;
        const point = snapToGrid(
          screenToFlowPosition({
            x: nativeEvent.clientX,
            y: nativeEvent.clientY,
          }),
        );
        setWireDraft((draft) =>
          draft ? { ...draft, route_points: [...draft.route_points, point] } : draft,
        );
        return;
      }

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
    [toolMode, wireDraft, pendingPlaceComponent, onPlaceSchematicComponent, screenToFlowPosition],
  );

  const handlePaneMouseMove = useCallback(
    (_event: React.MouseEvent) => {
      if (toolMode !== "wire" || !wireDraft) {
        return;
      }

      const nativeEvent = _event.nativeEvent as MouseEvent;
      const point = snapToGrid(
        screenToFlowPosition({
          x: nativeEvent.clientX,
          y: nativeEvent.clientY,
        }),
      );
      setWireDraft((draft) => (draft ? { ...draft, preview_point: point } : draft));
    },
    [toolMode, wireDraft, screenToFlowPosition],
  );

  useEffect(() => {
    if (toolMode !== "wire") {
      setWireDraft(null);
    }
  }, [toolMode]);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setWireDraft(null);
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  const cursorClass =
    toolMode === "place" ? "cursor-crosshair" : toolMode === "delete" ? "cursor-not-allowed" : "";
  const draftRoutePoints =
    wireDraft && wireDraft.preview_point
      ? [...wireDraft.route_points, wireDraft.preview_point]
      : (wireDraft?.route_points ?? []);
  const previewPoints = useMemo(() => {
    if (!wireDraft || draftRoutePoints.length === 0) {
      return [];
    }

    return [wireDraft.source_point, ...draftRoutePoints].map((point) => ({
      x: point.x * viewport.zoom + viewport.x,
      y: point.y * viewport.zoom + viewport.y,
    }));
  }, [draftRoutePoints, viewport.x, viewport.y, viewport.zoom, wireDraft]);

  return (
    <div className={`canvas ${cursorClass}`}>
      {toolMode === "wire" && (
        <div className="wire-guidance">
          {wireDraft
            ? `Wire ${wireDraft.from_component_id}.${wireDraft.from_pin_id} started - ${wireDraft.route_points.length} bend point${wireDraft.route_points.length === 1 ? "" : "s"}`
            : "Wire mode: click a visible pin to start a grid-snapped manual wire"}
        </div>
      )}
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        onNodesChange={onNodesChange}
        onNodeClick={handleNodeClick}
        onNodeDragStop={handleNodeDragStop}
        onEdgeClick={handleEdgeClick}
        onConnect={handleConnect}
        onPaneClick={handlePaneClick}
        onPaneMouseMove={handlePaneMouseMove}
        connectionMode={ConnectionMode.Loose}
        snapToGrid
        snapGrid={[GRID_SIZE, GRID_SIZE]}
        fitView
        fitViewOptions={{ maxZoom: 1.5, minZoom: 0.5 }}
      >
        {previewPoints.length > 1 && (
          <svg className="wire-preview" data-testid="wire-route-preview">
            <path d={manualWirePathFromPoints(previewPoints)} />
          </svg>
        )}
        <Background />
        <Controls />
        <MiniMap pannable zoomable />
      </ReactFlow>
    </div>
  );
}
