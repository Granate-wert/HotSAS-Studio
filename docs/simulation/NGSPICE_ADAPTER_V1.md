# ngspice Adapter v1

## Overview

The ngspice Adapter enables HotSAS Studio to execute real SPICE simulations via the external `ngspice` circuit simulator. All simulation logic lives in the Rust backend; the React frontend is a view adapter only. The adapter supports three analysis modes: Operating Point (OP), AC Sweep, and Transient.

An engine selection policy allows the user to choose between the built-in Mock engine (synthetic data for demos), the real ngspice engine, or Auto mode (ngspice when available, Mock as fallback).

## Engine Selection

| Engine    | Behavior                                                              |
| --------- | --------------------------------------------------------------------- |
| `mock`    | Always uses the built-in `MockSimulationEngine` (synthetic RC data)   |
| `ngspice` | Always uses the real `NgspiceSimulationAdapter`; fails if unavailable |
| `auto`    | Prefers ngspice; falls back to Mock with a warning if ngspice missing |

## Architecture

The simulation system follows the hexagonal architecture pattern:

```text
React SimulationScreen -> Tauri commands -> hotsas_api facade -> NgspiceSimulationService -> SimulationEnginePort -> adapter implementations
```

### Core Models (`hotsas_core`)

- `NgspiceAvailability` — `available`, `executable_path`, `version`, `message`, `warnings`
- `NgspiceRunStatus` — `Success`, `Failed`, `TimedOut`, `Unavailable`
- `NgspiceRunMetadata` — `run_id`, `engine`, `command`, `working_directory`, `netlist_path`, `stdout_path`, `stderr_path`, `raw_output_path`, `parsed_output_path`, `exit_code`, `elapsed_ms`
- `NgspiceSimulationRequest` — `project_id`, `profile_id`, `netlist`, `analysis_kind`, `output_variables`, `timeout_ms`
- `SimulationResult` (extended) — added `engine: String` and `metadata: BTreeMap<String, String>`

### Ports (`hotsas_ports`)

`SimulationEnginePort` was extended with ngspice-compatible methods:

- `engine_name() -> &str`
- `check_availability() -> Result<NgspiceAvailability, PortError>` (default: not implemented)
- `run_ac_sweep(project, profile) -> Result<SimulationResult, PortError>`
- `run_operating_point(project, profile) -> Result<SimulationResult, PortError>` (default: not implemented)
- `run_transient(project, profile) -> Result<SimulationResult, PortError>` (default: not implemented)
- `stop_simulation(run_id) -> Result<(), PortError>` (default: no-op)
- `get_result(run_id) -> Result<Option<SimulationResult>, PortError>` (default: not implemented)

### Adapters (`hotsas_adapters`)

- `NgspiceBinaryResolver` — discovers `ngspice` via `HOTSAS_NGSPICE_PATH` env var or PATH; extracts version via `--version`
- `NgspiceProcessRunner` — writes netlist to temp file, executes `ngspice -b -o stdout.log circuit.cir`, enforces timeout, captures output files
- `NgspiceOutputParser` — parses ASCII stdout and wrdata CSV-like files for AC, OP, and Transient results
- `NgspiceSimulationAdapter` — implements `SimulationEnginePort`; generates `.control` blocks, runs parser, maps to `SimulationResult`

### Application Layer (`hotsas_application`)

- `NgspiceSimulationService` — engine selection policy, availability checks, history cache

### API Layer (`hotsas_api`)

- `check_ngspice_availability() -> NgspiceAvailabilityDto`
- `run_simulation(request: SimulationRunRequestDto) -> SimulationResultDto`
- `simulation_history() -> Vec<SimulationResultDto>`

## Tauri Commands

| Command                      | Input                     | Output                     |
| ---------------------------- | ------------------------- | -------------------------- |
| `check_ngspice_availability` | —                         | `NgspiceAvailabilityDto`   |
| `run_simulation`             | `SimulationRunRequestDto` | `SimulationResultDto`      |
| `simulation_history`         | —                         | `Vec<SimulationResultDto>` |

