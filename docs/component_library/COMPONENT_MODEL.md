# Component Model

The component library model separates real components from schematic instances.

## ComponentDefinition

`ComponentDefinition` stores reusable component data:

- id, name, category;
- manufacturer and part number;
- parameters and ratings;
- symbol ids;
- footprint ids;
- simulation models;
- datasheets;
- tags and metadata.

## ComponentInstance

`ComponentInstance` stores a component placed in a circuit:

- instance id such as `R1`, `C1`, or `U1`;
- referenced definition id;
- selected symbol, footprint, and simulation model;
- position and rotation;
- connected nets;
- overridden parameters;
- notes.

## Symbol And Footprint

`SymbolDefinition` and `FootprintDefinition` are in the domain model from v1, even though the first release does not include a PCB editor.

The stored data prepares future exports:

- KiCad-compatible symbols;
- KiCad-compatible footprints;
- Altium workflow packages through import-friendly data;
- internal component library JSON;
- BOM CSV/JSON/XLSX;
- SPICE model bundles;
- datasheet bundles.

HotSAS Studio v1 does not generate proprietary Altium files directly.
