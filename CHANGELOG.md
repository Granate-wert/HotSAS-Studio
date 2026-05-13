# Changelog

All notable changes to HotSAS Studio are documented in this file.

## [v3.6-pre-fix4] - CAD-Style Manual Wire Routing Foundation

### Added

- CAD-style schematic symbols for resistor, capacitor, inductor, voltage source, ground, diode, op-amp, and MOSFET placeholder nodes.
- Left-side Schematic component palette with responsive layout breakpoints for the canvas and Properties panel.
- Manual wire routing foundation: click a pin to start, click canvas to add grid-snapped bend points, click a target pin to complete, and press `Escape` to cancel.
- Wire route geometry in frontend types, API DTOs, application service requests, and core `WireGeometry`.
- Persistence coverage for wire route points and API coverage for manual routing metadata.
- Frontend tests for symbol rendering, visible pin handles, route hydration, manual pin-to-pin routing, and draft cancellation.

### Changed

- React Flow edges use a manual polyline edge when persisted route points exist.
- Netlist generation remains connectivity-driven; route geometry is visual metadata and does not affect electrical topology.
- Schematic docs, verification log, acceptance matrix, README, and testing guide now document fix4 behavior and CAD limitations.

### Known limitations

- No post-creation bend-point editing, component rotation, drag-from-palette placement, buses, hierarchical sheets, live ERC, or full KiCad/Altium/EasyEDA/LTspice parity.
- Native Tauri desktop smoke is still required for OS-level dialogs and shell-integrated flows.

## [v3.6-pre] — Practical Schematic Construction Flow

### Added

- Tauri ACL fix: added 45 missing schematic editing and analysis commands to `permissions/hotsas.toml`, resolving `Command add_schematic_component not allowed by ACL` and similar denials.
- React Flow v12 placement coordinate fix: `SchematicCanvas` now wraps `ReactFlow` in `ReactFlowProvider` and uses `screenToFlowPosition` for accurate click-to-place coordinates.
- Frontend tests for schematic interaction:
  - Place mode hint rendering.
  - User-friendly error display instead of raw ACL messages.
  - Placeable palette item selection.

### Fixed

- `add_schematic_component` ACL denial — root cause was missing command entries in `permissions/hotsas.toml`.
- `move_schematic_component` ACL denial — same root cause.
- `place_schematic_component`, `delete_schematic_component`, `connect_schematic_pins`, `rename_schematic_net`, and 39 other commands now permitted by Tauri ACL.
- Inaccurate component placement coordinates when canvas is panned/zoomed.

## [v3.5] — Schematic Editor & Simulation Workflow Usability Gate

### Added

- Schematic Editor empty state: centered card with "New RC Demo", "Open Project" buttons and workflow guidance when no project or empty project.
- Tool modes wired to `SchematicCanvas`:
  - `place`: `onPaneClick` handler calls `onPlaceSchematicComponent` with click coordinates.
  - `delete`: `handleNodeClick` and `handleEdgeClick` call `onDeleteComponent` / `onDeleteWire`.
  - `wire`: native React Flow `onConnect` with cursor indicator.
  - `select`: standard behavior unchanged.
- Custom SVG component symbols in `GenericComponentNode`: resistor (zigzag), capacitor (plates), inductor (coil), voltage source (circle +/-), ground (earth), diode (triangle + bar).
- Net name labels on edges: maps `wire.net_id` to `net.name` instead of raw UUID.
- Disabled-state explanations via Mantine `Tooltip`:
  - `InteractiveSchematicToolbar` shows reason when disabled (e.g. "Open or create a project").
  - `SchematicToolbar` shows reasons for disabled Delete/Connect/Rename Net buttons.
- Bottom tab empty states:
  - `SimulationChart`: "No simulation results yet" with guidance.
  - `FormulaPanel`: "No formula results yet" with guidance.
  - `ReportPanel`: "No report generated yet" with guidance and disabled explanation.
  - `LibraryPanel`: "Open Component Library" button.
- `ProjectMetrics` circuit metrics: component count, net count, wire count.
- Frontend tests:
  - `SchematicScreen` empty state tests (no project, empty project, demo click).
  - `InteractiveSchematicToolbar` disabled tooltip test.
  - `SchematicScreen` canvas visibility test.

### Changed

