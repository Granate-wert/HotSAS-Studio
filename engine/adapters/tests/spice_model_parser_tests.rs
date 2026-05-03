use hotsas_adapters::SimpleSpiceModelParser;
use hotsas_ports::SpiceModelParserPort;

#[test]
fn parses_diode_model() {
    let parser = SimpleSpiceModelParser::new();
    let content = ".model 1N4148 D(IS=2.52n RS=0.568 N=1.752 CJO=4p M=0.4)\n";
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 1);
    assert_eq!(report.models[0].name, "1N4148");
    assert!(!report.models[0].parameters.is_empty());
}

#[test]
fn parses_bjt_model() {
    let parser = SimpleSpiceModelParser::new();
    let content = ".model Q2N2222 NPN(IS=1e-14 BF=200 VAF=100)\n";
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 1);
    assert_eq!(report.models[0].name, "Q2N2222");
}

#[test]
fn parses_mosfet_model() {
    let parser = SimpleSpiceModelParser::new();
    let content = ".model IRLZ44N NMOS(VTO=2.0 KP=50u RD=0.02 RS=0.02)\n";
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 1);
    assert_eq!(report.models[0].name, "IRLZ44N");
}

#[test]
fn parses_multiple_models_from_lib() {
    let parser = SimpleSpiceModelParser::new();
    let content = r#"
.model 1N4148 D(IS=2.52n)
.model Q2N2222 NPN(IS=1e-14 BF=200)
"#;
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 2);
}

#[test]
fn parses_subckt_name_and_pins() {
    let parser = SimpleSpiceModelParser::new();
    let content = r#"
.subckt LM358 IN+ IN- VCC VEE OUT
RIN IN+ IN- 1Meg
.ends LM358
"#;
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.subcircuits.len(), 1);
    assert_eq!(report.subcircuits[0].name, "LM358");
    assert_eq!(report.subcircuits[0].pins.len(), 5);
}

#[test]
fn parses_subckt_body_until_ends() {
    let parser = SimpleSpiceModelParser::new();
    let content = r#"
.subckt LM358 IN+ IN- VCC VEE OUT
RIN IN+ IN- 1Meg
EOUT OUT 0 VALUE={V(IN+)-V(IN-)}
.ends LM358
"#;
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert!(!report.subcircuits[0].body.is_empty());
}

#[test]
fn supports_line_continuation() {
    let parser = SimpleSpiceModelParser::new();
    let content = ".model 1N4148 D(IS=2.52n RS=0.568\n+ N=1.752 CJO=4p M=0.4)\n";
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 1);
}

#[test]
fn ignores_comment_lines() {
    let parser = SimpleSpiceModelParser::new();
    let content = r#"
* this is a comment
.model 1N4148 D(IS=2.52n)
; another comment
"#;
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 1);
}

#[test]
fn unknown_model_type_returns_warning() {
    let parser = SimpleSpiceModelParser::new();
    let content = ".model XYZ UNKNOWN(PARAM=1)\n";
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert_eq!(report.models.len(), 1);
    assert!(!report.warnings.is_empty());
}

#[test]
fn unsupported_directives_return_warnings() {
    let parser = SimpleSpiceModelParser::new();
    let content = r#"
.include other.lib
.param x=1
.model 1N4148 D(IS=2.52n)
"#;
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert!(!report.warnings.is_empty());
}

#[test]
fn empty_spice_file_returns_parsed_with_empty_report() {
    let parser = SimpleSpiceModelParser::new();
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), "")
        .unwrap();
    assert!(report.models.is_empty());
    assert!(report.subcircuits.is_empty());
}

#[test]
fn malformed_model_does_not_panic() {
    let parser = SimpleSpiceModelParser::new();
    let content = ".model\n";
    let report = parser
        .parse_spice_models_from_str(Some("test.lib".to_string()), content)
        .unwrap();
    assert!(report.models.is_empty());
}
