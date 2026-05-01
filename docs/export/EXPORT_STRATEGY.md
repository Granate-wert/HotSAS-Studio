# Export Strategy

The export system is adapter-based. Application services call exporter ports; concrete exporters live in adapters.

## v1 Exports

- SPICE netlist for the RC low-pass demo circuit.
- Markdown report as the primary report format.
- HTML report as the second report format.
- Project JSON through storage port.

## Placeholders

- PDF report exporter placeholder.
- KiCad symbol exporter placeholder.
- KiCad footprint exporter placeholder.
- Altium workflow package exporter placeholder.

## Later Exports

- CSV simulation data.
- BOM CSV/JSON/XLSX.
- SVG schematic.
- PDF report.
- KiCad `.kicad_sym`.
- KiCad `.kicad_mod`.
- KiCad project/schematic export.
- Altium workflow package containing BOM, component DB, KiCad-compatible symbols/footprints, SPICE models, datasheets, mapping report, and import README.

## No PCB/Gerber In v1

HotSAS Studio v1 is a schematic analysis and simulation slice. PCB editing and Gerber generation are intentionally out of scope.