- `SchematicScreen` layout refactored: top toolbar uses `schematic-topbar` flex row, canvas uses `flex: 1`, empty state replaces canvas when no components.
- `styles.css` updated with `.schematic-topbar`, `.schematic-empty-state`, cursor classes for place/delete modes.
- `GenericComponentNode` removed internal `onClick` to prevent double-fire with React Flow `onNodeClick`.
- `Workbench.tsx` passes `onCreateDemoProject` and `onLoadProjectPackage` to `SchematicScreen`.
- README roadmap stage updated to `v3.5 ACCEPTED WITH DOCUMENTED LIMITATIONS`.

### Fixed

- Double invocation of `onSelectComponent` on node click (removed node-internal onClick).
- Edge labels showing raw net UUIDs instead of human-readable net names.

## [v3.4-ui-report-fix] — Model Persistence UI Indicators & Report Section

### Added

- Frontend persistence status badges in `ModelAssignmentCard`:
  - Persisted, Package-backed, Derived builtin, Session-only, Missing asset, Stale reference, Unknown.
  - Persistence warning alert for missing/stale asset references.
- Frontend persistence status indicators in `SchematicSelectionInspector`:
  - Inherited/override assignment origin badge.
  - Persistence status badge (same states as ModelAssignmentCard).
  - Pin mapping and parameter binding counts.
  - Missing/stale diagnostics display.
- Backend report section `ModelPersistence` in `AdvancedReportService`:
  - Added to `ReportSectionKind` enum and `default_section_capabilities()`.
  - Builder `build_model_persistence()` generates `Model Persistence & Package Integrity` section.
  - Includes model catalog counts (assets, SPICE models, subcircuits, Touchstone datasets).
  - Includes assignment counts (component, instance, missing references, stale assignments).
  - Includes diagnostics list with severity mapping.
  - Markdown, HTML, JSON, and CSV summary renderers support the new section.
- API facade `build_model_persistence_summary()` helper method for `AdvancedReportContext` population.
- Frontend tests: 9 new tests across `ModelAssignmentCard` and `SchematicSelectionInspector` for persistence states.
- Rust tests: 5 new tests for `ModelPersistence` report section in core, application, and API layers.

### Changed

- `AdvancedReportContext` extended with `model_persistence_summary: Option<ProjectModelPersistenceSummary>`.
- `validate_project_model_persistence` refactored to use shared `build_model_persistence_summary` helper.
- Acceptance matrix updated: MP-036, MP-037, MP-038 changed from DEFERRED to PASS.
- README roadmap stage updated to `v3.4 ACCEPTED WITH DOCUMENTED LIMITATIONS`.

## [v3.4] — Model Persistence & Project Package Hardening

### Added

- Core domain models for model persistence in `hotsas_core::model_persistence`:
  - `PersistedModelAsset` — kind, source, status, content hash, package path, raw content, warnings.
  - `PersistedModelCatalog` — container for persisted model assets.
  - `PersistedInstanceModelAssignment` — pin mappings, parameter bindings, status, source.
  - `ProjectModelPersistenceSummary` — asset counts, assignment counts, diagnostics.
- `CircuitProject` hardening with `imported_model_catalog` and `persisted_model_assignments` fields (backward-compatible serde defaults).
- `ProjectPackageStoragePort` extension with `save/load_model_catalog` and `save/load_model_assignments`.
- Adapter `.circuit` package storage writes `models/catalog.json` and `models/assignments.json` inside packages.
- Adapter validation detects missing model assets and stale assignment references with diagnostics.
- Application service integration:
  - `ModelImportService.build_persisted_model_catalog()` converts imported models to persisted catalog.
  - `ComponentModelMappingService.build_persisted_instance_assignment()` converts runtime assignments to persisted form.
  - `AppServices` wires persistence into save/load project package flow.
- API facade methods: `get_project_model_catalog`, `validate_project_model_persistence`, `get_project_model_persistence_summary`.
- Tauri commands for model catalog and persistence validation.
- CLI integration: `validate` and `model-check` commands include model persistence diagnostics.
- Frontend TypeScript types (`ModelCatalogDto`, `ModelAssetDto`, etc.) and Zustand store extensions.
- Adapter model persistence tests: 4 new tests (catalog roundtrip, assignments roundtrip, missing asset diagnostic, legacy backward compatibility).
- Application model persistence service tests.
- Documentation: verification log and acceptance matrix for v3.4.

### Changed

- `CircuitProjectPackageStorage` now manages `models/` subdirectory inside `.circuit` packages.
- `FakeProjectPackageStorage` implementations across test files updated with new trait methods.

### Fixed

- Malformed multi-line `use` statements in test files caused by automated replacements.
- Missing trait method implementations in fake storages across ~25 test files.

## [v2.7] — CLI / Headless Mode Foundation

### Added

