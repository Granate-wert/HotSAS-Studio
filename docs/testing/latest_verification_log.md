# Latest Verification Log

## Current Version

[v2.9 — User-Circuit Netlist & Simulation End-to-End](./verification_logs/v2.9_user_circuit_netlist_simulation_e2e.md)

## v2.9 Summary

```text
Version: v2.9 — User-Circuit Netlist & Simulation End-to-End
Implementation commit: 8e76764
Branch: main
Push status: PASS / origin/main OK
Verification log: docs/testing/verification_logs/v2.9_user_circuit_netlist_simulation_e2e.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (~400 Rust tests, exit code 0)
- cargo build -p hotsas_cli --release — PASS
- npm run format:check — PASS
- npm run typecheck — PASS
- npm test — PASS (132 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

CLI binary:
- EXE path: engine/target/release/hotsas-cli.exe
- EXE size bytes: 3550720
- EXE SHA256: 244D45AB852E0D2219861D38FADD0A1C099E3AFE521D000F52A4218A3600545F

Desktop binary:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 13759488
- EXE SHA256: 68E85F3743BEC380D7BDC7C28631493C23D9636EEFF39D9334098B1DA3F0DB38

New in v2.9:
- User-circuit simulation workflow end-to-end
- SimulationWorkflowService with preflight validation, netlist generation, engine dispatch
- UserCircuitSpiceNetlistExporter: generic SPICE from arbitrary user circuits
- 7 new API facade methods + DTOs + Tauri commands
- Simulation panel with 9 UI component shells
- CLI user-circuit-simulate command
- Auto engine fallback with explicit warning
- Report integration for simulation results
- 25 new Rust tests + 132 frontend tests (all pass)
```

## Previous Versions

- [v2.8 — Interactive Schematic Editing MVP](./verification_logs/v2.8_interactive_schematic_editing_mvp.md)
- [v2.7 — CLI / Headless Mode Foundation](./verification_logs/v2.7_cli_headless_mode.md)
- [v2.6 — Project Persistence / Save-Load UX Hardening](./verification_logs/v2.6_project_persistence_save_load_ux.md)
- [v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)
- [v2.4 — Real Component Parameters](./verification_logs/v2.4_real_component_parameters.md)
