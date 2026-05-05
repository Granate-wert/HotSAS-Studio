# HotSAS Studio

**HotSAS Studio — Hardware-Oriented Schematic Analysis & Simulation Studio**

Desktop engineering application for schematic analysis, formula-driven circuit templates, SPICE-oriented simulation workflows, and report generation.

**Current app version: v0.1.4**
**Current roadmap stage: v2.7 next**

> App version (v0.1.4) and roadmap stage are different concepts.

Completed:
- v1.2 — Project Package Storage `.circuit`
- v1.3 — Schematic Editor Foundations
- v1.4 — Engineering Notebook / Calculator Foundations
- v1.5 — Component Library Foundation
- v1.6 — Selected Region Analysis Foundation
- v1.7 — Export Center v1
- v1.8 — ngspice Adapter v1
- v1.9 — SPICE/Touchstone Import Foundation
- v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate
- v2.0 — Product Beta Integration, Workflow Stabilization & Internal RC Build
- v2.1 — Formula Library Expansion & Formula UX Hardening
- v2.2 — DC-DC Calculators and Templates
- v2.3 — Advanced Reports
- v2.4 — Real Component Parameters
- v2.5 — Schematic Editor Hardening
- v2.6 — Project Persistence / Save-Load UX Hardening

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

### v2.4 — Real Component Parameters

- Typed parameter schema with groups (Electrical, Thermal, Mechanical).
- Validation: unit mismatch, out of range, tolerance exceeded, missing required.
- 8 typed bundles: Resistor, Capacitor, Inductor, Diode, BJT, MOSFET, OpAmp, Regulator.
- 27 real-like built-in components with proper footprints.
- `ComponentParameterService` with schema lookup, validation, bundle extraction.
- API DTOs, facade methods, Tauri commands.
- Frontend `ComponentDetailsPanel` Typed Parameters card.

### v2.5 — Schematic Editor Hardening

- Backend editing commands: add component, move component, delete component, connect pins, rename net.
- `SchematicEditingService` with validation after every edit.
- Component palette UI (Resistor, Capacitor, Inductor, Diode, OpAmp, MOSFET, Voltage Source, Ground).
- Schematic toolbar with Delete, Connect, Rename Net actions.
- Connection panel: select from/to component and pin, optional net name.
- Net label editor: select net, enter new name.
- Move component via React Flow `onNodeDragStop` → backend command.
- React Flow remains view adapter; Rust `CircuitModel` / `ProjectDto` is source of truth.
- 10 new Rust tests for schematic editing + 6 frontend tests.

### v2.3 — Advanced Reports

- Core models: `AdvancedReportRequest`, `AdvancedReportType` (6 variants), `ReportSectionKind` (14 kinds), `AdvancedReportModel`, `ReportSection`, `ReportContentBlock` (7 block types), `ReportSectionCapability`.
- `AdvancedReportService` with `generate_report`, `list_section_capabilities`, and 4 renderers: Markdown, HTML, JSON, CSV Summary.
- 13 section builders covering all data domains: project info, schematic, components, formulas, notebook, DCDC, selected region, simulation, netlist, E-series, BOM, imports, export history, warnings.
- API facade with `last_advanced_report` caching; 4 Tauri commands.
- Frontend: `AdvancedReportsScreen` with report type selector, section checklist, preview, and export.
- Navigation: new "Advanced Reports" screen accessible from sidebar.
- 28 new tests: 11 core model tests, 9 service tests, 8 API tests, 13 frontend screen tests.

### v2.4 — Real Component Parameters

