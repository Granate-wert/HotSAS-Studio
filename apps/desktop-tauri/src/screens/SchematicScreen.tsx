import { useEffect, useRef, useState, type ReactNode } from "react";
import { Button, Group, Paper, Stack, Tabs, Text, Tooltip, type ButtonProps } from "@mantine/core";
import {
  Cable,
  CheckCircle2,
  CircuitBoard,
  FileInput,
  FileText,
  FolderOpen,
  MousePointer2,
  Play,
  Plus,
  Save,
  SaveAll,
  Sigma,
  Trash2,
  Undo2,
  Redo2,
  Wrench,
} from "lucide-react";
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
import { PlaceableComponentPalette } from "../components/schematic-editor/PlaceableComponentPalette";
import { SchematicSelectionInspector } from "../components/schematic-editor/SchematicSelectionInspector";
import { NetlistPreviewPanel } from "../components/schematic-editor/NetlistPreviewPanel";
import { ErcIssuePanel } from "../components/schematic-editor/ErcIssuePanel";
import { UserCircuitSimulationPanel } from "../components/simulation/UserCircuitSimulationPanel";
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
  WireRoutePointDto,
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
  onValidateCurrentCircuit: () => void;
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
    route_points?: WireRoutePointDto[] | null;
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
  // demo / project creation
  onCreateDemoProject: () => void;
  onLoadProjectPackage: () => void;
  onSaveCurrentProject: () => void;
  onSaveProjectPackage: () => void;
  onCalculateCutoff: () => void;
  onSelectNearestE24: () => void;
  onRunMockAcSimulation: () => void;
};

function toolModeLabel(mode: "select" | "place" | "wire" | "delete") {
  return mode.charAt(0).toUpperCase() + mode.slice(1);
}

function disabledReason(hasProject: boolean, busy: boolean, fallback?: string) {
  if (!hasProject) return "Open or create a project first";
  if (busy) return "Wait for the current schematic operation to finish";
  return fallback;
}

type ReasonedButtonProps = ButtonProps & {
  children: ReactNode;
  reason?: string;
  testId?: string;
  disabled?: boolean;
  onClick?: () => void;
};

function ReasonedButton({ children, reason, disabled, testId, ...props }: ReasonedButtonProps) {
  return (
    <Tooltip label={disabled && reason ? reason : String(children)} disabled={!disabled || !reason}>
      <span>
        <Button
          {...props}
          disabled={disabled}
          title={disabled && reason ? reason : undefined}
          data-testid={testId}
        >
          {children}
        </Button>
      </span>
    </Tooltip>
  );
}

