# Schematic Editor Foundations

## Purpose

v1.3 adds the first editable schematic layer to HotSAS Studio.

## Source of truth

Rust `CircuitModel` / `CircuitDto` is the source of truth.
React Flow is only a view adapter.

## Pins and symbols

### Core models

- `PinDefinition` — id, name, number, electrical_type, position
- `ElectricalPinType` — Passive, Input, Output, Power, Ground, Bidirectional, NotConnected
- `PinPosition` — x, y, side
- `PinSide` — Left, Right, Top, Bottom
- `SymbolDefinition` — id, title, component_kind, pins, width, height

### Seed symbols

Built-in factory functions provide symbols for basic components:

- `resistor_symbol` — 2 passive pins (left, right)
- `capacitor_symbol` — 2 passive pins (top, bottom)
- `voltage_source_symbol` — p (power) and n (ground) pins
- `ground_symbol` — single gnd pin

## Property panel

- User clicks a component on the schematic
- UI calls `getSelectedComponent(instance_id)`
- Panel shows instance_id, component kind, title, parameters, symbol, pins
- User can edit a parameter value and click Apply
- UI calls `updateComponentParameter(instance_id, parameter_name, value, unit)`
- Backend returns updated `ProjectDto`
- UI updates store/project

## Circuit validation

`CircuitValidationService` checks:

- `empty_circuit` — no components
- `missing_ground` — no ground/reference net
- `duplicated_component_id` — duplicate instance ids
- `missing_required_parameter` — resistance for R, capacitance for C, amplitude for source
- `floating_net` — net with fewer than 2 connected pins
- `unknown_component_net` — wire references unknown component or net

## Limitations

- No PCB editor.
- No routing.
- No drag-and-drop component placement.
- No real ERC/DRC yet.
- No ngspice integration.
