import { render, screen } from "../../../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { InteractiveSchematicToolbar } from "../InteractiveSchematicToolbar";

describe("InteractiveSchematicToolbar", () => {
  it("renders all tool mode buttons", () => {
    render(<InteractiveSchematicToolbar toolMode="select" onSetToolMode={vi.fn()} />);
    expect(screen.getByText("Select")).toBeInTheDocument();
    expect(screen.getByText("Place")).toBeInTheDocument();
    expect(screen.getByText("Wire")).toBeInTheDocument();
    expect(screen.getByText("Delete")).toBeInTheDocument();
  });

  it("calls onSetToolMode when a button is clicked", async () => {
    const user = userEvent.setup();
    const onSetToolMode = vi.fn();
    render(<InteractiveSchematicToolbar toolMode="select" onSetToolMode={onSetToolMode} />);
    await user.click(screen.getByText("Place"));
    expect(onSetToolMode).toHaveBeenCalledWith("place");
  });

  it("disables buttons when disabled prop is true", () => {
    render(<InteractiveSchematicToolbar toolMode="select" onSetToolMode={vi.fn()} disabled />);
    expect(screen.getByRole("button", { name: "Select" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Place" })).toBeDisabled();
  });
});
