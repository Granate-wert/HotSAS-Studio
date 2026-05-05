# HotSAS Studio Testing Guide

## Purpose

This document lists the standard local verification commands and test coverage for HotSAS Studio.

---

## Rust Engine

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

To format Rust code:

```bash
cargo fmt
```

---

## Frontend

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd install
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```

To format frontend code:

```bash
npm.cmd run format
```

---

## Tauri Dev

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

---

## Tauri Release Build

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:build
```

The release executable is placed at:

```text
apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
```

---

## Why npm.cmd Is Used

On this Windows PowerShell setup, `npm.ps1` can be blocked by Execution Policy. Use `npm.cmd` for project scripts.

---

## Manual v1 Vertical Slice Smoke Check

1. Start the app with `npm.cmd run tauri:dev`.
2. Open the Start screen.
3. Create the RC low-pass demo project.
4. Verify that the schematic renders.
5. Calculate `fc`.
6. Request the nearest E24 value.
7. Generate the SPICE netlist.
8. Run the mock AC simulation.
9. Verify that the graph renders.
10. Export the Markdown report.
11. Export the HTML report.
12. Save the project JSON.

## Manual v1.6 Selected Region Smoke Check

1. Open the **Schematic** screen.
2. Switch to the **Region** tab in the side panel.
3. Select `R1` and `C1` via checkboxes.
4. Click **Preview** — verify preview card appears with component/net info.
5. Click **Analyze** — verify result card appears with status, template match, netlist fragment.
6. Click **Clear** — verify selection resets.

---

## Backend Test Coverage

### v1.1.2 — Core Verification

- **EngineeringValue parsing** (`core/tests/engineering_value_tests.rs`)
  - Positive cases: `10k`, `100n`, `1u`, `1M`, `1.5k`
  - Unit suffix cases: `100nF`, `10kOhm`, `1MHz`
  - Negative cases: empty string, `abc`, `10x`, `k10`, `1..5k`, `NaN`, `inf`

- **Preferred Values** (`core/tests/preferred_values_tests.rs`)
  - E24/E12/E6 nearest/lower/higher selection
  - Boundary cases: `9.9k`, `10k`, `10.1k`, `100n`, `1u`, `1M`
  - Invalid inputs: `0`, `-1`, `NaN`, `Infinity`
  - `generate_decade_values` structural correctness (sorted, unique, positive, finite)

- **RC Formula** (`core/tests/rc_formula_tests.rs`)
  - Formula identity and contract

- **Circuit Template** (`core/tests/circuit_template_tests.rs`)
  - Component presence (`V1`, `R1`, `C1`)
  - Net presence (`net_in`, `net_out`, `gnd`)
  - Signal path wiring
  - Formula-to-template binding

- **Formula Engine** (`adapters/tests/formula_engine_tests.rs`)
  - RC low-pass cutoff calculation
  - Wrong unit rejection
  - Zero/negative value rejection

- **Netlist Export** (`adapters/tests/netlist_export_tests.rs`)
  - Required fragments: `V1`, `R1`, `C1`, `net_in`, `net_out`, `.ac`, `.end`
  - Missing parameter/component error cases

- **Report Export** (`adapters/tests/report_export_tests.rs`)
  - Markdown sections and BOM table
  - HTML escaping safety (`<script>alert(1)</script>` → escaped)

- **JSON Storage** (`adapters/tests/json_storage_tests.rs`)
  - Save/load roundtrip
  - Parent directory creation
  - Missing/invalid/empty file errors

- **Full Vertical Slice** (`adapters/tests/full_vertical_slice_tests.rs`)
  - End-to-end backend flow: create → calculate → E24 → netlist → simulation → report → save/load

- **Application Services** (`application/tests/services_tests.rs`)
  - Demo creation and nearest E24
  - Missing parameter handling
  - Missing AC profile handling

- **API Errors** (`api/tests/api_error_tests.rs`)
  - Structured DTO codes and messages
  - State errors before project creation

- **Dependency Boundaries** (`api/tests/dependency_boundaries.rs`)
  - Crate dependency direction enforcement

