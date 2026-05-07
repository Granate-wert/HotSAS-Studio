use hotsas_api::{
    AcSweepSettingsDto, AdvancedReportExportRequestDto, AdvancedReportRequestDto, ApiError,
    FormulaCalculationRequestDto, FormulaVariableInputDto, HotSasApi, OperatingPointSettingsDto,
    ProjectOpenRequestDto, ReportExportOptionsDto, SimulationDiagnosticMessageDto,
    SimulationRunRequestDto, TransientSettingsDto, UserCircuitSimulationProfileDto,
};

use crate::output::{print_output, CliOutput, CliStatus};

pub fn handle_validate(api: &HotSasApi, path: String, json: bool) -> i32 {
    let result = api.validate_project_package(path);
    match result {
        Ok(report) => {
            let is_valid = report.errors.is_empty();
            let status = if is_valid {
                CliStatus::Success
            } else {
                CliStatus::ValidationError
            };
            let output = CliOutput {
                status,
                command: "validate".to_string(),
                warnings: report.warnings.clone(),
                errors: report.errors.clone(),
                data: Some(report),
            };
            print_output(&output, json);
            if is_valid {
                0
            } else {
                2
            }
        }
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "validate".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            exit_code(&e)
        }
    }
}

pub fn handle_formula(
    api: &HotSasApi,
    formula_id: String,
    variables: Vec<String>,
    json: bool,
) -> i32 {
    let vars = match parse_key_value_pairs(&variables) {
        Ok(v) => v,
        Err(e) => {
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "formula".to_string(),
                warnings: vec![],
                errors: vec![e.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", e);
            return 3;
        }
    };

    let request = FormulaCalculationRequestDto {
        formula_id,
        variables: vars,
    };

    let result = api.calculate_formula(request);
    match result {
        Ok(dto) => {
            let output = CliOutput {
                status: CliStatus::Success,
                command: "formula".to_string(),
                warnings: vec![],
                errors: vec![],
                data: Some(dto),
            };
            print_output(&output, json);
            0
        }
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "formula".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            exit_code(&e)
        }
    }
}

pub fn handle_netlist(api: &HotSasApi, path: String, out: Option<String>, json: bool) -> i32 {
    match load_project(api, &path, json) {
        Ok(_) => {}
        Err(code) => return code,
    }

    match api.generate_spice_netlist() {
        Ok(netlist) => {
            if let Some(out_path) = out {
                if let Err(e) = std::fs::write(&out_path, &netlist) {
                    let msg = format!("Failed to write netlist to {}: {}", out_path, e);
                    let output = CliOutput::<()> {
                        status: CliStatus::Error,
                        command: "netlist".to_string(),
                        warnings: vec![],
                        errors: vec![msg.clone()],
                        data: None,
                    };
                    print_output(&output, json);
                    eprintln!("{}", msg);
                    return 1;
                }
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "netlist".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "path": out_path })),
                };
                print_output(&output, json);
            } else {
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "netlist".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "netlist": netlist })),
                };
                print_output(&output, json);
            }
            0
        }
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "netlist".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            exit_code(&e)
        }
    }
}

fn generate_and_export_report(
    api: &HotSasApi,
    format: &str,
    out: Option<String>,
    json: bool,
) -> Result<(String, String), i32> {
    let ts = chrono::Local::now().timestamp_millis();
    let report_id = format!("cli_{}_{}", format, ts);
    let gen_request = AdvancedReportRequestDto {
        report_id: report_id.clone(),
        title: "Project Summary".to_string(),
        report_type: "ProjectSummary".to_string(),
        included_sections: vec!["ProjectInfo".to_string()],
        export_options: ReportExportOptionsDto {
            include_source_references: false,
            include_graph_references: false,
            include_assumptions: false,
            max_table_rows: None,
        },
        metadata: std::collections::BTreeMap::new(),
    };
    if let Err(e) = api.generate_advanced_report(gen_request) {
        let msg = format_error(&e);
        let output = CliOutput::<()> {
            status: CliStatus::Error,
            command: "export".to_string(),
            warnings: vec![],
            errors: vec![msg.clone()],
            data: None,
        };
        print_output(&output, json);
        eprintln!("{}", msg);
        return Err(exit_code(&e));
    }
    let export_request = AdvancedReportExportRequestDto {
        report_id,
        format: format.to_string(),
        output_path: out,
    };
    match api.export_advanced_report(export_request) {
        Ok(result) => Ok((result.content, format.to_string())),
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "export".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            Err(exit_code(&e))
        }
    }
}

