import { screen, waitFor } from "@testing-library/react";
import { render } from "../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { FormulaLibraryScreen } from "./FormulaLibraryScreen";

const mockPacks = [
  {
    pack_id: "filters",
    title: "Filters",
    version: "0.1.0",
    formula_count: 1,
    categories: ["filters/passive"],
  },
];

const mockFormulas = [
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

const mockDetails: Record<string, unknown> = {
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

vi.mock("../api", () => ({
  backend: {
    loadFormulaPacks: vi.fn(() => Promise.resolve(mockPacks)),
    listFormulas: vi.fn(() => Promise.resolve(mockFormulas)),
    listFormulaCategories: vi.fn(() => Promise.resolve(["filters/passive", "basic/dc"])),
    getFormula: vi.fn((id: string) => {
      const details = mockDetails[id];
      if (!details) {
        return Promise.reject(new Error(`Formula not found: ${id}`));
      }
      return Promise.resolve(details);
    }),
    getFormulaPackMetadata: vi.fn(() => Promise.resolve(mockPacks)),
    calculateFormula: vi.fn(() =>
      Promise.resolve({
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
      }),
    ),
    writeLog: vi.fn(() => Promise.resolve()),
  },
}));

describe("FormulaLibraryScreen", () => {
  it("renders loading state and then displays packs, categories and formulas", async () => {
    render(<FormulaLibraryScreen />);

    expect(screen.getByText("Formula Library")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /reload/i })).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("filters/passive")).toBeInTheDocument();
    });

    expect(
      screen.getByRole("button", { name: "RC Low-Pass Cutoff Frequency" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Ohm's Law" })).toBeInTheDocument();
  });

  it("shows formula details when a formula is selected", async () => {
    const user = userEvent.setup();
    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("RC Low-Pass Cutoff Frequency")).toBeInTheDocument();
    });

    const rcButton = screen.getByRole("button", { name: "RC Low-Pass Cutoff Frequency" });
    await user.click(rcButton);

    await waitFor(() => {
      expect(screen.getByLabelText("R value")).toBeInTheDocument();
    });

    expect(screen.getByLabelText("R value")).toHaveValue("10k");
    expect(screen.getByLabelText("C value")).toHaveValue("100n");
  });

  it("allows changing variable inputs without crashing", async () => {
    const user = userEvent.setup();
    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("RC Low-Pass Cutoff Frequency")).toBeInTheDocument();
    });

    const rcButton = screen.getByRole("button", { name: "RC Low-Pass Cutoff Frequency" });
    await user.click(rcButton);

    await waitFor(() => {
      expect(screen.getByLabelText("R value")).toBeInTheDocument();
    });

    const rInput = screen.getByLabelText("R value");
    await user.clear(rInput);
    await user.type(rInput, "4.7k");

    expect(rInput).toHaveValue("4.7k");

    const cInput = screen.getByLabelText("C value");
    await user.clear(cInput);
    await user.type(cInput, "220n");

    expect(cInput).toHaveValue("220n");
  });

  it("calls calculateFormula and displays result when Calculate is clicked", async () => {
    const user = userEvent.setup();
    const { backend } = await import("../api");
    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("RC Low-Pass Cutoff Frequency")).toBeInTheDocument();
    });

    const rcButton = screen.getByRole("button", { name: "RC Low-Pass Cutoff Frequency" });
    await user.click(rcButton);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: /calculate/i })).toBeInTheDocument();
    });

    const calculateButton = screen.getByRole("button", { name: /calculate/i });
    await user.click(calculateButton);

    await waitFor(() => {
      expect(screen.getByText(/result/i)).toBeInTheDocument();
    });

    expect(screen.getByText(/159.154943 Hz/)).toBeInTheDocument();
    expect(backend.calculateFormula).toHaveBeenCalled();
  });

  it("switches between formulas and updates details", async () => {
    const user = userEvent.setup();
    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("Ohm's Law")).toBeInTheDocument();
    });

    const ohmsButton = screen.getByRole("button", { name: "Ohm's Law" });
    await user.click(ohmsButton);

    await waitFor(() => {
      expect(screen.getByText(/relation between voltage/i)).toBeInTheDocument();
    });

    expect(screen.getByLabelText("I value")).toBeInTheDocument();
    expect(screen.getByLabelText("R value")).toBeInTheDocument();
  });

  it("displays an error alert when backend load fails", async () => {
    const { backend } = await import("../api");
    vi.mocked(backend.loadFormulaPacks).mockRejectedValueOnce(new Error("Network error"));

    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText(/network error/i)).toBeInTheDocument();
    });
  });

  it("handles formulas with null defaults gracefully", async () => {
    const user = userEvent.setup();
    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("Ohm's Law")).toBeInTheDocument();
    });

    const ohmsButton = screen.getByRole("button", { name: "Ohm's Law" });
    await user.click(ohmsButton);

    await waitFor(() => {
      expect(screen.getByLabelText("I value")).toBeInTheDocument();
    });

    const iInput = screen.getByLabelText("I value");
    expect(iInput).toHaveValue("");

    await user.type(iInput, "10m");
    expect(iInput).toHaveValue("10m");
  });

  it("handles malformed calculation result without crashing", async () => {
    const user = userEvent.setup();
    const { backend } = await import("../api");
    vi.mocked(backend.calculateFormula).mockResolvedValueOnce({
      formula_id: "rc_low_pass_cutoff",
      equation_id: "cutoff",
      expression: "fc = 1 / (2*pi*R*C)",
      outputs: [],
      warnings: ["Mock warning"],
    } as unknown as ReturnType<typeof backend.calculateFormula> extends Promise<infer T>
      ? T
      : never);

    render(<FormulaLibraryScreen />);

    await waitFor(() => {
      expect(screen.getByText("RC Low-Pass Cutoff Frequency")).toBeInTheDocument();
    });

    const rcButton = screen.getByRole("button", { name: "RC Low-Pass Cutoff Frequency" });
    await user.click(rcButton);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: /calculate/i })).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: /calculate/i }));

    await waitFor(() => {
      expect(screen.getByText(/mock warning/i)).toBeInTheDocument();
    });
  });
});
