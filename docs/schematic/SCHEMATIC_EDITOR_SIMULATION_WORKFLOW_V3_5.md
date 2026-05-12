# HotSAS Studio v3.5 — Schematic Editor & Simulation Workflow

## Overview

v3.5 turns the existing schematic/simulation foundation into a coherent user-facing workflow. The Schematic Editor page is now usable for simple circuit construction and simulation with guided steps, meaningful empty states, and responsive layout.

## Supported Workflow

1. **Create or load a project**
   - Click "New RC Demo" to load a pre-built RC low-pass filter.
   - Click "Open Project" to load a `.circuit` package.
   - Blank project shows a helpful empty state with workflow guidance.

2. **Place components**
   - Select **Place** mode in the toolbar.
   - Choose a component from the Placeable Components palette (Resistor, Capacitor, etc.).
   - Click on the canvas to place the component.
   - Switch back to **Select** mode to move components.

3. **Wire nets**
   - Select **Wire** mode.
   - Drag from a component pin handle to another pin handle.
   - React Flow creates a smoothstep edge representing the net.

4. **Edit values**
   - Select a component (Select mode + click).
   - Open the **Properties** tab in the right panel.
   - Edit parameter values and click **Apply**.

5. **Validate**
   - Open the **Validation** tab or click **Validate Circuit**.
   - Review errors and warnings.

6. **Generate netlist**
   - Open the **Netlist Preview** tab in the bottom panel.
   - Netlist generates automatically on tab click.

7. **Run simulation**
   - Open the **Simulation** tab in the right panel.
   - Select a simulation profile and probes.
   - Click **Preflight** to check readiness.
   - Click **Run** to execute the simulation.

8. **View results**
   - Open the **Graph** tab to see simulation curves.
   - Or navigate to the Simulation Dashboard for detailed results.

## Tool Modes

| Mode   | Icon          | Behavior                                |
| ------ | ------------- | --------------------------------------- |
| Select | Mouse pointer | Click to select, drag to move           |
| Place  | Plus          | Choose component, click canvas to place |
| Wire   | Link          | Drag between pin handles to connect     |
| Delete | Trash         | Click component or wire to delete       |

## Layout

The Schematic Editor uses a CSS Grid layout:

- **Top toolbar**: Schematic actions, tool modes, undo/redo, component palette.
- **Main area**: Canvas (left, flexible) + Side panel (right, 320px).
- **Bottom panel**: Netlist, Preview, Graph, Formula, Report, Library tabs.

Responsive considerations:

- Canvas never collapses below minimum height.
- Toolbar wraps on narrow viewports.
- Side panel remains fixed at 320px on desktop.

## Current Limitations

- **Placement**: Click-to-place only; no drag-to-place.
- **Wiring**: Uses React Flow native edges; no custom wire path editing.
- **Symbols**: Simplified SVG symbols (not full IEEE-style).
- **Rotation**: No UI for rotating components in the canvas.
- **Multi-select**: No bulk selection or operations.
- **Live ERC**: Validation is explicit only (no real-time during edit).
- **Simulation**: Mock engine is simplified; ngspice requires external installation.

## Relation to Previous Versions

- v2.5: Schematic Editor Hardening (backend edit operations)
- v2.8: Interactive Schematic Editing MVP (tool modes UI, but not wired)
- v2.9: User-Circuit Netlist & Simulation E2E (backend simulation workflow)
- v3.0: Simulation UX, ngspice, Probes & Graphs (simulation dashboard)
- v3.4: Model Persistence & Project Package Hardening (model readiness badges)
- v3.5: **This release** — wires tool modes to canvas, adds empty states, improves layout, guides workflow.
