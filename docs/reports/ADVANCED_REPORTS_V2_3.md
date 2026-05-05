# Advanced Reports (v2.3)

## Overview

The Advanced Reports module provides structured, multi-section report generation from project data, calculations, simulations, and design analyses. It replaces ad-hoc markdown/HTML export with a composable, typed report model that supports multiple output formats.

## Architecture

### Backend (Rust)

#### Core Models (`engine/core/src/advanced_report.rs`)

- **`AdvancedReportRequest`** — User-facing request with report type, included sections, export options, and metadata.
- **`AdvancedReportType`** — Enum with 6 variants: `ProjectSummary`, `CalculationReport`, `SimulationReport`, `SelectedRegionReport`, `DcdcDesignReport`, `FullProjectReport`.
- **`ReportSectionKind`** — 14 section kinds covering all major data domains: `ProjectInfo`, `SchematicSummary`, `ComponentSummary`, `FormulaCalculations`, `NotebookCalculations`, `DcdcCalculations`, `SelectedRegionAnalysis`, `SimulationResults`, `SpiceNetlist`, `ESeriesSelections`, `Bom`, `ImportedModels`, `ExportHistory`, `WarningsAndAssumptions`.
- **`AdvancedReportModel`** — The generated report containing sections, warnings, assumptions, source references, and metadata.
- **`ReportSection`** — A single section with `kind`, `title`, `status` (`Included` | `Empty` | `Unavailable` | `Error`), `blocks`, and `warnings`.
- **`ReportContentBlock`** — Typed content blocks: `Paragraph`, `KeyValueTable`, `DataTable`, `FormulaBlock`, `CodeBlock`, `GraphReference`, `WarningList`.
- **`ReportSectionCapability`** — Describes what each section can provide, which report types it supports, and whether it's enabled by default.

#### Application Service (`engine/application/src/services/advanced_report.rs`)

- **`AdvancedReportService`** — Main orchestrator.
  - `list_section_capabilities()` — Returns all 14 section capabilities.
  - `generate_report(request, context)` — Builds the report model from context. Each section builder is a separate private function. Missing data never panics; sections return `Empty` or `Unavailable`.
  - `render_report_markdown(report)` — Produces GitHub-flavored Markdown.
  - `render_report_html(report)` — Produces HTML with escaped entities.
  - `render_report_json(report)` — Produces pretty-printed JSON via `serde_json`.
  - `render_report_csv_summary(report)` — Produces a summary CSV with section statistics.

#### API Facade (`engine/api/src/facade.rs`)

- `list_report_section_capabilities()` — Lists capabilities as DTOs.
- `generate_advanced_report(request)` — Generates report, stores it in `last_advanced_report`, returns DTO.
- `export_advanced_report(request)` — Renders the last report to the requested format.
- `get_last_advanced_report()` — Returns the cached report if any.

#### Tauri Commands (`apps/desktop-tauri/src-tauri/src/lib.rs`)

Four commands registered in `generate_handler![]`:
- `list_report_section_capabilities`
- `generate_advanced_report`
- `export_advanced_report`
- `get_last_advanced_report`

### Frontend (React + TypeScript)

#### Types (`apps/desktop-tauri/src/types/index.ts`)

All Advanced Report DTOs are defined and typed: `AdvancedReportRequestDto`, `AdvancedReportDto`, `ReportSectionDto`, `ReportContentBlockDto`, `ReportKeyValueRowDto`, `ReportWarningDto`, `ReportSectionCapabilityDto`, `AdvancedReportExportRequestDto`, `AdvancedReportExportResultDto`.

#### API Layer (`apps/desktop-tauri/src/api/index.ts`)

Four wrapper methods invoke the Tauri commands with proper typing.

#### Zustand Store (`apps/desktop-tauri/src/store/index.ts`)

Added state fields:
- `reportSectionCapabilities`
- `lastAdvancedReport`
- `advancedReportPreview`
- `advancedReportExportResult`
- `advancedReportLoading`
- `advancedReportError`

Plus corresponding setters.

#### Screen (`apps/desktop-tauri/src/screens/AdvancedReportsScreen.tsx`)

Features:
- Report type selector (6 types)
- Section capability checklist with Select All / Clear All
- Generate Report button (disabled when no project or no sections selected)
- Report preview card showing sections, blocks, warnings, assumptions
- Export card with format selector (Markdown, HTML, JSON, CSV Summary) and optional output path
- Error alerts and loading spinners

#### Navigation (`apps/desktop-tauri/src/screens/navigation.tsx`)

