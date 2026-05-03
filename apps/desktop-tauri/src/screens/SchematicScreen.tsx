import { Tabs } from "@mantine/core";
import { FormulaPanel } from "../components/FormulaPanel";
import { LibraryPanel } from "../components/LibraryPanel";
import { PreBlock } from "../components/PreBlock";
import { ProjectMetrics } from "../components/ProjectMetrics";
import { ReportPanel } from "../components/ReportPanel";
import { SchematicCanvas } from "../components/SchematicCanvas";
import { CircuitValidationPanel } from "../components/schematic/CircuitValidationPanel";
import { SchematicPropertyPanel } from "../components/schematic/SchematicPropertyPanel";
import { SimulationChart } from "../components/SimulationChart";
import type { CircuitValidationReportDto, ProjectDto, SelectedComponentDto } from "../types";
import type { ProjectMetricsData } from "./screenTypes";

type SchematicScreenProps = ProjectMetricsData & {
  netlist: string;
  markdownReport: string;
  htmlReport: string;
  onMarkdown: () => void;
  onHtml: () => void;
  hasProject: boolean;
  selectedComponent: SelectedComponentDto | null;
  validationReport: CircuitValidationReportDto | null;
  onSelectComponent: (instanceId: string) => void;
  onValidate: (report: CircuitValidationReportDto) => void;
  onPropertyUpdate: () => void;
};

export function SchematicScreen({
  project,
  formulaResult,
  preferredValue,
  simulation,
  netlist,
  markdownReport,
  htmlReport,
  onMarkdown,
  onHtml,
  hasProject,
  selectedComponent,
  validationReport,
  onSelectComponent,
  onValidate,
  onPropertyUpdate,
}: SchematicScreenProps) {
  return (
    <div className="grid">
      <section className="schematic-panel">
        <SchematicCanvas project={project} onSelectComponent={onSelectComponent} />
      </section>

      <aside className="side-panel">
        <Tabs defaultValue="properties" className="tabs">
          <Tabs.List>
            <Tabs.Tab value="properties">Properties</Tabs.Tab>
            <Tabs.Tab value="validation">Validation</Tabs.Tab>
            <Tabs.Tab value="metrics">Metrics</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="properties">
            <SchematicPropertyPanel component={selectedComponent} onUpdate={onPropertyUpdate} />
          </Tabs.Panel>
          <Tabs.Panel value="validation">
            <CircuitValidationPanel report={validationReport} onValidate={onValidate} />
          </Tabs.Panel>
          <Tabs.Panel value="metrics">
            <ProjectMetrics
              project={project}
              formulaResult={formulaResult}
              preferredValue={preferredValue}
              simulation={simulation}
            />
          </Tabs.Panel>
        </Tabs>
      </aside>

      <section className="bottom-panel">
        <Tabs defaultValue="netlist" className="tabs">
          <Tabs.List>
            <Tabs.Tab value="netlist">Netlist</Tabs.Tab>
            <Tabs.Tab value="graph">Graph</Tabs.Tab>
            <Tabs.Tab value="formula">Formula</Tabs.Tab>
            <Tabs.Tab value="report">Report</Tabs.Tab>
            <Tabs.Tab value="library">Libraries</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="netlist">
            <PreBlock text={netlist || "Generate SPICE netlist"} />
          </Tabs.Panel>
          <Tabs.Panel value="graph">
            <SimulationChart simulation={simulation} />
          </Tabs.Panel>
          <Tabs.Panel value="formula">
            <FormulaPanel />
          </Tabs.Panel>
          <Tabs.Panel value="report">
            <ReportPanel
              markdownReport={markdownReport}
              htmlReport={htmlReport}
              onMarkdown={onMarkdown}
              onHtml={onHtml}
              disabled={!hasProject}
            />
          </Tabs.Panel>
          <Tabs.Panel value="library">
            <LibraryPanel />
          </Tabs.Panel>
        </Tabs>
      </section>
    </div>
  );
}
