import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { MantineProvider } from "@mantine/core";
import { ProbeManager } from "../ProbeManager";

function renderWithMantine(ui: React.ReactNode) {
  return render(<MantineProvider>{ui}</MantineProvider>);
}

const mockProbes = [
  {
    id: "p1",
    label: "V(in)",
    kind: "NodeVoltage",
    unit: "V",
    target: {
      net_id: "in",
      component_id: null,
      pin_id: null,
      positive_net_id: null,
      negative_net_id: null,
    },
  },
  {
    id: "p2",
    label: "V(out)",
    kind: "NodeVoltage",
    unit: "V",
    target: {
      net_id: "out",
      component_id: null,
      pin_id: null,
      positive_net_id: null,
      negative_net_id: null,
    },
  },
  {
    id: "p3",
    label: "I(R1)",
    kind: "ComponentCurrent",
    unit: "A",
    target: {
      net_id: null,
      component_id: "R1",
      pin_id: null,
      positive_net_id: null,
      negative_net_id: null,
    },
  },
];

describe("ProbeManager", () => {
  it("renders probe list", () => {
    renderWithMantine(<ProbeManager probes={mockProbes} selected={[]} onChange={vi.fn()} />);
    expect(screen.getByText(/V\(in\)/)).toBeInTheDocument();
    expect(screen.getByText(/V\(out\)/)).toBeInTheDocument();
    expect(screen.getByText(/I\(R1\)/)).toBeInTheDocument();
  });

  it("shows unsupported probe as disabled", () => {
    renderWithMantine(<ProbeManager probes={mockProbes} selected={[]} onChange={vi.fn()} />);
    const checkbox = screen.getByLabelText(/I\(R1\)/);
    expect(checkbox).toBeDisabled();
  });

  it("calls onChange when probe toggled", () => {
    const onChange = vi.fn();
    renderWithMantine(<ProbeManager probes={mockProbes} selected={[]} onChange={onChange} />);
    fireEvent.click(screen.getByLabelText(/V\(in\)/));
    expect(onChange).toHaveBeenCalledWith([mockProbes[0]]);
  });

  it("calls onSetDefaults when defaults button clicked", () => {
    const onSetDefaults = vi.fn();
    renderWithMantine(
      <ProbeManager
        probes={mockProbes}
        selected={[]}
        onChange={vi.fn()}
        onSetDefaults={onSetDefaults}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: /defaults/i }));
    expect(onSetDefaults).toHaveBeenCalled();
  });
});
