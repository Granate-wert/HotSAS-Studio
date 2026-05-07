# Latest Verification Log

## Current Version

[v2.8 — Interactive Schematic Editing MVP](./verification_logs/v2.8_interactive_schematic_editing_mvp.md)

## v2.8 Summary

```text
Version: v2.8 — Interactive Schematic Editing MVP
Implementation commit: 819a48d
Branch: main
Push status: PASS / origin/main OK
Verification log: docs/testing/verification_logs/v2.8_interactive_schematic_editing_mvp.md

Checks:
- cargo fmt --check — PASS
- cargo test — PASS (376 Rust tests, exit code 0)
- cargo build -p hotsas_cli --release — PASS
- npm run format:check — PASS
- npm run typecheck — PASS
- npm test — PASS (132 frontend tests)
- npm run build — PASS
- npm run tauri:build — PASS

CLI binary:
- EXE path: engine/target/release/hotsas-cli.exe
- EXE size bytes: 3419136
- EXE SHA256: E6C06B01B451793002517A3421F85DAA6921D38246A948B2A86A7F4AEF3C720A

Internal build:
- EXE path: apps/desktop-tauri/src-tauri/target/release/hotsas_desktop_tauri.exe
- EXE size bytes: 13550080
- EXE SHA256: 483AB136F92DB9EDE774D592D4235D04F92607E8BCA8C468E994BFF08407361C
- ZIP: NOT CREATED (manual bundling required)
- Public GitHub Release: NO

New in v2.8:
- Interactive schematic editing MVP
- Placeable component palette with click-to-place
- React Flow drag-to-connect wire tool (onConnect)
- Delete wire with backend cleanup
- Quick parameter editor integrated into selection inspector
- Undo/redo foundation (snapshot-based, bounded 50)
- Netlist preview panel from backend
- ERC issue panel
- Save/load roundtrip preserves interactive edits
- 376 Rust tests, 132 frontend tests
```

## Previous Versions

- [v2.7 — CLI / Headless Mode Foundation](./verification_logs/v2.7_cli_headless_mode.md)
- [v2.6 — Project Persistence / Save-Load UX Hardening](./verification_logs/v2.6_project_persistence_save_load_ux.md)
- [v2.5 — Schematic Editor Hardening (v2.5-fix applied)](./verification_logs/v2.5_schematic_editor_hardening.md)
- [v2.4 — Real Component Parameters](./verification_logs/v2.4_real_component_parameters.md)
