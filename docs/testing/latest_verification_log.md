# Latest Verification Log

## Current Version

[v2.1 — Formula Library Expansion & Formula UX Hardening](./verification_logs/v2.1_formula_library_expansion.md)

## v2.1 Summary

```text
Version: v2.1 — Formula Library Expansion & Formula UX Hardening
Implementation commit: 266ffbf
Verification log update commit: 4823e87
Verification log: docs/testing/verification_logs/v2.1_formula_library_expansion.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (245 Rust tests)
- npm run format:check — PASS
- npm run typecheck — PASS
- npm run test — PASS (76 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE SHA256: 1423262166135F13A95CE117ED23BE0E33616CC84CD716A2745B7F496FBF29D8
- ZIP path: apps/desktop-tauri/src-tauri/target/release/HotSAS-Studio-v2.1-internal-rc-windows-x64.zip
- ZIP SHA256: ED73C5140DC2087F2BE6B71254AE2E6868A530D21F845DE463686905244B9BD0
- Public GitHub Release: NO
```

## Previous Versions

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
