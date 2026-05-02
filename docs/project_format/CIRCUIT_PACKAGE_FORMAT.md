# HotSAS .circuit Project Package Format

## Purpose

`.circuit` is a folder-based project format for HotSAS Studio.
It replaces the previous single-JSON save with a structured directory layout that can grow with future features (models, symbols, footprints, exports).

## Folder structure

```text
my_project.circuit/
├── project.json              -- manifest
├── schematic.json            -- circuit model / schematic data
├── components.json           -- component definitions / instances placeholder
├── formulas.json             -- formula links and calculation snapshots
├── simulation_profiles.json  -- simulation profiles
├── reports/
│   └── index.json            -- report index
├── results/
│   └── index.json            -- simulation result index
├── models/
│   ├── spice/
│   └── touchstone/
├── symbols/                  -- future EDA workflow
├── footprints/               -- future EDA workflow
└── exports/                  -- generated exports
```

## project.json

The manifest. It is **not** the entire project — it only describes metadata and points to the other files.

Example:

```json
{
  "format_version": "1.0.0",
  "engine_version": "0.1.4",
  "project_id": "rc-low-pass-demo",
  "project_name": "RC Low-Pass Demo",
  "project_type": "CircuitProject",
  "created_at": "2026-05-03T00:00:00Z",
  "updated_at": "2026-05-03T00:00:00Z",
  "files": {
    "schematic": "schematic.json",
    "components": "components.json",
    "formulas": "formulas.json",
    "simulation_profiles": "simulation_profiles.json",
    "reports_index": "reports/index.json",
    "results_index": "results/index.json"
  }
}
```

## schematic.json

Contains the circuit model:

- `id`
- `title`
- `components` — component instances
- `wires`
- `nets`
- `labels`
- `probes`
- `annotations`

## components.json

Placeholder for future component library data.

```json
{
  "component_definitions": [],
  "component_instances": []
}
```

## formulas.json

Placeholder for formula links.

```json
{
  "formula_ids": [],
  "formula_results": []
}
```

## simulation_profiles.json

Array of simulation profiles used by the project.

## reports/index.json

```json
{
  "reports": []
}
```

## results/index.json

```json
{
  "results": []
}
```

## Versioning

- `format_version`: `1.0.0` for v1.2
- `engine_version`: matches the HotSAS Studio engine version

## Migration

Migration system is a placeholder; not implemented in v1.2.
The old `JsonProjectStorage` (single JSON file) is still supported and not removed.
