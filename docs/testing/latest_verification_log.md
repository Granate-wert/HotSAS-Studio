# Latest Verification Log

## Current Version

[v2.7 — CLI / Headless Mode Foundation](./verification_logs/v2.7_cli_headless_mode.md)

## v2.7 Summary

```text
Version: v2.7 — CLI / Headless Mode Foundation
Original v2.7 implementation commit: 99c848b
Review HEAD: 3a66c23
Fix commit: 398f8f6
Verification/docs commit: 398f8f6
Branch: main
Push status: PASS / origin/main OK
Verification log: docs/testing/verification_logs/v2.7_cli_headless_mode.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (364 Rust tests, exit code 0)
- prettier format:check — PASS
- typecheck (tsc --noEmit) — PASS
- vitest run — PASS (17 test files, 103 frontend tests)
- vite build — PASS
- tauri:build — PASS

CLI binary:
- EXE path: engine/target/release/hotsas-cli.exe
- EXE size bytes: 3406336
- EXE SHA256: FA48232BB09E8D5AD9B99D8F4CD5960E65DD1A854E6F33CA89929DA89E8B0E3F

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
- csv-summary and json exports delegate to AdvancedReportService via HotSasApi
- --timeout argument on simulate command
- 13 CLI integration tests covering all commands, JSON output, and timeout
- docs/cli/CLI_HEADLESS_MODE_V2_7.md created
- docs/testing/verification_logs/v2.7_cli_headless_mode.md created
- docs/testing/acceptance_matrices/v2.7_cli_headless_mode_acceptance_matrix.md created
```

## v2.7-fix Summary

```text
Fix commit: 398f8f6
Fixed findings:
- B-001: Dirty git state resolved (committed Cargo.toml/Cargo.lock changes)
- M-001: Removed dead engine/cli/src/errors.rs
- m-001: Export JSON delegates to AdvancedReportService instead of manual serde
- m-002: csv-summary uses unique timestamp-based report ID
- m-003: Added --timeout CLI arg to simulate command
```

## Previous Versions

- [v2.6 — Project Persistence / Save-Load UX Hardening](./verification_logs/v2.6_project_persistence_save_load_ux.md)
- [v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)
- [v2.4 — Real Component Parameters](./verification_logs/v2.4_real_component_parameters.md)