- New `hotsas_cli` crate with `hotsas-cli` binary in `engine/cli/`.
- CLI commands delegating to `HotSasApi` facade (zero duplicated business logic):
  - `validate <path>` — validate a `.circuit` project package.
  - `formula <id> [key=value...]` — evaluate a formula with variables.
  - `netlist <path> [--out <file>]` — generate SPICE netlist.
  - `export <path> <format> [--out <file>]` — export report (markdown, html, json, csv-summary).
  - `simulate <path> <profile> [--engine <engine>] [--out <file>]` — run simulation (mock or ngspice).
  - `library check` — verify built-in component library integrity.
  - `--version` / `--help` — standard CLI metadata.
- Global `--json` flag for structured JSON output on every command.
- Strict exit code policy: 0 success, 1 internal/IO, 2 validation/input, 3 usage, 4 unsupported.
- `CliOutput<T>` generic output wrapper with human-readable and JSON modes.
- `build_headless_api()` factory reusing the same adapter wiring as Tauri desktop.
- `initialize_cli()` loading built-in formula packs and component library on startup.
- 10 CLI integration tests in `engine/cli/tests/cli_integration.rs` (version, help, validate, formula, netlist, export, simulate, library check, JSON output, error codes).
- Total Rust tests: 361 (all passing).

### Changed

- `engine/Cargo.toml` workspace now includes `cli` member.
- `engine/cli/Cargo.toml` depends on `clap` (derive feature), `serde`, `serde_json`, and all internal workspace crates.

## [0.1.4-fix] — Generic Formula Engine Completion Gate

### Added

- `ErrorBoundary` React component (`src/components/ErrorBoundary.tsx`) to catch render errors and prevent black screen crashes.
- UI workflow tests with Vitest + React Testing Library + jsdom:
  - `src/components/ErrorBoundary.test.tsx` — 4 tests (render, error catch, reset, custom fallback).
  - `src/screens/FormulaLibraryScreen.test.tsx` — 8 tests (load, select, input, calculate, switch, errors, null defaults, malformed results).
  - Test utilities: `src/test-setup.ts`, `src/test-utils.tsx`, `src/api/__mocks__/index.ts`.
- `npm.cmd run test` / `npm.cmd run test:watch` scripts in `package.json`.
- **Debug logging system** for diagnosing runtime issues:
  - Rust backend: `log` + `simplelog` (`TermLogger` + `WriteLogger`) writing to console and `%APPDATA%/HotSAS Studio/logs/hotsas.log`.
  - Frontend logger (`src/utils/logger.ts`) with levels (trace/debug/info/warn/error), in-memory ring buffer (2000 entries), and forwarding to backend via `write_log` Tauri command.
  - `DebugLogPanel` component (`src/components/DebugLogPanel.tsx`) — modal window for viewing, copying, and clearing logs.
  - All Tauri commands instrumented with entry/exit logging (`create_rc_low_pass_demo_project`, `calculate_formula`, `load_formula_packs`, etc.).
  - `FormulaLibraryScreen` logs user actions: formula selection, input changes, calculate requests, success/failure.

### Fixed

- Black screen crash in `FormulaLibraryScreen` when modifying variable inputs.
- Defensive null/undefined checks for all arrays (`variables`, `equations`, `outputs`, `calculationResult.outputs`, `calculationResult.warnings`).
- Safe `event.currentTarget?.value` access in variable input `onChange` handlers.
- Wrapped `Workbench` in `ErrorBoundary` inside `App.tsx`.

### Changed

- `docs/testing/TESTING.md` — added v1.1.4-fix test section, updated test count to 63+ Rust + 12 frontend tests, added `npm.cmd run test` to pre-commit commands.
- `docs/formula_library/FORMULA_PACK_FORMAT.md` — clarified that frontend does not evaluate formulas; evaluation happens in Rust backend.

## [0.1.4] — Generic FormulaEnginePort

### Added