- **Circuit Query** (`core/tests/circuit_query.rs`)
  - Component and parameter retrieval
  - Missing parameter reporting

- **Error DTO** (`api/tests/error_dto.rs`)
  - Structured error exposure

### v1.1.3 — Formula Registry

- **FormulaPack Loader** (`adapters/tests/formula_pack_loader_tests.rs`)
  - Load `filters.yaml` with RC low-pass formula
  - Load `basic_electronics.yaml` with `ohms_law` and `voltage_divider`
  - Load `op_amp.yaml` and `smps.yaml`
  - Load JSON formula pack
  - Load all builtin packs from directory in deterministic order
  - Reject invalid YAML and invalid packs
  - Reject formula with missing `id`
  - Reject formula with no equations

- **FormulaRegistry** (`application/tests/formula_registry_tests.rs`)
  - List formulas, categories, pack metadata
  - Find formula by id, category, linked template
  - Find `ohms_law` and `voltage_divider`
  - Validate linked template bindings
  - Reject duplicate formula ids and missing formulas

- **Formula Registry API** (`api/tests/formula_registry_api_tests.rs`)
  - Load pack metadata and list formulas
  - Return formula details and not-found errors

### v1.1.4 — Generic Formula Engine

- **Generic Formula Engine** (`adapters/tests/formula_engine_generic_tests.rs`)
  - `evaluate_formula` for RC low-pass with `R=10k`, `C=100n` → `fc ≈ 159.15 Hz`
  - `evaluate_formula` for Ohm's law with `I=2mA`, `R=10k` → `V=20V`
  - `evaluate_formula` for voltage divider with `Vin=5V`, `R1=10k`, `R2=10k` → `Vout=2.5V`
  - Missing variable rejection
  - Wrong unit rejection
  - Zero/negative value rejection
  - Unsupported expression rejection
  - `validate_expression` supported/unsupported results

- **Generic Formula Service** (`application/tests/formula_service_generic_tests.rs`)
  - Calculate formula from registry by `formula_id`
  - Missing formula reporting
  - RC low-pass compatibility path still works

- **Formula Calculation API** (`api/tests/formula_calculation_api_tests.rs`)
  - `calculate_formula` for RC low-pass
  - `calculate_formula` for Ohm's law
  - Missing formula, missing variable, unsupported expression errors

### v1.1.4-fix — Generic Formula Engine Completion Gate

- **ErrorBoundary** (`src/components/ErrorBoundary.test.tsx`)
  - Renders children when healthy
  - Catches render errors and displays fallback UI
  - Supports custom fallback
  - Allows reset after error

- **FormulaLibraryScreen UI workflows** (`src/screens/FormulaLibraryScreen.test.tsx`)
  - Loads and displays packs, categories, formulas
  - Shows formula details on selection
  - Allows changing variable inputs without crashing
  - Calls `calculateFormula` and displays results
  - Switches between formulas
  - Displays backend error alerts
  - Handles null defaults gracefully
  - Handles malformed calculation results gracefully

### v1.1.4-fix.2 — Hygiene, Formula Pack YAML, HTML Escaping, Verification

This stage verifies:

- Formula pack YAML validity and readability
- `FormulaPackLoader` runtime loading for all builtin packs
- `FormulaRegistry` contains `rc_low_pass_cutoff`, `ohms_law`, `voltage_divider`
- Generic `FormulaEngine` evaluation for supported expressions
- `FormulaService` calculation via registry
- API `calculate_formula` end-to-end
- Tauri command registration (`calculate_formula`, `write_log`)
- `FormulaLibraryScreen` backend calculation (React does not compute formulas)
- Safe HTML escaping in `MarkdownReportExporter`
- `cargo fmt --check` and `npm.cmd run format:check` pass

### v1.1.5 — Exact E-Series Tables

- **Exact static tables** (`core/src/preferred_value_tables.rs`)
  - E3/E6/E12/E24/E48/E96/E192 base values
  - Length correctness (3/6/12/24/48/96/192)
  - Sorted, unique, positive, finite
  - Known values for E48, E96, E192

