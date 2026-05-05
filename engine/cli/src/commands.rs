use hotsas_api::{
    AdvancedReportExportRequestDto, AdvancedReportRequestDto, ApiError,
    FormulaCalculationRequestDto, FormulaVariableInputDto, HotSasApi, ProjectOpenRequestDto,
    ReportExportOptionsDto, SimulationRunRequestDto,
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
        "json" => {
            let project_result = api.open_project_package(ProjectOpenRequestDto {
                path: path.clone(),
                confirm_discard_unsaved: false,
            });
            match project_result {
                Ok(result) => match serde_json::to_string_pretty(&result.project) {
                    Ok(c) => (c, "json"),
                    Err(e) => {
                        let msg = format!("JSON serialization error: {}", e);
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
                },
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
            }
        }
        "csv-summary" => {
            // Delegate to AdvancedReport service: generate report then export as CSV summary.
            let gen_request = AdvancedReportRequestDto {
                report_id: "cli_csv_summary".to_string(),
                title: "Project Summary".to_string(),
                report_type: "project_summary".to_string(),
                included_sections: vec!["project_overview".to_string()],
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
                return exit_code(&e);
            }
            let export_request = AdvancedReportExportRequestDto {
                report_id: "cli_csv_summary".to_string(),
                format: "csv_summary".to_string(),
                output_path: out.clone(),
            };
            match api.export_advanced_report(export_request) {
                Ok(result) => (result.content, "csv"),
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
            }
        }
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
        timeout_ms: None,
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
