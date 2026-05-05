# HotSAS Studio v2.4 — Real Component Parameters

## Overview

v2.4 introduced a typed parameter schema system that complements the existing flat `BTreeMap<String, ValueWithUnit>` storage in `ComponentDefinition`. It provides validation, grouping, tolerance specification, and metadata without breaking backward compatibility.

---

## Typed Parameter Schema

### Core Models

```text
ComponentParameterDefinition
  - name, key, description
  - unit: EngineeringUnit
  - kind: Primary | Secondary | Thermal | Mechanical | Simulation | Metadata
  - required, editable
  - tolerance: Option<ComponentTolerance>
  - value_range: Option<(f64, f64)>
  - default_value: Option<ValueWithUnit>

ComponentParameterGroup
  - name, key
  - parameters: Vec<ComponentParameterDefinition>

ComponentParameterSchema
  - category: String
  - groups: Vec<ComponentParameterGroup>
```

### Tolerance Model

```text
ComponentTolerance
  - SymmetricPercent { value: f64 }
  - Asymmetric { minus: f64, plus: f64 }
  - MinMax { min: f64, max: f64 }
```

Methods:
- `is_within(nominal, actual) -> bool`
- `bounds(nominal) -> (f64, f64)`

### Validation Errors

```text
ParameterValidationError
  - UnitMismatch { expected, actual }
  - OutOfRange { min, max, actual }
  - ToleranceExceeded { nominal, actual, tolerance }
  - MissingRequired { key }
```

---

## Typed Bundles

Extractors for 8 component categories:

| Bundle | Primary Fields |
|--------|---------------|
| `ResistorParameters` | resistance, tolerance, power_rating, tempco |
| `CapacitorParameters` | capacitance, tolerance, voltage_rating, dielectric, esr |
| `InductorParameters` | inductance, current_rating, dc_resistance, shielded |
| `DiodeParameters` | forward_voltage, reverse_voltage, forward_current, reverse_recovery |
| `BjtParameters` | vce_max, ic_max, power, hfe_typ, hfe_min |
| `MosfetParameters` | vds_max, id_max, rds_on, vgs_th, qg, ciss, coss |
| `OpAmpParameters` | gbw, input_offset_voltage, slew_rate, input_bias_current, supply_min, supply_max |
| `RegulatorParameters` | output_voltage, input_voltage_max, dropout_voltage, max_current, psrr, line_regulation |

Each bundle implements `from_map` and `to_map` for roundtrip conversion with the flat `BTreeMap` storage.

---

## Expanded Component Library

Built-in seeds expanded from 12 generic to **27 real-like components**:

### Resistors (4)
- generic_resistor (10k, 0.25W, 5%)
- resistor_10k_0603 (10k, 1%, 0.1W, 0603)
- resistor_1k_axial (1k, 5%, 0.25W, axial)
- resistor_100r_0805 (100R, 1%, 0.125W, 0805)

### Capacitors (4)
- generic_capacitor (100n, 50V, 10%)
- capacitor_100n_0603 (100n, 50V, X7R, 0603)
- capacitor_10u_0805 (10u, 25V, X5R, 0805)
- capacitor_100u_electrolytic (100u, 25V, electrolytic)

### Inductors (2)
- generic_inductor (10u, 1A)
- inductor_47u (47uH, 0.5A)

### Diodes (3)
- generic_diode
- diode_1n4148 (100V, 300mA, DO-35)
- diode_schottky_ss14 (40V, 1A, SOD-123)

### BJTs (4)
- generic_npn_bjt
- generic_pnp_bjt
- bjt_2n2222 (40V, 600mA, TO-92)
- bjt_2n2907 (40V, 600mA, TO-92)

### MOSFETs (3)
- generic_n_mosfet (60V, 10A)
- generic_p_mosfet (60V, 10A)
- mosfet_irfz44n (55V, 49A, 17.5mΩ, TO-220)

### Op-amps (3)
- generic_op_amp (1MHz, 1mV offset)
- op_amp_lm358 (1MHz, 0.3V/us, SOIC-8)
- op_amp_rail_rail_placeholder

### Regulators (1)
- ldo_ams1117_3v3 (3.3V, 1A, SOT-223)

### Sources (2)
- generic_voltage_source
- ground_reference

---

## API / Tauri Commands

| Command | Description |
|---------|-------------|
| `get_component_parameter_schema(category)` | Returns schema for category or null |
| `validate_component_parameters(component_id)` | Returns list of validation issues |
| `get_typed_component_parameters(component_id)` | Returns typed bundle for component |

---

## Frontend Integration

- `ComponentLibraryScreen` fetches typed parameters alongside component details.
- `ComponentDetailsPanel` displays a **Typed Parameters** card with category-specific values.
- TypeScript types: `TypedComponentParametersDto`, `ParameterBundleDto` (tagged union).

---

## Limitations

- No online component lookup (DigiKey, Mouser, LCSC).
- No procurement database or pricing/stock data.
- No datasheet PDF parser.
- SOA (Safe Operating Area) is placeholder only.
- Placeholder components (rail-to-rail op-amp, 4-switch DC-DC) are marked as such.