- **Preferred value lookup** (`core/tests/preferred_values_tests.rs`)
  - `nearest_preferred_value` via exact tables
  - `lower_preferred_value` inclusive behavior
  - `higher_preferred_value` inclusive behavior
  - `generate_decade_values` for E96 decade 10–100
  - `calculate_error_percent` accuracy
  - Invalid input handling (0, -1, NaN, Infinity)

### v1.2 — Project Package Storage `.circuit`

- **Core models** (`core/tests/project_package_tests.rs`)
  - `ProjectPackageManifest` serialization/deserialization
  - `ProjectPackageFiles` default paths
  - `ProjectPackageValidationReport` missing files representation

- **Package Storage Adapter** (`adapters/tests/project_package_storage_tests.rs`)
  - Save creates `.circuit` folder and required files
  - Save creates subdirectories (`reports`, `results`, `models/spice`, etc.)
  - Load roundtrip preserves project id, name, components
  - Validation reports valid for complete package
  - Validation reports missing `project.json`
  - Package dir without `.circuit` extension returns error

- **Project Package Service** (`application/tests/project_package_service_tests.rs`)
  - `ProjectPackageService` save/load roundtrip
  - `AppServices` exposes project package service

- **Project Package API** (`api/tests/project_package_api_tests.rs`)
  - `save_project_package` without project → state error
  - `create_rc_low_pass_demo_project` then save → manifest
  - `validate_project_package` returns valid report

- **Frontend** (`src/components/Workbench.tsx`)
  - Save `.circuit` package button
  - Load `.circuit` package button
  - Package path input and result display

---

## v1.3 — Schematic Editor Foundations

### Tests

- **Symbol / Pin Models** (`core/tests/symbol_pin_tests.rs`)
  - Resistor symbol has 2 passive pins
  - Capacitor symbol has 2 passive pins
  - Voltage source has p/n pins
  - Ground has gnd pin
  - Pin positions are finite

- **Circuit Validation** (`application/tests/circuit_validation_tests.rs`)
  - Valid RC low-pass has no errors
  - Missing ground returns error
  - Empty circuit returns error
  - Duplicated component id returns error
  - Missing required parameter returns error
  - Floating net returns warning

- **Schematic Editor API** (`api/tests/schematic_editor_api_tests.rs`)
  - `get_selected_component` R1 returns parameters and symbol
  - Missing component id returns error
  - `update_component_parameter` changes project
  - Invalid value returns error
  - `validate_current_circuit` returns report

- **Frontend Schematic** (`src/components/schematic/__tests__/`)
  - PropertyPanel renders placeholder when no selection
  - PropertyPanel renders selected component parameters
  - ValidationPanel renders Validate Circuit button
  - ValidationPanel calls onValidate after validate click

---

## v1.4-fix — Engineering Notebook Integration, Documentation, Verification

### Tests

- **Core Notebook Models** (`core/tests/notebook_models_tests.rs`)
  - `EngineeringNotebook` serializes/deserializes
  - `NotebookBlock` stores result
  - `NotebookEvaluationResult` stores outputs
  - `NotebookHistoryEntry` stores status

- **Engineering Notebook Service** (`application/tests/engineering_notebook_tests.rs`)
  - Assignment `R = 10k` creates variable
  - Formula call with literal values `rc_low_pass_cutoff(R=10k, C=100n)`
  - Formula call with variables `rc_low_pass_cutoff(R=R, C=C)`
  - `ohms_law(I=2m, R=10k)` returns `V=20V`
  - `voltage_divider(Vin=5, R1=10k, R2=10k)` returns `Vout=2.5V`
  - `nearestE(15.93k, E24, Ohm)` returns nearest E24 value
  - `nearestE(15.93k, E96, Ohm)` returns nearest E96 value
  - Unsupported expression `sin(5)` returns controlled unsupported
  - Malformed input returns controlled error

