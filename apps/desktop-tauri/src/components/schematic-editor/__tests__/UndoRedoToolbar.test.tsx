import { render, screen } from "../../../test-utils";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { UndoRedoToolbar } from "../UndoRedoToolbar";

describe("UndoRedoToolbar", () => {
  it("renders undo and redo buttons", () => {
    render(
      <UndoRedoToolbar
        canUndo
        canRedo
        lastActionLabel="Add R1"
        nextRedoLabel={null}
        onUndo={vi.fn()}
        onRedo={vi.fn()}
      />,
    );
    expect(screen.getByText("Undo")).toBeInTheDocument();
    expect(screen.getByText("Redo")).toBeInTheDocument();
    expect(screen.getByText("Last: Add R1")).toBeInTheDocument();
  });

  it("calls onUndo when undo button clicked", async () => {
    const user = userEvent.setup();
    const onUndo = vi.fn();
    render(
      <UndoRedoToolbar
        canUndo
        canRedo
        lastActionLabel={null}
        nextRedoLabel={null}
        onUndo={onUndo}
        onRedo={vi.fn()}
      />,
    );
    await user.click(screen.getByText("Undo"));
    expect(onUndo).toHaveBeenCalled();
  });

  it("calls onRedo when redo button clicked", async () => {
    const user = userEvent.setup();
    const onRedo = vi.fn();
    render(
      <UndoRedoToolbar
        canUndo
        canRedo
        lastActionLabel={null}
        nextRedoLabel={null}
        onUndo={vi.fn()}
        onRedo={onRedo}
      />,
    );
    await user.click(screen.getByText("Redo"));
    expect(onRedo).toHaveBeenCalled();
  });

  it("disables undo when canUndo is false", () => {
    render(
      <UndoRedoToolbar
        canUndo={false}
        canRedo
        lastActionLabel={null}
        nextRedoLabel={null}
        onUndo={vi.fn()}
        onRedo={vi.fn()}
      />,
    );
    expect(screen.getByRole("button", { name: "Undo" })).toBeDisabled();
  });

  it("disables redo when canRedo is false", () => {
    render(
      <UndoRedoToolbar
        canUndo
        canRedo={false}
        lastActionLabel={null}
        nextRedoLabel={null}
        onUndo={vi.fn()}
        onRedo={vi.fn()}
      />,
    );
    expect(screen.getByRole("button", { name: "Redo" })).toBeDisabled();
  });
});
