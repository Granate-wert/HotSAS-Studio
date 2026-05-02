# Formula Pack Format

Formula packs are runtime-loaded JSON or YAML files. In v1.1.3 they feed the
Formula Registry used by the Formula Library UI. Formula evaluation is still
limited to the existing RC low-pass backend calculation path; a generic formula
engine is planned for a later version.

## Pack Shape

```yaml
packId: filters
title: Filters
version: 0.1.0
formulas:
  - id: rc_low_pass_cutoff
    title: RC Low-Pass Cutoff Frequency
    category: filters/passive
    description: Cutoff frequency of a first-order RC low-pass filter.
    variables:
      R:
        unit: Ohm
        description: Resistance
        default: 10k
      C:
        unit: F
        description: Capacitance
        default: 100n
    equations:
      - id: cutoff
        latex: "f_c = \\frac{1}{2\\pi R C}"
        expression: "fc = 1 / (2*pi*R*C)"
        solve_for: ["fc", "R", "C"]
    outputs:
      fc:
        unit: Hz
        description: Cutoff frequency
    linkedCircuitTemplateId: rc_low_pass_template
    mapping:
      R: R1.resistance
      C: C1.capacitance
      Vin: net_in
      Vout: net_out
    defaultSimulation:
      type: ac_sweep
      start: 10 Hz
      stop: 1 MHz
      pointsPerDecade: 100
```

## Required Fields

Pack:

- `packId`
- `title`
- `version`
- non-empty `formulas`

Formula:

- `id`
- `title`
- `category`
- non-empty `equations`
- non-empty `outputs`
- variables and outputs must include a `unit` field

Optional fields:

- `description`
- `assumptions`
- `limitations`
- `linkedCircuitTemplateId`
- `mapping`
- `defaultSimulation`
- `examples`

## Supported Formats

v1.1.3 supports:

- `.yaml`
- `.yml`
- `.json`

Files in `shared/formula_packs` are loaded by the Rust backend through
`FormulaPackFileLoader`. React does not read formula pack files directly.

## v1 Value Rules

The v1 engine supports `EngineeringValue` / `ValueWithUnit` parsing for:

- prefixes: `k`, `M`, `m`, `u`, `n`, `p`;
- examples: `10k`, `100n`, `1u`, `1M`;
- units: `Ohm`, `F`, `Hz`, `V`, `A`, and unitless `""`.

The model is intentionally extensible for future unit dimensions, complex
values, dB, impedance, tolerances, and engineering notation display.

## Circuit Template Binding

Use `linkedCircuitTemplateId` to connect a formula to a circuit template.
For the RC low-pass vertical slice:

```yaml
linkedCircuitTemplateId: rc_low_pass_template
mapping:
  R: R1.resistance
  C: C1.capacitance
  Vin: net_in
  Vout: net_out
```

The registry can validate known template ids, but full template-driven formula
evaluation remains out of scope for v1.1.3.

## Formula Evaluation in v1.1.4-fix

At this stage HotSAS supports limited backend formula evaluation through `FormulaEnginePort`.

Supported expressions:

- `fc = 1 / (2*pi*R*C)`
- `V = I * R`
- `Vout = Vin * R2 / (R1 + R2)`

The frontend does not evaluate formulas. React sends variable strings to the Rust backend. The backend parses units, validates inputs, evaluates the formula, and returns outputs.

Full symbolic parser is out of scope. Math.js / SymPy / Lcapy integration is planned later.
