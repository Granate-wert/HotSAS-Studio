# Latest Verification Log

## Current Version

[v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate](./verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md)

## v1.10 Summary

```text
Version: v1.10 — Internal Alpha EXE Build & v2.0 Readiness Gate
Implementation commit: e44830b
Verification log: docs/testing/verification_logs/v1.10_internal_alpha_build_and_v2_readiness_gate.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (166+ Rust tests)
- npm run format:check — PASS
- npm run typecheck — PASS
- npm run test — PASS (68 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE SHA256: 866E006D2DB2881ABBB3DCF0AFD655BE1643F6DD9D1E0E82CADF6D42764D0145
- ZIP path: apps/desktop-tauri/src-tauri/target/release/HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip
- ZIP SHA256: 5B3777A47C4F0575D4077F850EF114C07714609363FA7AB4E4883F95208506DA
- Public GitHub Release: NO
```

## Previous Versions

- [v1.9 — SPICE/Touchstone Import Foundation](./verification_logs/v1.9_spice_touchstone_import_foundation.md)
- [v1.8 — ngspice Adapter v1](./verification_logs/v1.8_ngspice_adapter_v1.md)
- [v1.7 — Export Center v1](./verification_logs/v1.7_export_center_v1.md)
- [v1.6 — Selected Region Analysis Foundation](./verification_logs/v1.6_selected_region_analysis_foundation.md)
- [v1.5 — Component Library Foundation](./verification_logs/v1.5_component_library_foundation.md)
- [v1.4 — Engineering Notebook Foundations](./verification_logs/v1.4_engineering_notebook.md)
- [v1.3 — Schematic Editor Foundations](./verification_logs/v1.3_schematic_editor_foundations.md)
- [v1.2 — Project Package Storage .circuit](./verification_logs/v1.2_project_package_storage.md)
- [v1.1.5 — Exact E-Series Tables](./verification_logs/v1.1.5_exact_e_series.md)
