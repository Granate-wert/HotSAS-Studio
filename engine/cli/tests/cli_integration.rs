use std::process::Command;

fn hotsas_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_hotsas-cli"))
}

#[test]
fn cli_version_returns_success() {
    let output = hotsas_cli().arg("--version").output().unwrap();
    assert!(output.status.success(), "--version should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("hotsas-cli"),
        "version output should contain binary name"
    );
}

#[test]
fn cli_help_returns_success_with_all_commands() {
    let output = hotsas_cli().arg("--help").output().unwrap();
    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validate"), "help should mention validate");
    assert!(stdout.contains("formula"), "help should mention formula");
    assert!(stdout.contains("netlist"), "help should mention netlist");
    assert!(stdout.contains("export"), "help should mention export");
    assert!(stdout.contains("simulate"), "help should mention simulate");
    assert!(
        stdout.contains("model-check"),
        "help should mention model-check"
    );
    assert!(stdout.contains("library"), "help should mention library");
}

#[test]
fn cli_library_check_returns_success() {
    let output = hotsas_cli().args(["library", "check"]).output().unwrap();
    assert!(output.status.success(), "library check should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("SUCCESS") || stdout.contains("success"),
        "library check should report success"
    );
}

#[test]
fn cli_library_check_json_returns_valid_json() {
    let output = hotsas_cli()
        .args(["library", "check", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "library check --json should succeed"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success", "JSON status should be success");
    assert!(
        json["data"].is_object(),
        "JSON data should contain library object"
    );
}

#[test]
fn cli_formula_ohms_law_returns_success() {
    let api = hotsas_cli::build_headless_api();
    let _ = hotsas_cli::initialize_cli(&api);

    let output = hotsas_cli()
        .args(["formula", "ohms_law", "V=10", "I=0.5", "R=1k", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "formula ohms_law should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
}

#[test]
fn cli_validate_nonexistent_project_returns_error() {
    let bad_path = std::env::temp_dir().join("nonexistent_circuit_12345.circuit");
    let output = hotsas_cli()
        .args(["validate", bad_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success(), "validate on bad path should fail");
    let code = output.status.code();
    assert!(
        code == Some(1) || code == Some(2),
        "validate on bad path should return exit code 1 or 2, got {:?}",
        code
    );
}

#[test]
fn cli_validate_existing_demo_project_returns_success() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!("hotsas_cli_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args(["validate", package_path.to_str().unwrap()])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "validate on demo project should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn save_demo_package(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_{}_{}_{}",
        name,
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");
    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();
    (temp_dir, package_path)
}

fn set_first_component_definition(package_path: &std::path::Path, definition_id: &str) {
    let schematic_path = package_path.join("schematic.json");
    let raw = std::fs::read_to_string(&schematic_path).unwrap();
    let mut schematic: serde_json::Value = serde_json::from_str(&raw).unwrap();
    schematic["components"][0]["definition_id"] = serde_json::Value::String(definition_id.into());
    std::fs::write(
        &schematic_path,
        serde_json::to_string_pretty(&schematic).unwrap(),
    )
    .unwrap();
}

#[test]
fn cli_model_check_simple_rc_ready() {
    let (temp_dir, package_path) = save_demo_package("model_check_ready");

    let output = hotsas_cli()
        .args(["model-check", package_path.to_str().unwrap(), "--json"])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "model-check should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["can_simulate"], true);
    assert!(json["data"]["summary"]["ready"].as_u64().unwrap() >= 1);
    assert_eq!(json["data"]["summary"]["blocking"], 0);
    assert_eq!(
        json["data"]["components"][0]["model_status"],
        "assigned_builtin"
    );
    assert_eq!(
        json["data"]["components"][0]["pin_mappings"][0]["model_pin_index"],
        0
    );
}

#[test]
fn cli_model_check_reports_placeholder_model() {
    let (temp_dir, package_path) = save_demo_package("model_check_placeholder");
    set_first_component_definition(&package_path, "generic_op_amp");

    let output = hotsas_cli()
        .args(["model-check", package_path.to_str().unwrap(), "--json"])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "placeholder model-check should exit successfully with warning. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "warning");
    assert_eq!(json["data"]["components"][0]["model_status"], "placeholder");
    assert_eq!(
        json["data"]["components"][0]["diagnostics"][0]["code"],
        "PLACEHOLDER_MODEL"
    );
    assert!(json["data"]["summary"]["placeholder"].as_u64().unwrap() >= 1);
    assert!(json["data"]["summary"]["warning"].as_u64().unwrap() >= 1);
}

#[test]
fn cli_model_check_reports_missing_model() {
    let (temp_dir, package_path) = save_demo_package("model_check_missing");
    set_first_component_definition(&package_path, "custom_unknown");

    let output = hotsas_cli()
        .args(["model-check", package_path.to_str().unwrap(), "--json"])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        !output.status.success(),
        "missing model-check should fail validation"
    );
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "validationerror");
    assert_eq!(json["data"]["can_simulate"], false);
    assert_eq!(json["data"]["components"][0]["model_status"], "missing");
    assert_eq!(
        json["data"]["components"][0]["diagnostics"][0]["code"],
        "MISSING_MODEL"
    );
    assert!(json["data"]["summary"]["missing"].as_u64().unwrap() >= 1);
    assert!(json["data"]["summary"]["blocking"].as_u64().unwrap() >= 1);
}

#[test]
fn cli_netlist_demo_project_returns_success() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir =
        std::env::temp_dir().join(format!("hotsas_cli_netlist_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args(["netlist", package_path.to_str().unwrap(), "--json"])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "netlist on demo project should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert!(
        json["data"]["netlist"].is_string(),
        "netlist data should contain netlist string"
    );
}

#[test]
fn cli_export_markdown_demo_project_returns_success() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir =
        std::env::temp_dir().join(format!("hotsas_cli_export_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "export",
            package_path.to_str().unwrap(),
            "markdown",
            "--out",
            temp_dir.join("report.md").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let markdown = std::fs::read_to_string(temp_dir.join("report.md")).unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "export markdown on demo project should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(markdown.contains("Model Mapping Readiness"));
    assert!(markdown.contains("Pin mapping") || markdown.contains("Pin Mapping"));
}

#[test]
fn cli_export_json_demo_project_returns_success() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_json_export_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args(["export", package_path.to_str().unwrap(), "json", "--json"])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "export json on demo project should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert!(json["data"]["content"]
        .as_str()
        .unwrap()
        .contains("ModelMappingReadiness"));
}

#[test]
fn cli_simulate_mock_demo_project_returns_success() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!("hotsas_cli_sim_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulate",
            package_path.to_str().unwrap(),
            "ac_sweep",
            "--engine",
            "mock",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "simulate mock on demo project should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
}

#[test]
fn cli_simulate_accepts_timeout_argument() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir =
        std::env::temp_dir().join(format!("hotsas_cli_timeout_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulate",
            package_path.to_str().unwrap(),
            "ac_sweep",
            "--engine",
            "mock",
            "--timeout",
            "5000",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "simulate with timeout should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
}

#[test]
fn cli_simulate_rejects_invalid_timeout() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_bad_timeout_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulate",
            package_path.to_str().unwrap(),
            "ac_sweep",
            "--engine",
            "mock",
            "--timeout",
            "not_a_number",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        !output.status.success(),
        "simulate with invalid timeout should fail"
    );
}

#[test]
fn cli_user_circuit_simulate_mock_ac_returns_series() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!("hotsas_cli_ucs_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "user-circuit-simulate",
            package_path.to_str().unwrap(),
            "mock-ac",
            "--engine",
            "Mock",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "user-circuit-simulate should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["status"], "Succeeded");
    assert_eq!(json["data"]["engine_used"], "mock");
    assert!(
        json["data"]["result"].is_object(),
        "result should contain simulation data"
    );
}

#[test]
fn cli_user_circuit_simulate_json_contains_status_and_engine() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir =
        std::env::temp_dir().join(format!("hotsas_cli_ucs_json_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "user-circuit-simulate",
            package_path.to_str().unwrap(),
            "mock-op",
            "--engine",
            "Mock",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "user-circuit-simulate op should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["status"], "Succeeded");
    assert_eq!(json["data"]["engine_used"], "mock");
    assert!(
        json["data"]["generated_netlist"].is_string(),
        "generated_netlist should be present"
    );
}

#[test]
fn cli_user_circuit_simulate_auto_fallback_contains_mock_warning() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir =
        std::env::temp_dir().join(format!("hotsas_cli_ucs_auto_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "user-circuit-simulate",
            package_path.to_str().unwrap(),
            "mock-ac",
            "--engine",
            "Auto",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "user-circuit-simulate auto should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["engine_used"], "mock");
    assert!(
        json["data"]["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["message"]
                .as_str()
                .unwrap_or("")
                .contains("ngspice unavailable")),
        "auto fallback should produce ngspice unavailable warning"
    );
}

#[test]
fn cli_user_circuit_simulate_invalid_profile_returns_exit_code_2() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_ucs_bad_profile_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "user-circuit-simulate",
            package_path.to_str().unwrap(),
            "invalid-profile",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        !output.status.success(),
        "user-circuit-simulate with invalid profile should fail"
    );
    let code = output.status.code();
    assert!(
        code == Some(2) || code == Some(1),
        "invalid profile should return exit code 1 or 2, got {:?}",
        code
    );
}
