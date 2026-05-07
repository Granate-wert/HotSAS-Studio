import { useEffect, useRef, useState } from "react";
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
import { InteractiveSchematicToolbar } from "../components/schematic-editor/InteractiveSchematicToolbar";
import { PlaceableComponentPalette } from "../components/schematic-editor/PlaceableComponentPalette";
import { SchematicSelectionInspector } from "../components/schematic-editor/SchematicSelectionInspector";
import { QuickParameterEditor } from "../components/schematic-editor/QuickParameterEditor";
import { UndoRedoToolbar } from "../components/schematic-editor/UndoRedoToolbar";
import { NetlistPreviewPanel } from "../components/schematic-editor/NetlistPreviewPanel";
import { ErcIssuePanel } from "../components/schematic-editor/ErcIssuePanel";
import type {
  CircuitValidationIssueDto,
  CircuitValidationReportDto,
  NetlistPreviewDto,
  PlaceableComponentDto,
  ProjectDto,
  SchematicSelectionDetailsDto,
  SchematicToolCapabilityDto,
  SelectedComponentDto,
  UndoRedoStateDto,
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
  // v2.8 interactive schematic editing
  schematicToolMode: "select" | "place" | "wire" | "delete";
  placeableComponents: PlaceableComponentDto[];
  pendingPlaceComponent: PlaceableComponentDto | null;
  pendingWireStart: { componentId: string; pinId: string } | null;
  selectedSchematicEntity: { kind: "component" | "wire" | "net"; id: string } | null;
  schematicSelectionDetails: SchematicSelectionDetailsDto | null;
  undoRedoState: UndoRedoStateDto | null;
  netlistPreview: NetlistPreviewDto | null;
  schematicInteractionLoading: boolean;
  schematicInteractionError: string | null;
  onLoadPlaceableComponents: () => void;
  onPlaceSchematicComponent: (request: {
    component_definition_id: string;
    x: number;
    y: number;
    rotation_deg: number;
  }) => void;
  onDeleteSchematicWire: (wireId: string) => void;
  onUpdateSchematicQuickParameter: (
    componentId: string,
    parameterId: string,
    value: string,
  ) => void;
  onGetSchematicSelectionDetails: (kind: "component" | "wire" | "net", id: string) => void;
  onUndoSchematicEdit: () => void;
  onRedoSchematicEdit: () => void;
  onGetSchematicUndoRedoState: () => void;
  onGenerateCurrentSchematicNetlistPreview: () => void;
  onSetSchematicToolMode: (mode: "select" | "place" | "wire" | "delete") => void;
  onSetPendingPlaceComponent: (component: PlaceableComponentDto | null) => void;
  onSetPendingWireStart: (start: { componentId: string; pinId: string } | null) => void;
  onSetSelectedSchematicEntity: (
    entity: { kind: "component" | "wire" | "net"; id: string } | null,
  ) => void;
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
  // v2.8
  schematicToolMode,
  placeableComponents,
  pendingPlaceComponent,
  pendingWireStart,
  selectedSchematicEntity,
  schematicSelectionDetails,
  undoRedoState,
  netlistPreview,
  schematicInteractionLoading,
  schematicInteractionError,
  onLoadPlaceableComponents,
  onPlaceSchematicComponent,
  onDeleteSchematicWire,
  onUpdateSchematicQuickParameter,
  onGetSchematicSelectionDetails,
  onUndoSchematicEdit,
  onRedoSchematicEdit,
  onGetSchematicUndoRedoState,
  onGenerateCurrentSchematicNetlistPreview,
  onSetSchematicToolMode,
  onSetPendingPlaceComponent,
  onSetPendingWireStart,
  onSetSelectedSchematicEntity,
}: SchematicScreenProps) {
  const [showConnectionPanel, setShowConnectionPanel] = useState(false);
  const [showNetEditor, setShowNetEditor] = useState(false);
  const [showNetlistPreview, setShowNetlistPreview] = useState(false);
  const hasLoadedCapabilities = useRef(false);
  const hasLoadedPlaceable = useRef(false);

  useEffect(() => {
    if (hasProject && schematicCapabilities.length === 0 && !hasLoadedCapabilities.current) {
      hasLoadedCapabilities.current = true;
      onLoadSchematicCapabilities();
    }
  }, [hasProject, schematicCapabilities.length, onLoadSchematicCapabilities]);

  useEffect(() => {
    if (hasProject && placeableComponents.length === 0 && !hasLoadedPlaceable.current) {
      hasLoadedPlaceable.current = true;
      onLoadPlaceableComponents();
    }
  }, [hasProject, placeableComponents.length, onLoadPlaceableComponents]);

  useEffect(() => {
    if (hasProject) {
      onGetSchematicUndoRedoState();
    }
  }, [hasProject, project, onGetSchematicUndoRedoState]);

  const selectedInstanceId = selectedComponent?.instance_id ?? null;
  const validationWarnings = validationReport?.warnings ?? [];
  const validationErrors = validationReport?.errors ?? [];

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
        <InteractiveSchematicToolbar
          toolMode={schematicToolMode}
          onSetToolMode={onSetSchematicToolMode}
          disabled={!hasProject || schematicEditLoading || schematicInteractionLoading}
        />
        <UndoRedoToolbar
          canUndo={undoRedoState?.can_undo ?? false}
          canRedo={undoRedoState?.can_redo ?? false}
          lastActionLabel={undoRedoState?.last_action_label ?? null}
          nextRedoLabel={undoRedoState?.next_redo_label ?? null}
          onUndo={onUndoSchematicEdit}
          onRedo={onRedoSchematicEdit}
          disabled={!hasProject || schematicInteractionLoading}
        />
        {schematicToolMode === "place" && hasProject && (
          <PlaceableComponentPalette
            components={placeableComponents}
            onSelect={onSetPendingPlaceComponent}
            selected={pendingPlaceComponent}
            disabled={schematicInteractionLoading}
          />
        )}
        {hasProject && schematicToolMode !== "place" && (
          <ComponentPalette onAdd={onAddComponent} disabled={schematicEditLoading} />
        )}
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
        {schematicInteractionError && (
          <div style={{ padding: 8, background: "#ff444433", color: "#ffcccc" }}>
            {schematicInteractionError}
          </div>
        )}
      </section>

      <section className="schematic-panel">
        <SchematicCanvas
          project={project}
          onSelectComponent={(id) => {
            onSelectComponent(id);
            onSetSelectedSchematicEntity({ kind: "component", id });
            onGetSchematicSelectionDetails("component", id);
          }}
          onMoveComponent={onMoveComponent}
          onSelectWire={(id) => {
            onSetSelectedSchematicEntity({ kind: "wire", id });
            onGetSchematicSelectionDetails("wire", id);
          }}
          onConnect={(req) => onConnectPins({ ...req, net_name: null })}
          disabled={!hasProject || schematicEditLoading || schematicInteractionLoading}
        />
        {pendingPlaceComponent && (
          <div
            style={{
              position: "absolute",
              bottom: 8,
              left: 8,
              padding: 8,
              background: "#333",
              borderRadius: 4,
            }}
          >
            Click canvas to place {pendingPlaceComponent.name}
          </div>
        )}
      </section>

      <aside className="side-panel">
        <Tabs defaultValue="properties" className="tabs">
          <Tabs.List>
            <Tabs.Tab value="properties">Properties</Tabs.Tab>
            <Tabs.Tab value="selection">Selection</Tabs.Tab>
            <Tabs.Tab value="validation">Validation</Tabs.Tab>
            <Tabs.Tab value="metrics">Metrics</Tabs.Tab>
            <Tabs.Tab value="region">Region</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="properties">
            <SchematicPropertyPanel component={selectedComponent} onUpdate={onPropertyUpdate} />
          </Tabs.Panel>
          <Tabs.Panel value="selection">
            <SchematicSelectionInspector
              entity={selectedSchematicEntity}
              details={schematicSelectionDetails}
              onDeleteWire={onDeleteSchematicWire}
              onUpdateParameter={onUpdateSchematicQuickParameter}
              loading={schematicInteractionLoading}
            />
          </Tabs.Panel>
          <Tabs.Panel value="validation">
            <CircuitValidationPanel report={validationReport} onValidate={onValidate} />
            <div style={{ marginTop: 8, padding: 8 }}>
              <SchematicEditStatusPanel warnings={validationWarnings} />
            </div>
            <ErcIssuePanel errors={validationErrors} warnings={validationWarnings} />
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
            <Tabs.Tab
              value="netlist-preview"
              onClick={() => onGenerateCurrentSchematicNetlistPreview()}
            >
              Netlist Preview
            </Tabs.Tab>
            <Tabs.Tab value="graph">Graph</Tabs.Tab>
            <Tabs.Tab value="formula">Formula</Tabs.Tab>
            <Tabs.Tab value="report">Report</Tabs.Tab>
            <Tabs.Tab value="library">Libraries</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="netlist">
            <PreBlock text={netlist || "Generate SPICE netlist"} />
          </Tabs.Panel>
          <Tabs.Panel value="netlist-preview">
            <NetlistPreviewPanel preview={netlistPreview} loading={schematicInteractionLoading} />
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