- **Notebook API** (`api/tests/notebook_api_tests.rs`)
  - `evaluate_notebook_input` assignment returns variable
  - `evaluate_notebook_input` formula returns output
  - `get_notebook_state` returns variables/history
  - `clear_notebook` clears state
  - `apply_notebook_output_to_component` without project returns state error
  - Unsupported input returns controlled unsupported result

- **Frontend Notebook Components** (`src/components/notebook/__tests__/NotebookComponents.test.tsx`)
  - `NotebookInput` renders placeholder and buttons
  - Evaluate/Clear buttons call handlers
  - `NotebookResultCard` displays output
  - `NotebookResultCard` displays unsupported hint
  - `NotebookVariableTable` displays variables
  - `NotebookHistory` displays entries
  - `PreferredValueQuickTools` renders buttons and inserts templates
  - `ApplyNotebookOutputPanel` renders apply buttons

---

## v1.5 — Component Library Foundation

### Tests

- **Core Component Library** (`core/tests/component_library_tests.rs`)
  - Built-in library has at least 12 components
  - Component IDs are unique
  - Every component has id, name, category
  - Every component has at least one tag
  - Generic resistor, capacitor, op-amp exist
  - Resistor has resistance parameter
  - Capacitor has capacitance parameter
  - Symbol IDs referenced by components exist in seed symbols
  - Footprint IDs referenced by components exist in library
  - Footprints exist for common packages

- **Component Library Storage Adapter** (`adapters/tests/component_library_storage_tests.rs`)
  - Load built-in library returns non-empty library
  - Save and load library JSON roundtrip preserves components
  - Save library creates parent directories
  - Load missing library returns controlled error
  - Load invalid JSON returns controlled error

- **Component Library Service** (`application/tests/component_library_service_tests.rs`)
  - List components returns all definitions
  - Search by name returns matching component
  - Search is case-insensitive
  - Filter by category works
  - Filter by has_footprint works
  - Get component by ID returns definition
  - Get missing component returns error
  - Assign component to instance updates definition_id
  - Assign preserves existing overridden parameters

- **Component Library API** (`api/tests/component_library_api_tests.rs`)
  - Load built-in component library returns metadata
  - List components returns non-empty list
  - Search components returns filtered results
  - Get component details returns parameters and previews
  - Assign without project returns state error
  - Create RC demo then assign generic resistor to R1 works
  - Assign unknown component returns error
  - Assign unknown instance returns error

- **Frontend Component Library** (`src/components/component-library/__tests__/ComponentLibrary.test.tsx`)
  - Screen renders and loads built-in library on mount
  - Search input calls searchComponents
  - Component table displays components
  - Selecting component calls getComponentDetails
  - Details panel displays parameters
  - Empty state renders without crash
  - Error state renders readable message

---

### v1.6 — Selected Region Analysis Foundation

- **Selected Region Analysis Service** (`application/tests/selected_region_analysis_tests.rs`)
  - Preview selected region returns components and nets
  - Analyze selected region matches RC low-pass template
  - Validate empty selection returns error
  - Preview single component has boundary nets
  - Analyze unsupported region returns partial result with netlist/warnings

- **Frontend Selected Region** (`src/components/selected-region/__tests__/SelectedRegionPanel.test.tsx`)
  - Renders component checkboxes
  - Selecting components updates count
  - Preview button calls backend and shows preview card
  - Analyze button calls backend and shows result card
  - Clear button resets selection

---

### v1.7 — Export Center v1

- **Export Center API** (`api/tests/export_center_api_tests.rs`)
  - List export capabilities returns all nine formats
  - Export without project returns state error
  - Export SPICE netlist with project returns success
  - Export BOM CSV contains expected headers
  - Export SVG schematic contains SVG tag
  - Export history returns empty list initially

- **Frontend Export Center** (`src/screens/ExportScreen.test.tsx`)
  - Renders export center title and description
  - Disables export buttons when no project is loaded
  - Enables export buttons when project exists
  - Calls onLoadCapabilities on mount when capabilities are empty
  - Calls onExport with correct format when button clicked
  - Toggles write-to-file switch and shows output directory input
  - Displays last export result when provided

