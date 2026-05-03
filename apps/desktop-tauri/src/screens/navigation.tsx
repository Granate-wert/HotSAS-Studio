import {
  Activity,
  Calculator,
  CircuitBoard,
  FileText,
  Sigma,
  TableProperties,
  Upload,
} from "lucide-react";
import type { ReactNode } from "react";

export type ScreenId =
  | "start"
  | "schematic"
  | "notebook"
  | "formulas"
  | "components"
  | "simulation"
  | "export"
  | "import";

export const navigationItems: Array<{ id: ScreenId; label: string; icon: ReactNode }> = [
  { id: "start", label: "Start", icon: <CircuitBoard size={18} /> },
  { id: "schematic", label: "Schematic", icon: <CircuitBoard size={18} /> },
  { id: "notebook", label: "Engineering Notebook", icon: <Calculator size={18} /> },
  { id: "formulas", label: "Formula Library", icon: <Sigma size={18} /> },
  { id: "components", label: "E Component Library", icon: <TableProperties size={18} /> },
  { id: "simulation", label: "Simulation Results", icon: <Activity size={18} /> },
  { id: "export", label: "Export Center", icon: <FileText size={18} /> },
  { id: "import", label: "Import Models", icon: <Upload size={18} /> },
];
