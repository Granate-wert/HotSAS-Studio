import { Stack, Text } from "@mantine/core";
import { Background, Controls, MiniMap, ReactFlow, type Edge, type Node } from "@xyflow/react";
import { useMemo } from "react";
import type { ProjectDto } from "../types";

export function SchematicCanvas({ project }: { project: ProjectDto | null }) {
  const { nodes, edges } = useMemo(() => {
    if (!project) {
      return { nodes: [], edges: [] };
    }

    const nodes: Node[] = project.schematic.components.map((component) => ({
      id: component.instance_id,
      type: "default",
      position: { x: component.x, y: component.y },
      data: {
        label: (
          <Stack gap={2}>
            <Text fw={700} size="sm">
              {component.instance_id}
            </Text>
            <Text size="xs" c="dimmed">
              {component.definition_id}
            </Text>
          </Stack>
        ),
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
  }, [project]);

  return (
    <div className="canvas">
      <ReactFlow nodes={nodes} edges={edges} fitView>
        <Background />
        <Controls />
        <MiniMap pannable zoomable />
      </ReactFlow>
    </div>
  );
}
