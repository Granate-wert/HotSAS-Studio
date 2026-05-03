import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { render } from "../../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { CircuitValidationPanel } from "../CircuitValidationPanel";

vi.mock("../../../api", () => ({
  backend: {
    validateCurrentCircuit: vi.fn(() =>
      Promise.resolve({
        valid: false,
        warnings: [
          {
            code: "floating_net",
            message: "Net 'x' has only 1 connected pin(s).",
            component_id: null,
            net_id: "x",
          },
        ],
        errors: [
          {
            code: "missing_ground",
            message: "No ground net.",
            component_id: null,
            net_id: null,
          },
        ],
      }),
    ),
  },
}));

describe("CircuitValidationPanel", () => {
  it("renders validate button", () => {
    render(<CircuitValidationPanel report={null} onValidate={() => {}} />);
    expect(screen.getByRole("button", { name: /validate circuit/i })).toBeInTheDocument();
  });

  it("calls onValidate after validate click", async () => {
    const user = userEvent.setup();
    const onValidate = vi.fn();
    render(<CircuitValidationPanel report={null} onValidate={onValidate} />);

    await user.click(screen.getByRole("button", { name: /validate circuit/i }));

    await screen.findByText(/validate circuit/i);
    expect(onValidate).toHaveBeenCalledTimes(1);
    const report = onValidate.mock.calls[0][0];
    expect(report.valid).toBe(false);
    expect(report.errors[0].code).toBe("missing_ground");
    expect(report.warnings[0].code).toBe("floating_net");
  });
});
