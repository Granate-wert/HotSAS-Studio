# HotSAS Studio — Latest Verification Log

**Version:** v1.8 — ngspice Adapter v1
**Date:** 2026-05-03
**Commit:** 76c8b19

See full log: [`docs/testing/verification_logs/v1.8_ngspice_adapter_v1.md`](./verification_logs/v1.8_ngspice_adapter_v1.md)

## Quick Results

| Check                | Result            |
| -------------------- | ----------------- |
| cargo fmt --check    | PASS              |
| cargo test           | PASS (146+ tests) |
| npm run format:check | PASS              |
| npm run typecheck    | PASS              |
| npm run test         | PASS (55 tests)   |

## Status

- v1.8 ngspice Adapter v1 is complete and verified.
- Core domain models extended, `SimulationEnginePort` extended, `NgspiceSimulationAdapter` with resolver/runner/parser, `NgspiceSimulationService` with engine selection policy, API DTOs/facade, Tauri commands, frontend `SimulationResultsScreen` with engine status, selector, run buttons, result card, ECharts graph, Zustand store integration, and tests are all implemented.
- Dependency boundary test preserved: `api` crate does not depend on `adapters` directly (api tests use inline fake structs).
- ECharts mocked in test setup to prevent jsdom canvas issues.
- Prettier and rustfmt formatting applied and verified.
- Ready for v1.9 or next iteration.
