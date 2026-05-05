import { useEffect, useState } from "react";
import { Tabs } from "@mantine/core";
import { FormulaPanel } from "../components/FormulaPanel";
import { LibraryPanel } from "../components/LibraryPanel";
import { PreBlock } from "../components/PreBlock";
import { ProjectMetrics } from "../components/ProjectMetrics";
import { ReportPanel } from "../components/ReportPanel";
import { SchematicCanvas } from "../components/SchematicCanvas";
import { SelectedRegionPanel } from "../components/selected-region";
import { CircuitValidationPanel } from "../components/schematic/CircuitValidationPanel";
import { SchematicPropertyPanel } from "../components/schematic/SchematicPropertyPanel";
import { SimulationChart } from "../components/SimulationChart";
import { ComponentPalette } from "../components/schematic-editor/ComponentPalette";
import { ConnectionPanel } from "../components/schematic-editor/ConnectionPanel";
import { NetLabelEditor } from "../components/schematic-editor/NetLabelEditor";
import { SchematicEditStatusPanel } from "../components/schematic-editor/SchematicEditStatusPanel";
import { SchematicToolbar } from "../components/schematic-editor/SchematicToolbar";
import type {
  CircuitValidationIssueDto,
  CircuitValidationReportDto,
  ProjectDto,
  SchematicToolCapabilityDto,
  SelectedComponentDto,
} from "../types";
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
  schematicCapabilities: SchematicToolCapabilityDto[];
  schematicEditLoading: boolean;
  schematicEditError: string | null;
  pendingConnectionStart: { componentId: string; pinId: string } | null;
  onLoadSchematicCapabilities: () => void;
  onAddComponent: (kind: string) => void;
  onMoveComponent: (instanceId: string, x: number, y: number) => void;
  onDeleteComponent: (instanceId: string) => void;
  onConnectPins: (request: {
    from_component_id: string;
    from_pin_id: string;
    to_component_id: string;
    to_pin_id: string;
    net_name?: string | null;
  }) => void;
  onRenameNet: (netId: string, newName: string) => void;
  onSetPendingConnectionStart: (start: { componentId: string; pinId: string } | null) => void;
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
  schematicCapabilities,
  schematicEditLoading,
  schematicEditError,
  pendingConnectionStart,
  onLoadSchematicCapabilities,
  onAddComponent,
  onMoveComponent,
  onDeleteComponent,
  onConnectPins,
  onRenameNet,
  onSetPendingConnectionStart,
}: SchematicScreenProps) {
  const [showConnectionPanel, setShowConnectionPanel] = useState(false);
  const [showNetEditor, setShowNetEditor] = useState(false);

  useEffect(() => {
    if (hasProject && schematicCapabilities.length === 0) {
      onLoadSchematicCapabilities();
    }
  }, [hasProject, schematicCapabilities.length, onLoadSchematicCapabilities]);

  const selectedInstanceId = selectedComponent?.instance_id ?? null;
  const validationWarnings = validationReport?.warnings ?? [];

  return (
    <div className="grid" style={{ gridTemplateRows: "auto 1fr auto" }}>
      <section>
        <SchematicToolbar
          selectedComponentId={selectedInstanceId}
          onDelete={onDeleteComponent}
          onConnect={() => setShowConnectionPanel((s) => !s)}
          onRenameNet={() => setShowNetEditor((s) => !s)}
          disabled={!hasProject || schematicEditLoading}
          editError={schematicEditError}
        />
        {hasProject && <ComponentPalette onAdd={onAddComponent} disabled={schematicEditLoading} />}
        {showConnectionPanel && hasProject && (
          <ConnectionPanel
            components={project?.schematic.components ?? []}
            onConnect={(req) => {
              onConnectPins(req);
              setShowConnectionPanel(false);
            }}
            onCancel={() => setShowConnectionPanel(false)}
          />
        )}
        {showNetEditor && hasProject && (
          <NetLabelEditor
            nets={project?.schematic.nets ?? []}
            onRename={(netId, newName) => {
              onRenameNet(netId, newName);
              setShowNetEditor(false);
            }}
            onCancel={() => setShowNetEditor(false)}
          />
        )}
      </section>

      <section className="schematic-panel">
        <SchematicCanvas
          project={project}
          onSelectComponent={onSelectComponent}
          onMoveComponent={onMoveComponent}
          disabled={!hasProject || schematicEditLoading}
        />
      </section>

      <aside className="side-panel">
        <Tabs defaultValue="properties" className="tabs">
          <Tabs.List>
            <Tabs.Tab value="properties">Properties</Tabs.Tab>
            <Tabs.Tab value="validation">Validation</Tabs.Tab>
            <Tabs.Tab value="metrics">Metrics</Tabs.Tab>
            <Tabs.Tab value="region">Region</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="properties">
            <SchematicPropertyPanel component={selectedComponent} onUpdate={onPropertyUpdate} />
          </Tabs.Panel>
          <Tabs.Panel value="validation">
            <CircuitValidationPanel report={validationReport} onValidate={onValidate} />
            <div style={{ marginTop: 8, padding: 8 }}>
              <SchematicEditStatusPanel warnings={validationWarnings} />
            </div>
          </Tabs.Panel>
          <Tabs.Panel value="metrics">
            <ProjectMetrics
              project={project}
              formulaResult={formulaResult}
              preferredValue={preferredValue}
              simulation={simulation}
            />
          </Tabs.Panel>
          <Tabs.Panel value="region">
            <SelectedRegionPanel project={project} />
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
