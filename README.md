# HotSAS Studio

**HotSAS Studio - Hardware-Oriented Schematic Analysis & Simulation Studio** is a
desktop engineering application for schematic analysis, formula-driven circuit
templates, SPICE-oriented simulation workflows, and report generation.

The v1 vertical slice focuses on a single RC low-pass demo project:

1. Create the demo project.
2. Render the schematic from backend DTOs.
3. Calculate `fc = 1 / (2*pi*R*C)`.
4. Select the nearest E24 value.
5. Generate a SPICE netlist.
6. Run a mock AC simulation.
7. Display a graph.
8. Export Markdown and HTML reports.
9. Save the project as JSON.

## Stack

- Desktop: Tauri
- UI: React, TypeScript, Vite, Mantine, Zustand
- Schematic view adapter: React Flow / xyflow
- Charts: Apache ECharts
- Engine: Rust workspace

Mantine is a pragmatic v1 UI kit only. It does not influence backend
architecture.

## Formula Packs

v1.1.3 loads formula packs at runtime from `shared/formula_packs` through the
Rust backend. The Formula Library UI receives DTOs through Tauri commands; React
does not parse YAML or calculate formulas.

v1.1.4 adds generic backend formula evaluation for the supported RC low-pass,
Ohm's Law, and voltage divider expressions.

## Architecture

`engine/` is a Rust workspace:

```text
engine/
├── Cargo.toml
├── core/         crate: hotsas_core
├── ports/        crate: hotsas_ports
├── application/  crate: hotsas_application
├── adapters/     crate: hotsas_adapters
└── api/          crate: hotsas_api
```

Dependency direction:

- `hotsas_core` is pure domain code and does not depend on application,
  adapters, api, Tauri, React, or UI.
- `hotsas_application` depends on `hotsas_core` and ports/interfaces.
- `hotsas_adapters` implements ports and may depend on core/application
  contracts.
- `hotsas_api` depends on application and DTO contracts.
- Tauri calls `hotsas_api`.
- React calls only Tauri commands.

React Flow is a view adapter. The source of truth for schematic state is
`CircuitModel` / `CircuitDto`, not React Flow nodes and edges.

## Run

Rust must be installed first:

```powershell
rustup default stable
cargo --version
rustc --version
```

Install frontend dependencies and run the desktop shell:

```powershell
cd apps\desktop-tauri
npm.cmd install
npm.cmd run tauri:dev
```

Build checks:

```powershell
cd engine
cargo test

cd ..\apps\desktop-tauri
npm.cmd run build
```

## Development Checks

Rust engine:

```bash
cd engine
cargo fmt --check
cargo test
```

Frontend:

```bash
cd apps/desktop-tauri
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```

Tauri dev:

```bash
cd apps/desktop-tauri
npm.cmd run tauri:dev
```

Details: `docs/testing/TESTING.md`.

## Roadmap

- v1: RC low-pass vertical slice with mock simulation and Markdown/HTML reports.
- v2: richer formula engine, user formula packs, stronger unit model, more
  templates.
- v3: real ngspice adapter, SQLite storage, import/export expansion.
- Later: KiCad-compatible symbol/footprint export and Altium workflow package.
  No PCB editor is implemented in v1.