New `ScreenId` variant `"reports"` with label "Advanced Reports" and `BarChart3` icon.

#### Workbench Integration (`apps/desktop-tauri/src/components/Workbench.tsx`)

- `runAdvancedReport` helper for isolated loading/error state
- Actions: `loadReportSectionCapabilities`, `generateAdvancedReport`, `exportAdvancedReport`
- `renderScreen` case for `"reports"` renders `<AdvancedReportsScreen />`

## Report Flow

1. User navigates to **Advanced Reports** screen.
2. Capabilities auto-load on mount if empty.
3. User selects report type and toggles sections.
4. Clicking **Generate Report** calls `generateAdvancedReport`, which:
   - Calls `backend.generateAdvancedReport(request)`
   - Stores result in `lastAdvancedReport` and `advancedReportPreview`
5. Preview renders immediately with all sections, blocks, warnings, and assumptions.
6. User can **Export** to Markdown, HTML, JSON, or CSV Summary.

## Section Builders

Each section builder receives `&AdvancedReportContext` and returns a `ReportSection`:

| Builder | Kind | Behavior when data missing |
|---|---|---|
| `build_project_info_section` | `ProjectInfo` | `Empty` if no project |
| `build_schematic_summary_section` | `SchematicSummary` | `Empty` if no project |
| `build_component_summary_section` | `ComponentSummary` | `Empty` if no project |
| `build_formula_calculations_section` | `FormulaCalculations` | `Unavailable` if no notebook |
| `build_notebook_calculations_section` | `NotebookCalculations` | `Unavailable` if no notebook |
| `build_dcdc_calculations_section` | `DcdcCalculations` | `Unavailable` if no DCDC result |
| `build_selected_region_analysis_section` | `SelectedRegionAnalysis` | `Unavailable` if no region result |
| `build_simulation_results_section` | `SimulationResults` | `Unavailable` if no simulation |
| `build_spice_netlist_section` | `SpiceNetlist` | `Unavailable` if no netlist |
| `build_e_series_selections_section` | `ESeriesSelections` | `Unavailable` if no notebook |
| `build_bom_section` | `Bom` | `Empty` if no project |
| `build_imported_models_section` | `ImportedModels` | `Empty` if no imported models |
| `build_export_history_section` | `ExportHistory` | `Empty` if no history |
| `build_warnings_and_assumptions_section` | `WarningsAndAssumptions` | Always `Included`, aggregates warnings |

## Output Formats

| Format | Description | Use case |
|---|---|---|
| `markdown` | GitHub-flavored Markdown | Human-readable docs, GitHub upload |
| `html` | Escaped HTML with sections | Web publishing, email |
| `json` | Full `serde_json` serialization | Programmatic consumption |
| `csv_summary` | Section statistics CSV | Spreadsheets, quick overviews |

## Testing

### Rust Tests

- `engine/core/tests/advanced_report_model_tests.rs` — 11 tests for core model construction, defaults, and display formatting.
- `engine/application/tests/advanced_report_service_tests.rs` — 9 tests for service capabilities, generation, rendering, and edge cases.
- `engine/api/tests/advanced_report_api_tests.rs` — 8 tests for API facade methods and export formats.

### Frontend Tests

- `apps/desktop-tauri/src/screens/AdvancedReportsScreen.test.tsx` — 13 tests for rendering, interactions, report generation, export, and error states.

## Files Changed

### Backend
- `engine/core/src/advanced_report.rs` — New domain models
- `engine/application/src/services/advanced_report.rs` — New service
- `engine/application/src/services/mod.rs` — Re-export `AdvancedReportService`
- `engine/application/src/lib.rs` — Re-export `AdvancedReportService`
- `engine/api/src/dto.rs` — Advanced Report DTOs and `From` impls
- `engine/api/src/facade.rs` — Facade methods and `last_advanced_report` state
- `engine/application/Cargo.toml` — Added `serde_json` dependency

### Frontend
- `apps/desktop-tauri/src/types/index.ts` — Advanced Report DTO types
- `apps/desktop-tauri/src/api/index.ts` — API wrappers
- `apps/desktop-tauri/src/store/index.ts` — Store fields and setters
- `apps/desktop-tauri/src/screens/AdvancedReportsScreen.tsx` — New screen
- `apps/desktop-tauri/src/screens/AdvancedReportsScreen.test.tsx` — Screen tests
- `apps/desktop-tauri/src/screens/navigation.tsx` — `"reports"` entry
- `apps/desktop-tauri/src/components/Workbench.tsx` — Integration

### Documentation
- `docs/reports/ADVANCED_REPORTS_V2_3.md` — This file