- **Typed parameter schema** (`component_parameters.rs`): `ComponentParameterDefinition`, `ComponentParameterGroup`, `ComponentParameterSchema`, `ComponentParameterValue`, `ComponentParameterSource`, `ComponentTolerance` (symmetric %, asymmetric, min/max).
- **Parameter validation**: unit mismatch, out-of-range, tolerance exceeded, missing required — with structured `ParameterValidationError`.
- **Typed bundles** for 8 component categories: `ResistorParameters`, `CapacitorParameters`, `InductorParameters`, `DiodeParameters`, `BjtParameters`, `MosfetParameters`, `OpAmpParameters`, `RegulatorParameters` — each with `from_map` / `to_map` roundtrips.
- **Schema builders** for all 8 categories with grouped parameters (Electrical, Thermal, Supply, Output, etc.).
- **ComponentParameterService** in application layer: schema lookup, component validation, instance override validation, typed bundle extraction, parameter resolution (default → override).
- **Seed expansion**: built-in library grew from 12 generic to **27 real-like components**:
  - Resistors: generic, 10k 1% 0603, 1k 5% axial, 100R 1% 0805
  - Capacitors: generic, 100nF 50V X7R 0603, 10uF 25V X5R 0805, 100uF 25V electrolytic
  - Inductors: generic, 47uH 0.5A
  - Diodes: generic, 1N4148, SS14 Schottky
  - BJTs: generic NPN/PNP, 2N2222, 2N2907
  - MOSFETs: generic N/P, IRFZ44N power
  - Op-amps: generic, LM358, rail-to-rail placeholder
  - Regulators: AMS1117-3.3 LDO
  - Sources/ground preserved
- **New footprints**: SMD 0603, SMD 0805 placeholders.
- **API DTOs**: `ComponentParameterSchemaDto`, `ComponentParameterDefinitionDto`, `ComponentParameterIssueDto`, `TypedComponentParametersDto`, `ParameterBundleDto` (tagged union).
- **Facade methods**: `get_component_parameter_schema`, `validate_component_parameters`, `get_typed_component_parameters`.
- **Tauri commands**: `get_component_parameter_schema`, `validate_component_parameters`, `get_typed_component_parameters`.
- **Frontend enhancements**:
  - `ComponentLibraryScreen` fetches typed parameters on selection.
  - `ComponentDetailsPanel` displays a "Typed Parameters" card with category-specific values (resistance, capacitance, VDS, GBW, etc.).
  - TypeScript types for all new DTOs.
- **Tests**: 16 core unit tests (tolerance, validation, schema, roundtrips), 5 service unit tests, 13 expanded component library integration tests (SMD footprints, real-like parts, schema validation), 89 frontend tests — all PASS.
- `tauri:build --no-bundle` successful.

### v2.2 — DC-DC Calculators and Templates

- Added DC-DC core models: `DcdcTopology`, `DcdcOperatingMode`, `DcdcInput`, `DcdcCalculationResult`, warnings, simulation plan.
- Added `DcdcCalculatorService` with ideal first-order formulas for Buck, Boost, Inverting Buck-Boost, and a controlled 4-switch placeholder.
- Added 4 DC-DC circuit templates (buck, boost, inverting buck-boost, 4-switch placeholder) with component helpers.
- Replaced old `smps.yaml` placeholder with `dcdc.yaml` containing 13 formulas across all topologies.
- Added `CelsiusPerWatt` to `EngineeringUnit`.
- Added API DTOs, facade methods, and Tauri commands: `calculate_dcdc`, `list_dcdc_templates`, `generate_dcdc_netlist_preview`, `run_dcdc_mock_transient_preview`, `create_dcdc_demo_project`.
- Added frontend `DcdcCalculatorScreen` with topology selector, input fields, calculate action, results/warnings panel.
- Added navigation entry **DC-DC Calculator** in the left sidebar.
- React remains view adapter only; all DC-DC math lives in Rust.
- 200+ Rust tests, 76 frontend tests — all PASS.

### v2.1 — Formula Library Expansion & Formula UX Hardening

- Added generic **expression evaluator** (`expression_evaluator.rs`): shunting-yard tokenizer + RPN evaluator.
- Supported operators: `+ - * / ^`, functions: `sqrt`, `exp`, `ln`, `log10`, `pow`, `abs`, constants: `pi`, variables, parentheses, unary minus.
- Extended core models: `FormulaExample` with structured inputs/expected outputs/notes; new `EngineeringUnit` variants (`Watt`, `Percent`, `Henry`, `Second`, `CelsiusPerWatt`, `KelvinPerWatt`) and `Giga` prefix.
- Expanded formula packs from 3 to **44 formulas** across 8 YAML files:
  - `basic_electronics` (8), `ac_impedance` (6), `transient` (5), `filters` (7), `op_amp` (7), `power_thermal` (5), `utilities` (4), `smps` (2 placeholders).
