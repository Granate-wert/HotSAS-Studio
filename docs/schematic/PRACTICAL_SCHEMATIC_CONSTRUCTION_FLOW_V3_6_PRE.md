# HotSAS Studio v3.6-pre — Practical Schematic Construction Flow

## Overview

This document describes the v3.6-pre stage, which fixes the critical ACL blocker that prevented basic schematic editing operations from working in the UI.

## Problem statement

Before v3.6-pre, attempting to place a component from the Component Palette resulted in:

```text
Command add_schematic_component not allowed by ACL
```

Additionally, component movement and other schematic editing commands were blocked by the same root cause.

## Root cause

Tauri v2 uses an ACL (Access Control List) permission system. Commands must be:

1. Registered in `generate_handler!` — DONE (already present in `lib.rs`)
2. Listed in the permission manifest (`permissions/hotsas.toml`) — **MISSING 45 commands**
3. Referenced by the active capability (`capabilities/default.json`) — DONE (`allow-hotsas-commands`)

The schematic editing commands (and several analysis/export commands) were registered in the handler but not added to the permission manifest.

## Fix

Added the following commands to `permissions/hotsas.toml`:

```text
add_filter_network_analysis_to_advanced_report
add_s_parameter_analysis_to_advanced_report
add_schematic_component
analyze_touchstone_s_parameters
assign_component_to_selected_instance
calculate_dcdc
clear_last_filter_network_analysis
clear_last_s_parameter_analysis
connect_schematic_pins
delete_schematic_component
delete_schematic_wire
export_advanced_report
export_filter_network_analysis_csv
export_s_parameter_csv
generate_advanced_report
generate_current_schematic_netlist_preview
generate_dcdc_netlist_preview
get_component_details
get_component_parameter_schema
get_last_advanced_report
get_last_filter_network_analysis
get_last_s_parameter_analysis
get_project_model_catalog
get_schematic_selection_details
get_schematic_undo_redo_state
get_typed_component_parameters
list_components
list_dcdc_templates
list_placeable_components
list_report_section_capabilities
list_schematic_editor_capabilities
load_builtin_component_library
move_schematic_component
place_schematic_component
redo_schematic_edit
rename_schematic_net
run_dcdc_mock_transient_preview
run_filter_network_analysis
search_components
suggest_filter_analysis_ports
undo_schematic_edit
update_schematic_quick_parameter
validate_component_parameters
validate_filter_network_analysis_request
validate_project_model_persistence
```

## Placement coordinate fix

In addition to the ACL fix, component placement coordinates were improved:

- `SchematicCanvas` now wraps `ReactFlow` in `ReactFlowProvider`
- `useReactFlow().screenToFlowPosition` converts screen coordinates to flow coordinates
- This ensures accurate placement even when the canvas is panned or zoomed

## User workflow

After v3.6-pre, the following workflow is expected to work:

1. **Open or create project** — New RC Demo or Load Project
2. **Place components** — Select Place mode, click component type in palette, click canvas
3. **Move components** — Drag existing components on canvas
4. **Connect pins** — Select Wire mode, drag from one pin to another
5. **Edit values** — Select component, edit in Properties/Selection panel, click Update
6. **Validate** — Validation tab shows diagnostics
7. **Netlist** — Netlist Preview tab generates SPICE netlist
8. **Simulate** — Simulation tab runs mock/ngspice analysis
9. **Save/load** — Save .circuit preserves all edits

## v3.6-pre-fix: Component Parameter Editing

After the initial v3.6-pre ACL fix, manual smoke testing revealed that practical component parameter editing was not working correctly in the user UI. The v3.6-pre-fix addresses this:

### Problem

- Selecting a component showed editable fields only for parameters already in `overridden_parameters`
- Freshly placed components had empty `overridden_parameters`, so no editable fields appeared
- The `update_component_quick_parameter` backend service parsed all values as `Unitless`, so `4.7k` for resistance became `4700` unitless instead of `4700 Ohm`
- The netlist would show incorrect values after editing

### Fix

1. **`get_schematic_selection_details`** now includes default parameters from the component definition, not just instance overrides
2. **`SchematicEditableFieldDto`** now includes a `unit` field for each parameter
3. **`update_component_quick_parameter`** infers the correct engineering unit from the parameter ID:
   - `resistance` → Ohm
   - `capacitance` → Farad
   - `inductance` → Henry
   - `voltage` / `ac_magnitude` / `dc_voltage` → Volt
   - `current` → Ampere
   - `frequency` → Hertz
4. **`QuickParameterEditor`** shows unit labels next to inputs and uses controlled values to prevent stale state
5. Human-readable labels are used in the inspector (e.g., "Resistance" instead of "resistance")

### Verification

- Edit R1 resistance → netlist shows updated value
- Edit C1 capacitance → netlist shows updated value
- Save .circuit → load .circuit → edited values persist
- Invalid values (e.g., "abc") show clear validation errors

## Known limitations

- Wire mode: all handles use `type="source"`, which may limit bidirectional connection UX in React Flow v12. Connections still work via `isConnectableStart/End`.
- No drag-to-place: components are placed by clicking palette then clicking canvas.
- No component rotation UI.
- No live ERC: validation must be triggered explicitly.
- No multi-select or bulk operations.

## Relation to v3.5

v3.5 laid the groundwork: tool modes, empty states, SVG symbols, and disabled button guidance. v3.6-pre removes the ACL blocker that prevented those features from being usable.

## Next stage

After v3.6-pre, the project returns to RF workflow hardening:

- RF Visualization & Smith Chart Lite
- S-parameter workflow hardening
- Touchstone comparison and markers
- Impedance matching aids
- ngspice small-signal/network extraction research
