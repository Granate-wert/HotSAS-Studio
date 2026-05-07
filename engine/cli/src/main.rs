use clap::{Parser, Subcommand};
use hotsas_cli::{build_headless_api, commands, initialize_cli};

#[derive(Parser)]
#[command(name = "hotsas-cli", version, about = "HotSAS Studio CLI")]
struct Cli {
    #[arg(long, global = true, help = "Output results as JSON")]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a HotSAS project package
    Validate {
        #[arg(help = "Path to the project package directory")]
        path: String,
    },
    /// Evaluate a formula with optional variables
    Formula {
        #[arg(help = "Formula ID to evaluate")]
        formula_id: String,
        #[arg(help = "Variable assignments in key=value format")]
        variables: Vec<String>,
        #[arg(long, help = "Output raw JSON result")]
        json: bool,
    },
    /// Generate SPICE netlist from a project
    Netlist {
        #[arg(help = "Path to the project package directory")]
        path: String,
        #[arg(long, help = "Output file path")]
        out: Option<String>,
    },
    /// Export project report in various formats
    Export {
        #[arg(help = "Path to the project package directory")]
        path: String,
        #[arg(help = "Export format: markdown, html, json, csv-summary")]
        format: String,
        #[arg(long, help = "Output file path")]
        out: Option<String>,
    },
    /// Run simulation on a project
    Simulate {
        #[arg(help = "Path to the project package directory")]
        path: String,
        #[arg(help = "Simulation profile: ac_sweep, transient")]
        profile: String,
        #[arg(long, help = "Simulation engine: mock (default) or ngspice")]
        engine: Option<String>,
        #[arg(long, help = "Output file path for JSON results")]
        out: Option<String>,
        #[arg(long, help = "Timeout in milliseconds")]
        timeout: Option<u64>,
    },
    /// Run user-circuit simulation workflow on a project
    UserCircuitSimulate {
        #[arg(help = "Path to the project package directory")]
        path: String,
        #[arg(help = "Simulation profile ID: mock-ac, mock-op, mock-transient, auto-ac")]
        profile: String,
        #[arg(long, help = "Simulation engine: Mock, Ngspice, Auto")]
        engine: Option<String>,
        #[arg(long, help = "Output file path for JSON results")]
        out: Option<String>,
    },
    /// Library management commands
    Library {
        #[command(subcommand)]
        command: LibraryCommand,
    },
    /// Run simulation diagnostics on a project
    SimulateDiagnostics {
        #[arg(help = "Path to the project package directory")]
        path: String,
        #[arg(
            long,
            help = "Simulation profile ID: mock-ac, mock-op, mock-transient, auto-ac"
        )]
        profile: Option<String>,
        #[arg(long, help = "Output file path for JSON results")]
        out: Option<String>,
    },
    /// List simulation run history for a project
    SimulationHistory {
        #[arg(help = "Path to the project package directory")]
        path: String,
        #[arg(long, help = "Delete a specific run by ID")]
        delete: Option<String>,
        #[arg(long, help = "Clear all history")]
        clear: bool,
    },
}

#[derive(Subcommand)]
enum LibraryCommand {
    /// Check built-in component library integrity
    Check,
}

fn main() {
    let cli = Cli::parse();
    let api = build_headless_api();
    let _ = initialize_cli(&api);
    let result = match cli.command {
        Commands::Validate { path } => commands::handle_validate(&api, path, cli.json),
        Commands::Formula {
            formula_id,
            variables,
            json,
        } => commands::handle_formula(&api, formula_id, variables, json || cli.json),
        Commands::Netlist { path, out } => commands::handle_netlist(&api, path, out, cli.json),
        Commands::Export { path, format, out } => {
            commands::handle_export(&api, path, format, out, cli.json)
        }
        Commands::Simulate {
            path,
            profile,
            engine,
            out,
            timeout,
        } => commands::handle_simulate(&api, path, profile, engine, out, timeout, cli.json),
        Commands::UserCircuitSimulate {
            path,
            profile,
            engine,
            out,
        } => commands::handle_user_circuit_simulate(&api, path, profile, engine, out, cli.json),
        Commands::Library { command } => match command {
            LibraryCommand::Check => commands::handle_library_check(&api, cli.json),
        },
        Commands::SimulateDiagnostics { path, profile, out } => {
            commands::handle_simulate_diagnostics(&api, path, profile, out, cli.json)
        }
        Commands::SimulationHistory {
            path,
            delete,
            clear,
        } => commands::handle_simulation_history(&api, path, delete, clear, cli.json),
    };
    std::process::exit(result);
}
