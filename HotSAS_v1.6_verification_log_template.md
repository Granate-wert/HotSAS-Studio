# HotSAS Studio Verification Log Template

## Version / Task

v1.6 — Selected Region Analysis Foundation

## Date

YYYY-MM-DD HH:MM

## Git

Branch:

Commit before changes:

Implementation commit:

Verification log update commit:

Final HEAD:

Remote:

Git status before:

```text
PASTE OUTPUT HERE
```

Git status after:

```text
PASTE OUTPUT HERE
```

Changed files:

```text
PASTE OUTPUT HERE
```

---

## Summary of implementation

- Added/updated:
  - ...
- User-facing changes:
  - ...
- Backend changes:
  - ...
- Frontend changes:
  - ...
- Documentation changes:
  - ...

---

## User-facing status after v1.6

```text
[PASS/FAIL] User can select a region / choose components.
[PASS/FAIL] User can preview selected region.
[PASS/FAIL] User can see selected components.
[PASS/FAIL] User can see boundary/internal nets.
[PASS/FAIL] User can choose input/output/reference.
[PASS/FAIL] User can run Analyze Selection.
[PASS/FAIL] User can see result summary.
[PASS/FAIL] User can see matched template or controlled unsupported message.
[PASS/FAIL] User can see SPICE/netlist fragment.
[PASS/FAIL] User can see warnings/errors.
```

---

## Raw command log file

Required file:

```text
docs/testing/raw_logs/v1.6_selected_region_analysis_commands.txt
```

Status:

```text
CREATED / NOT CREATED
```

---

## Rust checks

### cargo fmt --check

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

### cargo test

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo test
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

Expected:

```text
test result: ok
0 failures
```

---

## Frontend checks

### npm.cmd run format:check

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

### npm.cmd run typecheck

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run typecheck
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

### npm.cmd run test

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run test
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

### npm.cmd run build

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run build
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

### npm.cmd run tauri:build

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:build
```

Status: PASS / FAIL / NOT RUN

Output summary:

```text
PASTE SUMMARY HERE
```

EXE:

```text
Path:
Size:
Build time:
```

---

## Manual / UI smoke test

Command:

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

Status: PASS / FAIL / NOT RUN

Checks:

```text
[OK/FAIL] приложение запускается
[OK/FAIL] RC demo project создаётся
[OK/FAIL] Schematic Editor открывается
[OK/FAIL] можно выбрать R1/C1
[OK/FAIL] Selected Region panel открывается
[OK/FAIL] Preview Region показывает selected components
[OK/FAIL] Preview Region показывает boundary nets
[OK/FAIL] можно выбрать input/output/reference
[OK/FAIL] Analyze Selection возвращает result
[OK/FAIL] RC low-pass region распознаётся или controlled partial result
[OK/FAIL] netlist fragment отображается
[OK/FAIL] unsupported selection не ломает UI
[OK/FAIL] Formula Library всё ещё работает
[OK/FAIL] Engineering Notebook всё ещё работает
[OK/FAIL] Component Library всё ещё работает
```

Notes:

```text
PASTE NOTES HERE
```

---

## Agent self-audit against v1.6 TZ

### Scope

```text
[PASS/FAIL] No real ngspice added
[PASS/FAIL] No PCB/routing/Gerber added
[PASS/FAIL] No full symbolic solver added
[PASS/FAIL] React does not calculate selected region analysis
[PASS/FAIL] RC vertical slice not broken
[PASS/FAIL] Formula Library not broken
[PASS/FAIL] Engineering Notebook not broken
[PASS/FAIL] Component Library not broken
```

### Core

```text
[PASS/FAIL] engine/core/src/selected_region.rs exists
[PASS/FAIL] SelectedCircuitRegion model exists
[PASS/FAIL] RegionPort model exists
[PASS/FAIL] SelectedRegionAnalysisRequest model exists
[PASS/FAIL] SelectedRegionAnalysisResult model exists
[PASS/FAIL] Region issues/warnings model exists
[PASS/FAIL] Netlist fragment model exists
```

### Application

```text
[PASS/FAIL] SelectedRegionAnalysisService exists
[PASS/FAIL] preview_selected_region implemented
[PASS/FAIL] analyze_selected_region implemented
[PASS/FAIL] validate_selected_region implemented
[PASS/FAIL] boundary nets detection implemented
[PASS/FAIL] subcircuit view implemented
[PASS/FAIL] known template matching implemented or explicitly documented as limited
[PASS/FAIL] netlist fragment generation implemented
[PASS/FAIL] unsupported arbitrary region handled without panic
```

### API / Tauri

```text
[PASS/FAIL] DTOs added
[PASS/FAIL] facade methods added
[PASS/FAIL] Tauri commands added
[PASS/FAIL] Tauri commands registered
[PASS/FAIL] Tauri capabilities/permissions checked
```

### Frontend

```text
[PASS/FAIL] frontend types added
[PASS/FAIL] frontend API methods added
[PASS/FAIL] selected-region UI components added
[PASS/FAIL] selection flow integrated
[PASS/FAIL] UI uses backend DTO for boundary nets
[PASS/FAIL] UI displays warnings/errors
[PASS/FAIL] UI displays netlist fragment
```

### Tests / Docs / Git

```text
[PASS/FAIL] Rust tests added/updated
[PASS/FAIL] Frontend tests added/updated
[PASS/FAIL] docs/selected_region/SELECTED_REGION_ANALYSIS_FOUNDATION.md created
[PASS/FAIL] README updated
[PASS/FAIL] docs/testing/TESTING.md updated
[PASS/FAIL] docs/testing/latest_verification_log.md updated
[PASS/FAIL] docs/testing/verification_logs/v1.6_selected_region_analysis.md created
[PASS/FAIL] docs/testing/raw_logs/v1.6_selected_region_analysis_commands.txt created
[PASS/FAIL] changes committed
[PASS/FAIL] changes pushed to origin/main
```

---

## Known limitations

```text
- ...
```

Expected v1.6 limitations:

```text
- Arbitrary symbolic solver is not implemented.
- Real ngspice is not implemented.
- Graph data may be placeholders/specs only.
- Report integration may return markdown fragment without full project persistence if documented.
```

---

## Final result

Overall status: PASS / FAIL / PARTIAL

Ready for v1.7: YES / NO

Reason:

```text
...
```
