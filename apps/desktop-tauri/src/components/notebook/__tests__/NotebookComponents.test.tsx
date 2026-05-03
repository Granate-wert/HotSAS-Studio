import { screen, fireEvent } from "@testing-library/react";
import { render } from "../../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { NotebookInput } from "../NotebookInput";
import { NotebookResultCard } from "../NotebookResultCard";
import { NotebookVariableTable } from "../NotebookVariableTable";
import { NotebookHistory } from "../NotebookHistory";
import { PreferredValueQuickTools } from "../PreferredValueQuickTools";
import { ApplyNotebookOutputPanel } from "../ApplyNotebookOutputPanel";

const mockValue = { original: "10k", si_value: 10000.0, unit: "Ohm", display: "10 kΩ" };

const mockResult = {
  input: "R = 10k",
  status: "Success",
  kind: "Assignment",
  outputs: [{ name: "R", value: mockValue }],
  variables: [{ name: "R", value: mockValue }],
  message: null,
  warnings: [],
};

const mockUnsupportedResult = {
  input: "sin(5)",
  status: "unsupported",
  kind: "Text",
  outputs: [],
  variables: [],
  message: "Unsupported notebook expression in v1.4",
  warnings: [],
};

const mockState = {
  variables: [{ name: "R", value: mockValue }],
  history: [
    {
      id: "hist-0",
      input: "R = 10k",
      result_summary: "R=10k",
      status: "Success",
    },
  ],
};

const mockComponent = {
  instance_id: "R1",
  component_kind: "resistor",
  title: "Resistor",
  parameters: [{ name: "resistance", value: "10k", unit: "Ohm" }],
  symbol: null,
};

describe("NotebookInput", () => {
  it("renders placeholder and buttons", () => {
    render(
      <NotebookInput
        input=""
        onChange={() => {}}
        onEvaluate={() => {}}
        onClear={() => {}}
        loading={false}
      />,
    );
    expect(screen.getByPlaceholderText(/R = 10k/i)).toBeInTheDocument();
    expect(screen.getByText("Evaluate")).toBeInTheDocument();
    expect(screen.getByText("Clear")).toBeInTheDocument();
  });

  it("calls onEvaluate when Evaluate clicked", () => {
    const onEvaluate = vi.fn();
    render(
      <NotebookInput
        input="test"
        onChange={() => {}}
        onEvaluate={onEvaluate}
        onClear={() => {}}
        loading={false}
      />,
    );
    fireEvent.click(screen.getByText("Evaluate"));
    expect(onEvaluate).toHaveBeenCalled();
  });

  it("calls onClear when Clear clicked", () => {
    const onClear = vi.fn();
    render(
      <NotebookInput
        input=""
        onChange={() => {}}
        onEvaluate={() => {}}
        onClear={onClear}
        loading={false}
      />,
    );
    fireEvent.click(screen.getByText("Clear"));
    expect(onClear).toHaveBeenCalled();
  });
});

describe("NotebookResultCard", () => {
  it("displays output for successful result", () => {
    render(<NotebookResultCard result={mockResult} />);
    expect(screen.getByText("R = 10k")).toBeInTheDocument();
    expect(screen.getByText("Success")).toBeInTheDocument();
    expect(screen.getByText("R")).toBeInTheDocument();
    expect(screen.getByText("10k")).toBeInTheDocument();
  });

  it("displays unsupported hint for unsupported input", () => {
    render(<NotebookResultCard result={mockUnsupportedResult} />);
    expect(screen.getByText(/v1.4 supports assignments/i)).toBeInTheDocument();
  });
});

describe("NotebookVariableTable", () => {
  it("displays variables", () => {
    render(<NotebookVariableTable state={mockState} />);
    expect(screen.getByText("Variables")).toBeInTheDocument();
    expect(screen.getByText("R")).toBeInTheDocument();
    expect(screen.getByText("10k")).toBeInTheDocument();
  });

  it("returns null when no variables", () => {
    render(<NotebookVariableTable state={{ variables: [], history: [] }} />);
    expect(screen.queryByText("Variables")).not.toBeInTheDocument();
  });
});

describe("NotebookHistory", () => {
  it("displays history entries", () => {
    render(<NotebookHistory state={mockState} />);
    expect(screen.getByText("History")).toBeInTheDocument();
    expect(screen.getByText("R = 10k")).toBeInTheDocument();
  });

  it("returns null when no history", () => {
    render(<NotebookHistory state={{ variables: [], history: [] }} />);
    expect(screen.queryByText("History")).not.toBeInTheDocument();
  });
});

describe("PreferredValueQuickTools", () => {
  it("renders quick tool buttons", () => {
    render(<PreferredValueQuickTools onInsert={() => {}} />);
    expect(screen.getByText("nearestE24")).toBeInTheDocument();
    expect(screen.getByText("nearestE96")).toBeInTheDocument();
    expect(screen.getByText("lowerE96")).toBeInTheDocument();
    expect(screen.getByText("higherE96")).toBeInTheDocument();
  });

  it("calls onInsert with template when clicked", () => {
    const onInsert = vi.fn();
    render(<PreferredValueQuickTools onInsert={onInsert} />);
    fireEvent.click(screen.getByText("nearestE24"));
    expect(onInsert).toHaveBeenCalledWith("nearestE(15.93k, E24, Ohm)");
  });
});

describe("ApplyNotebookOutputPanel", () => {
  it("renders apply buttons when component selected", () => {
    const onApply = vi.fn();
    render(
      <ApplyNotebookOutputPanel
        result={mockResult}
        selectedComponent={mockComponent}
        onApply={onApply}
      />,
    );
    expect(screen.getByText(/Apply to R1/i)).toBeInTheDocument();
    fireEvent.click(screen.getByText("R"));
    expect(onApply).toHaveBeenCalledWith("R");
  });

  it("returns null when no component selected", () => {
    render(
      <ApplyNotebookOutputPanel result={mockResult} selectedComponent={null} onApply={() => {}} />,
    );
    expect(screen.queryByText(/Apply to/i)).not.toBeInTheDocument();
  });
});
