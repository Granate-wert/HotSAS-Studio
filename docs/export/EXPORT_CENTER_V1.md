# Export Center v1

## Overview

The Export Center provides a unified interface for generating and downloading design artifacts in multiple formats. All export logic lives in the Rust backend; the React frontend is a view adapter only.

## Available Formats (v1.7)

| Format                   | ID                        | Extension | Description                                            |
| ------------------------ | ------------------------- | --------- | ------------------------------------------------------ |
| Markdown Report          | `markdown_report`         | `.md`     | Human-readable report with sections, formulas, and BOM |
| HTML Report              | `html_report`             | `.html`   | Self-contained HTML with escaped content               |
| SPICE Netlist            | `spice_netlist`           | `.cir`    | SPICE-compatible netlist for circuit simulation        |
| CSV Simulation Data      | `csv_simulation_data`     | `.csv`    | Simulation graph series exported as CSV                |
| BOM (CSV)                | `bom_csv`                 | `.csv`    | Bill of Materials in CSV format                        |
| BOM (JSON)               | `bom_json`                | `.json`   | Bill of Materials in JSON format                       |
| Component Library (JSON) | `component_library_json`  | `.json`   | Full component library as JSON                         |
| SVG Schematic            | `svg_schematic`           | `.svg`    | Placeholder SVG schematic image                        |
| Altium Workflow Package  | `altium_workflow_package` | `.zip`    | Placeholder Altium Designer workflow package           |

## Architecture

The export system follows the hexagonal architecture pattern:

```text
React ExportScreen -> Tauri commands -> hotsas_api facade -> ExportCenterService -> exporter ports -> adapter implementations
```

### Core Models (`hotsas_core`)

- `ExportFormat` — enum with 9 variants
- `ExportCapability` — metadata: label, description, file extension, availability
- `ExportResult` — content, file path, success flag, message
- `ExportHistoryEntry` — timestamp, format, file path, success

### Ports (`hotsas_ports`)

- `BomExporterPort` — `export_bom_csv`, `export_bom_json`
- `SimulationDataExporterPort` — `export_simulation_csv`
- `ComponentLibraryExporterPort` — `export_component_library_json`
- `SchematicExporterPort` — `export_svg_schematic`

Existing ports reused:

- `ReportExporterPort` — Markdown/HTML reports
- `NetlistExporterPort` — SPICE netlist

### Adapters (`hotsas_adapters`)

- `BomCsvExporter` — generates CSV from project components
- `BomJsonExporter` — generates JSON array of BOM lines
- `CsvSimulationDataExporter` — converts `SimulationResult.graph_series` to CSV
- `ComponentLibraryJsonExporter` — serializes `ComponentLibrary` to JSON
- `SvgSchematicExporter` — deterministic placeholder SVG with component rectangles
- `AltiumWorkflowPackageExporter` — placeholder markdown description

### Application Service (`hotsas_application`)

`ExportCenterService` provides:

- `list_capabilities()` — returns all 9 `ExportCapability` items
- `export_to_string(format, project, ...)` — generates content in memory
- `export_to_file(format, project, ..., output_dir)` — writes to `hotsas_export_<format>_<timestamp>.<ext>`
- `record_export(result)` / `list_history()` — export history tracking

## Frontend

The `ExportScreen` component displays:

- List of available export formats as buttons with file extension badges
- "Write to file" toggle with output directory input
- Content preview for the last export
- Load History button showing recent exports

Exports are disabled when no project is loaded.

## File I/O

`export_to_file` is the first file-writing feature in the export adapters. It:

1. Generates content via the appropriate exporter
2. Creates the output directory if needed
3. Writes the file with a timestamped name
4. Returns the file path in `ExportResult`

## Tests

- **Rust**: 6 integration tests in `api/tests/export_center_api_tests.rs`
- **React**: 7 tests in `src/screens/ExportScreen.test.tsx`

## Placeholders

The following formats are intentionally placeholder-quality in v1.7:

- **SVG Schematic** — simple rectangles and text, not a real schematic renderer
- **Altium Workflow Package** — markdown description only, no real Altium files
- **PDF Report** — not implemented (empty `PdfReportExporterPlaceholder` exists from earlier versions)

Real implementations for these will come in later versions.