- Refactored `SimpleFormulaEngine`: generic evaluator path for all formulas; backward-compatible hardcoded branches for `rc_low_pass_cutoff`, `ohms_law`, `voltage_divider`.
- Extended DTOs and frontend types with `assumptions`, `limitations`, `examples`.
- Formula Library UI now displays assumptions, limitations, and clickable example presets.
- 245 Rust tests, 76 frontend tests — all PASS.

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

### v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate

- Added core diagnostics models: `AppDiagnosticsReport`, `ModuleDiagnostics`, `ModuleStatus`, `ReadinessCheck`, `ReadinessStatus`.
- Added `AppDiagnosticsService` with `get_app_diagnostics` and `run_readiness_self_check`.
- Added API DTOs and facade methods for diagnostics.
- Added Tauri commands: `get_app_diagnostics`, `run_readiness_self_check`.
- Added frontend: types, API wrappers, Zustand store fields (`appDiagnostics`, `readinessSelfCheckResult`, `diagnosticsLoading`, `diagnosticsError`).
- Added `DiagnosticsScreen` with module cards, readiness checks, refresh/self-check buttons.
- Added navigation item **Diagnostics** in the left sidebar.
- Added tests: 6 Rust application tests, 5 Rust API tests, 7 frontend tests.
- Added documentation: `docs/builds/INTERNAL_ALPHA_BUILD.md`, `docs/user_manual/QUICK_START_ALPHA.md`.
- Built and verified Windows `.exe` release artifact.
- This is **not** a public release. No GitHub Release or public tag was created.

### v1.9 — SPICE/Touchstone Import Foundation

- Added core domain models: `ImportedModelKind`, `SpiceModelDefinition`, `SpiceSubcircuitDefinition`, `SpiceImportReport`, `TouchstoneNetworkData`, `TouchstoneImportReport`, `SpicePinMapping`.
- Extended `SimulationModel` with `display_name`, `source_file_name`, `model_kind`, `raw_model_id` (backward-compatible with serde defaults).
- Added port traits: `SpiceModelParserPort`, `TouchstoneParserPort`.
- Added `SimpleSpiceModelParser`: parses `.model` and `.subckt`, line continuations, comments, heuristic op-amp detection, warnings for unsupported directives.
- Added `SimpleTouchstoneParser`: parses `.s1p` / `.s2p`, option line, RI/MA/DB formats, frequency units, reference impedance.
- Added `ModelImportService` with `import_spice_from_text`, `import_touchstone_from_text`, `list_imported_models`, `get_imported_model`, `validate_spice_pin_mapping`, `attach_imported_model_to_component`.
- Added API DTOs: `SpiceImportRequestDto`, `SpiceImportReportDto`, `TouchstoneImportRequestDto`, `TouchstoneImportReportDto`, `ImportedModelSummaryDto`, `ImportedModelDetailsDto`, `SpicePinMappingRequestDto`, `SpicePinMappingValidationReportDto`, `AttachImportedModelRequestDto`.
- Added facade methods and Tauri commands: `import_spice_model`, `import_touchstone_model`, `list_imported_models`, `get_imported_model`, `validate_spice_pin_mapping`, `attach_imported_model_to_component`.
- Added frontend: types, API wrappers, Zustand store fields (`spiceImportReport`, `touchstoneImportReport`, `importedModels`, `selectedImportedModel`).
- Added `ImportModelsScreen` with tabs: SPICE import, Touchstone import, Imported Library list with model details.
- React remains view adapter only; all parsing logic lives in Rust backend.
- 29 new Rust tests (spice parser 12, touchstone parser 10, model import service 7) + 6 frontend tests.

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