pub fn handle_export(
    api: &HotSasApi,
    path: String,
    format: String,
    out: Option<String>,
    json: bool,
) -> i32 {
    match load_project(api, &path, json) {
        Ok(_) => {}
        Err(code) => return code,
    }

    let (content, ext): (String, &str) = match format.as_str() {
        "markdown" | "md" => match api.export_markdown_report() {
            Ok(c) => (c, "md"),
            Err(e) => {
                let msg = format_error(&e);
                let output = CliOutput::<()> {
                    status: CliStatus::Error,
                    command: "export".to_string(),
                    warnings: vec![],
                    errors: vec![msg.clone()],
                    data: None,
                };
                print_output(&output, json);
                eprintln!("{}", msg);
                return exit_code(&e);
            }
        },
        "html" => match api.export_html_report() {
            Ok(c) => (c, "html"),
            Err(e) => {
                let msg = format_error(&e);
                let output = CliOutput::<()> {
                    status: CliStatus::Error,
                    command: "export".to_string(),
                    warnings: vec![],
                    errors: vec![msg.clone()],
                    data: None,
                };
                print_output(&output, json);
                eprintln!("{}", msg);
                return exit_code(&e);
            }
        },
        "json" => match generate_and_export_report(api, "json", out.clone(), json) {
            Ok((c, _)) => (c, "json"),
            Err(code) => return code,
        },
        "csv-summary" => match generate_and_export_report(api, "csv_summary", out.clone(), json) {
            Ok((c, _)) => (c, "csv"),
            Err(code) => return code,
        },
        other => {
            let msg = format!("Unsupported export format: {}", other);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "export".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            return 4;
        }
    };

    if let Some(out_path) = out {
        if let Err(e) = std::fs::write(&out_path, &content) {
            let msg = format!("Failed to write export to {}: {}", out_path, e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "export".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            return 1;
        }
        let output = CliOutput {
            status: CliStatus::Success,
            command: "export".to_string(),
            warnings: vec![],
            errors: vec![],
            data: Some(serde_json::json!({ "path": out_path })),
        };
        print_output(&output, json);
    } else {
        if json {
            let output = CliOutput {
                status: CliStatus::Success,
                command: "export".to_string(),
                warnings: vec![],
                errors: vec![],
                data: Some(serde_json::json!({ "content": content, "format": ext })),
            };
            print_output(&output, json);
        } else {
            println!("{}", content);
        }
    }
    0
}

pub fn handle_simulate(
    api: &HotSasApi,
    path: String,
    profile: String,
    engine: Option<String>,
    out: Option<String>,
    timeout: Option<u64>,
    json: bool,
) -> i32 {
    match load_project(api, &path, json) {
        Ok(_) => {}
        Err(code) => return code,
    }

    let engine_name = engine.unwrap_or_else(|| "mock".to_string());
    let request = SimulationRunRequestDto {
        engine: engine_name,
        analysis_kind: profile,
        profile_id: None,
        output_variables: vec![],
        timeout_ms: timeout,
    };

    match api.run_simulation(request) {
        Ok(result) => {
            if let Some(out_path) = out {
                let json_str = match serde_json::to_string_pretty(&result) {
                    Ok(s) => s,
                    Err(e) => {
                        let msg = format!("Serialization error: {}", e);
                        let output = CliOutput::<()> {
                            status: CliStatus::Error,
                            command: "simulate".to_string(),
                            warnings: vec![],
                            errors: vec![msg.clone()],
                            data: None,
                        };
                        print_output(&output, json);
                        eprintln!("{}", msg);
                        return 1;
                    }
                };
                if let Err(e) = std::fs::write(&out_path, json_str) {
                    let msg = format!("Failed to write simulation result to {}: {}", out_path, e);
                    let output = CliOutput::<()> {
                        status: CliStatus::Error,
                        command: "simulate".to_string(),
                        warnings: vec![],
                        errors: vec![msg.clone()],
                        data: None,
                    };
                    print_output(&output, json);
                    eprintln!("{}", msg);
                    return 1;
                }
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "simulate".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "path": out_path })),
                };
                print_output(&output, json);
            } else {
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "simulate".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(result),
                };
                print_output(&output, json);
            }
            0
        }
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "simulate".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            exit_code(&e)
        }
    }
}

