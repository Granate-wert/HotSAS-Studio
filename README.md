# HotSAS Studio

**HotSAS Studio — Hardware-Oriented Schematic Analysis & Simulation Studio**

Desktop engineering application for schematic analysis, formula-driven circuit templates, SPICE-oriented simulation workflows, and report generation.

**Current app version: v0.1.4**
**Current roadmap stage: v1.9 next**

Completed:
- v1.2 — Project Package Storage `.circuit`
- v1.3 — Schematic Editor Foundations
- v1.4 — Engineering Notebook / Calculator Foundations
- v1.5 — Component Library Foundation
- v1.6 — Selected Region Analysis Foundation
- v1.7 — Export Center v1
- v1.8 — ngspice Adapter v1

---

## What Is Implemented

### v0.1.0 — RC Low-Pass Vertical Slice

- Create and save/load circuit projects as JSON folders.
- Render schematics from backend DTOs via React Flow (view adapter only).
- Calculate `fc = 1 / (2*pi*R*C)` through the backend FormulaService.
- Select nearest E24 preferred values.
- Generate SPICE netlist for RC low-pass.
- Run mock AC simulation with gain/phase graph.
- Export Markdown and HTML reports.
- Export Center with 9 formats: SPICE netlist, BOM CSV/JSON, simulation CSV, component library JSON, SVG schematic, Altium workflow placeholder.

### v0.1.1 — Architecture Hardening

- Split `hotsas_application` into focused services: `ProjectService`, `FormulaService`, `PreferredValuesService`, `CircuitTemplateService`, `NetlistGenerationService`, `SimulationService`, `ExportService`.
- Added `CircuitQueryService` in core for reusable component/parameter access.
- Split `hotsas_api` into `dto.rs`, `error.rs`, `facade.rs`.
- Added structured `ApiErrorDto` with `code`, `message`, `details`.
- Refactored React frontend into `api/`, `store/`, `types/`, `screens/`, `components/`.

### v0.1.2 — Backend Test Expansion

- 63+ Rust tests covering all crates:
  - `core`: EngineeringValue, PreferredValues, circuit templates, formulas
  - `adapters`: formula engine, netlist export, report export (Markdown + HTML escaping), JSON storage, full vertical slice
  - `application`: registry, services
  - `api`: error DTOs, state errors, dependency boundaries

### v0.1.3 — FormulaPackLoader + FormulaRegistry

- Runtime loading of formula packs from `shared/formula_packs` (YAML/JSON).
- `FormulaPackFileLoader` with validation.
- `FormulaRegistryService`: listing, lookup, categories, metadata, duplicate detection, binding validation.
- Formula Library UI receives formulas through backend DTOs.

### v0.1.4 — Generic FormulaEnginePort

- Generic `FormulaEnginePort` methods: `evaluate_formula`, `evaluate_expression`, `validate_expression`.
- `SimpleFormulaEngine` supports an allowlist of expressions:
  - `fc = 1 / (2*pi*R*C)`
  - `V = I * R`
  - `Vout = Vin * R2 / (R1 + R2)`
- `FormulaService.calculate_formula(formula_id, variables)` via Registry.
- API command `calculate_formula` + Tauri command.
- Formula Library UI: variable inputs + **Calculate** button.
- Old RC-specific commands preserved for compatibility.

### v1.4 — Engineering Notebook / Calculator Foundations

- Added `EngineeringNotebook` domain model with blocks, variables, history, and evaluation results.
- Added `EngineeringNotebookService` with parsers for assignments, formula calls, and preferred-value commands.
- Added backend API DTOs and facade methods: `evaluate_notebook_input`, `get_notebook_state`, `clear_notebook`, `apply_notebook_output_to_component`.
- Added Tauri commands for notebook integration.
- Added frontend notebook UI components: `NotebookInput`, `NotebookResultCard`, `NotebookVariableTable`, `NotebookHistory`, `PreferredValueQuickTools`, `ApplyNotebookOutputPanel`.
- Integrated notebook into `CalculatorScreen`.
- Unsupported expressions return controlled results with a helpful hint.

