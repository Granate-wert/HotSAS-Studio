import type { FormulaResultDto, PreferredValueDto, ProjectDto, SimulationResultDto } from '../types';

export type VerticalSliceActions = {
  onCreateDemo: () => void;
  onCalculate: () => void;
  onNearestE24: () => void;
  onNetlist: () => void;
  onSimulation: () => void;
  onMarkdown: () => void;
  onHtml: () => void;
};

export type ProjectMetricsData = {
  project: ProjectDto | null;
  formulaResult: FormulaResultDto | null;
  preferredValue: PreferredValueDto | null;
  simulation: SimulationResultDto | null;
};