## Frontend Types

```typescript
interface NgspiceAvailabilityDto {
  available: boolean;
  executable_path?: string;
  version?: string;
  message?: string;
  warnings: string[];
}

interface SimulationRunRequestDto {
  engine: "mock" | "ngspice" | "auto";
  analysis_kind: "operating_point" | "ac_sweep" | "transient";
  profile_id?: string;
  output_variables: string[];
  timeout_ms?: number;
}

interface SimulationResultDto {
  id: string;
  profile_id: string;
  status: string;
  engine: string;
  graph_series: GraphSeriesDto[];
  measurements: KeyValueDto[];
  warnings: string[];
  errors: string[];
  raw_data_path?: string;
  metadata?: KeyValueDto[];
}
```

## Zustand Store Extensions

The simulation state was added to the global store:

- `ngspiceAvailability: NgspiceAvailabilityDto | null`
- `selectedSimulationEngine: string`
- `simulationHistory: SimulationResultDto[]`
- `isSimulationRunning: boolean`
- `simulationError: string | null`

## SimulationResultsScreen

Replaces the old `SimulationScreen` with:

- **Engine Status Card** — shows availability, executable path, version, and warnings
- **Engine Selector** — `SegmentedControl` for Auto / Mock / ngspice
- **Run Buttons** — Operating Point, AC Sweep, Transient (disabled when no project or running)
- **Result Card** — status, engine name, warnings, errors
- **SimulationChart** — ECharts line graph for `graph_series` data

## Netlist Generation

The `NgspiceSimulationAdapter` generates a SPICE netlist from the `CircuitProject` model using `CircuitQueryService` to extract component parameters. The netlist includes a `.control` block with the appropriate analysis command:

```spice
* HotSAS Studio - RC Low-Pass Demo
* Source of truth: CircuitModel
V1 net_in 0 AC 1
R1 net_in net_out 10k
C1 net_out 0 100n
.control
ac dec 100 10 1e6
wrdata ac_out.csv v(net_out)
.endc
.end
```

## Error Handling

- **ngspice not installed** — `NgspiceAvailability.available = false` with a user-friendly message
- **ngspice selected but unavailable** — controlled `ApiError` with code `SIMULATION_ERROR`
- **Timeout** — enforced by `NgspiceProcessRunner`; returns `NgspiceRunStatus::TimedOut`
- **Parse errors** — `NgspiceOutputParser` returns warnings for invalid numeric rows without panic
- **Missing project** — Run buttons are disabled; facade returns state error if invoked

## Testing

### Rust Tests

| Test File                             | Tests | Focus                                          |
| ------------------------------------- | ----- | ---------------------------------------------- |
| `ngspice_binary_resolver_tests.rs`    | 3     | Env path, PATH fallback, invalid path handling |
| `ngspice_parser_tests.rs`             | 7     | AC, OP, transient parsing; empty/garbage input |
| `ngspice_simulation_service_tests.rs` | 5     | Engine policy, fallback, history               |
| `ngspice_simulation_api_tests.rs`     | 4     | Facade DTO mapping, controlled errors          |

Real integration tests are opt-in via `HOTSAS_RUN_NGSPICE_INTEGRATION=1`.

### Frontend Tests

`SimulationResultsScreen.test.tsx` covers rendering, engine selector, run buttons, availability display, result card, and error display.

## Security & Safety

- The adapter never panics on missing binary, invalid paths, or malformed output
- Timeout prevents runaway processes
- No shell interpolation; executable path is passed directly to `std::process::Command`
- Netlist is written to a temp directory; no user input reaches the command line

## Compatibility Notes

- `SimulationResult` gained `engine` and `metadata` fields; existing exporters handle them gracefully via defaults
- `AppServices::new` signature changed from 11 to 12 arguments (added `ngspice_engine`)
- All existing test wiring was updated to pass a fake ngspice engine as the 6th argument
