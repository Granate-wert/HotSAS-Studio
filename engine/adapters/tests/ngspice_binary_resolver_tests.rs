use hotsas_adapters::NgspiceBinaryResolver;

#[test]
fn resolver_returns_unavailable_when_path_missing() {
    // Ensure env is not set
    std::env::remove_var("HOTSAS_NGSPICE_PATH");
    let resolver = NgspiceBinaryResolver::new();
    let availability = resolver.resolve().expect("resolver should not panic");
    assert!(!availability.available);
}

#[test]
fn resolver_uses_env_path_when_set_to_invalid() {
    std::env::set_var("HOTSAS_NGSPICE_PATH", "/nonexistent/ngspice");
    let resolver = NgspiceBinaryResolver::new();
    let availability = resolver.resolve().expect("resolver should not panic");
    assert!(!availability.available);
    assert_eq!(
        availability.executable_path,
        Some("/nonexistent/ngspice".to_string())
    );
    std::env::remove_var("HOTSAS_NGSPICE_PATH");
}

#[test]
fn resolver_detects_invalid_path_without_panic() {
    let resolver = NgspiceBinaryResolver::new();
    let availability = resolver.resolve().expect("resolver should not panic");
    // On a typical CI/agent machine without ngspice installed, this should be unavailable
    // The test simply verifies it does not panic.
    assert!(availability.message.is_some());
}
