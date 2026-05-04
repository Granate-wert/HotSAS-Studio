import { screen, fireEvent } from "@testing-library/react";
import { render } from "../../test-utils";
import { describe, it, expect, vi } from "vitest";
import { DiagnosticsScreen } from "../DiagnosticsScreen";
import type { AppDiagnosticsReportDto } from "../../types";

function makeReport(overrides?: Partial<AppDiagnosticsReportDto>): AppDiagnosticsReportDto {
  return {
    app_name: "HotSAS Studio",
    app_version: "1.10.0",
    roadmap_stage: "v1.10 internal alpha",
    build_profile: "release",
    modules: [
      {
        id: "formula_registry",
        title: "Formula Registry",
        status: "ready",
        summary: "Engine ready",
        details: {},
      },
      {
        id: "component_library",
        title: "Component Library",
        status: "ready",
        summary: "15 components",
        details: {},
      },
    ],
    checks: [
      {
        id: "formula_calculation",
        title: "Formula calculation smoke",
        status: "pass",
        message: "OK",
      },
    ],
    warnings: [],
    ...overrides,
  };
}

describe("DiagnosticsScreen", () => {
  it("renders diagnostics title", () => {
    render(
      <DiagnosticsScreen
        diagnostics={makeReport()}
        readinessResult={null}
        loading={false}
        error={null}
        onRefreshDiagnostics={vi.fn()}
        onRunSelfCheck={vi.fn()}
      />,
    );
    expect(screen.getByText(/Internal Alpha \/ Diagnostics/i)).toBeInTheDocument();
  });

  it("loads module cards", () => {
    render(
      <DiagnosticsScreen
        diagnostics={makeReport()}
        readinessResult={null}
        loading={false}
        error={null}
        onRefreshDiagnostics={vi.fn()}
        onRunSelfCheck={vi.fn()}
      />,
    );
    expect(screen.getByText("Formula Registry")).toBeInTheDocument();
    expect(screen.getByText("Component Library")).toBeInTheDocument();
  });

  it("shows Ready and Limited statuses", () => {
    render(
      <DiagnosticsScreen
        diagnostics={makeReport({
          modules: [
            { id: "a", title: "A", status: "ready", summary: "ok", details: {} },
            { id: "b", title: "B", status: "limited", summary: "ok", details: {} },
            { id: "c", title: "C", status: "unavailable", summary: "ok", details: {} },
          ],
        })}
        readinessResult={null}
        loading={false}
        error={null}
        onRefreshDiagnostics={vi.fn()}
        onRunSelfCheck={vi.fn()}
      />,
    );
    expect(screen.getByText("ready")).toBeInTheDocument();
    expect(screen.getByText("limited")).toBeInTheDocument();
    expect(screen.getByText("unavailable")).toBeInTheDocument();
  });

  it("Refresh diagnostics calls backend API", () => {
    const onRefresh = vi.fn();
    render(
      <DiagnosticsScreen
        diagnostics={null}
        readinessResult={null}
        loading={false}
        error={null}
        onRefreshDiagnostics={onRefresh}
        onRunSelfCheck={vi.fn()}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: /Refresh diagnostics/i }));
    expect(onRefresh).toHaveBeenCalledTimes(1);
  });

  it("Run readiness self-check calls backend API", () => {
    const onSelfCheck = vi.fn();
    render(
      <DiagnosticsScreen
        diagnostics={null}
        readinessResult={null}
        loading={false}
        error={null}
        onRefreshDiagnostics={vi.fn()}
        onRunSelfCheck={onSelfCheck}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: /Run readiness self-check/i }));
    expect(onSelfCheck).toHaveBeenCalledTimes(1);
  });

  it("shows backend error message if command fails", () => {
    render(
      <DiagnosticsScreen
        diagnostics={null}
        readinessResult={null}
        loading={false}
        error="Network error"
        onRefreshDiagnostics={vi.fn()}
        onRunSelfCheck={vi.fn()}
      />,
    );
    expect(screen.getByText("Network error")).toBeInTheDocument();
  });

  it("does not automatically run heavy self-check on first render", () => {
    const onSelfCheck = vi.fn();
    render(
      <DiagnosticsScreen
        diagnostics={null}
        readinessResult={null}
        loading={false}
        error={null}
        onRefreshDiagnostics={vi.fn()}
        onRunSelfCheck={onSelfCheck}
      />,
    );
    expect(onSelfCheck).not.toHaveBeenCalled();
  });
});
