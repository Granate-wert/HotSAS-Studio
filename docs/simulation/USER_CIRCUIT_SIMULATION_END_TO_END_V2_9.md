# HotSAS Studio v2.9 — User-Circuit Netlist & Simulation End-to-End

## Overview

This document describes the end-to-end user-circuit simulation workflow introduced in v2.9. After v2.8 enabled interactive schematic editing, v2.9 closes the loop:

```
Build circuit → Validate → Generate SPICE netlist → Select profile → Run simulation → View results/graphs/table → Add to report/export
```

This is the first full user simulation workflow for user-built circuits, not just demo/mock vertical slices.

## Architecture

### Backend (Rust) — Source of Truth

All netlist generation, simulation execution, and result production happens in the Rust backend. The frontend remains a view adapter only.

| Layer       | Component                         | Responsibility                                      |
| ----------- | --------------------------------- | --------------------------------------------------- |
| Core        | `UserCircuitSimulationProfile`    | Domain model for simulation profiles                |
| Core        | `UserCircuitSimulationRun`        | Domain model for simulation run + result            |
| Application | `SimulationWorkflowService`       | Orchestrates preflight, netlist, execution, caching |
| Adapters    | `UserCircuitSpiceNetlistExporter` | Generic SPICE netlist from arbitrary user circuits  |
| API         | Facade methods                    | 7 new DTO methods for frontend                      |
| CLI         | `user-circuit-simulate`           | CLI command for saved user projects                 |

### Frontend (React/TypeScript)

| Component                      | Responsibility                 |
| ------------------------------ | ------------------------------ |
| `UserCircuitSimulationPanel`   | Main orchestrator panel        |
| `SimulationProfileSelector`    | Profile list + selection       |
| `SimulationProbeSelector`      | Probe checklist                |
| `SimulationPreflightCard`      | Validation results display     |
| `SimulationRunControls`        | Engine selector + Run button   |
| `UserCircuitSimulationResults` | Results container              |
| `SimulationSeriesChart`        | Wraps chart with series data   |
| `SimulationMeasurementsTable`  | Mantine Table for measurements |
| `SimulationRawOutputCard`      | Netlist/raw excerpt display    |

## Simulation Workflow

### 1. List Default Profiles

`SimulationWorkflowService::list_default_simulation_profiles()` returns 4 built-in profiles:

- `mock-op` — Operating Point (Mock engine)
- `mock-ac` — AC Sweep (Mock engine)
- `mock-transient` — Transient (Mock engine)
- `auto-ac` — AC Sweep (Auto engine, fallback to mock)

### 2. Suggest Probes

`SimulationWorkflowService::suggest_simulation_probes()` scans project nets and suggests `NodeVoltage` probes for each net.

### 3. Preflight Validation

`SimulationWorkflowService::validate_circuit_for_simulation()` checks:

- Schematic has components
- Schematic has nets
- Each non-ground component has connections
- Probe targets resolve to existing nets/components
- Ground reference exists
- Netlist generates successfully

Returns `SimulationPreflightResult` with `can_run`, `blocking_errors`, `warnings`, `generated_netlist_preview`.

### 4. Run Simulation

`SimulationWorkflowService::run_user_circuit_simulation()`:

1. Generates netlist via `UserCircuitSpiceNetlistExporter`
2. Appends analysis directive (`.ac`, `.op`, `.tran`)
3. Dispatches to engine based on profile:
   - `Mock` → `MockSimulationEngine`
   - `Ngspice` → `NgspiceSimulationService`
   - `Auto` → tries ngspice, falls back to mock with warning
4. Maps `SimulationResult` → `UserCircuitSimulationResult`
5. Caches run in session-local `Mutex<BTreeMap>`

### 5. Netlist Generation

`UserCircuitSpiceNetlistExporter` generates SPICE from arbitrary user circuits:

- Reads `ComponentInstance::overridden_parameters` (set via QuickParameterEditor in v2.8)
- Ground detection: `ground_reference` definition_id or net named "gnd"/"ground"
- Ground nets renamed to SPICE node `0`
- Component SPICE letters: R, C, L, V, D
- Sanitizes net names: lowercase, underscores

### 6. Engine Fallback

`Auto` mode tries ngspice first; on any failure falls back to mock and adds warning:

```
"ngspice unavailable in auto mode; fallback to mock engine"
```

### 7. Report Integration

`simulation_result_to_report_section()` creates a `ReportSection` with:

- Paragraph with profile name and engine
- DataTable for measurements
- Paragraph with series count
- WarningList for warnings

