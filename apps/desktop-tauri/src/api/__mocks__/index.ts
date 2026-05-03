import type {
  FormulaCalculationRequestDto,
  FormulaDetailsDto,
  FormulaEvaluationResultDto,
  FormulaPackDto,
  FormulaSummaryDto,
} from "../../types";

const mockPacks: FormulaPackDto[] = [
  {
    pack_id: "filters",
    title: "Filters",
    version: "0.1.0",
    formula_count: 1,
    categories: ["filters/passive"],
  },
];

const mockFormulas: FormulaSummaryDto[] = [
  {
    id: "rc_low_pass_cutoff",
    title: "RC Low-Pass Cutoff Frequency",
    category: "filters/passive",
    description: "Cutoff frequency of a first-order RC low-pass filter.",
    linked_circuit_template_id: "rc_low_pass_template",
  },
  {
    id: "ohms_law",
    title: "Ohm's Law",
    category: "basic/dc",
    description: "Relation between voltage, current, and resistance.",
    linked_circuit_template_id: null,
  },
];

const mockDetails: Record<string, FormulaDetailsDto> = {
  rc_low_pass_cutoff: {
    id: "rc_low_pass_cutoff",
    title: "RC Low-Pass Cutoff Frequency",
    category: "filters/passive",
    description: "Cutoff frequency of a first-order RC low-pass filter.",
    variables: [
      {
        name: "R",
        unit: "Ohm",
        description: "Resistance",
        default: {
          original: "10k",
          si_value: 10000,
          unit: "Ohm",
          display: "10000.000000 Ohm",
        },
      },
      {
        name: "C",
        unit: "F",
        description: "Capacitance",
        default: {
          original: "100n",
          si_value: 1e-7,
          unit: "F",
          display: "0.000000 F",
        },
      },
    ],
    equations: [
      {
        id: "cutoff",
        latex: "f_c = \\frac{1}{2\\pi R C}",
        expression: "fc = 1 / (2*pi*R*C)",
        solve_for: ["fc", "R", "C"],
      },
    ],
    outputs: [
      {
        name: "fc",
        unit: "Hz",
        description: "Cutoff frequency",
      },
    ],
    linked_circuit_template_id: "rc_low_pass_template",
    mapping: {
      R: "R1.resistance",
      C: "C1.capacitance",
    },
    default_simulation: "ac_sweep",
  },
  ohms_law: {
    id: "ohms_law",
    title: "Ohm's Law",
    category: "basic/dc",
    description: "Relation between voltage, current, and resistance.",
    variables: [
      {
        name: "I",
        unit: "A",
        description: "Current",
        default: null,
      },
      {
        name: "R",
        unit: "Ohm",
        description: "Resistance",
        default: {
          original: "10k",
          si_value: 10000,
          unit: "Ohm",
          display: "10000.000000 Ohm",
        },
      },
    ],
    equations: [
      {
        id: "ohms_law",
        latex: "V = I R",
        expression: "V = I * R",
        solve_for: ["V", "I", "R"],
      },
    ],
    outputs: [
      {
        name: "V",
        unit: "V",
        description: "Voltage",
      },
    ],
    linked_circuit_template_id: null,
    mapping: null,
    default_simulation: null,
  },
};

export const backend = {
  loadFormulaPacks: () => Promise.resolve(mockPacks),
  listFormulas: () => Promise.resolve(mockFormulas),
  listFormulaCategories: () => Promise.resolve(["filters/passive", "basic/dc"]),
  getFormula: (id: string) => {
    const details = mockDetails[id];
    if (!details) {
      return Promise.reject(new Error(`Formula not found: ${id}`));
    }
    return Promise.resolve(details);
  },
  getFormulaPackMetadata: () => Promise.resolve(mockPacks),
  calculateFormula: (_request: FormulaCalculationRequestDto) => {
    return Promise.resolve<FormulaEvaluationResultDto>({
      formula_id: "rc_low_pass_cutoff",
      equation_id: "cutoff",
      expression: "fc = 1 / (2*pi*R*C)",
      outputs: [
        {
          name: "fc",
          value: {
            original: "159.154943",
            si_value: 159.154943,
            unit: "Hz",
            display: "159.154943 Hz",
          },
        },
      ],
      warnings: [],
    });
  },
  previewSelectedRegion: (_componentIds: string[]) =>
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
  analyzeSelectedRegion: (_request: unknown) =>
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
  validateSelectedRegion: (_request: unknown) => Promise.resolve([]),
  listExportCapabilities: () =>
    Promise.resolve([
      {
        format: "markdown_report",
        label: "Markdown Report",
        description: "",
        file_extension: "md",
        available: true,
      },
      {
        format: "spice_netlist",
        label: "SPICE Netlist",
        description: "",
        file_extension: "cir",
        available: true,
      },
    ]),
  exportFile: (_request: unknown) =>
    Promise.resolve({
      format: "markdown_report",
      content: "# Report",
      file_path: null,
      success: true,
      message: "Exported",
    }),
  exportHistory: () => Promise.resolve([]),
  importSpiceModel: (_request: unknown) =>
    Promise.resolve({
      status: "Parsed",
      models: [],
      subcircuits: [],
      warnings: [],
      errors: [],
    }),
  importTouchstoneModel: (_request: unknown) =>
    Promise.resolve({
      status: "Parsed",
      summary: null,
      warnings: [],
      errors: [],
    }),
  listImportedModels: () => Promise.resolve([]),
  getImportedModel: (_modelId: string) =>
    Promise.resolve({
      id: "mock-model",
      kind: "Unknown",
      name: "Mock Model",
      source_format: "spice",
      spice_model: null,
      spice_subcircuit: null,
      touchstone_summary: null,
    }),
  validateSpicePinMapping: (_request: unknown) =>
    Promise.resolve({
      valid: true,
      warnings: [],
      errors: [],
    }),
  attachImportedModelToComponent: (_request: unknown) =>
    Promise.resolve({
      id: "mock-component",
      name: "Mock",
      category: "test",
      description: "",
      parameters: [],
      ratings: [],
      symbol: null,
      footprint: null,
      simulation_model: null,
    }),
  writeLog: (_level: string, _message: string) => Promise.resolve(),
};
