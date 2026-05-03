import { screen, fireEvent, waitFor } from "@testing-library/react";
import { render } from "../../../test-utils";
import { SelectedRegionPanel } from "../SelectedRegionPanel";
import { useHotSasStore } from "../../../store";
import type { ProjectDto } from "../../../types";

vi.mock("../../../api", () => ({
  backend: {
    previewSelectedRegion: vi.fn((_componentIds: string[]) =>
      Promise.resolve({
        region: {
          id: "region-1",
          title: "Selected Region",
          component_ids: _componentIds,
          internal_nets: [],
          boundary_nets: [],
          input_port: null,
          output_port: null,
          reference_node: null,
          analysis_direction: "Custom",
          analysis_mode: "AllAvailable",
        },
        selected_components: [],
        detected_internal_nets: [],
        detected_boundary_nets: [],
        suggested_input_nets: [],
        suggested_output_nets: [],
        suggested_reference_nodes: [],
        warnings: [],
        errors: [],
      }),
    ),
    analyzeSelectedRegion: vi.fn((_request: unknown) =>
      Promise.resolve({
        region: {
          id: "region-1",
          title: "Selected Region",
          component_ids: [],
          internal_nets: [],
          boundary_nets: [],
          input_port: null,
          output_port: null,
          reference_node: null,
          analysis_direction: "Custom",
          analysis_mode: "AllAvailable",
        },
        status: "Partial",
        summary: "Mock analysis result",
        matched_template: null,
        equivalent_circuit: null,
        transfer_function: null,
        measurements: [],
        graph_specs: [],
        netlist_fragment: null,
        warnings: [],
        errors: [],
        report_section_markdown: null,
      }),
    ),
    validateSelectedRegion: vi.fn((_request: unknown) => Promise.resolve([])),
  },
}));

const mockProject: ProjectDto = {
  id: "demo",
  name: "Demo",
  format_version: "1.0",
  engine_version: "0.1.4",
  project_type: "circuit",
  schematic: {
    id: "sch1",
    title: "Schematic",
    components: [
      {
        instance_id: "R1",
        definition_id: "generic_resistor",
        component_kind: "Resistor",
        display_label: "R1",
        x: 0,
        y: 0,
        rotation_degrees: 0,
        parameters: [],
        symbol: null,
        pins: [],
        connected_nets: [],
      },
      {
        instance_id: "C1",
        definition_id: "generic_capacitor",
        component_kind: "Capacitor",
        display_label: "C1",
        x: 10,
        y: 0,
        rotation_degrees: 0,
        parameters: [],
        symbol: null,
        pins: [],
        connected_nets: [],
      },
    ],
    wires: [],
    nets: [],
  },
};

describe("SelectedRegionPanel", () => {
  beforeEach(() => {
    useHotSasStore.setState({
      selectedRegionComponentIds: [],
      selectedRegionPreview: null,
      selectedRegionAnalysisResult: null,
      busy: false,
      error: null,
    });
  });

  it("renders component checkboxes", () => {
    render(<SelectedRegionPanel project={mockProject} />);
    expect(screen.getByLabelText(/R1 \(Resistor\)/)).toBeInTheDocument();
    expect(screen.getByLabelText(/C1 \(Capacitor\)/)).toBeInTheDocument();
  });

  it("selecting components updates count", () => {
    render(<SelectedRegionPanel project={mockProject} />);
    fireEvent.click(screen.getByLabelText(/R1 \(Resistor\)/));
    expect(screen.getByText(/1 selected/)).toBeInTheDocument();
  });

  it("Preview button calls backend and shows preview card", async () => {
    render(<SelectedRegionPanel project={mockProject} />);
    fireEvent.click(screen.getByLabelText(/R1 \(Resistor\)/));
    fireEvent.click(screen.getByRole("button", { name: /Preview/i }));
    await waitFor(() => {
      expect(screen.getByText(/Preview/)).toBeInTheDocument();
    });
  });

  it("Analyze button calls backend and shows result card", async () => {
    render(<SelectedRegionPanel project={mockProject} />);
    fireEvent.click(screen.getByLabelText(/R1 \(Resistor\)/));
    fireEvent.click(screen.getByRole("button", { name: /Preview/i }));
    await waitFor(() => {
      expect(screen.getByText(/Preview/)).toBeInTheDocument();
    });
    fireEvent.click(screen.getByRole("button", { name: /Analyze/i }));
    await waitFor(() => {
      expect(screen.getByText(/Analysis Result/)).toBeInTheDocument();
    });
  });

  it("Clear button resets selection", () => {
    render(<SelectedRegionPanel project={mockProject} />);
    fireEvent.click(screen.getByLabelText(/R1 \(Resistor\)/));
    fireEvent.click(screen.getByRole("button", { name: /Clear/i }));
    expect(screen.getByText(/0 selected/)).toBeInTheDocument();
  });
});
