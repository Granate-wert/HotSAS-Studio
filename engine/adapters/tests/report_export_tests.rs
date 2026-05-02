use hotsas_adapters::MarkdownReportExporter;
use hotsas_core::{BomLine, EngineeringUnit, ReportModel, ReportSection, ValueWithUnit};
use hotsas_ports::ReportExporterPort;
use std::collections::BTreeMap;

#[test]
fn markdown_report_contains_vertical_slice_sections_and_bom() {
    let exporter = MarkdownReportExporter;
    let report = report_with_body("fc calculation");

    let markdown = exporter.export_markdown(&report).unwrap();

    for fragment in [
        "# RC Low-Pass Demo Report",
        "## Formula",
        "fc",
        "## Preferred Value",
        "## SPICE Netlist",
        "## Simulation",
        "## BOM",
        "R1",
        "C1",
    ] {
        assert!(
            markdown.contains(fragment),
            "markdown must contain fragment {fragment:?}"
        );
    }
}

#[test]
fn html_report_escapes_script_content() {
    let exporter = MarkdownReportExporter;
    let report = report_with_body("<script>alert(1)</script> & raw");

    let html = exporter.export_html(&report).unwrap();

    assert!(
        !html.contains("<script>alert(1)</script>"),
        "HTML output must not contain an active script tag"
    );
    assert!(
        html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
        "HTML output must contain escaped script text"
    );
    assert!(
        html.contains("&amp; raw"),
        "HTML output must escape ampersands"
    );
}

fn report_with_body(body: &str) -> ReportModel {
    ReportModel {
        id: "report".to_string(),
        title: "RC Low-Pass Demo Report".to_string(),
        sections: vec![
            ReportSection {
                title: "Formula".to_string(),
                body_markdown: format!("fc calculation: {body}"),
            },
            ReportSection {
                title: "Preferred Value".to_string(),
                body_markdown: "Nearest E24 value".to_string(),
            },
            ReportSection {
                title: "SPICE Netlist".to_string(),
                body_markdown: "V1 R1 C1 .ac".to_string(),
            },
            ReportSection {
                title: "Simulation".to_string(),
                body_markdown: "Mock AC simulation".to_string(),
            },
        ],
        included_schematic_images: vec![],
        included_formulas: vec![],
        included_simulation_results: vec![],
        included_bom: vec![
            BomLine {
                designator: "R1".to_string(),
                quantity: 1,
                value: Some(ValueWithUnit::new_si(10_000.0, EngineeringUnit::Ohm)),
                description: "Resistor".to_string(),
            },
            BomLine {
                designator: "C1".to_string(),
                quantity: 1,
                value: Some(ValueWithUnit::new_si(100e-9, EngineeringUnit::Farad)),
                description: "Capacitor".to_string(),
            },
        ],
        export_settings: BTreeMap::new(),
    }
}
