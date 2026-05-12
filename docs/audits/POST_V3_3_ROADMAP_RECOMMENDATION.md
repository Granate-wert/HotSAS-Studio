# Post-v3.3 Roadmap Recommendation

## Gate Recommendation

Post-v3.3 can proceed to v3.4 planning after verification remains green. The recommended v3.4 scope is:

```text
v3.4 - Persistence & Project Package Hardening for Model Assignments and Imported Models
```

## Why This Scope

v3.1, v3.2, and v3.3 all remain accepted with documented limitations around persistence and RF-analysis depth. The next safe product step is not a new RF visualization feature. It is hardening project packages so imported SPICE/Touchstone model catalogs, selected assignments, and user-editable parameter bindings survive save/load predictably.

## Recommended v3.4 Scope

- Persist imported model catalog/package assets in `.circuit` projects.
- Persist component model assignment records, including selected builtin/imported model references.
- Persist user-editable parameter binding records.
- Add backward-compatible package migration or validation diagnostics for older project packages.
- Add UI states that clearly distinguish saved, unsaved, imported, missing, and unresolved model references.
- Add CLI validation output for missing imported assets and stale model references.

## Defer Beyond v3.4

- Smith chart.
- Calibration/de-embedding.
- VNA-grade accuracy claims.
- 3-port/4-port Touchstone support.
- Production RF CAD claims.
- Large visual redesign.

## Alpha/Internal Release Gate

An internal alpha gate is reasonable only after v3.4 persistence hardening or after the team explicitly accepts the current persistence limitations. For engineering trust, persistence hardening should come before adding new RF features.
