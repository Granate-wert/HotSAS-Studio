# Engineering Notebook / Calculator Foundations

## Overview

The Engineering Notebook provides an interactive scratchpad for electrical-engineering calculations inside HotSAS Studio. Users type expressions, formula calls, or preferred-value commands and get immediate results that can be applied back to schematic components.

## Architecture

### Backend (Rust)

| Layer          | File                                                      | Responsibility                                                                                                     |
| -------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Core domain    | `engine/core/src/notebook.rs`                             | `EngineeringNotebook`, `NotebookBlock`, `NotebookHistoryEntry`, `NotebookEvaluationResult`                         |
| Application    | `engine/application/src/services/engineering_notebook.rs` | Parse input, evaluate via `FormulaService` / `PreferredValuesService`, produce `NotebookEvaluationResult`          |
| AppServices    | `engine/application/src/services/app_services.rs`         | Wires `EngineeringNotebookService` into `AppServices`                                                              |
| API DTOs       | `engine/api/src/dto.rs`                                   | Serializable notebook DTOs with `From` impls                                                                       |
| API facade     | `engine/api/src/facade.rs`                                | `HotSasApi::evaluate_notebook_input`, `get_notebook_state`, `clear_notebook`, `apply_notebook_output_to_component` |
| Tauri commands | `apps/desktop-tauri/src-tauri/src/lib.rs`                 | Exposes notebook endpoints to the frontend                                                                         |

### Frontend (React + TypeScript)

| Component                  | Path                                                   |
| -------------------------- | ------------------------------------------------------ |
| `EngineeringNotebook`      | `src/components/notebook/EngineeringNotebook.tsx`      |
| `NotebookInput`            | `src/components/notebook/NotebookInput.tsx`            |
| `NotebookResultCard`       | `src/components/notebook/NotebookResultCard.tsx`       |
| `NotebookVariableTable`    | `src/components/notebook/NotebookVariableTable.tsx`    |
| `NotebookHistory`          | `src/components/notebook/NotebookHistory.tsx`          |
| `PreferredValueQuickTools` | `src/components/notebook/PreferredValueQuickTools.tsx` |
| `ApplyNotebookOutputPanel` | `src/components/notebook/ApplyNotebookOutputPanel.tsx` |

Store integration: `notebookState`, `lastNotebookResult`, `setNotebookState`, `setLastNotebookResult`, `clearNotebookState` in the Zustand store.

## Supported Input Patterns

| Pattern         | Example                                | Result                      |
| --------------- | -------------------------------------- | --------------------------- |
| Assignment      | `R = 10k`                              | Stores variable `R = 10 kΩ` |
| Formula call    | `rc_low_pass_cutoff(R=10k, C=100n)`    | Computes `fc = 159.155 Hz`  |
| Preferred value | `nearestE(15.93k, E24, Ohm)`           | Finds nearest E24 resistor  |
| Free expression | (planned) engineering math expressions |

## Data Flow

1. User types input and presses Enter.
2. Frontend calls `evaluateNotebookInput(input)`.
3. Backend locks the in-memory `EngineeringNotebook`, evaluates input, updates `variables` and `history`, and returns a `NotebookEvaluationResultDto`.
4. Frontend displays the result card and updates the variable table / history.
5. User may select an output and apply it to a component parameter via `applyNotebookOutputToComponent`.

## Tests

- **Backend**: 101 tests pass (all existing + new notebook tests in `notebook_models_tests.rs`, `engineering_notebook_tests.rs`, `notebook_api_tests.rs`).
- **Frontend**: 16 tests pass.
- **Format / typecheck**: `cargo fmt`, `cargo test`, `prettier`, `tsc --noEmit`, `vitest run` all green.

## Changelog (v1.4)

- Added `EngineeringNotebook` domain model and evaluation pipeline.
- Added `EngineeringNotebookService` with parsers for assignment, formula calls, and preferred-value commands.
- Added Tauri commands and frontend UI for notebook input, results, variables, history, and apply-to-component.
- Integrated notebook into `CalculatorScreen`.
