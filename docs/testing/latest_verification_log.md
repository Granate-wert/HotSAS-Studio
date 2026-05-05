# Latest Verification Log

## Current Version

[v2.6 — Project Persistence / Save-Load UX Hardening](./verification_logs/v2.6_project_persistence_save_load_ux.md)

## v2.6 Summary

```text
Version: v2.6 — Project Persistence / Save-Load UX Hardening
Implementation commit: 568ffa3
Verification/docs commit: 568ffa3
Branch: main
Verification log: docs/testing/verification_logs/v2.6_project_persistence_save_load_ux.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (200+ Rust tests, exit code 0)
- typecheck (tsc --noEmit) — PASS
- vitest run — PASS (17 test files, 103 frontend tests)
- vite build — PASS
- tauri:build — PASS

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 13430272
- EXE SHA256: C9AD31ADBA958C5832B6B002511487C14C3BE7267D348D282ED57A615073E6D4
- ZIP: NOT CREATED (manual bundling required)
- Public GitHub Release: NO

New in v2.6:
- ProjectSessionState, RecentProjectEntry, ProjectSave/Open result core models
- ProjectSessionService with dirty tracking, save/open/recent projects logic
- LocalSettingsStorage adapter for JSON recent-projects persistence
- API facade methods: get_project_session_state, save_current_project, save_project_as, open_project_package, list/remove/clear recent projects
- 7 new Tauri commands with permissions
- Frontend types, API wrappers, store state/setters for project session
- ProjectToolbar, RecentProjectsPanel, UnsavedChangesBanner, ProjectPersistenceStatus components
- Dirty tracking integrated into existing mutating schematic/component commands
- Frontend tests for new project UI components
- Rust integration tests for project_session service and API
```

## Previous Versions

- [v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)
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
