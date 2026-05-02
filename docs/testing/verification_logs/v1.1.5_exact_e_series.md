# HotSAS Studio Verification Log

## Version / Task

v1.1.5 — Exact E-Series Tables

## Date

2026-05-03

## Git

Branch: main
Commit before changes: 3331b78
Commit after changes: TBD
Git status before: clean (untracked files only)
Git status after: clean

## Summary of changes

- Added exact E48/E96/E192 static tables in `engine/core/src/preferred_value_tables.rs`.
- Removed approximate generation (`generated_base_values`, `round_significant`) from production path.
- Updated `base_values` to return `&'static [f64]` via static tables.
- Added preferred value tests: length, quality, known values, nearest/lower/higher, error percent.
- Added documentation `docs/formula_library/PREFERRED_VALUES.md`.

## Rust checks

### cargo fmt --check

Status: PASS

### cargo test

Status: PASS (71 tests, 0 failures)

## Frontend checks

### npm.cmd run format:check

Status: PASS

### npm.cmd run typecheck

Status: PASS

### npm.cmd run test

Status: PASS (12 tests)

### npm.cmd run build

Status: PASS

## Manual / UI smoke test

Status: NOT RUN

Check:

- RC vertical slice still works.
- Formula Library still loads.
- E24 nearest still works.
- No frontend regressions.

## Agent self-check

- E48 table length: PASS
- E96 table length: PASS
- E192 table length: PASS
- generated_base_values not used in production: PASS
- invalid values return errors: PASS
- docs updated: PASS

## Final result

Overall status: PASS

Ready for next version: YES