### v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate

- **App Diagnostics Service** (`application/tests/app_diagnostics_tests.rs`)
  - Diagnostics report contains expected module IDs (formula_registry, component_library, export_center, simulation, import_models, project_package, schematic_editor, engineering_notebook, selected_region)
  - Component library module reports ready/limited, not panic
  - Export center module reports 9 capabilities
  - Simulation module handles ngspice unavailable as warning/limited, not error
  - Import models module reports SPICE and Touchstone support
  - Readiness self-check returns checks with pass/warn/fail/not_run statuses

- **App Diagnostics API** (`api/tests/app_diagnostics_api_tests.rs`)
  - `get_app_diagnostics` returns report with modules
  - `run_readiness_self_check` returns checks
  - API DTO conversion preserves module statuses
  - Formula registry module reports ready
  - Component library module reports ready/limited

- **Frontend Diagnostics** (`src/screens/__tests__/DiagnosticsScreen.test.tsx`)
  - Renders diagnostics title
  - Loads module cards
  - Shows Ready/Limited/Unavailable statuses
  - Refresh diagnostics calls backend API
  - Run readiness self-check calls backend API
  - Shows backend error message if command fails
  - Does not automatically run heavy self-check on first render

## Manual v1.7 Export Center Smoke Check

1. Open the **Export Center** screen.
2. Verify that 9 export format buttons are listed (Markdown, HTML, SPICE, CSV Sim, BOM CSV, BOM JSON, Library JSON, SVG, Altium).
3. Click **Markdown Report** — verify content preview appears.
4. Toggle **Write to file** switch, set output directory, click **SPICE Netlist** — verify success message with file path.
5. Click **Load History** — verify history entries appear.

---

## Manual v1.10 Internal Alpha Smoke Check

- [ ] Release EXE starts.
- [ ] No console window appears for release EXE.
- [ ] Start screen opens.
- [ ] Diagnostics screen opens.
- [ ] Refresh diagnostics works.
- [ ] Run readiness self-check works.
- [ ] Formula Library screen opens.
- [ ] Engineering Notebook screen opens.
- [ ] Component Library screen opens.
- [ ] Simulation screen opens.
- [ ] Import Models screen opens.
- [ ] Export Center screen opens.
- [ ] ngspice unavailable is shown as controlled warning/limited status.

---

## v2.0 — Product Beta Integration

### Rust tests

- product workflow service tests (`engine/application/tests/product_workflow_tests.rs`)
- product workflow API tests (`engine/api/tests/product_workflow_api_tests.rs`)
- diagnostics regression
- integration smoke self-check

### Frontend tests

- Product Beta / Project Hub screen (`src/screens/__tests__/ProductBetaScreen.test.tsx`)
- guided workflow cards
- self-check actions
- error states

### Manual v2.0 Product Beta Smoke Check

- [ ] Release EXE starts.
- [ ] No console window appears.
- [ ] Start / Project Hub opens.
- [ ] Create integrated demo project works.
- [ ] Schematic screen opens and shows RC demo.
- [ ] Formula Library opens.
- [ ] Formula calculation works.
- [ ] Engineering Notebook opens.
- [ ] Notebook assignment/formula command works or limited status shown.
- [ ] Component Library opens.
- [ ] Component details open.
- [ ] Import Models screen opens.
- [ ] SPICE text import smoke works.
- [ ] Touchstone text import smoke works.
- [ ] Simulation screen opens.
- [ ] Mock simulation works.
- [ ] ngspice unavailable is controlled warning if ngspice absent.
- [ ] Selected Region screen/panel opens.
- [ ] Region preview/analyze works for RC demo or limited status shown.
- [ ] Export Center opens.
- [ ] Markdown export works.
- [ ] SPICE netlist export works.
- [ ] BOM export works.
- [ ] SVG schematic export works.
- [ ] Diagnostics opens.
- [ ] Run readiness self-check works.
- [ ] Product Beta screen opens.
- [ ] Refresh workflow status works.
- [ ] Run product beta self-check works.

