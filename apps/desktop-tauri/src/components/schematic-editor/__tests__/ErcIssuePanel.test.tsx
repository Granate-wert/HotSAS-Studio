import { render, screen } from "../../../test-utils";
import { describe, expect, it } from "vitest";
import { ErcIssuePanel } from "../ErcIssuePanel";

describe("ErcIssuePanel", () => {
  it("shows no issues message", () => {
    render(<ErcIssuePanel errors={[]} warnings={[]} />);
    expect(screen.getByText("No ERC issues")).toBeInTheDocument();
  });

  it("renders errors", () => {
    render(
      <ErcIssuePanel
        errors={[{ code: "E1", message: "No ground", component_id: null, net_id: null }]}
        warnings={[]}
      />,
    );
    expect(screen.getByText("[E] E1: No ground")).toBeInTheDocument();
  });

  it("renders warnings", () => {
    render(
      <ErcIssuePanel
        errors={[]}
        warnings={[{ code: "W1", message: "Floating net", component_id: "R1", net_id: "n1" }]}
      />,
    );
    expect(screen.getByText("[W] W1: Floating net (comp: R1) (net: n1)")).toBeInTheDocument();
  });
});
