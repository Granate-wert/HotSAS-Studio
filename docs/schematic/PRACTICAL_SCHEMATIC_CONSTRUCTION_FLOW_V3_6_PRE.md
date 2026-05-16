# HotSAS Studio v3.6-pre - Practical Schematic Construction Flow

## Overview

v3.6-pre is the practical schematic construction gate for HotSAS Studio. The original gate removed the Tauri ACL blocker for schematic editing, and the follow-up fixes hardened component parameter editing. The v3.6-pre-fix4 update adds the CAD-style foundation that was still missing from the Schematic page:

- Real schematic symbol rendering for common components instead of generic rounded cards.
- A left-side component palette so placement tools are always near the canvas.
- Manual wire routing with grid-snapped bend points.
- Wire route geometry persisted through DTOs and project save/load.
- Focused frontend and Rust tests for symbols, routing, connectivity, and persistence.

This is still a foundation, not a full KiCad/Altium/EasyEDA-class editor.

## v3.6-pre-ui-polish — Engineering CAD Workspace Polish

The UI polish pass keeps the existing schematic construction behavior and improves the workspace around it. It does not add RF analysis, Smith chart work, S-parameter hardening, or new analysis features.

User-facing changes:

- The Schematic page top chrome now avoids duplicate command bars and prevents toolbar controls from rendering partially under or against the native titlebar/window edge.
- The Schematic toolbar is grouped by engineering workflow: Project, Edit, Analysis, Tools, and Export.
- Raw Save JSON/path workflows are de-emphasized from the primary Schematic workflow; advanced path entry remains available in the project toolbar.
- The left palette is compact and grouped by Passive, Sources, Semiconductors, and Op-Amps.
- The right-side Engineering Inspector is always visible and shows component identity, type/value, pins and connected nets, model/readiness status, diagnostics, and actions.
- Disabled controls expose reasons, including no project loaded, select component first, nothing to undo/redo, and feature not implemented yet.
- A lower engineering status bar shows active tool, grid/snap state, project state, selected entity, and validation state.

Verification notes:

- Frontend tests cover grouped toolbar labels/actions, component palette groups, selected and no-selection inspector states, disabled button reasons, and status bar state.
- Browser fallback smoke used Microsoft Edge headless screenshots at 1024x768, 1366x768, and 1440x900 against a temporary Vite Schematic harness. The harness was removed before final diff cleanup.
- Native Tauri manual smoke remains recommended for OS-window/titlebar behavior and file dialogs.

## v3.6-pre-fix4 - CAD-Style Manual Wire Routing Foundation

Manual routing now works through the Schematic canvas in wire mode:

1. Select **Wire** mode.
2. Click a source pin.
3. Click empty canvas locations to add grid-snapped bend points.
4. Click a target pin to create the wire.
5. Press `Escape` to cancel the active wire draft.

The route geometry is carried as `route_points` through the frontend request, API DTO, application service, core `WireGeometry`, and project save/load path. Netlist generation continues to use pin connectivity, so schematic route shape does not affect electrical connectivity.

The implementation intentionally keeps route editing modest for this gate. Existing wires display manual route geometry, but there is no post-creation drag editing of individual bend points yet.

## Schematic Symbol Rendering Foundation

The canvas now renders schematic symbols for:

- Resistor
- Capacitor
- Inductor
- Voltage source
- Ground
- Diode
- Op-amp
- MOSFET placeholder

The symbols are React/SVG nodes with visible pin handles and compact labels near the symbol. Unsupported or library-specific component categories still fall back to the generic component card so the editor remains tolerant of older project data and future component library additions.

## Schematic Page UI/UX Redesign Notes

The Schematic screen now uses a three-column workbench layout:

- Left panel: placeable component palette.
- Center: React Flow schematic canvas, grid, tool guidance, and wire preview.
- Right panel: properties, region, validation, netlist, and simulation tabs.

This keeps placement, drawing, and inspection separated. Responsive breakpoints collapse the side panels into the flow on narrower screens so the canvas remains usable at laptop widths.

## Earlier v3.6-pre Fixes

### Tauri ACL Fix

The original v3.6-pre gate fixed the critical blocker:

```text
Command add_schematic_component not allowed by ACL
```

The root cause was that schematic editing and analysis commands were registered in `generate_handler!` but missing from `permissions/hotsas.toml`. The permission manifest now includes the schematic editing commands required by placement, movement, deletion, wiring, parameter editing, validation, netlist preview, simulation, and analysis flows.

### Component Parameter Editing Fix

The v3.6-pre parameter editing follow-up fixed practical value editing:

- Freshly placed components now show editable default parameters from the component definition.
- Editable field DTOs include engineering units.
- Quick parameter updates parse values with the expected unit family, such as Ohm, Farad, Henry, Volt, Ampere, and Hertz.
- Netlist generation uses updated instance override values.
- Save/load preserves edited parameters.

## Expected User Workflow

1. Create or load a project.
2. Open the Schematic page.
3. Use the left palette to select a component.
4. Click the canvas to place it.
5. Drag placed components to organize the circuit.
6. Select **Wire** mode and click pin-to-pin with optional bend points.
7. Select a component and edit its properties in the right panel.
8. Run validation and netlist preview.
9. Save and reload the `.circuit` package to preserve component positions, values, wires, and manual route points.

## Known Limitations Versus CAD Tools

HotSAS v3.6-pre-fix4 is not yet a full schematic CAD replacement.

- No component rotation or mirror controls.
- No drag-from-palette placement.
- No post-creation route vertex editing.
- No automatic wire cleanup or orthogonal rerouting.
- No symbol library editor.
- No bus wiring, net labels as first-class placement objects, or hierarchical sheets.
- No live electrical-rule checking during editing.
- No multi-select, grouping, copy/paste, or bulk operations.
- No full KiCad/Altium/EasyEDA/LTspice parity for hotkeys, sheet management, annotation, or schematic rule workflows.

## RF Roadmap Boundary

The fix4 scope intentionally stays inside schematic construction. It does not add RF analysis features. After this gate, the RF roadmap remains:

- RF Visualization & Smith Chart Lite
- S-parameter workflow hardening
- Touchstone comparison and markers
- Impedance matching aids
- ngspice small-signal/network extraction research

## Acceptance Status

Status: **ACCEPT WITH DOCUMENTED LIMITATIONS**

The Schematic page is now suitable for basic circuit construction workflows with recognizable symbols, manual wire routes, property editing, validation, netlist preview, and save/load persistence. The remaining gaps are editor polish and advanced CAD behavior rather than the original blockers that prevented practical schematic construction.