---

## v2.2 — DC-DC Calculators and Templates

### Rust tests

- Formula pack loader tests updated for `dcdc.yaml` (`adapters/tests/formula_pack_loader_tests.rs`)
- All existing suites regression: 200+ tests PASS

### Frontend tests

- 76 UI tests PASS (existing suites + DC-DC screen integration)

### Manual v2.2 DC-DC Calculator Smoke Check

1. Open the **DC-DC Calculator** screen from the left sidebar.
2. Select **Buck** topology.
3. Enter `Vin=12V`, `Vout=5V`, `Iout=1A`, `fs=100kHz`.
4. Click **Calculate**.
5. Verify duty cycle ≈ `0.4167`.
6. Verify minimum inductance is a positive finite value.
7. Verify output capacitor ripple current is shown.
8. Verify switch peak current is shown.
9. Verify CCM boundary current is shown.
10. Verify warnings/assumptions panel is visible.
11. Select **Boost** topology — verify calculation updates.
12. Select **Inverting Buck-Boost** — verify calculation updates.
13. Select **4-Switch Buck-Boost** — verify controlled placeholder with limitation warnings.
14. Click **Netlist Preview** — verify SPICE-like structural preview appears.
15. Click **Mock Transient** — verify transient preview result appears.

## v2.4 — Real Component Parameters

### Rust tests

- Core typed parameter schema (`core/src/component_parameters.rs`)
  - `ComponentParameterSchema`, `ComponentParameterDefinition`, `ComponentParameterGroup`
  - `ComponentTolerance` with symmetric/asymmetric/minmax variants
  - `ParameterValidationError` unit/range/tolerance/missing variants
  - Typed bundles for 8 categories: Resistor, Capacitor, Inductor, Diode, BJT, MOSFET, OpAmp, Regulator
  - `schema_for_category()` builder returns correct grouped schemas
  - `validate_map()` checks units, ranges, tolerances, missing required fields

- Core component seeds (`core/src/component_seeds.rs`)
  - 27 built-in components with real-like parameters
  - Resistor seeds: 10k 0603, 1k axial, 100R 0805
  - Capacitor seeds: 100n X7R 0603, 10u X5R 0805, 100u electrolytic
  - Semiconductor seeds: 1N4148, SS14, 2N2222, 2N2907, IRFZ44N, LM358
  - SMD and through-hole footprints

- Component parameter service (`application/tests/component_parameter_service_tests.rs`)
  - `schema_for_component()` returns schema for known components
  - `validate_component()` returns empty issues for valid parameters
  - `validate_component()` returns issues for out-of-range values
  - `extract_typed_parameters()` builds typed bundles from flat params
  - `resolve_instance_parameters()` merges instance overrides with base

- API DTOs (`api/src/dto.rs`) — compile-time verified
- API facade (`api/src/facade.rs`) — compile-time verified
- Tauri commands (`apps/desktop-tauri/src-tauri/src/lib.rs`) — compile-time verified

### Frontend tests

- 89 UI tests PASS (existing suites + ComponentDetailsPanel typed params integration)

### Manual v2.4 Component Parameters Smoke Check

1. Open the **Component Library** screen.
2. Click on a **Resistor** component (e.g., `R_10k_0603`).
3. Verify that the **Typed Parameters** card shows:
   - Resistance: `10 kΩ`
   - Tolerance: `±1%`
   - Power rating: `0.1 W`
   - Temperature coefficient: `±100 ppm/°C`
4. Click on a **Capacitor** component (e.g., `C_100n_0603_X7R`).
5. Verify that the **Typed Parameters** card shows:
   - Capacitance: `100 nF`
   - Tolerance: `±10%`
   - Voltage rating: `50 V`
   - Dielectric: `X7R`
6. Click on a **MOSFET** component (e.g., `M_IRFZ44N`).
7. Verify that the **Typed Parameters** card shows VDS, RDS(on), ID fields.
8. Verify that no console errors appear during navigation.

