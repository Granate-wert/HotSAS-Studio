# CLI / Headless Mode Foundation (v2.7)

## Overview

`hotsas-cli` is a headless terminal interface for HotSAS Studio. It exposes all core backend operations through a command-line interface without duplicating any business logic — every command delegates directly to the existing `HotSasApi` facade.

## Binary

```text
engine/target/release/hotsas-cli.exe
```

Built from the `hotsas_cli` crate in `engine/cli/`.

## Architecture

```
hotsas-cli (clap parser)
    ↓
build_headless_api() — same adapter wiring as Tauri
    ↓
initialize_cli() — load formula packs + component library
    ↓
HotSasApi facade method
    ↓
AppServices → existing storage / formula / netlist / export / simulation services
```

## Commands

| Command                                                        | Arguments                                                                                           | Exit codes               |
| -------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | ------------------------ |
| `validate <path>`                                              | Path to `.circuit` package directory                                                                | 0 valid, 2 invalid       |
| `formula <id> [k=v...]`                                        | Formula ID + variable assignments                                                                   | 0 success, 2 input error |
| `netlist <path> [--out <file>]`                                | Project path + optional output file                                                                 | 0 success, 1/2 error     |
| `export <path> <format> [--out <file>]`                        | Project path + format (markdown/html/json/csv-summary) + optional output file                       | 0 success, 1/2/4 error   |
| `simulate <path> <profile> [--engine <engine>] [--out <file>]` | Project path + profile (ac_sweep/transient) + optional engine (mock/ngspice) + optional output file | 0 success, 1/2 error     |
| `library check`                                                | —                                                                                                   | 0 success                |
| `--version`                                                    | —                                                                                                   | 0                        |
| `--help`                                                       | —                                                                                                   | 0                        |

## Global Flags

- `--json` — Output structured JSON instead of human-readable text.

## Exit Code Policy

| Code | Meaning                                                |
| ---- | ------------------------------------------------------ |
| 0    | Success                                                |
| 1    | Internal / IO / engine / export / simulation error     |
| 2    | Validation / invalid input / not found / missing state |
| 3    | Usage error (e.g. malformed `key=value`)               |
| 4    | Unsupported feature (e.g. unknown export format)       |

## Examples

### Validate a project package

```bash
hotsas-cli validate ./my_project.circuit
hotsas-cli validate ./my_project.circuit --json
```

### Evaluate a formula

```bash
hotsas-cli formula ohms_law V=10 I=0.5 R=1k
hotsas-cli formula rc_low_pass_cutoff R=10k C=100n --json
```

### Generate SPICE netlist

```bash
hotsas-cli netlist ./my_project.circuit --out ./netlist.cir
hotsas-cli netlist ./my_project.circuit --json
```

### Export reports

```bash
hotsas-cli export ./my_project.circuit markdown --out report.md
hotsas-cli export ./my_project.circuit html --out report.html
hotsas-cli export ./my_project.circuit json
hotsas-cli export ./my_project.circuit csv-summary --out summary.csv
```

### Run simulation

```bash
hotsas-cli simulate ./my_project.circuit ac_sweep --engine mock --json
hotsas-cli simulate ./my_project.circuit transient --engine ngspice --out result.json
```

### Check component library

```bash
hotsas-cli library check
hotsas-cli library check --json
```

## CSV-Summary Export

The `csv-summary` format delegates to the `AdvancedReportService`:

1. Generates a project-summary advanced report.
2. Renders it as CSV via `render_report_csv_summary()`.

No manual CSV string building happens in the CLI layer.

## JSON Output Schema

Every `--json` response follows the `CliOutput<T>` schema:

```json
{
  "status": "success" | "validation_error" | "error" | "warning",
  "command": "validate",
  "warnings": [],
  "errors": [],
  "data": { ... }
}
```

## Testing

CLI integration tests live in `engine/cli/tests/cli_integration.rs`:

- `cli_version_returns_success`
- `cli_help_returns_success_with_all_commands`
- `cli_library_check_returns_success`
- `cli_library_check_json_returns_valid_json`
- `cli_formula_ohms_law_returns_success`
- `cli_validate_nonexistent_project_returns_error`
- `cli_validate_existing_demo_project_returns_success`
- `cli_netlist_demo_project_returns_success`
- `cli_export_markdown_demo_project_returns_success`
- `cli_simulate_mock_demo_project_returns_success`

Run them with:

```bash
cd engine
cargo test -p hotsas_cli --test cli_integration
```
