# Formula Library Expansion — v2.1

## Overview

v2.1 expands the formula library from 3 hardcoded formulas to **44 runtime-loaded formulas** across 8 YAML packs, backed by a generic expression evaluator.

## Expression Evaluator

File: `engine/adapters/src/expression_evaluator.rs`

Algorithm: shunting-yard tokenizer → RPN conversion → stack-based evaluation.

### Supported operators

| Operator | Description               |
| -------- | ------------------------- |
| `+`      | Addition                  |
| `-`      | Subtraction / unary minus |
| `*`      | Multiplication            |
| `/`      | Division                  |
| `^`      | Power                     |
| `()`     | Parentheses               |

### Supported functions

| Function    | Description       |
| ----------- | ----------------- |
| `sqrt(x)`   | Square root       |
| `exp(x)`    | Exponential (e^x) |
| `ln(x)`     | Natural logarithm |
| `log10(x)`  | Base-10 logarithm |
| `pow(x, y)` | Power function    |
| `abs(x)`    | Absolute value    |
| `pi`        | Constant π        |

### Features

- Variables are resolved from a `BTreeMap<String, f64>`.
- Expressions may include an `=` sign; the evaluator extracts the RHS.
- Unary minus is handled at the start of expressions or after `(` or another operator.
- Division by zero and invalid function domains return controlled errors.

### Tests

24 unit tests in `expression_evaluator::tests`, all PASS.

## Formula Packs

All packs are in `shared/formula_packs/` and loaded at runtime.

| Pack                | Formulas | Version | Status      |
| ------------------- | -------- | ------- | ----------- |
| `basic_electronics` | 8        | 0.2.0   | Supported   |
| `ac_impedance`      | 6        | 0.2.0   | Supported   |
| `transient`         | 5        | 0.2.0   | Supported   |
| `filters`           | 7        | 0.2.0   | Supported   |
| `op_amp`            | 7        | 0.2.0   | Supported   |
| `power_thermal`     | 5        | 0.2.0   | Supported   |
| `utilities`         | 4        | 0.2.0   | Supported   |
| `smps`              | 2        | —       | Placeholder |

### Total

- **44 formulas** in registry
- **42 supported** (calculable through generic evaluator)
- **2 placeholders** (SMPS buck/boost — expressions not yet implemented)

## FormulaExample Model

Each formula may include structured examples:

```yaml
examples:
  - title: "10 kΩ, 100 nF"
    inputs:
      R: 10k
      C: 100n
    expectedOutputs:
      fc: "159.15"
    notes: null
```

DTO: `FormulaExampleDto` with `title`, `inputs: FormulaExampleValueDto[]`, `expected_outputs: FormulaExampleValueDto[]`, `notes: string | null`.

## UI Changes

- Formula Library screen now shows:
  - **Assumptions** (blue alert)
  - **Limitations** (orange alert)
  - **Examples** as clickable buttons that pre-fill variable inputs
- Variable inputs table displays `default?.original` as placeholder text.

## New Engineering Units

| Unit             | Symbol | Use case             |
| ---------------- | ------ | -------------------- |
| `Watt`           | W      | Power                |
| `Percent`        | %      | Efficiency, ratios   |
| `Henry`          | H      | Inductance           |
| `Second`         | s      | Time constants       |
| `CelsiusPerWatt` | °C/W   | Thermal resistance   |
| `KelvinPerWatt`  | K/W    | Thermal resistance   |
| `Giga`           | G      | Prefix (10⁹) for GHz |

## Backward Compatibility

`SimpleFormulaEngine` preserves exact behavior for:

- `rc_low_pass_cutoff`
- `ohms_law`
- `voltage_divider`

All other formulas route through the generic evaluator.

## Testing

- **Rust**: 245 tests PASS (including 24 expression evaluator tests, 6 generic engine integration tests, 9 pack loader tests)
- **Frontend**: 76 tests PASS