## v2.5 — Schematic Editor Hardening

### Rust tests

- Schematic editing service (`application/tests/schematic_editing_tests.rs`)
  - Add resistor component succeeds
  - Duplicate instance id returns error
  - Move component updates backend position
  - Move unknown component returns error
  - Delete component removes instance
  - Delete unknown component returns error
  - Connect pins creates net and wire
  - Connect unknown component returns error
  - Rename net succeeds
  - Rename net empty name returns error

- API facade (`api/src/facade.rs`) — compile-time verified
- Tauri commands (`apps/desktop-tauri/src-tauri/src/lib.rs`) — compile-time verified

### Frontend tests

- 95 UI tests PASS (existing suites + SchematicScreen v2.5 integration)

### Manual v2.5 Schematic Editor Smoke Check

1. Open the **Schematic Editor** screen.
2. Verify that the **Schematic Toolbar** is visible with Delete, Connect, Rename Net buttons.
3. Verify that the **Component Palette** shows Resistor, Capacitor, Inductor, etc.
4. Click **Add Resistor** — verify new resistor appears on canvas.
5. Drag the resistor — verify it moves and position updates via backend.
6. Select the resistor — verify **Delete** button becomes enabled.
7. Click **Delete** — verify component is removed.
8. Add two components and click **Connect** — verify Connection Panel appears.
9. Select from/to components and pins, click **Connect** — verify wire appears.
10. Click **Rename Net** — verify Net Label Editor appears.
11. Select a net, enter new name, click **Rename** — verify net name updates.
12. Verify validation warnings appear in the Validation tab after edits.
13. Verify old screens still work: Component Library, Formula Library, Export Center, DC-DC.

## v2.6 — Project Persistence / Save-Load UX Hardening

### Rust tests

- **Project Session Service** (`engine/application/tests/project_session_tests.rs`)
  - New session is clean (no project, not dirty)
  - `mark_dirty` sets dirty flag
  - `save_project_as` sets path and clears dirty
  - `save_current_project` without path returns error
  - `open_project_package` without confirm on dirty returns error
  - Recent projects updated after save
  - Remove recent project works

- **Project Session API** (`engine/api/tests/project_session_api_tests.rs`)
  - `get_project_session_state` returns clean initially
  - `save_project_as` without project returns error
  - `open_project_package` without confirm on dirty fails
  - `list_recent_projects` works

- **Dependency Boundaries** (`api/tests/dependency_boundaries.rs`) — verified no `hotsas_adapters` in `hotsas_api` dependencies

### Frontend tests

- **Project Toolbar** (`src/components/project/__tests__/ProjectToolbar.test.tsx`)
  - Renders no-project state
  - Shows dirty indicator when dirty
  - Calls onSave when Save clicked

- **Recent Projects Panel** (`src/components/project/__tests__/RecentProjectsPanel.test.tsx`)
  - Renders empty state
  - Renders recent projects
  - Calls onRemove when remove clicked

- **Unsaved Changes Banner** (`src/components/project/__tests__/UnsavedChangesBanner.test.tsx`)
  - Renders nothing when not dirty
  - Renders banner when dirty

### Manual v2.6 Save/Load Smoke Check

1. Open the app.
2. Create the RC low-pass demo project.
3. Verify **Project Toolbar** shows project name and dirty indicator.
4. Add a component via the palette.
5. Verify dirty indicator appears.
6. Click **Save As**, enter a path, confirm.
7. Verify dirty indicator clears.
8. Verify the project appears in **Recent Projects**.
9. Close and reopen the app.
10. Open the saved project from **Recent Projects**.
11. Verify schematic edits are preserved.
12. Make another edit, then try to open a different project.
13. Verify **Unsaved Changes** confirmation appears.

---

## Test Summary

As of v2.6, the Rust workspace runs **200+ tests** across all crates with **zero failures**, and the frontend runs **103 UI tests** with **zero failures**.

---

## Commands Before Commit

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test

cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
npm.cmd run test
npm.cmd run tauri:build
```
