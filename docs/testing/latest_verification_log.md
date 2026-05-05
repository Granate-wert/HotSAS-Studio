# Latest Verification Log

## Current Version

[v2.7 — CLI / Headless Mode Foundation](./verification_logs/v2.7_cli_headless_mode.md)

## v2.7 Summary

```text
Version: v2.7 — CLI / Headless Mode Foundation
Implementation commit: TBD
Verification/docs commit: TBD
Branch: main
Verification log: docs/testing/verification_logs/v2.7_cli_headless_mode.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (361 Rust tests, exit code 0)
- prettier format:check — PASS
- typecheck (tsc --noEmit) — PASS
- vitest run — PASS (17 test files, 103 frontend tests)
- vite build — PASS
- tauri:build — PASS

CLI binary:
- EXE path: engine/target/release/hotsas-cli.exe
- EXE size bytes: 3205632
- EXE SHA256: 4F772D82892268A2B17C0169852D6FA96D97D7551BDA5899E4EA9AD913B4450F

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 13430272
- EXE SHA256: E0BFA4DD507584B8E5331DB27B309341749C086851649C66E57BDD2C0F02E307
- ZIP: NOT CREATED (manual bundling required)
- Public GitHub Release: NO

New in v2.7:
- hotsas_cli crate with hotsas-cli binary
- Commands: validate, formula, netlist, export, simulate, library check
- Global --json flag and CliOutput<T> wrapper
- Exit codes: 0/1/2/3/4
- 100% delegation to HotSasApi facade (zero business logic duplication)
- build_headless_api() reuses same adapter wiring as Tauri
- initialize_cli() loads built-in formula packs and component library
- csv-summary export delegates to AdvancedReportService::render_report_csv_summary()
- 10 CLI integration tests covering all commands and JSON output
- docs/cli/CLI_HEADLESS_MODE_V2_7.md created
- docs/testing/verification_logs/v2.7_cli_headless_mode.md created
- docs/testing/acceptance_matrices/v2.7_cli_headless_mode_acceptance_matrix.md created
```

## Previous Versions

- [v2.6 — Project Persistence / Save-Load UX Hardening](./verification_logs/v2.6_project_persistence_save_load_ux.md)
- [v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)
- [v2.4 — Real Component Parameters](./verification_logs/v2.4_real_component_parameters.md)
