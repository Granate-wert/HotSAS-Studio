# HotSAS Studio v3.1 - Component Model Mapping & SPICE Assignment

## Goal

v3.1 adds an explicit model-assignment layer between component definitions, placed component
instances, imported SPICE models, netlist export, and simulation preflight diagnostics.

The user-facing intent is simple:

- show whether a component has a simulation model;
- distinguish real builtin/imported models from placeholders;
- allow assigning a model to a placed component instance;
- report missing, placeholder, or invalid model mapping before simulation;
- keep React as a view adapter only.

## Implemented Foundation

Current implementation status is partial, with the main v3.1 runtime paths implemented and
remaining persistence/import richness explicitly deferred.

Implemented:

- Core assignment/readiness models in `engine/core/src/component_model_mapping.rs`.
- `SimulationModelKind` and `SimulationModel.kind` in `engine/core/src/models.rs`.
- Builtin primitive model seeds for resistor, capacitor, inductor, voltage source, and ground.
- Placeholder model kind for generic op-amp and LM358.
- `ComponentModelMappingService` in application services.
- API DTOs and facade methods for listing models, reading assignment, assigning instance model,
  and evaluating project readiness.
- Tauri commands and permissions for the same API surface.
- User-circuit netlist exporter support for selected primitive/model/subcircuit/placeholder models.
- Simulation preflight diagnostics include model mapping diagnostics.
- Frontend DTOs, backend wrappers, Zustand state, and component-library model assignment UI card.
- Component model assignments now populate inferred pin mappings and required parameter bindings.
- Invalid selected model ids, missing required parameters, placeholder models, and missing models emit
  stable diagnostics with code, severity, component id, and model id where applicable.
- CLI `hotsas-cli model-check <project.circuit> --json` reports project readiness, per-component
  model status, diagnostics, and summary counts.
- Advanced report JSON/Markdown rendering and legacy Markdown/HTML report export include a model
  mapping readiness section.
- The selected schematic instance inspector shows inherited/override assignment status and readiness.

## Assignment Statuses

The backend exposes stable snake_case strings to the frontend:

| Status              | Meaning                                                                  |
| ------------------- | ------------------------------------------------------------------------ |
| `missing`           | No SPICE model is available or assigned.                                 |
| `placeholder`       | A limited placeholder model exists; results are not production accurate. |
| `assigned_builtin`  | A builtin primitive model is assigned.                                   |
| `assigned_imported` | An imported model reference is assigned.                                 |
| `assigned_manual`   | User/manual model assignment is present.                                 |
| `invalid`           | Assignment exists but is not usable.                                     |

## Readiness

`SimulationReadiness` is computed in Rust and contains:

- `can_simulate`
- `can_export_netlist`
- `uses_placeholder`
- `blocking_count`
- `warning_count`
- `status_label`

Frontend components display this DTO directly and do not recalculate readiness.

## Persistence Policy

- Builtin primitive assignments are derived from the builtin component library and component kind.
  They are not persisted as duplicated instance data unless the user explicitly selects an instance
  model override.
- Instance-level assignment persistence uses `ComponentInstance.selected_simulation_model_id` in the
  `.circuit` schematic JSON. This is implemented for assignments to models already present on the
  component definition.
- Imported model reference persistence is partial. What works now: an attached component definition
  can carry `SimulationModel.raw_model_id`, `source_path`, and `pin_mapping`, and the runtime can use
  that definition-level model reference for assignment/readiness/export checks.
- Project/session-level only today: imported model details live in the model import service state and
  are available to the current API/session for listing/attachment workflows.
- Not persisted yet: a complete imported model catalog, imported raw model assets, and their
  project-package asset/index lifecycle are not saved as a durable package-level catalog in v3.1.
- Pin mapping persistence is partial. What works now: `SimulationModel.pin_mapping` persists explicit
  model-pin-to-component-pin mappings on a component definition, while derived builtin pin mappings
  are recomputed.
- Parameter binding persistence is derived in v3.1. What works now: required bindings for builtin
  primitives are inferred from component kind and component/instance parameters. Not persisted yet:
  user-editable parameter binding records and imported-model parameter schemas.
- Expected future completion point: a later persistence-focused stage should add durable imported
  model package indexes/assets and persisted user-editable parameter binding records before
  CMM-018 can move from PARTIAL/DEFERRED to PASS.

## Current Limitations

- `SimulationModel` currently has `kind`, `raw_model`, `raw_model_id`, and `pin_mapping`, but does
  not yet expose the richer fields mentioned in some historical context exports:
  `is_builtin_primitive`, `limitations`, `warnings`, `parameters`, `description`.
- Imported models are listed as available, but instance assignment remains primarily tied to models
  already present on a component definition.
- Full imported model package persistence and a user-managed imported model catalog need a separate
  stage before CMM-006/CMM-018 can be called complete.

## Netlist Behavior

The user-circuit netlist exporter now checks `selected_simulation_model_id`.

- Primitive models use standard SPICE element lines.
- `.model` assignments use model names and include raw model text if present.
- `.subckt` assignments produce `X...` instance lines and include raw subcircuit text if present.
- Placeholder assignments emit a warning comment and use a primitive fallback only when the
  component kind supports one.
- Unsupported model kinds emit controlled comment output.

## UI Behavior

The component library details panel can display a model assignment card:

- current status and readiness;
- current model name;
- available model selector;
- pin mapping rows;
- parameter binding rows;
- mapping diagnostics.

The UI calls backend commands and does not infer readiness, validate pins, or generate netlists.

The schematic selection inspector also displays inherited/override model assignment state for the
selected component instance when backend selection details are available.
