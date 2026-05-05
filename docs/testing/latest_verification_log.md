# Latest Verification Log

## Current Version

[v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)

## v2.5-fix Summary

```text
Version: v2.5-fix — Schematic Editing Validation & Verification Correction
Original v2.5 implementation commit: c31cadc
Fix commit: TBD
Verification/docs commit: TBD
Branch: main
Verification log: docs/testing/verification_logs/v2.5_schematic_editor_hardening.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (200+ Rust tests, exit code 0)
- npm run format:check — PASS
- npm run typecheck — PASS
- npm run test — PASS (95 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 13262336
- EXE SHA256: 4A6F584786FE5CCF6ED6076BC2198FB42CF9161B9800FAA9106624C308A25B96
- ZIP: NOT CREATED (manual bundling required)
- Public GitHub Release: NO

Fixes applied:
- cargo test exit code 0 (cleaned all warnings)
- connect_pins validates real pin ids via seed_symbol_for_kind
- delete_component removes stale wires/nets/connected_pins using component_id
- Added component_id to ConnectedPin core model + DTO + frontend types
- Git hygiene: removed 4 service files from tracking, updated .gitignore
- New Rust tests: 6 schematic editing tests for pin validation and delete cleanup
```

## Previous Versions

- [v2.4 — Real Component Parameters](./verification_logs/v2.4_real_component_parameters.md)
- [v2.1 — Formula Library Expansion & Formula UX Hardening](./verification_logs/v2.1_formula_library_expansion.md)
- [v2.0 — Product Beta Integration, Workflow Stabilization & Internal RC Build](./verification_logs/v2.0_product_beta_integration.md)
- [v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate](./verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md)
- [v1.9 — SPICE/Touchstone Import Foundation](./verification_logs/v1.9_spice_touchstone_import_foundation.md)
- [v1.8 — ngspice Adapter v1](./verification_logs/v1.8_ngspice_adapter_v1.md)
- [v1.7 — Export Center v1](./verification_logs/v1.7_export_center_v1.md)
- [v1.6 — Selected Region Analysis Foundation](./verification_logs/v1.6_selected_region_analysis_foundation.md)
- [v1.5 — Component Library Foundation](./verification_logs/v1.5_component_library_foundation.md)
- [v1.4 — Engineering Notebook Foundations](./verification_logs/v1.4_engineering_notebook.md)
- [v1.3 — Schematic Editor Foundations](./verification_logs/v1.3_schematic_editor_foundations.md)
- [v1.2 — Project Package Storage .circuit](./verification_logs/v1.2_project_package_storage.md)
- [v1.1.5 — Exact E-Series Tables](./verification_logs/v1.1.5_exact_e_series.md)
