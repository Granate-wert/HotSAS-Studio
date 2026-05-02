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
  - Load JSON formula pack
  - Load all builtin packs from directory in deterministic order
  - Reject invalid YAML and invalid packs

- **FormulaRegistry** (`application/tests/formula_registry_tests.rs`)
  - List formulas, categories, pack metadata
  - Find formula by id, category, linked template
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

---

## Test Summary

As of v1.1.4-fix, the Rust workspace runs **63+ tests** across all crates with **zero failures**, and the frontend runs **12 UI tests** with **zero failures**.

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
```
