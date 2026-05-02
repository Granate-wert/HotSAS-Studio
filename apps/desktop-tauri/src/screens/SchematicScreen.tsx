import { Tabs } from "@mantine/core";
import { FormulaPanel } from "../components/FormulaPanel";
import { LibraryPanel } from "../components/LibraryPanel";
import { PreBlock } from "../components/PreBlock";
import { ProjectMetrics } from "../components/ProjectMetrics";
import { ReportPanel } from "../components/ReportPanel";
import { SchematicCanvas } from "../components/SchematicCanvas";
import { SimulationChart } from "../components/SimulationChart";
import type { ProjectMetricsData } from "./screenTypes";

type SchematicScreenProps = ProjectMetricsData & {
  netlist: string;
  markdownReport: string;
  htmlReport: string;
  onMarkdown: () => void;
  onHtml: () => void;
  hasProject: boolean;
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
}: SchematicScreenProps) {
  return (
    <div className="grid">
      <section className="schematic-panel">
        <SchematicCanvas project={project} />
      </section>

      <aside className="side-panel">
        <ProjectMetrics
          project={project}
          formulaResult={formulaResult}
          preferredValue={preferredValue}
          simulation={simulation}
        />
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
