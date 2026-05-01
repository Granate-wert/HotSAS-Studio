use std::fs;

#[test]
fn workspace_crates_keep_dependency_direction() {
    let core = fs::read_to_string("../core/Cargo.toml").unwrap();
    let application = fs::read_to_string("../application/Cargo.toml").unwrap();
    let adapters = fs::read_to_string("../adapters/Cargo.toml").unwrap();
    let api = fs::read_to_string("Cargo.toml").unwrap();

    assert!(!core.contains("hotsas_application"));
    assert!(!core.contains("hotsas_adapters"));
    assert!(!core.contains("hotsas_api"));
    assert!(!core.contains("tauri"));

    assert!(application.contains("hotsas_core"));
    assert!(application.contains("hotsas_ports"));
    assert!(!application.contains("hotsas_adapters"));
    assert!(!application.contains("hotsas_api"));

    assert!(adapters.contains("hotsas_core"));
    assert!(adapters.contains("hotsas_ports"));
    assert!(!adapters.contains("hotsas_api"));

    assert!(api.contains("hotsas_application"));
    assert!(!api.contains("hotsas_adapters"));
}
