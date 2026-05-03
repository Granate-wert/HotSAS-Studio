use hotsas_adapters::SimpleTouchstoneParser;
use hotsas_ports::TouchstoneParserPort;

#[test]
fn parses_s1p_ri() {
    let parser = SimpleTouchstoneParser::new();
    let content = "# Hz S RI R 50\n1e9 0.5 0.1\n2e9 0.4 0.2\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    let network = report.network.unwrap();
    assert_eq!(network.port_count, 1);
    assert_eq!(network.points.len(), 2);
}

#[test]
fn parses_s1p_ma() {
    let parser = SimpleTouchstoneParser::new();
    let content = "# GHz S MA R 50\n1.0 0.5 30.0\n2.0 0.4 45.0\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    let network = report.network.unwrap();
    assert_eq!(network.port_count, 1);
    assert_eq!(network.points.len(), 2);
}

#[test]
fn parses_s1p_db() {
    let parser = SimpleTouchstoneParser::new();
    let content = "# MHz S DB R 50\n100 -6 30.0\n200 -8 45.0\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    let network = report.network.unwrap();
    assert_eq!(network.port_count, 1);
    assert_eq!(network.points.len(), 2);
}

#[test]
fn parses_s2p_ri() {
    let parser = SimpleTouchstoneParser::new();
    let content = "# GHz S RI R 50\n1.0 0.1 0.2 0.3 0.4 0.5 0.6 0.7 0.8\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s2p".to_string()), content)
        .unwrap();
    let network = report.network.unwrap();
    assert_eq!(network.port_count, 2);
    assert_eq!(network.points.len(), 1);
    assert_eq!(network.points[0].values.len(), 4);
}

#[test]
fn parses_frequency_units_hz_khz_mhz_ghz() {
    let parser = SimpleTouchstoneParser::new();
    for (unit, freq, expected_hz) in [
        ("Hz", 1e9, 1e9),
        ("kHz", 1e6, 1e9),
        ("MHz", 1e3, 1e9),
        ("GHz", 1.0, 1e9),
    ] {
        let content = format!("# {unit} S RI R 50\n{freq} 0.5 0.1\n");
        let report = parser
            .parse_touchstone_from_str(Some("test.s1p".to_string()), &content)
            .unwrap();
        let network = report.network.unwrap();
        assert!(
            (network.points[0].frequency_hz - expected_hz).abs() < 1.0,
            "unit {unit} failed"
        );
    }
}

#[test]
fn parses_reference_impedance() {
    let parser = SimpleTouchstoneParser::new();
    let content = "# GHz S RI R 75\n1.0 0.5 0.1\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    let network = report.network.unwrap();
    assert_eq!(network.reference_impedance_ohm, 75.0);
}

#[test]
fn ignores_comments() {
    let parser = SimpleTouchstoneParser::new();
    let content = "! comment\n# GHz S RI R 50\n1.0 0.5 0.1\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    assert!(report.network.is_some());
}

#[test]
fn missing_option_line_uses_defaults_with_warning() {
    let parser = SimpleTouchstoneParser::new();
    let content = "1.0 0.5 0.1\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    assert!(report.network.is_some());
    assert!(!report.warnings.is_empty());
}

#[test]
fn wrong_column_count_returns_warning() {
    let parser = SimpleTouchstoneParser::new();
    let content = "# GHz S RI R 50\n1.0 0.5\n";
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), content)
        .unwrap();
    assert!(!report.warnings.is_empty());
}

#[test]
fn empty_touchstone_returns_error() {
    let parser = SimpleTouchstoneParser::new();
    let report = parser
        .parse_touchstone_from_str(Some("test.s1p".to_string()), "")
        .unwrap();
    assert!(report.network.is_none());
    assert!(!report.errors.is_empty());
}