pub fn handle_user_circuit_simulate(
    api: &HotSasApi,
    path: String,
    profile_id: String,
    engine: Option<String>,
    out: Option<String>,
    json: bool,
) -> i32 {
    match load_project(api, &path, json) {
        Ok(_) => {}
        Err(code) => return code,
    }

    // Build a profile DTO from the profile ID
    let profile = build_user_circuit_profile(&profile_id, engine);
    let profile = match profile {
        Ok(p) => p,
        Err(e) => {
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "user-circuit-simulate".to_string(),
                warnings: vec![],
                errors: vec![e.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", e);
            return 2;
        }
    };

    match api.run_current_circuit_simulation(profile) {
        Ok(result) => {
            if let Some(out_path) = out {
                let json_str = match serde_json::to_string_pretty(&result) {
                    Ok(s) => s,
                    Err(e) => {
                        let msg = format!("Serialization error: {}", e);
                        let output = CliOutput::<()> {
                            status: CliStatus::Error,
                            command: "user-circuit-simulate".to_string(),
                            warnings: vec![],
                            errors: vec![msg.clone()],
                            data: None,
                        };
                        print_output(&output, json);
                        eprintln!("{}", msg);
                        return 1;
                    }
                };
                if let Err(e) = std::fs::write(&out_path, json_str) {
                    let msg = format!("Failed to write simulation result to {}: {}", out_path, e);
                    let output = CliOutput::<()> {
                        status: CliStatus::Error,
                        command: "user-circuit-simulate".to_string(),
                        warnings: vec![],
                        errors: vec![msg.clone()],
                        data: None,
                    };
                    print_output(&output, json);
                    eprintln!("{}", msg);
                    return 1;
                }
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "user-circuit-simulate".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "path": out_path })),
                };
                print_output(&output, json);
            } else {
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "user-circuit-simulate".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(result),
                };
                print_output(&output, json);
            }
            0
        }
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "user-circuit-simulate".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            exit_code(&e)
        }
    }
}

fn build_user_circuit_profile(
    profile_id: &str,
    engine: Option<String>,
) -> Result<UserCircuitSimulationProfileDto, String> {
    let engine = engine.unwrap_or_else(|| "Mock".to_string());
    match profile_id {
        "mock-ac" | "ac-sweep" => Ok(UserCircuitSimulationProfileDto {
            id: "mock-ac".to_string(),
            name: "AC Sweep".to_string(),
            analysis_type: "AcSweep".to_string(),
            engine,
            probes: vec![],
            ac: Some(AcSweepSettingsDto {
                start_hz: 10.0,
                stop_hz: 1_000_000.0,
                points_per_decade: 100,
            }),
            transient: None,
            op: None,
        }),
        "mock-op" | "operating-point" => Ok(UserCircuitSimulationProfileDto {
            id: "mock-op".to_string(),
            name: "Operating Point".to_string(),
            analysis_type: "OperatingPoint".to_string(),
            engine,
            probes: vec![],
            ac: None,
            transient: None,
            op: Some(OperatingPointSettingsDto {
                include_node_voltages: true,
                include_branch_currents: true,
            }),
        }),
        "mock-transient" | "transient" => Ok(UserCircuitSimulationProfileDto {
            id: "mock-transient".to_string(),
            name: "Transient".to_string(),
            analysis_type: "Transient".to_string(),
            engine,
            probes: vec![],
            ac: None,
            transient: Some(TransientSettingsDto {
                step_seconds: 1e-6,
                stop_seconds: 1e-3,
            }),
            op: None,
        }),
        other => Err(format!("Unknown profile ID: {}", other)),
    }
}

