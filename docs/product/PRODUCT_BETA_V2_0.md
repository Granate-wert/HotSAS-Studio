# HotSAS Studio v2.0 — Product Beta

## Purpose

v2.0 is the first integrated internal product beta. It brings together all previously implemented modules into a guided engineering workflow.

## What works

- **Project Hub** — create integrated demo projects, view workflow status
- **Schematic Editor** — template-based schematic with component selection and validation
- **Formula Library** — registry with RC low-pass, Ohm's law, voltage divider
- **Engineering Notebook** — interactive assignments and formula calls
- **Component Library** — built-in library with 12+ components
- **SPICE/Touchstone Import** — text-based model import with parser validation
- **Simulation** — mock AC sweep + ngspice adapter (when available)
- **Selected Region Analysis** — preview and analyze schematic regions
- **Export Center** — 9 export formats (Markdown, HTML, SPICE, CSV, BOM, SVG, etc.)
- **Diagnostics** — module readiness dashboard with self-check

## What is still limited

- No PCB editor
- No routing/Gerber
- No full symbolic solver
- No advanced formula packs v2.1 yet
- No DC-DC calculator pack v2.2 yet
- ngspice may be unavailable unless installed
- Touchstone visualization later
- Interactive pin mapper later

## Guided workflow

```text
1. Create/open project
2. View schematic
3. Calculate formulas
4. Use Engineering Notebook
5. Assign components from library
6. Import SPICE/Touchstone models
7. Run simulation
8. Analyze selected region
9. Export results
10. Check Diagnostics
```

## Internal build

- Windows EXE built via `npm run tauri:build`
- Internal ZIP created for distribution
- No public GitHub Release

## Next steps

- v2.1 — Formula Library Expansion
- v2.2 — DC-DC Calculator Templates
- v2.3 — Advanced Reports
