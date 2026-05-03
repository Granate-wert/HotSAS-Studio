# HotSAS Studio — Component Library Foundation (v1.5)

## Goal

Provide a built-in component library so users can browse, search, and assign real component definitions to schematic instances without leaving the application.

## What Is Included

### Built-in Library

The built-in library (`hotsas_core::component_seeds::built_in_component_library`) contains **12 components** covering common analog discrete and basic active parts:

| ID                       | Name                   | Category  | Symbol         | Footprint                             |
| ------------------------ | ---------------------- | --------- | -------------- | ------------------------------------- |
| `generic_resistor`       | Generic Resistor       | resistor  | resistor       | axial_resistor_placeholder            |
| `generic_capacitor`      | Generic Capacitor      | capacitor | capacitor      | radial_capacitor_placeholder          |
| `generic_inductor`       | Generic Inductor       | inductor  | inductor       | inductor_placeholder                  |
| `generic_diode`          | Generic Diode          | diode     | diode          | do_41_diode_placeholder               |
| `generic_led`            | Generic LED            | led       | led            | led_5mm_placeholder                   |
| `generic_npn_bjt`        | Generic NPN BJT        | bjt       | bjt_npn        | to_92_placeholder, sot23_placeholder  |
| `generic_pnp_bjt`        | Generic PNP BJT        | bjt       | bjt_pnp        | to_92_placeholder, sot23_placeholder  |
| `generic_n_mosfet`       | Generic N-MOSFET       | mosfet    | mosfet_n       | to_220_placeholder, soic8_placeholder |
| `generic_p_mosfet`       | Generic P-MOSFET       | mosfet    | mosfet_p       | to_220_placeholder, soic8_placeholder |
| `generic_op_amp`         | Generic Op-Amp         | opamp     | op_amp         | soic8_placeholder                     |
| `generic_voltage_source` | Generic Voltage Source | source    | voltage_source | —                                     |
| `ground_reference`       | Ground Reference       | ground    | ground         | ground_virtual_placeholder            |

### ComponentDefinition

`ComponentDefinition` is the reusable library entry. It stores:

- `id`, `name`, `category`
- `manufacturer`, `part_number`
- `parameters`: engineering values with units (e.g. `resistance = 10 kOhm`)
- `ratings`: max/AbsMax values (e.g. `power = 0.25 W`)
- `symbol_ids`: references to seed symbols used for schematic preview
- `footprint_ids`: references to placeholder footprints
- `simulation_models`: SPICE model references (empty in v1.5)
- `datasheets`: URLs or file references (empty in v1.5)
- `tags`: searchable labels (e.g. `passive`, `resistor`, `generic`)
- `metadata`: free-form key/value for unsupported units (e.g. `V/us` slew rate)

### ComponentInstance

`ComponentInstance` is a component placed in a circuit. It stores:

- `instance_id`: e.g. `R1`, `C1`, `U1`
- `definition_id`: reference to a `ComponentDefinition` from the library
- `selected_symbol_id`: chosen symbol for schematic rendering
- `selected_footprint_id`: chosen footprint for future PCB use
- `selected_simulation_model_id`: chosen model for simulation
- `position`, `rotation`
- `connected_nets`
- `overridden_parameters`: instance-specific values
- `notes`

### Symbol Preview

The library does not store full SVG graphics. Instead:

- Each `ComponentDefinition` references seed symbol IDs.
- `seed_symbol_for_kind()` in `core/src/symbol.rs` returns a lightweight `SymbolDefinition` with pin metadata.
- The React frontend renders a simple preview from pin positions and electrical types.

Supported seed symbols in v1.5:

- `resistor`, `capacitor`, `inductor`
- `diode`, `led`
- `bjt_npn`, `bjt_pnp`
- `mosfet_n`, `mosfet_p`
- `op_amp`
- `voltage_source`, `ground`

### Footprint Preview

Footprints are metadata placeholders only in v1.5. Each `FootprintDefinition` stores:

- `id`, `name`, `package_name`
- `pad_count`
- `metadata`

No real PCB geometry or Gerber data is generated.

### Assign-to-Schematic Flow

1. User selects a component instance in the schematic editor.
2. User opens the Component Library screen.
3. User searches or browses the built-in library.
4. User selects a library component and views its details + previews.
5. User clicks **Assign to Selected Component**.
6. Backend (`ComponentLibraryService::assign_component_to_instance`) updates:
   - `ComponentInstance.definition_id`
   - `selected_symbol_id`
   - `selected_footprint_id`
   - Merges default parameters without overwriting existing overrides.
7. Updated project is returned to the frontend and stored in Zustand.

## Architecture

```text
React UI -> Tauri commands -> hotsas_api -> hotsas_application -> hotsas_ports -> hotsas_core
                                       ^
                                       |
                            hotsas_adapters (JsonComponentLibraryStorage)
```

- `ComponentLibraryPort` defines the trait contract.
- `JsonComponentLibraryStorage` implements load/save for JSON libraries.
- `ComponentLibraryService` owns search, filter, and assignment logic.
- `HotSasApi` facade keeps the current `ComponentLibrary` in backend state.
- React only renders UI; all logic runs in Rust.

## Search and Filter

`ComponentLibraryQuery` supports:

- Full-text search across `id`, `name`, `description`, `part_number`, `tags`
- Filter by `category`
- Filter by `tags`
- Filter by `manufacturer`
- Boolean filters: `has_symbol`, `has_footprint`, `has_simulation_model`

Search is case-insensitive.

## What Is Placeholder

- Footprints: only metadata placeholders, no real PCB shapes.
- Simulation models: empty vectors, no SPICE model import yet.
- Datasheets: empty vectors, no PDF/URL handling yet.
- Online lookup: no DigiKey, Mouser, or LCSC integration.

## What Is Not Included

- No PCB editor.
- No routing or Gerber export.
- No KiCad symbol/footprint export.
- No Altium workflow package generation.
- No online component lifecycle, pricing, or stock lookup.
- No real SPICE model import or ngspice integration.
- No DC-DC calculator beyond existing formula packs.
- No full symbolic solver.

## Files

- `engine/core/src/component_library.rs`
- `engine/core/src/component_seeds.rs`
- `engine/ports/src/lib.rs`
- `engine/adapters/src/component_library_storage.rs`
- `engine/application/src/services/component_library.rs`
- `engine/api/src/dto.rs`
- `engine/api/src/facade.rs`
- `apps/desktop-tauri/src-tauri/src/lib.rs`
- `apps/desktop-tauri/src/types/index.ts`
- `apps/desktop-tauri/src/api/index.ts`
- `apps/desktop-tauri/src/store/index.ts`
- `apps/desktop-tauri/src/screens/ComponentLibraryScreen.tsx`
- `apps/desktop-tauri/src/components/component-library/*.tsx`