pub fn handle_simulate_diagnostics(
    api: &HotSasApi,
    path: String,
    profile: Option<String>,
    out: Option<String>,
    json: bool,
) -> i32 {
    match load_project(api, &path, json) {
        Ok(_) => {}
        Err(code) => return code,
    }

    let mut all_warnings: Vec<String> = vec![];
    let mut all_errors: Vec<String> = vec![];

    // 1. ngspice diagnostics
    let ngspice_diag = match api.check_ngspice_diagnostics() {
        Ok(d) => {
            if !d.availability.available {
                all_warnings.push(format!(
                    "ngspice unavailable: {}",
                    d.availability.message.clone().unwrap_or_default()
                ));
            }
            Some(d)
        }
        Err(e) => {
            let msg = format_error(&e);
            all_errors.push(format!("ngspice diagnostics failed: {msg}"));
            None
        }
    };

    // 2. preflight diagnostics (if profile provided)
    let preflight_diagnostics: Option<Vec<SimulationDiagnosticMessageDto>> =
        if let Some(profile_id) = profile {
            match build_user_circuit_profile(&profile_id, None) {
                Ok(profile_dto) => match api.diagnose_simulation_preflight(profile_dto) {
                    Ok(diagnostics) => {
                        for d in &diagnostics {
                            match d.severity.as_str() {
                                "Blocking" => all_errors
                                    .push(format!("[{}] {}: {}", d.code, d.title, d.message)),
                                "Error" => all_errors
                                    .push(format!("[{}] {}: {}", d.code, d.title, d.message)),
                                "Warning" => all_warnings
                                    .push(format!("[{}] {}: {}", d.code, d.title, d.message)),
                                _ => {}
                            }
                        }
                        Some(diagnostics)
                    }
                    Err(e) => {
                        let msg = format_error(&e);
                        all_errors.push(format!("preflight diagnostics failed: {msg}"));
                        None
                    }
                },
                Err(e) => {
                    all_errors.push(format!("invalid profile: {e}"));
                    None
                }
            }
        } else {
            None
        };

    // 3. last run diagnostics
    let last_run_diagnostics: Option<Vec<SimulationDiagnosticMessageDto>> = match api
        .diagnose_last_simulation_run()
    {
        Ok(diagnostics) => {
            for d in &diagnostics {
                match d.severity.as_str() {
                    "Blocking" => {
                        all_errors.push(format!("[{}] {}: {}", d.code, d.title, d.message))
                    }
                    "Error" => all_errors.push(format!("[{}] {}: {}", d.code, d.title, d.message)),
                    "Warning" => {
                        all_warnings.push(format!("[{}] {}: {}", d.code, d.title, d.message))
                    }
                    _ => {}
                }
            }
            Some(diagnostics)
        }
        Err(_) => {
            // No last run is not an error
            None
        }
    };

    let blocking_count = preflight_diagnostics
        .as_ref()
        .map(|d| d.iter().filter(|x| x.severity == "Blocking").count())
        .unwrap_or(0)
        + last_run_diagnostics
            .as_ref()
            .map(|d| d.iter().filter(|x| x.severity == "Blocking").count())
            .unwrap_or(0);
    let error_count = preflight_diagnostics
        .as_ref()
        .map(|d| d.iter().filter(|x| x.severity == "Error").count())
        .unwrap_or(0)
        + last_run_diagnostics
            .as_ref()
            .map(|d| d.iter().filter(|x| x.severity == "Error").count())
            .unwrap_or(0);
    let warning_count = preflight_diagnostics
        .as_ref()
        .map(|d| d.iter().filter(|x| x.severity == "Warning").count())
        .unwrap_or(0)
        + last_run_diagnostics
            .as_ref()
            .map(|d| d.iter().filter(|x| x.severity == "Warning").count())
            .unwrap_or(0)
        + ngspice_diag
            .as_ref()
            .map(|d| d.warnings.len() + d.errors.len())
            .unwrap_or(0);

    let data = serde_json::json!({
        "ngspice_diagnostics": ngspice_diag,
        "preflight_diagnostics": preflight_diagnostics,
        "last_run_diagnostics": last_run_diagnostics,
        "summary": {
            "blocking_count": blocking_count,
            "error_count": error_count,
            "warning_count": warning_count,
            "info_count": preflight_diagnostics.as_ref().map(|d| d.iter().filter(|x| x.severity == "Info").count()).unwrap_or(0)
                + last_run_diagnostics.as_ref().map(|d| d.iter().filter(|x| x.severity == "Info").count()).unwrap_or(0),
        }
    });

    let status = if blocking_count > 0 || !all_errors.is_empty() {
        CliStatus::ValidationError
    } else if warning_count > 0 || !all_warnings.is_empty() {
        CliStatus::Warning
    } else {
        CliStatus::Success
    };

    let output = CliOutput {
        status,
        command: "simulate-diagnostics".to_string(),
        warnings: all_warnings,
        errors: all_errors,
        data: Some(data),
    };

    let json_str = match serde_json::to_string_pretty(&output) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Serialization error: {e}");
            return 1;
        }
    };

    if let Some(out_path) = out {
        if let Err(e) = std::fs::write(&out_path, &json_str) {
            let msg = format!("Failed to write diagnostics to {out_path}: {e}");
            let err_output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "simulate-diagnostics".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&err_output, json);
            eprintln!("{msg}");
            return 1;
        }
        let file_output = CliOutput {
            status: CliStatus::Success,
            command: "simulate-diagnostics".to_string(),
            warnings: vec![],
            errors: vec![],
            data: Some(serde_json::json!({ "path": out_path })),
        };
        print_output(&file_output, json);
    } else {
        print_output(&output, json);
    }

    if blocking_count > 0 {
        2
    } else if error_count > 0 {
        1
    } else {
        0
    }
}

