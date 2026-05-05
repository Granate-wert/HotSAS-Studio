import { render, screen, fireEvent } from "../../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { ProjectToolbar } from "../ProjectToolbar";

describe("ProjectToolbar", () => {
  it("renders no project state", () => {
    render(
      <ProjectToolbar
        session={null}
        onNewDemo={vi.fn()}
        onOpen={vi.fn()}
        onSave={vi.fn()}
        onSaveAs={vi.fn()}
      />,
    );
    expect(screen.getByText("No project")).toBeInTheDocument();
  });

  it("shows dirty indicator when dirty", () => {
    render(
      <ProjectToolbar
        session={{
          current_project_id: "1",
          current_project_name: "Test",
          current_project_path: "/test.circuit",
          dirty: true,
          last_saved_at: null,
          last_loaded_at: null,
          last_error: null,
        }}
        onNewDemo={vi.fn()}
        onOpen={vi.fn()}
        onSave={vi.fn()}
        onSaveAs={vi.fn()}
      />,
    );
    expect(screen.getByText("Unsaved changes")).toBeInTheDocument();
  });

  it("calls onSave when Save clicked", () => {
    const onSave = vi.fn();
    render(
      <ProjectToolbar
        session={{
          current_project_id: "1",
          current_project_name: "Test",
          current_project_path: "/test.circuit",
          dirty: true,
          last_saved_at: null,
          last_loaded_at: null,
          last_error: null,
        }}
        onNewDemo={vi.fn()}
        onOpen={vi.fn()}
        onSave={onSave}
        onSaveAs={vi.fn()}
      />,
    );
    fireEvent.click(screen.getByText("Save"));
    expect(onSave).toHaveBeenCalled();
  });
});