### v1.6 — Selected Region Analysis Foundation

- Added core domain models: `SelectedCircuitRegion`, `RegionPort`, `SelectedRegionPreview`, `SelectedRegionAnalysisResult`, `MatchedRegionTemplate`, net topology helpers.
- Added `SelectedRegionAnalysisService` with:
  - `preview_selected_region` — builds subcircuit view, detects internal/boundary/external nets, suggests ports
  - `analyze_selected_region` — validates, matches RC low-pass / voltage divider templates, generates SPICE netlist fragment
  - `validate_selected_region` — returns structured issues (empty selection, unknown components, missing ports)
- Added API DTOs and facade methods for preview, analyze, validate.
- Added Tauri commands: `preview_selected_region`, `analyze_selected_region`, `validate_selected_region`.
- Added frontend types, API wrappers, Zustand store fields, and UI components:
  - `SelectedRegionPanel` with component checkboxes and Preview/Analyze/Clear buttons
  - `SelectedRegionPreviewCard` and `SelectedRegionResultCard`
- Integrated **Region** tab into `SchematicScreen` side panel.
- React remains view adapter only; all topology detection and template matching live in Rust.

### v1.8 — ngspice Adapter v1

- Added `NgspiceAvailability`, `NgspiceRunStatus`, `NgspiceRunMetadata`, `NgspiceSimulationRequest` core models.
- Extended `SimulationEnginePort` with `engine_name()`, `check_availability()`, `run_ac_sweep()`, `run_operating_point()`, `run_transient()`, `stop_simulation()`, `get_result()`.
- Added `NgspiceSimulationAdapter` in `hotsas_adapters`: binary resolver, process runner, output parser, netlist control block builder.
- Added `NgspiceSimulationService` with engine selection policy: Mock / ngspice / Auto (fallback to Mock with warning).
- Added API DTOs: `NgspiceAvailabilityDto`, `SimulationRunRequestDto`, `SimulationRunMetadataDto`.
- Added facade methods: `check_ngspice_availability`, `run_simulation`, `simulation_history`.
- Added Tauri commands: `check_ngspice_availability`, `run_simulation`, `simulation_history`.
- Updated frontend: types, API wrappers, Zustand store fields (`ngspiceAvailability`, `selectedSimulationEngine`, `simulationHistory`, `isSimulationRunning`).
- Replaced `SimulationScreen` with `SimulationResultsScreen`: engine status card, engine selector (Auto/Mock/ngspice), run buttons (OP, AC Sweep, Transient), result card with ECharts graph.
- React remains view adapter only; all ngspice logic (netlist generation, process execution, output parsing) lives in Rust backend.
- Real ngspice integration tests are opt-in via `HOTSAS_RUN_NGSPICE_INTEGRATION=1`.
- 19 new Rust tests (resolver, parser, service, API) + 7 frontend tests.

### v1.3 — Schematic Editor Foundations

- Added pin/symbol foundations: `PinDefinition`, `ElectricalPinType`, `PinPosition`, `PinSide`, `SymbolDefinition`.
- Added seed symbols for resistor, capacitor, voltage source, ground.
- Added `CircuitValidationService` with checks: empty circuit, missing ground, duplicated ids, missing parameters, floating nets, unknown references.
- Added backend API: `get_selected_component`, `update_component_parameter`, `validate_current_circuit`.
- Added custom React Flow nodes: ResistorNode, CapacitorNode, VoltageSourceNode, GroundNode.
- Added `SchematicPropertyPanel` for viewing/editing component parameters.
- Added `CircuitValidationPanel` for running circuit validation.
- React Flow remains view adapter only; backend remains source of truth.

---

## Technology Stack