pub fn handle_simulation_history(
    api: &HotSasApi,
    path: String,
    delete: Option<String>,
    clear: bool,
    json: bool,
) -> i32 {
    match load_project(api, &path, json) {
        Ok(_) => {}
        Err(code) => return code,
    }

    if clear {
        match api.clear_simulation_history() {
            Ok(_) => {
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "simulation-history".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "cleared": true })),
                };
                print_output(&output, json);
                0
            }
            Err(e) => {
                let msg = format_error(&e);
                let output = CliOutput::<()> {
                    status: CliStatus::Error,
                    command: "simulation-history".to_string(),
                    warnings: vec![],
                    errors: vec![msg.clone()],
                    data: None,
                };
                print_output(&output, json);
                eprintln!("{msg}");
                exit_code(&e)
            }
        }
    } else if let Some(run_id) = delete {
        match api.delete_simulation_history_run(run_id.clone()) {
            Ok(_) => {
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "simulation-history".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "deleted_run_id": run_id })),
                };
                print_output(&output, json);
                0
            }
            Err(e) => {
                let msg = format_error(&e);
                let output = CliOutput::<()> {
                    status: CliStatus::Error,
                    command: "simulation-history".to_string(),
                    warnings: vec![],
                    errors: vec![msg.clone()],
                    data: None,
                };
                print_output(&output, json);
                eprintln!("{msg}");
                exit_code(&e)
            }
        }
    } else {
        match api.list_simulation_history() {
            Ok(runs) => {
                let output = CliOutput {
                    status: CliStatus::Success,
                    command: "simulation-history".to_string(),
                    warnings: vec![],
                    errors: vec![],
                    data: Some(serde_json::json!({ "runs": runs })),
                };
                print_output(&output, json);
                0
            }
            Err(e) => {
                let msg = format_error(&e);
                let output = CliOutput::<()> {
                    status: CliStatus::Error,
                    command: "simulation-history".to_string(),
                    warnings: vec![],
                    errors: vec![msg.clone()],
                    data: None,
                };
                print_output(&output, json);
                eprintln!("{msg}");
                exit_code(&e)
            }
        }
    }
}

