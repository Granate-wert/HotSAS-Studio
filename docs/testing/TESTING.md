# HotSAS Studio Testing Guide

## Purpose

This document lists the standard local verification commands for HotSAS Studio.

## Rust Engine

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test
```

To format Rust code:

```bash
cargo fmt
```

## Frontend

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd install
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```

To format frontend code:

```bash
npm.cmd run format
```

## Tauri Dev

```bash
cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run tauri:dev
```

## Why npm.cmd Is Used

On this Windows PowerShell setup, `npm.ps1` can be blocked by Execution Policy.
Use `npm.cmd` for project scripts.

## Manual v1 Vertical Slice Smoke Check

1. Start the app with `npm.cmd run tauri:dev`.
2. Open the Start screen.
3. Create the RC low-pass demo project.
4. Verify that the schematic renders.
5. Calculate `fc`.
6. Request the nearest E24 value.
7. Generate the SPICE netlist.
8. Run the mock AC simulation.
9. Verify that the graph renders.
10. Export the Markdown report.
11. Export the HTML report.
12. Save the project JSON.

## Backend Test Coverage

v1.1.2 covers:

- EngineeringValue parsing;
- Preferred E-series selection;
- RC low-pass formula;
- Circuit template and formula binding;
- SPICE netlist export;
- Markdown and HTML report export;
- JSON storage;
- API error DTO;
- Full backend vertical slice.

v1.1.3 adds:

- FormulaPack YAML loading;
- FormulaPack JSON loading;
- `shared/formula_packs` directory loading;
- invalid pack validation;
- FormulaRegistry listing, lookup, categories, metadata, duplicate detection, and binding validation;
- API DTOs for formula packs, summaries, details, variables, equations, and outputs.

v1.1.4 adds:

- generic FormulaEnginePort tests;
- generic FormulaService tests through FormulaRegistry;
- API `calculate_formula` tests;
- compatibility coverage for the existing RC low-pass vertical slice.

## Commands Before Commit

```bash
cd "D:\Документы\vscode\HotSAS Studio\engine"
cargo fmt --check
cargo test

cd "D:\Документы\vscode\HotSAS Studio\apps\desktop-tauri"
npm.cmd run format:check
npm.cmd run typecheck
npm.cmd run build
```