- **Desktop:** Tauri v2
- **UI:** React 19, TypeScript 5.9, Vite 7, Mantine 8, Zustand 5
- **Schematic view adapter:** React Flow / xyflow
- **Charts:** Apache ECharts 6
- **Engine:** Rust workspace (Clean Architecture / Hexagonal Architecture)

Mantine is a pragmatic UI kit only. It does not influence backend architecture.

---

## Project Structure

```text
HotSAS Studio/
├── engine/                    # Rust workspace
│   ├── Cargo.toml
│   ├── core/                  # hotsas_core — domain models
│   ├── ports/                 # hotsas_ports — trait contracts
│   ├── application/           # hotsas_application — use case services
│   ├── adapters/              # hotsas_adapters — port implementations
│   └── api/                   # hotsas_api — DTOs, facade, Tauri commands
│
├── apps/
│   └── desktop-tauri/         # Tauri v2 + React shell
│       ├── src/               # Frontend (api, store, types, screens, components)
│       └── src-tauri/         # Rust Tauri composition root
│
├── shared/
│   ├── formula_packs/         # YAML/JSON formula packs (runtime loaded)
│   └── test_projects/         # Sample project fixtures
│
└── docs/
    ├── architecture/
    ├── component_library/
    ├── export/
    ├── formula_library/
    ├── selected_region/
    ├── simulation/
    └── testing/
```

---

## Architecture

`engine/` is a Rust workspace with strict dependency direction:

```text
React -> Tauri commands -> hotsas_api -> hotsas_application -> hotsas_ports -> hotsas_core
                                                          ^                    ^
                                                          |                    |
                                                 hotsas_adapters implements ports
```

Rules:

- `hotsas_core` is pure domain code. No dependency on application, adapters, api, Tauri, React, or UI.
- `hotsas_application` depends on `hotsas_core` and `hotsas_ports`.
- `hotsas_adapters` implements `hotsas_ports`.
- `hotsas_api` depends on `hotsas_application` and DTO contracts.
- Tauri owns the composition root.
- React calls **only** Tauri commands.

React Flow is a view adapter. The source of truth for schematic state is `CircuitModel` / `CircuitDto`, not React Flow nodes and edges.

Details: `docs/architecture/ARCHITECTURE.md`.

---

## Prerequisites

- [Rust](https://rustup.rs/) (`rustup default stable`)
- [Node.js](https://nodejs.org/) with npm
- Windows: Build Tools for Visual Studio (C++ workload) for Tauri

Verify:

```powershell
rustc --version
cargo --version
node --version
npm --version
```

---

## Run (Development)

Install frontend dependencies and start the desktop shell:

```powershell
cd apps\desktop-tauri
npm.cmd install
npm.cmd run tauri:dev
```

The dev window opens at 1440×960. The dev server runs on `http://127.0.0.1:1420` inside the Tauri WebView.

---

## Build (Release)

```powershell
cd apps\desktop-tauri
npm.cmd run tauri:build
```

The release executable is placed at:

```text
apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
```

It is built as a **Windows GUI** application (no background console).

---

## Development Checks

Run these before committing:

**Rust engine:**

```powershell
cd engine
cargo fmt --check
cargo test
```

**Frontend:**

```powershell
cd apps/desktop-tauri
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```

**Full integration:**

```powershell
cd apps/desktop-tauri
npm.cmd run tauri:build
```

Details: `docs/testing/TESTING.md`.

---

## Roadmap

- **v0.1.x (current):** RC low-pass vertical slice, generic formula evaluation for allowlisted expressions, runtime formula packs.
- **v0.2.x:** Exact E48/E96/E192 tables, richer formula engine, stronger unit model, more circuit templates.
- **v0.3.x:** Component Library Manager, Engineering Calculator / Notebook, canvas editing with feedback to Rust state.
- **v1.0.0:** Real ngspice adapter, SQLite storage, import/export expansion.
- **Later:** KiCad-compatible symbol/footprint export and Altium workflow package.

A PCB editor is **not** planned for v1.

---

## License

MIT