## CLI Integration

New CLI command:

```powershell
hotsas-cli user-circuit-simulate project.circuit mock-ac --engine Mock --json
hotsas-cli user-circuit-simulate project.circuit mock-op --engine Auto --out result.json
```

Profiles supported:

- `mock-ac`, `ac-sweep` → AC Sweep
- `mock-op`, `operating-point` → Operating Point
- `mock-transient`, `transient` → Transient

## API Methods (Tauri Commands)

| Command                                   | DTO                                   | Description                        |
| ----------------------------------------- | ------------------------------------- | ---------------------------------- |
| `list_user_circuit_simulation_profiles`   | `UserCircuitSimulationProfileDto[]`   | List default profiles              |
| `suggest_user_circuit_simulation_probes`  | `SimulationProbeDto[]`                | Suggest probes for current circuit |
| `validate_current_circuit_for_simulation` | `SimulationPreflightResultDto`        | Preflight validation               |
| `run_current_circuit_simulation`          | `UserCircuitSimulationRunDto`         | Run simulation                     |
| `get_last_user_circuit_simulation`        | `UserCircuitSimulationRunDto \| null` | Get cached last run                |
| `clear_last_user_circuit_simulation`      | `void`                                | Clear cached run                   |
| `add_last_simulation_to_advanced_report`  | `AdvancedReportDto`                   | Add to report                      |

## Test Coverage

### Rust Tests

| Test File                                                            | Count | Status  |
| -------------------------------------------------------------------- | ----- | ------- |
| `engine/application/tests/user_circuit_simulation_workflow_tests.rs` | 11    | ✅ Pass |
| `engine/api/tests/user_circuit_simulation_api_tests.rs`              | 9     | ✅ Pass |
| `engine/cli/tests/cli_integration.rs` (user-circuit section)         | 4     | ✅ Pass |
| `engine/adapters/tests/user_circuit_netlist.rs`                      | 1     | ✅ Pass |

### Frontend Tests

| Test Suite        | Count | Status  |
| ----------------- | ----- | ------- |
| Vitest all suites | 132   | ✅ Pass |

## Acceptance Criteria (UCS-001..UCS-020)

| ID      | Criterion                                                               | Status     |
| ------- | ----------------------------------------------------------------------- | ---------- |
| UCS-001 | SimulationWorkflowService exists with preflight + run                   | ✅         |
| UCS-002 | UserCircuitSpiceNetlistExporter generates SPICE from arbitrary circuits | ✅         |
| UCS-003 | Ground nets renamed to 0; ground_reference detected                     | ✅         |
| UCS-004 | Netlist reads overridden_parameters                                     | ✅         |
| UCS-005 | Profile list returns OP/AC/Transient + Auto                             | ✅         |
| UCS-006 | Probe suggestion scans nets                                             | ✅         |
| UCS-007 | Preflight validates components, nets, probes, ground                    | ✅         |
| UCS-008 | Mock engine runs all three analysis types                               | ✅         |
| UCS-009 | Ngspice engine path exists (fallback tested)                            | ✅         |
| UCS-010 | Auto fallback produces warning                                          | ✅         |
| UCS-011 | Result contains measurements + series                                   | ✅         |
| UCS-012 | Last-run cache session-local                                            | ✅         |
| UCS-013 | Report section generated                                                | ✅         |
| UCS-014 | API facade exposes 7 methods                                            | ✅         |
| UCS-015 | Tauri commands wired + permissions                                      | ✅         |
| UCS-016 | CLI simulate user-built project                                         | ✅         |
| UCS-017 | Persistence policy documented (session-only)                            | ✅         |
| UCS-018 | Rust + frontend tests pass                                              | ✅         |
| UCS-019 | Docs/logs updated                                                       | ✅         |
| UCS-020 | Desktop/CLI builds pass                                                 | ⏳ Pending |

## Known Limitations

- Simulation result cache is session-local only (not persisted to `.circuit` file)
- Only R, C, L, V, D component types supported in netlist generation
- Full waveform viewer not implemented
- Advanced probe expression language not implemented
- Ngspice must be installed separately for real SPICE simulation

## Changes Since v2.8

- Added `SimulationWorkflowService` with full workflow orchestration
- Added `UserCircuitSpiceNetlistExporter` for generic netlist generation
- Added 7 new API facade methods + DTOs
- Added 7 new Tauri commands
- Added frontend simulation panel with 9 components
- Added `user-circuit-simulate` CLI command
- Fixed `validate_circuit_for_simulation` `can_run` logic bug
