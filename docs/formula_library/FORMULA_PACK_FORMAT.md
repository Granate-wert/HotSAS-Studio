# Formula Pack Format

Formula packs are user-extensible JSON or YAML files. Formulas are not stored only in code.

## Required Shape

```yaml
id: rc_low_pass_cutoff
title: RC Low-Pass Cutoff Frequency
category: filters/passive
description: Cutoff frequency of a first-order RC low-pass filter

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

## v1 Value Rules

The v1 engine supports `EngineeringValue` / `ValueWithUnit` parsing for:

- prefixes: `k`, `M`, `m`, `u`, `n`, `p`;
- examples: `10k`, `100n`, `1u`, `1M`;
- units: `Ohm`, `F`, `Hz`.

The model is intentionally extensible for future unit dimensions, complex values, dB, impedance, tolerances, and engineering notation display.
