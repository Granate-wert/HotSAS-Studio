# Changelog

All notable changes to HotSAS Studio are documented in this file.

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
