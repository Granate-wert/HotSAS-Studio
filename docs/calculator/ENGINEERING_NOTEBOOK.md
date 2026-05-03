# Engineering Notebook / Calculator Foundations

## Purpose

The Engineering Notebook provides an interactive scratchpad for electrical-engineering calculations inside HotSAS Studio. Users type expressions, formula calls, or preferred-value commands and get immediate results that can be applied back to schematic components.

## Architecture

### Backend

| Layer          | File                                                      | Responsibility                                                                                                     |
| -------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Core domain    | `engine/core/src/notebook.rs`                             | `EngineeringNotebook`, `NotebookBlock`, `NotebookHistoryEntry`, `NotebookEvaluationResult`                         |
| Application    | `engine/application/src/services/engineering_notebook.rs` | Parse input, evaluate via `FormulaService` / `PreferredValuesService`, produce `NotebookEvaluationResult`          |
| AppServices    | `engine/application/src/services/app_services.rs`         | Wires `EngineeringNotebookService` into `AppServices`                                                              |
| API DTOs       | `engine/api/src/dto.rs`                                   | Serializable notebook DTOs with `From` impls                                                                       |
| API facade     | `engine/api/src/facade.rs`                                | `HotSasApi::evaluate_notebook_input`, `get_notebook_state`, `clear_notebook`, `apply_notebook_output_to_component` |
| Tauri commands | `apps/desktop-tauri/src-tauri/src/lib.rs`                 | Exposes notebook endpoints to the frontend                                                                         |

### Frontend

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

### Assignments

```text
R = 10k
C = 100n
Vin = 5
I = 2m
```

### Formula calls

```text
rc_low_pass_cutoff(R=10k, C=100n)
rc_low_pass_cutoff(R=R, C=C)
ohms_law(I=2m, R=10k)
voltage_divider(Vin=5, R1=10k, R2=10k)
```

### Preferred values

```text
nearestE(15.93k, E24, Ohm)
nearestE(15.93k, E96, Ohm)
lowerE(15.93k, E96, Ohm)
higherE(15.93k, E96, Ohm)
```

## Unsupported in v1.4

- Free math expressions like `sin(...)`
- Arbitrary algebra
- Plotting
- Symbolic solve

Unsupported input returns a controlled result with a helpful hint:

```text
v1.4 supports assignments, formula calls and nearestE/lowerE/higherE commands. Free math expressions like sin(...) are planned later.
```

## Data Flow

1. User types input and presses Enter.
2. Frontend calls `evaluateNotebookInput(input)`.
3. Backend locks the in-memory `EngineeringNotebook`, evaluates input, updates `variables` and `history`, creates a `NotebookBlock`, and returns a `NotebookEvaluationResultDto`.
4. Frontend displays the result card and updates the variable table / history.
5. User may select an output and apply it to a component parameter via `applyNotebookOutputToComponent`.

## Apply output to component

If a notebook result contains outputs and a component is selected in the schematic, the user can apply an output value directly to a component parameter.

## Tests

- **Core**: `engine/core/tests/notebook_models_tests.rs` — 4 tests
- **Application**: `engine/application/tests/engineering_notebook_tests.rs` — 9 tests
- **API**: `engine/api/tests/notebook_api_tests.rs` — 6 tests
- **Frontend**: `src/components/notebook/__tests__/NotebookComponents.test.tsx` — 13 tests

## Limitations

- Notebook state is in-memory only (not persisted to disk).
- Only a limited set of input patterns is supported.
- Apply-to-component requires a project to be open.
