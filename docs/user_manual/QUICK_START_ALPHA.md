# Quick Start — Internal Alpha Build

## How to run the internal alpha build

1. Obtain the ZIP file `HotSAS-Studio-v1.10-internal-alpha-windows-x64.zip`.
2. Extract it to a folder with an ASCII path, e.g. `C:\HotSAS-alpha\`.
3. Double-click `hotsas_desktop_tauri.exe`.
4. The app opens as a standard Windows desktop application.

## Screens you can check

- **Start** — Create an RC demo project.
- **Schematic** — View the circuit canvas and component properties.
- **Engineering Notebook** — Evaluate assignments and formula calls.
- **Formula Library** — Browse packs, categories, and formulas.
- **Component Library** — Search and view built-in components.
- **Selected Region** — Preview and analyze selected components.
- **Simulation Results** — Run mock AC simulation and check ngspice status.
- **Import Models** — Paste SPICE `.model` or Touchstone `.s2p` text.
- **Export Center** — Preview Markdown, HTML, SPICE netlist, BOM, SVG.
- **Diagnostics** — View module status and run readiness self-check.

## Minimal smoke test

1. Open the app.
2. Click **New RC Demo** on the toolbar.
3. Navigate through the left sidebar:
   - Schematic
   - Formula Library
   - Component Library
   - Simulation Results
   - Import Models
   - Export Center
   - Diagnostics
4. In **Diagnostics**, click **Refresh diagnostics** and **Run readiness self-check**.
5. All modules should show `Ready` or `Limited`; no crashes should occur.

## What to do if ngspice is unavailable

- This is a **controlled warning**, not a crash.
- The Simulation screen will show "ngspice not found" and fall back to the mock engine.
- To use ngspice, install it separately and set the environment variable `HOTSAS_NGSPICE_PATH` to the executable path.

## What is not supported yet

- No PCB layout or routing.
- No Gerber/DRC export.
- No full symbolic solver.
- No production-ready SPICE model manager.
- No Touchstone graph viewer.
- External component libraries cannot be loaded.
