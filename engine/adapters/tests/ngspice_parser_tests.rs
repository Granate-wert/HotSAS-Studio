use hotsas_adapters::NgspiceOutputParser;

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/ngspice/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture not found: {path}"))
}

#[test]
fn ac_output_parses_frequency_series() {
    let parser = NgspiceOutputParser::new();
    let content = fixture("ac_output.csv");
    let parsed = parser.parse_wrdata_file(&content).expect("should parse");
    assert_eq!(parsed.series.len(), 1);
    let series = &parsed.series[0];
    assert_eq!(series.points.len(), 6);
    assert!((series.points[0].x - 10.0).abs() < 1e-6);
    assert!((series.points[0].y - 0.99995).abs() < 1e-6);
}

#[test]
fn transient_output_parses_time_series() {
    let parser = NgspiceOutputParser::new();
    let content = fixture("tran_output.csv");
    let parsed = parser.parse_wrdata_file(&content).expect("should parse");
    assert_eq!(parsed.series.len(), 2);
    assert_eq!(parsed.series[0].points.len(), 4);
    assert_eq!(parsed.series[1].points.len(), 4);
}

#[test]
fn empty_file_returns_error() {
    let parser = NgspiceOutputParser::new();
    let result = parser.parse_wrdata_file("");
    assert!(result.is_err());
}

#[test]
fn invalid_numeric_rows_return_warnings() {
    let parser = NgspiceOutputParser::new();
    let content = "10 abc\n20 0.5\n";
    let parsed = parser
        .parse_wrdata_file(content)
        .expect("should parse valid rows");
    assert_eq!(parsed.series.len(), 1);
    assert_eq!(parsed.series[0].points.len(), 1);
    assert!(!parsed.warnings.is_empty());
}

#[test]
fn parser_does_not_panic_on_garbage() {
    let parser = NgspiceOutputParser::new();
    let result = parser.parse_wrdata_file("!!! garbage !!!\n### no numbers\n");
    assert!(result.is_err());
}

#[test]
fn op_stdout_parses_node_voltages() {
    let parser = NgspiceOutputParser::new();
    let content = fixture("op_stdout.txt");
    let measurements = parser
        .parse_operating_point_stdout(&content)
        .expect("should parse OP");
    assert!(measurements.contains_key("v(net_in)"));
    assert!(measurements.contains_key("v(net_out)"));
    assert!((measurements["v(net_in)"] - 1.0).abs() < 1e-6);
}

#[test]
fn op_stdout_empty_returns_error() {
    let parser = NgspiceOutputParser::new();
    let result = parser.parse_operating_point_stdout("no voltages here\n");
    assert!(result.is_err());
}
