import { Background, Controls, MiniMap, ReactFlow, type Edge, type Node } from "@xyflow/react";
import { useCallback, useMemo } from "react";
import type { ProjectDto } from "../types";
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
  onSelectComponent?: (instanceId: string) => void;
};

export function SchematicCanvas({ project, onSelectComponent }: SchematicCanvasProps) {
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
        label: wire.net_id,
        type: "smoothstep",
      }));

    return { nodes, edges };
  }, [project, onSelectComponent]);

  const handleNodeClick = useCallback(
    (_event: React.MouseEvent, node: Node) => {
      onSelectComponent?.(node.id);
    },
    [onSelectComponent],
  );

  return (
    <div className="canvas">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodeClick={handleNodeClick}
        fitView
      >
        <Background />
        <Controls />
        <MiniMap pannable zoomable />
      </ReactFlow>
    </div>
  );
}
