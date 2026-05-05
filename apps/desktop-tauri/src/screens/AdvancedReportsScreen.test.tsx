import { describe, it, expect, vi } from "vitest";
import { screen, fireEvent, waitFor } from "@testing-library/react";
import { render } from "../test-utils";
import { AdvancedReportsScreen } from "./AdvancedReportsScreen";
import type {
  AdvancedReportDto,
  ReportSectionCapabilityDto,
} from "../types";

const mockCapabilities: ReportSectionCapabilityDto[] = [
  {
    kind: "ProjectInfo",
    title: "Project Information",
    description: "Basic project metadata",
    default_enabled: true,
    supported_report_types: ["ProjectSummary", "FullProjectReport"],
  },
  {
    kind: "SchematicSummary",
    title: "Schematic Summary",
    description: "Component and net overview",
    default_enabled: true,
    supported_report_types: ["ProjectSummary", "FullProjectReport"],
  },
  {
    kind: "FormulaCalculations",
    title: "Formula Calculations",
    description: "Engineering notebook results",
    default_enabled: false,
    supported_report_types: ["CalculationReport", "FullProjectReport"],
  },
];

const mockReport: AdvancedReportDto = {
  id: "report-123",
  title: "Test Report",
  report_type: "ProjectSummary",
  generated_at: "2026-05-05T00:00:00Z",
  project_id: "proj-1",
  project_name: "Demo Project",
  sections: [
    {
      kind: "ProjectInfo",
      title: "Project Information",
      status: "Included",
      blocks: [
        {
          block_type: "Paragraph",
          title: null,
          text: "This is a test project.",
          rows: null,
          columns: null,
          data_rows: null,
          equation: null,
          substituted_values: null,
          result: null,
          language: null,
          content: null,
          series_names: null,
          x_unit: null,
          y_unit: null,
          items: null,
        },
        {
          block_type: "KeyValueTable",
          title: "Details",
          text: null,
          rows: [
            { key: "Name", value: "Demo", unit: null },
            { key: "Version", value: "1.0", unit: null },
          ],
          columns: null,
          data_rows: null,
          equation: null,
          substituted_values: null,
          result: null,
          language: null,
          content: null,
          series_names: null,
          x_unit: null,
          y_unit: null,
          items: null,
        },
      ],
      warnings: [],
    },
  ],
  warnings: [],
  assumptions: ["Assumed room temperature 25°C"],
  source_references: [],
  metadata: {},
};

function renderScreen(props: Partial<React.ComponentProps<typeof AdvancedReportsScreen>> = {}) {
  return render(
    <AdvancedReportsScreen
      hasProject={false}
      capabilities={mockCapabilities}
      lastReport={null}
      previewReport={null}
      exportResult={null}
      loading={false}
      error={null}
      onLoadCapabilities={vi.fn()}
      onGenerateReport={vi.fn()}
      onExportReport={vi.fn()}
      {...props}
    />,
  );
}

describe("AdvancedReportsScreen", () => {
  it("renders title and description", () => {
    renderScreen();
    expect(screen.getByText("Advanced Reports")).toBeInTheDocument();
    expect(screen.getByText(/Generate structured, multi-section reports/)).toBeInTheDocument();
  });

  it("calls onLoadCapabilities on mount when capabilities are empty", () => {
    const onLoad = vi.fn();
    renderScreen({ capabilities: [], onLoadCapabilities: onLoad });
    expect(onLoad).toHaveBeenCalledTimes(1);
  });

  it("does not call onLoadCapabilities when capabilities already loaded", () => {
    const onLoad = vi.fn();
    renderScreen({ capabilities: mockCapabilities, onLoadCapabilities: onLoad });
    expect(onLoad).not.toHaveBeenCalled();
  });

  it("disables generate button when no project is loaded", () => {
    renderScreen({ hasProject: false });
    const generateBtn = screen.getByRole("button", { name: /Generate Report/i });
    expect(generateBtn).toBeDisabled();
  });

  it("enables generate button when project exists and sections selected", () => {
    renderScreen({ hasProject: true });
    const generateBtn = screen.getByRole("button", { name: /Generate Report/i });
    expect(generateBtn).toBeEnabled();
  });

  it("allows toggling section selection", () => {
    renderScreen({ hasProject: true });
    const checkboxes = screen.getAllByRole("checkbox");
    expect(checkboxes.length).toBeGreaterThanOrEqual(3);

    // First checkbox should be checked by default (ProjectInfo has default_enabled: true)
    expect(checkboxes[0]).toBeChecked();

    fireEvent.click(checkboxes[0]);
    expect(checkboxes[0]).not.toBeChecked();
  });

  it("calls onGenerateReport with correct parameters", async () => {
    const onGenerate = vi.fn();
    renderScreen({ hasProject: true, onGenerateReport: onGenerate });

    const generateBtn = screen.getByRole("button", { name: /Generate Report/i });
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(onGenerate).toHaveBeenCalledTimes(1);
    });

    const [reportType, includedSections, title] = onGenerate.mock.calls[0];
    expect(reportType).toBe("ProjectSummary");
    expect(includedSections).toContain("ProjectInfo");
    expect(includedSections).toContain("SchematicSummary");
    expect(typeof title).toBe("string");
  });

  it("renders report preview when report is provided", () => {
    renderScreen({ hasProject: true, previewReport: mockReport });
    expect(screen.getByText(/Report Preview:/)).toBeInTheDocument();
    expect(screen.getByText(/Test Report/)).toBeInTheDocument();
    expect(screen.getByText("This is a test project.")).toBeInTheDocument();
  });

  it("renders assumptions when present", () => {
    renderScreen({ hasProject: true, previewReport: mockReport });
    expect(screen.getByText(/Assumed room temperature/)).toBeInTheDocument();
  });

  it("displays error alert when error prop is set", () => {
    renderScreen({ error: "Failed to load capabilities" });
    expect(screen.getByText("Failed to load capabilities")).toBeInTheDocument();
  });

  it("shows loading state on buttons when loading is true", () => {
    renderScreen({ hasProject: true, loading: true });
    const generateBtn = screen.getByRole("button", { name: /Generate Report/i });
    expect(generateBtn).toBeDisabled();
  });

  it("allows selecting report type", () => {
    renderScreen({ hasProject: true });
    // The Select input is a readonly input with the label above it
    const selectInput = screen.getByRole("textbox", { name: /Report Type/i });
    expect(selectInput).toBeInTheDocument();
    expect(selectInput).toHaveValue("Project Summary");
  });

  it("calls onExportReport when export button clicked", async () => {
    const onExport = vi.fn();
    renderScreen({ hasProject: true, previewReport: mockReport, onExportReport: onExport });

    const exportBtn = screen.getByRole("button", { name: /Export$/i });
    fireEvent.click(exportBtn);

    await waitFor(() => {
      expect(onExport).toHaveBeenCalledTimes(1);
    });

    const [reportId, format] = onExport.mock.calls[0];
    expect(reportId).toBe("report-123");
    expect(format).toBe("markdown");
  });
});