function ToolbarGroup({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="cad-toolbar-group">
      <Text size="xs" fw={700} c="dimmed">
        {label}
      </Text>
      <Group gap={4} wrap="nowrap">
        {children}
      </Group>
    </div>
  );
}

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
  onValidateCurrentCircuit,
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
  // demo
  onCreateDemoProject,
  onLoadProjectPackage,
  onSaveCurrentProject,
  onSaveProjectPackage,
  onCalculateCutoff,
  onSelectNearestE24,
  onRunMockAcSimulation,
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

  const selectedInstanceId =
    selectedSchematicEntity?.kind === "component"
      ? selectedSchematicEntity.id
      : (selectedComponent?.instance_id ?? null);
  const validationWarnings = validationReport?.warnings ?? [];
  const validationErrors = validationReport?.errors ?? [];
  const selectedCanvasComponent =
    project?.schematic.components.find(
      (component) => component.instance_id === selectedSchematicEntity?.id,
    ) ?? null;
  const operationBusy = schematicEditLoading || schematicInteractionLoading;
  const projectName = project?.name ?? "No project";
  const validationStatus = validationReport
    ? validationReport.valid
      ? "Valid"
      : `${validationErrors.length} errors / ${validationWarnings.length} warnings`
    : "Not run";

  const isEmptyProject = !project || project.schematic.components.length === 0;

  return (
    <div className="grid">
      <div className="schematic-topbar" data-testid="engineering-cad-toolbar">
        <div className="cad-workspace-title">
          <Text size="sm" fw={700}>
            Schematic Editor
          </Text>
          <Text size="xs" c="dimmed">
            Engineering CAD workspace
          </Text>
        </div>
        <ToolbarGroup label="Project">
          <ReasonedButton
            size="xs"
            leftSection={<CircuitBoard size={14} />}
            onClick={onCreateDemoProject}
          >
            New
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<FolderOpen size={14} />}
            onClick={onLoadProjectPackage}
          >
            Open
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<Save size={14} />}
            onClick={onSaveCurrentProject}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Save
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<SaveAll size={14} />}
            disabled
            reason="Use Project toolbar advanced path controls for Save As"
          >
            Save As
          </ReasonedButton>
        </ToolbarGroup>

        <ToolbarGroup label="Edit">
          <ReasonedButton
            size="xs"
            variant={schematicToolMode === "select" ? "filled" : "light"}
            leftSection={<MousePointer2 size={14} />}
            onClick={() => onSetSchematicToolMode("select")}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
            testId="cad-tool-select"
          >
            Select
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant={schematicToolMode === "place" ? "filled" : "light"}
            leftSection={<Plus size={14} />}
            onClick={() => onSetSchematicToolMode("place")}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Place
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant={schematicToolMode === "wire" ? "filled" : "light"}
            leftSection={<Cable size={14} />}
            onClick={() => onSetSchematicToolMode("wire")}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Wire
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            color="red"
            leftSection={<Trash2 size={14} />}
            onClick={() => {
              if (selectedInstanceId) onDeleteComponent(selectedInstanceId);
            }}
            disabled={!hasProject || operationBusy || !selectedInstanceId}
            reason={
              disabledReason(hasProject, operationBusy) ??
              (!selectedInstanceId ? "Select component first" : undefined)
            }
            testId="delete-selected-component"
          >
            Delete
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="subtle"
            onClick={() => setShowConnectionPanel((s) => !s)}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
            testId="connect-pins-button"
          >
            Connect
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="subtle"
            onClick={() => setShowNetEditor((s) => !s)}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
            testId="rename-net-button"
          >
            Rename Net
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<Undo2 size={14} />}
            onClick={onUndoSchematicEdit}
            disabled={!hasProject || operationBusy || !(undoRedoState?.can_undo ?? false)}
            reason={
              disabledReason(hasProject, operationBusy) ??
              (!(undoRedoState?.can_undo ?? false) ? "Nothing to undo" : undefined)
            }
          >
            Undo
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<Redo2 size={14} />}
            onClick={onRedoSchematicEdit}
            disabled={!hasProject || operationBusy || !(undoRedoState?.can_redo ?? false)}
            reason={
              disabledReason(hasProject, operationBusy) ??
              (!(undoRedoState?.can_redo ?? false) ? "Nothing to redo" : undefined)
            }
          >
            Redo
          </ReasonedButton>
        </ToolbarGroup>

        <ToolbarGroup label="Analysis">
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<CheckCircle2 size={14} />}
            onClick={onValidateCurrentCircuit}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Validate
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<FileText size={14} />}
            onClick={onGenerateCurrentSchematicNetlistPreview}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
            testId="cad-analysis-netlist"
          >
            Netlist
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<Play size={14} />}
            onClick={onRunMockAcSimulation}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Run AC
          </ReasonedButton>
        </ToolbarGroup>

        <ToolbarGroup label="Tools">
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<Wrench size={14} />}
            onClick={onCalculateCutoff}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Calculate fc
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<Sigma size={14} />}
            onClick={onSelectNearestE24}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Nearest E24
          </ReasonedButton>
        </ToolbarGroup>

        <ToolbarGroup label="Export">
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<SaveAll size={14} />}
            onClick={onSaveProjectPackage}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Save .circuit
          </ReasonedButton>
          <ReasonedButton
            size="xs"
            variant="light"
            leftSection={<FileText size={14} />}
            onClick={onMarkdown}
            disabled={!hasProject || operationBusy}
            reason={disabledReason(hasProject, operationBusy)}
          >
            Export report
          </ReasonedButton>
        </ToolbarGroup>

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
        {(schematicEditError || schematicInteractionError) && (
          <div style={{ padding: 8, background: "#ff444433", color: "#ffcccc" }}>
            {schematicEditError ?? schematicInteractionError}
          </div>
        )}
      </div>

      <aside className="schematic-left-panel" data-testid="schematic-left-panel">
        <Text size="xs" fw={700} c="dimmed" mb={8}>
          Components
        </Text>
        {hasProject ? (
          schematicToolMode === "place" ? (
            <PlaceableComponentPalette
              components={placeableComponents}
              onSelect={onSetPendingPlaceComponent}
              selected={pendingPlaceComponent}
              disabled={schematicInteractionLoading}
            />
          ) : (
            <ComponentPalette onAdd={onAddComponent} disabled={schematicEditLoading} />
          )
        ) : (
          <Text size="sm" c="dimmed">
            Create or open a project to place schematic components.
          </Text>
        )}
      </aside>

      <section className="schematic-panel" style={{ position: "relative" }}>
        {isEmptyProject ? (
          <div className="schematic-empty-state">
            <Paper p="xl" radius="md" bg="#121722" withBorder style={{ maxWidth: 420 }}>
              <Stack gap="md" align="center">
                <CircuitBoard size={48} color="#7db2ff" />
                <Text fw={700} size="lg">
                  Schematic Editor
                </Text>
                <Text size="sm" c="dimmed">
                  Create a new circuit project or load the RC low-pass demo to get started.
                </Text>
                <Stack gap="xs" w="100%">
                  <Button
                    leftSection={<Play size={16} />}
                    onClick={onCreateDemoProject}
                    disabled={schematicInteractionLoading}
                    fullWidth
                  >
                    New RC Demo
                  </Button>
                  <Button
                    leftSection={<FileInput size={16} />}
                    onClick={onLoadProjectPackage}
                    variant="light"
                    disabled={schematicInteractionLoading}
                    fullWidth
                  >
                    Open Project
                  </Button>
                </Stack>
                <Text size="xs" c="dimmed">
                  {
                    "Workflow: place components -> wire nets -> edit values -> validate -> netlist -> simulate"
                  }
                </Text>
              </Stack>
            </Paper>
          </div>
        ) : (
          <>
            <SchematicCanvas
              project={project}
              toolMode={schematicToolMode}
              pendingPlaceComponent={pendingPlaceComponent}
              onSelectComponent={(id) => {
                onSelectComponent(id);
                onSetSelectedSchematicEntity({ kind: "component", id });
                onGetSchematicSelectionDetails("component", id);
              }}
              onMoveComponent={onMoveComponent}
              onDeleteComponent={onDeleteComponent}
              onSelectWire={(id) => {
                onSetSelectedSchematicEntity({ kind: "wire", id });
                onGetSchematicSelectionDetails("wire", id);
              }}
              onDeleteWire={onDeleteSchematicWire}
              onConnect={(req) => onConnectPins({ ...req, net_name: null })}
              onPlaceSchematicComponent={onPlaceSchematicComponent}
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
          </>
        )}
      </section>

      <aside className="side-panel">
        <section className="inspector-shell">
          <SchematicSelectionInspector
            entity={selectedSchematicEntity}
            details={schematicSelectionDetails}
            selectedComponent={selectedCanvasComponent}
            nets={project?.schematic.nets ?? []}
            validationReport={validationReport}
            onDeleteComponent={onDeleteComponent}
            onDeleteWire={onDeleteSchematicWire}
            onUpdateParameter={onUpdateSchematicQuickParameter}
            loading={schematicInteractionLoading}
          />
        </section>
        <Tabs defaultValue="properties" className="tabs inspector-tabs">
          <Tabs.List>
            <Tabs.Tab value="properties">Properties</Tabs.Tab>
            <Tabs.Tab value="validation">Validation</Tabs.Tab>
            <Tabs.Tab value="metrics">Metrics</Tabs.Tab>
            <Tabs.Tab value="simulation">Simulation</Tabs.Tab>
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
          <Tabs.Panel value="simulation">
            <UserCircuitSimulationPanel />
          </Tabs.Panel>
          <Tabs.Panel value="region">
            <SelectedRegionPanel project={project} />
          </Tabs.Panel>
        </Tabs>
      </aside>

      <section className="engineering-status-bar" data-testid="engineering-status-bar">
        <Text size="xs">Tool: {toolModeLabel(schematicToolMode)}</Text>
        <Text size="xs">Grid: 20 px / Snap on</Text>
        <Text size="xs">Project: {projectName}</Text>
        <Text size="xs">
          Selected: {selectedSchematicEntity ? selectedSchematicEntity.id : "None"}
        </Text>
        <Text size="xs">Validation: {validationStatus}</Text>
      </section>

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