- Generic `FormulaEnginePort` methods: `evaluate_formula`, `evaluate_expression`, `validate_expression`.
- `SimpleFormulaEngine` allowlist evaluator supporting:
  - `fc = 1 / (2*pi*R*C)` (RC low-pass cutoff)
  - `V = I * R` (Ohm's law)
  - `Vout = Vin * R2 / (R1 + R2)` (Voltage divider)
- `FormulaService.calculate_formula(formula_id, variables)` via `FormulaRegistryService`.
- API DTOs and Tauri command `calculate_formula`.
- Formula Library UI: variable text inputs and **Calculate** button.
- Backend tests for generic formula engine, service, and API.

### Changed

- Formula packs YAML normalized to readable multiline `solve_for`.
- Old RC-specific commands (`calculate_rc_low_pass`) preserved for backward compatibility.

## [0.1.3] — FormulaPackLoader + FormulaRegistry

### Added

- `FormulaPack` / `FormulaPackMetadata` domain models in `hotsas_core`.
- `FormulaPackFileLoader` in `hotsas_adapters`: loads `.yaml`, `.yml`, `.json` packs from files or directories.
- `FormulaPackValidationError` and pack validation rules.
- `FormulaRegistryService` in `hotsas_application`: lists formulas, categories, metadata; validates bindings; detects duplicates.
- API DTOs: `FormulaPackDto`, `FormulaSummaryDto`, `FormulaDetailsDto`, `FormulaVariableDto`, `FormulaEquationDto`, `FormulaOutputDto`.
- Tauri commands: `load_formula_packs`, `list_formulas`, `list_formula_categories`, `get_formula`, `get_formula_pack_metadata`.
- Formula Library UI connected to backend registry.

### Changed

- `shared/formula_packs/*.yaml` normalized to valid multiline YAML.
- `docs/testing/TESTING.md` and `docs/formula_library/FORMULA_PACK_FORMAT.md` updated.

## [0.1.2] — Backend Test Expansion

### Added

- `EngineeringValue` parsing tests (positive, suffix, negative cases).
- `PreferredValues` tests for E24/E12/E6, boundary cases, invalid inputs, `generate_decade_values`.
- RC formula and circuit template binding tests.
- SPICE netlist export tests (positive + missing component/parameter errors).
- Markdown/HTML report export tests (including HTML escaping safety).
- JSON storage roundtrip, parent directory, and error tests.
- API error DTO and state error tests.
- Full backend vertical slice integration test (create → calculate → E24 → netlist → simulation → report → save/load).

## [0.1.1] — Formatting and Build/Test Infrastructure

### Added

- Prettier 3.8 with `.prettierrc` and `.prettierignore`.
- npm scripts: `format`, `format:check`, `typecheck`.
- `docs/testing/TESTING.md` with verification commands and smoke test checklist.
- Expanded root `.gitignore` for Rust, Node, Tauri, OS/IDE artifacts.

### Changed

- All Rust code formatted with `rustfmt`.
- All TypeScript/React code formatted with Prettier.

## [0.1.0] — Architecture Hardening

### Added

- `CircuitQueryService` in `hotsas_core` for reusable component/parameter access.
- Separate application services: `ProjectService`, `FormulaService`, `PreferredValuesService`, `CircuitTemplateService`, `NetlistGenerationService`, `SimulationService`, `ExportService`.
- `AppServices` as a composition facade instead of a god service.
- `ApiErrorDto` with structured `code`, `message`, `details`.
- Frontend directory structure: `api/`, `store/`, `types/`, `screens/`, `components/`.
- Tauri v2 permissions (`permissions/hotsas.toml`) and capabilities for custom commands.
- Windows GUI subsystem for release builds.

## [0.0.1] — Initial Vertical Slice

### Added

- Rust workspace: `hotsas_core`, `hotsas_ports`, `hotsas_application`, `hotsas_adapters`, `hotsas_api`.
- Domain models: `CircuitProject`, `CircuitModel`, `ComponentDefinition`, `ComponentInstance`, `Net`, `Wire`, `FormulaDefinition`, `CircuitTemplate`, `SimulationProfile`, `SimulationResult`, `ReportModel`.
- `EngineeringValue` / `ValueWithUnit` with prefix parsing (`p`, `n`, `u`, `m`, `k`, `M`).
- Preferred values module: E3, E6, E12, E24, E48, E96, E192.
- Hardcoded RC low-pass template, formula, and AC sweep profile.
- `JsonProjectStorage` adapter.
- `SimpleFormulaEngine` (RC-specific).
- `SpiceNetlistExporter` (RC-specific).
- `MockSimulationEngine` (RC-specific, 80-point AC sweep).
- `MarkdownReportExporter` and basic `HtmlReportExporter`.
- Tauri commands: `create_rc_low_pass_demo_project`, `calculate_rc_low_pass`, `nearest_e24_for_resistor`, `generate_spice_netlist`, `run_mock_ac_simulation`, `export_markdown_report`, `export_html_report`, `save_project_json`.
- React UI shell with 7 screens: Start, Schematic, Engineering Notebook, Formula Library, Component Library, Simulation Results, Export Center.
- Seed formula packs in `shared/formula_packs/`.
