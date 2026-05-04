# Latest Verification Log

## Current Version

[v2.2 — DC-DC Calculators and Templates](./verification_logs/v2.2_dcdc_calculators_and_templates.md)

## v2.2 Summary

```text
Version: v2.2 — DC-DC Calculators and Templates
Implementation commit: 53467fe
Verification/docs commit: 4777197
Verification log: docs/testing/verification_logs/v2.2_dcdc_calculators_and_templates.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (200+ Rust tests)
- npm run format:check — PASS
- npm run typecheck — PASS
- npm run test — PASS (76 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 12605952
- EXE SHA256: A4EB99FC1710A9CE5B6229F5F1B8152457574CDB7B096459162E75C435111FCF
- ZIP: NOT CREATED (manual bundling required)
- Public GitHub Release: NO
```

## Previous Versions

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
