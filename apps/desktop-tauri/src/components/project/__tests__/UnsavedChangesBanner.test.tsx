import { render, screen } from "../../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { UnsavedChangesBanner } from "../UnsavedChangesBanner";

describe("UnsavedChangesBanner", () => {
  it("renders nothing when not dirty", () => {
    render(
      <UnsavedChangesBanner
        session={{
          current_project_id: "1",
          current_project_name: "Test",
          current_project_path: "/test.circuit",
          dirty: false,
          last_saved_at: null,
          last_loaded_at: null,
          last_error: null,
        }}
        onSave={vi.fn()}
        onSaveAs={vi.fn()}
      />,
    );
    expect(screen.queryByText(/Project .* has unsaved changes/)).not.toBeInTheDocument();
  });

  it("renders banner when dirty", () => {
    render(
      <UnsavedChangesBanner
        session={{
          current_project_id: "1",
          current_project_name: "Test",
          current_project_path: "/test.circuit",
          dirty: true,
          last_saved_at: null,
          last_loaded_at: null,
          last_error: null,
        }}
        onSave={vi.fn()}
        onSaveAs={vi.fn()}
      />,
    );
    expect(screen.getByText("Unsaved changes")).toBeInTheDocument();
    expect(screen.getByText(/Project "Test" has unsaved changes/)).toBeInTheDocument();
  });
});
