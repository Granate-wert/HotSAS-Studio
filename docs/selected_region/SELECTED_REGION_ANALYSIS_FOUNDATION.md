# Selected Region Analysis Foundation (v1.6)

## Overview

v1.6 introduces the **Selected Region Analysis Foundation** — a backend-driven subsystem that lets users select a subset of components on a schematic, preview the resulting subcircuit topology, and run structural/template-based analysis on that region.

React remains a view adapter only. All boundary net detection, template matching, netlist generation, and validation happen in Rust (`core` + `application`).

## Architecture

```
frontend (React)
  └─ SelectedRegionPanel ──invoke──► Tauri commands
                                        └─ HotSasApi facade
                                              └─ SelectedRegionAnalysisService (application)
                                                    └─ selected_region models (core)
```

### Layers

| Layer | Responsibility |
|-------|---------------|
| `core` | Domain models: `SelectedCircuitRegion`, `RegionPort`, `SelectedRegionPreview`, `SelectedRegionAnalysisResult`, `MatchedRegionTemplate`, net topology helpers |
| `application` | `SelectedRegionAnalysisService`: preview, validate, analyze, build subcircuit view, detect boundary nets, match known templates, generate netlist fragments |
| `api` | DTOs + facade methods: `preview_selected_region`, `analyze_selected_region`, `validate_selected_region` |
| `src-tauri` | Tauri commands exposing the three facade methods |
| frontend | Types, API wrappers, Zustand store fields, `SelectedRegionPanel` + preview/result cards |

## Core Models

Key types (all `Debug + Clone + PartialEq + Serialize + Deserialize`):

- `SelectedCircuitRegion` — main region model with component IDs, nets, ports, direction, mode
- `RegionPort` — positive_net + optional negative_net/label
- `RegionAnalysisDirection` — `LeftToRight`, `RightToLeft`, `Custom`
- `RegionAnalysisMode` — `Structural`, `TemplateBased`, `NumericMock`, `AllAvailable`
- `SelectedRegionPreview` — what the user sees before analysis (components, nets, suggestions, warnings)
- `SelectedRegionAnalysisResult` — full result with status, template match, transfer function, measurements, netlist fragment

## Application Service

### `preview_selected_region(circuit, component_ids)`

Builds a `SubcircuitView`, detects internal/boundary/external nets, and suggests input/output/reference candidates.

### `analyze_selected_region(circuit, request)`

1. Validates the request
2. Builds subcircuit view
3. Matches known templates (RC low-pass, voltage divider)
4. Generates a SPICE netlist fragment for the selected components
5. Returns a controlled result — never panics

### `validate_selected_region(circuit, request)`

Returns issues for:
- Empty selection
- Unknown components
- Missing ports (when mode != Structural)
- Missing reference node (warning)

### Template Matching (v1.6 scope)

Only two templates are recognized:
- **RC low-pass**: R + C with signal path `net_in → R → net_out → C → gnd`
- **Voltage divider**: R1 + R2 with `Vin → R1 → Vout → R2 → GND`

All other selections return `Status::Partial` with a netlist fragment and warnings.

## API / DTOs

New DTOs added to `engine/api/src/dto.rs`:
- `RegionPortDto`
- `SelectedRegionAnalysisRequestDto`
- `SelectedCircuitRegionDto`
- `RegionComponentSummaryDto`
- `RegionNetSummaryDto`
- `SelectedRegionIssueDto`
- `SelectedRegionPreviewDto`
- `MatchedRegionTemplateDto`
- `EquivalentCircuitSummaryDto`
- `RegionTransferFunctionDto`
- `RegionMeasurementDto`
- `RegionGraphSpecDto`
- `RegionNetlistFragmentDto`
- `SelectedRegionAnalysisResultDto`

Facade methods added to `HotSasApi`:
- `preview_selected_region(component_ids)` → `SelectedRegionPreviewDto`
- `analyze_selected_region(request)` → `SelectedRegionAnalysisResultDto`
- `validate_selected_region(request)` → `Vec<SelectedRegionIssueDto>`

## Tauri Commands

Three new commands registered in `generate_handler!`:
- `preview_selected_region`
- `analyze_selected_region`
- `validate_selected_region`

## Frontend

### Types

Added to `apps/desktop-tauri/src/types/index.ts`:
- All selected-region DTO TypeScript types

### API

Added to `apps/desktop-tauri/src/api/index.ts`:
- `previewSelectedRegion(componentIds)`
- `analyzeSelectedRegion(request)`
- `validateSelectedRegion(request)`

### Store

Added to `useHotSasStore`:
- `selectedRegionComponentIds`
- `selectedRegionPreview`
- `selectedRegionAnalysisResult`
- Setters for each

### UI Components

- `SelectedRegionPanel` — component checkboxes, Preview/Analyze/Clear buttons, renders preview/result cards
- `SelectedRegionPreviewCard` — displays preview info, suggestions, warnings, errors
- `SelectedRegionResultCard` — displays status, template match, transfer function, netlist fragment, measurements, warnings/errors

Integrated into `SchematicScreen` as a new **Region** tab in the side panel.

## Known Limitations (by design)

- No arbitrary symbolic solver
- No real ngspice integration
- Transfer function only for matched templates
- Graph specs are placeholders for future simulation integration
- Report section markdown is generated but not persisted into project report (controlled placeholder)

## Testing

### Rust

- `engine/application/tests/selected_region_analysis_tests.rs` (5 tests):
  - `preview_selected_region_returns_components_and_nets`
  - `analyze_selected_region_matches_rc_low_pass`
  - `validate_empty_selection_returns_error`
  - `preview_single_component_has_boundary_nets`
  - `analyze_unsupported_region_returns_partial`

### Frontend

- `apps/desktop-tauri/src/components/selected-region/__tests__/SelectedRegionPanel.test.tsx` (5 tests):
  - renders component checkboxes
  - selecting components updates count
  - Preview button calls backend and shows preview card
  - Analyze button calls backend and shows result card
  - Clear button resets selection

## Verification Log

- `cargo test` — 125+ PASS
- `cargo fmt --check` — PASS
- `npm test` — 41 PASS
- `npx tsc --noEmit` — PASS (0 errors)
