import type { NodeProps } from "@xyflow/react";
import { GenericComponentNode } from "./GenericComponentNode";

export function GroundNode(props: NodeProps) {
  return <GenericComponentNode {...props} />;
}
