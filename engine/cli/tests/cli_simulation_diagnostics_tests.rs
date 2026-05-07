use std::process::Command;

fn hotsas_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_hotsas-cli"))
}

#[test]
fn cli_simulate_diagnostics_help_mentions_command() {
    let output = hotsas_cli().arg("--help").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("simulate-diagnostics"),
        "help should mention simulate-diagnostics"
    );
    assert!(
        stdout.contains("simulation-history"),
        "help should mention simulation-history"
    );
}

#[test]
fn cli_simulate_diagnostics_json_contains_ngspice_diagnostics() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir =
        std::env::temp_dir().join(format!("hotsas_cli_simdiag_test_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulate-diagnostics",
            package_path.to_str().unwrap(),
            "--profile",
            "mock-ac",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    let status = json["status"].as_str().unwrap_or("");
    assert!(
        status == "success" || status == "warning",
        "simulate-diagnostics should return success or warning, got {status}"
    );
    assert!(
        json["data"]["ngspice_diagnostics"].is_object(),
        "should contain ngspice_diagnostics"
    );
    assert!(
        json["data"]["ngspice_diagnostics"]["availability"].is_object(),
        "should contain availability"
    );
    assert!(
        json["data"]["preflight_diagnostics"].is_array(),
        "should contain preflight_diagnostics"
    );
    assert!(
        json["data"]["summary"].is_object(),
        "should contain summary"
    );
}

#[test]
fn cli_simulate_diagnostics_without_profile_returns_ngspice_only() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_simdiag_no_profile_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulate-diagnostics",
            package_path.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    let status = json["status"].as_str().unwrap_or("");
    assert!(
        status == "success" || status == "warning",
        "simulate-diagnostics should return success or warning, got {status}"
    );
    assert!(
        json["data"]["ngspice_diagnostics"].is_object(),
        "should contain ngspice_diagnostics"
    );
    assert!(
        json["data"]["preflight_diagnostics"].is_null(),
        "preflight should be null without profile"
    );
}

#[test]
fn cli_simulate_diagnostics_with_profile_contains_preflight() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_sim_preflight_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulate-diagnostics",
            package_path.to_str().unwrap(),
            "--profile",
            "mock-ac",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    let status = json["status"].as_str().unwrap_or("");
    assert!(
        status == "success" || status == "warning",
        "simulate-diagnostics should return success or warning, got {status}"
    );
    assert!(
        json["data"]["preflight_diagnostics"].is_array(),
        "should contain preflight_diagnostics when profile is provided"
    );
    // last_run_diagnostics may be null if no run exists in this session
}

#[test]
fn cli_simulation_history_empty_project_returns_empty_list() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_simhist_empty_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulation-history",
            package_path.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "simulation-history should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert!(json["data"]["runs"].is_array(), "should contain runs array");
}

#[test]
fn cli_simulation_history_clear_returns_success() {
    let api = hotsas_cli::build_headless_api();
    api.create_rc_low_pass_demo_project().unwrap();

    let temp_dir = std::env::temp_dir().join(format!(
        "hotsas_cli_simhist_clear_test_{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let package_path = temp_dir.join("demo_project.circuit");

    api.save_project_package(package_path.to_str().unwrap().to_string())
        .unwrap();

    let output = hotsas_cli()
        .args([
            "simulation-history",
            package_path.to_str().unwrap(),
            "--clear",
            "--json",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "simulation-history --clear should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["cleared"], true);
}
