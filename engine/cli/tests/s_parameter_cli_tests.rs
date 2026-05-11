use std::process::Command;
use std::io::Write;

fn hotsas_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_hotsas-cli"))
}

#[test]
fn cli_sparams_help_mentions_command() {
    let output = hotsas_cli().arg("--help").output().unwrap();
    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("sparams"),
        "help should mention sparams command"
    );
}

#[test]
fn cli_sparams_analyze_valid_s2p_returns_success() {
    let mut tmpfile = std::env::temp_dir();
    tmpfile.push("hotsas_test_sample.s2p");
    let content = "# Hz S RI R 50.0\n1000000 0.5 0.0 0.9 0.1 0.9 0.1 0.4 0.0\n10000000 0.3 0.1 0.8 0.2 0.8 0.2 0.3 0.1\n";
    {
        let mut f = std::fs::File::create(&tmpfile).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    let output = hotsas_cli()
        .args(["sparams", tmpfile.to_str().unwrap(), "--json"])
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "sparams analyze should succeed. stdout: {stdout}, stderr: {stderr}"
    );

    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(json["status"], "success", "JSON status should be success");
    assert!(json["data"].is_object(), "JSON data should contain result object");
    assert!(
        json["data"]["dataset"]["port_count"].is_number(),
        "should include port_count"
    );

    let _ = std::fs::remove_file(&tmpfile);
}

#[test]
fn cli_sparams_analyze_missing_file_returns_error() {
    let output = hotsas_cli()
        .args(["sparams", "/nonexistent/path/file.s2p", "--json"])
        .output()
        .unwrap();

    assert!(!output.status.success(), "missing file should fail");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("should still be valid JSON");
    assert_eq!(json["status"], "error", "JSON status should be error");
}

#[test]
fn cli_sparams_analyze_with_out_writes_file() {
    let mut tmpfile = std::env::temp_dir();
    tmpfile.push("hotsas_test_out.s2p");
    let mut outpath = std::env::temp_dir();
    outpath.push("hotsas_sparams_out.json");

    let content = "# Hz S RI R 50.0\n1000000 0.5 0.0 0.9 0.1 0.9 0.1 0.4 0.0\n";
    {
        let mut f = std::fs::File::create(&tmpfile).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    let output = hotsas_cli()
        .args([
            "sparams",
            tmpfile.to_str().unwrap(),
            "--out",
            outpath.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    assert!(output.status.success(), "sparams with --out should succeed");
    assert!(
        outpath.exists(),
        "output file should be created"
    );

    let written = std::fs::read_to_string(&outpath).unwrap();
    let json: serde_json::Value = serde_json::from_str(&written).expect("written file should be valid JSON");
    assert!(json["dataset"].is_object(), "written file should contain raw DTO with dataset");
    assert_eq!(json["dataset"]["port_count"], 2);

    let _ = std::fs::remove_file(&tmpfile);
    let _ = std::fs::remove_file(&outpath);
}
