# HotSAS Studio Architecture

HotSAS Studio uses Clean Architecture / Hexagonal Architecture. The UI is a thin client. All domain behavior flows through Rust application services and ports.

## Layers

- UI Layer: Tauri + React + TypeScript. It displays DTOs and sends user actions to Tauri commands.
- API Layer: `hotsas_api`. It owns DTO conversion and the Tauri-facing command facade.
- Application Layer: `hotsas_application`. It owns use cases such as project creation, formula calculation, preferred value lookup, netlist generation, simulation, export, and storage.
- Ports Layer: `hotsas_ports`. It defines contracts for storage, formula engines, netlist exporters, simulation engines, and report exporters.
- Domain Core: `hotsas_core`. It owns pure models and domain functions.
- Adapters Layer: `hotsas_adapters`. It implements ports for JSON storage, simple formula calculation, mock simulation, SPICE netlist export, and Markdown/HTML export.

## Dependency Direction

```text
React -> Tauri commands -> hotsas_api -> hotsas_application -> hotsas_ports -> hotsas_core
                                                         ^                 ^
                                                         |                 |
                                                hotsas_adapters implements ports
```

Rules:

- `hotsas_core` has no dependency on application, adapters, api, Tauri, React, or UI.
- `hotsas_application` depends on `hotsas_core` and `hotsas_ports`.
- `hotsas_adapters` implements `hotsas_ports`.
- `hotsas_api` depends on `hotsas_application` and DTOs.
- Tauri owns the composition root and wires adapters into application services.
- React calls only Tauri commands.

## Thin UI Rule

React must not:

- calculate E-series;
- calculate formulas;
- generate SPICE netlists;
- run simulations;
- write directly to storage.

React Flow converts `CircuitDto` to visual nodes and edges only. It is not the project model.

## Future Extraction

The engine can later move into a local process such as `hotsas-engine.exe` because the API boundary already uses DTOs and application services. The same application layer can also serve a CLI, egui lite UI, Qt/Avalonia shell, or a future engine-server.
