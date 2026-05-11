# Latest Verification Log

## Current Version

[v3.1 - Component Model Mapping & SPICE Model Assignment (partial/completion pass)](./verification_logs/v3.1_component_model_mapping_spice_assignment.md)

## In-Progress Development Log

[v3.2 - Two-Port / Filter Network Analysis Foundation (PARTIAL)](./verification_logs/v3.2_two_port_filter_network_analysis.md)

```text
Current v3.2 status: PARTIAL; backend/API/CLI foundation PASS; UI deferred.
Latest evidence:
- Core two-port/filter domain models implemented in hotsas_core
- TwoPortFilterAnalysisService with validation, mock/template analysis,
  filter kind detection, metric estimation, CSV export, report section generation
- API facade: 7 methods (suggest, validate, run, get/clear last, export CSV, add to report)
- Tauri commands: 7 commands registered
- Frontend: TypeScript types, API wrappers, Zustand store additions
- CLI: hotsas-cli filter-analyze <path> [--method] [--out]
- Rust tests: filter_analysis_api_tests.rs 7 tests PASS
- Full Rust workspace: 250+ tests PASS, no regressions
- Frontend: 165 Vitest tests PASS, no regressions
- cargo fmt --check, cargo test, cargo build -p hotsas_cli --release PASS
- npm.cmd run typecheck, npm.cmd run test PASS
- npm.cmd run build, npm.cmd run tauri:build PASS
- git diff --check PASS

Remaining before ACCEPT:
- Frontend UI screen/component with port config, sweep controls, Bode chart,
  metrics table, diagnostics panel is deferred
- S-parameters deferred to v3.3
```

## v3.0 Summary

```text
Version: v3.0 — Simulation UX, ngspice Hardening, Probes & Graph Workflow
Backend/API/CLI commit: 47a946c
Frontend/docs commit: 1af7cd6
Fix/docs commit: ce67989
Branch: main
Push status: PASS / origin/main OK
Verification log: docs/testing/verification_logs/v3.0_simulation_ux_ngspice_probes_graphs.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (~400 Rust tests, exit code 0)
- cargo build -p hotsas_cli --release — PASS
- npm run format:check — PASS
- npm run typecheck — PASS
- npm test — PASS (157 frontend tests, 32 files)
- npm run build — PASS
- npm run tauri:build — PASS

CLI binary:
- EXE path: engine/target/release/deps/hotsas_cli.exe
- EXE size bytes: 3646464
- EXE SHA256: 79232DCD0625AA6FD92AA2F44242C9022C659ADC75150C109F6F143307124535

Desktop binary:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 13966848
- EXE SHA256: 4B12AE43E28FA0656D766B625156736AC7B2A17503442F996208B4980ABEEC8E

New in v3.0:
- SimulationDashboard with 6 tabs (setup/diagnostics/results/graph/history/export)
- NgspiceDiagnosticsCard, SimulationDiagnosticsPanel with severity + suggested fixes
- ProbeManager with default probe suggestions
- SimulationRunHistoryPanel with delete/clear actions
- SimulationGraphControls + SimulationGraphView with series visibility
- SimulationSeriesExportPanel for CSV/JSON export
- SimulationDiagnosticsService, SimulationHistoryService, SimulationGraphService
- 23 new Rust tests + 157 frontend tests (all pass)
- CLI simulate-diagnostics and simulation-history commands
- Architecture rule preserved: React remains view adapter only
```

## Previous Versions

- [v2.9 — User-Circuit Netlist & Simulation End-to-End](./verification_logs/v2.9_user_circuit_netlist_simulation_e2e.md)
- [v2.8 — Interactive Schematic Editing MVP](./verification_logs/v2.8_interactive_schematic_editing_mvp.md)
- [v2.7 — CLI / Headless Mode Foundation](./verification_logs/v2.7_cli_headless_mode.md)
- [v2.6 — Project Persistence / Save-Load UX Hardening](./verification_logs/v2.6_project_persistence_save_load_ux.md)
- [v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)
- [v2.4 — Real Component Parameters](./verification_logs/v2.4_real_component_parameters.md)
