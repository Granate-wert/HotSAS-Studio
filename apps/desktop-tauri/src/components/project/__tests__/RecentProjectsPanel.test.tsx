import { render, screen, fireEvent } from "../../../test-utils";
import { describe, expect, it, vi } from "vitest";
import { RecentProjectsPanel } from "../RecentProjectsPanel";

describe("RecentProjectsPanel", () => {
  it("renders empty state", () => {
    render(
      <RecentProjectsPanel
        projects={[]}
        onOpen={vi.fn()}
        onRemove={vi.fn()}
        onClearMissing={vi.fn()}
      />,
    );
    expect(screen.getByText("No recent projects.")).toBeInTheDocument();
  });

  it("renders recent projects", () => {
    render(
      <RecentProjectsPanel
        projects={[
          {
            path: "/test.circuit",
            display_name: "test.circuit",
            last_opened_at: "2026-05-05T12:00:00Z",
            exists: true,
          },
        ]}
        onOpen={vi.fn()}
        onRemove={vi.fn()}
        onClearMissing={vi.fn()}
      />,
    );
    expect(screen.getByText("test.circuit")).toBeInTheDocument();
  });

  it("calls onRemove when remove clicked", () => {
    const onRemove = vi.fn();
    render(
      <RecentProjectsPanel
        projects={[
          {
            path: "/test.circuit",
            display_name: "test.circuit",
            last_opened_at: "2026-05-05T12:00:00Z",
            exists: true,
          },
        ]}
        onOpen={vi.fn()}
        onRemove={onRemove}
        onClearMissing={vi.fn()}
      />,
    );
    fireEvent.click(screen.getByTitle("Remove"));
    expect(onRemove).toHaveBeenCalledWith("/test.circuit");
  });
});
