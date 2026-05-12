import {
  Activity,
  BarChart3,
  Calculator,
  CircuitBoard,
  FileText,
  Filter,
  HeartPulse,
  Radio,
  Rocket,
  Sigma,
  TableProperties,
  Upload,
  Zap,
} from "lucide-react";
import type { ReactNode } from "react";

export type ScreenId =
  | "start"
  | "schematic"
  | "notebook"
  | "formulas"
  | "components"
  | "simulation"
  | "filter-analysis"
  | "s-parameter-analysis"
  | "export"
  | "import"
  | "diagnostics"
  | "product-beta"
  | "dcdc"
  | "reports";

export const navigationItems: Array<{ id: ScreenId; label: string; icon: ReactNode }> = [
  { id: "start", label: "Start", icon: <CircuitBoard size={18} /> },
  { id: "schematic", label: "Schematic", icon: <CircuitBoard size={18} /> },
  { id: "notebook", label: "Engineering Notebook", icon: <Calculator size={18} /> },
  { id: "formulas", label: "Formula Library", icon: <Sigma size={18} /> },
  { id: "components", label: "Component Library", icon: <TableProperties size={18} /> },
  { id: "simulation", label: "Simulation Dashboard", icon: <Activity size={18} /> },
  { id: "filter-analysis", label: "Filter Analysis", icon: <Filter size={18} /> },
  { id: "s-parameter-analysis", label: "S-Parameters", icon: <Radio size={18} /> },
  { id: "dcdc", label: "DC-DC Calculators", icon: <Zap size={18} /> },
  { id: "export", label: "Export Center", icon: <FileText size={18} /> },
  { id: "import", label: "Import Models", icon: <Upload size={18} /> },
  { id: "diagnostics", label: "Diagnostics", icon: <HeartPulse size={18} /> },
  { id: "reports", label: "Advanced Reports", icon: <BarChart3 size={18} /> },
  { id: "product-beta", label: "Product Beta", icon: <Rocket size={18} /> },
];