pub fn handle_library_check(api: &HotSasApi, json: bool) -> i32 {
    match api.load_builtin_component_library() {
        Ok(library) => {
            let output = CliOutput {
                status: CliStatus::Success,
                command: "library check".to_string(),
                warnings: vec![],
                errors: vec![],
                data: Some(library),
            };
            print_output(&output, json);
            0
        }
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "library check".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("{}", msg);
            exit_code(&e)
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load_project(api: &HotSasApi, path: &str, json: bool) -> Result<(), i32> {
    let request = ProjectOpenRequestDto {
        path: path.to_string(),
        confirm_discard_unsaved: false,
    };
    match api.open_project_package(request) {
        Ok(_) => Ok(()),
        Err(e) => {
            let msg = format_error(&e);
            let output = CliOutput::<()> {
                status: CliStatus::Error,
                command: "load_project".to_string(),
                warnings: vec![],
                errors: vec![msg.clone()],
                data: None,
            };
            print_output(&output, json);
            eprintln!("Error loading project '{}': {}", path, msg);
            Err(exit_code(&e))
        }
    }
}

fn parse_key_value_pairs(raw: &[String]) -> Result<Vec<FormulaVariableInputDto>, String> {
    let mut vars = Vec::new();
    for item in raw {
        let parts: Vec<&str> = item.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid variable format '{}'. Expected key=value.",
                item
            ));
        }
        let name = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        vars.push(FormulaVariableInputDto {
            name,
            value,
            unit: None,
        });
    }
    Ok(vars)
}

fn format_error(e: &ApiError) -> String {
    match e {
        ApiError::InvalidInput(s) => format!("Invalid input: {}", s),
        ApiError::State(s) => format!("State error: {}", s),
        ApiError::Application(ae) => format!("Application error: {}", ae),
        ApiError::Core(ce) => format!("Core error: {}", ce),
    }
}

fn exit_code(e: &ApiError) -> i32 {
    match e {
        ApiError::InvalidInput(_) => 2,
        ApiError::State(_) => 1,
        ApiError::Core(_) => 1,
        ApiError::Application(ae) => match ae {
            hotsas_application::ApplicationError::InvalidInput(_)
            | hotsas_application::ApplicationError::NotFound(_)
            | hotsas_application::ApplicationError::MissingProjectState(_)
            | hotsas_application::ApplicationError::FormulaNotFound(_)
            | hotsas_application::ApplicationError::DuplicateFormulaId(_)
            | hotsas_application::ApplicationError::InvalidFormulaPack(_)
            | hotsas_application::ApplicationError::InvalidBinding(_) => 2,
            hotsas_application::ApplicationError::State(_)
            | hotsas_application::ApplicationError::Core(_)
            | hotsas_application::ApplicationError::Port(_)
            | hotsas_application::ApplicationError::Storage(_)
            | hotsas_application::ApplicationError::Export(_)
            | hotsas_application::ApplicationError::Simulation(_) => 1,
        },
    }
}
