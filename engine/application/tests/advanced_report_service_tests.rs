use hotsas_application::AdvancedReportService;
use hotsas_core::advanced_report::{
    AdvancedReportContext, AdvancedReportRequest, AdvancedReportType, ReportExportOptions,
    ReportSectionKind, ReportSectionStatus,
};

fn make_service() -> AdvancedReportService {
    AdvancedReportService::new()
}

fn make_request(report_type: AdvancedReportType, sections: Vec<ReportSectionKind>) -> AdvancedReportRequest {
    AdvancedReportRequest {
        report_id: "test-report-1".to_string(),
        title: "Test Report".to_string(),
        report_type,
        included_sections: sections,
        export_options: ReportExportOptions {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: Default::default(),
    }
}

fn empty_context() -> AdvancedReportContext {
    AdvancedReportContext {
        project: None,
        notebook: None,
        simulation_result: None,
        dcdc_result: None,
        selected_region_result: None,
        netlist: None,
        export_history: vec![],
        imported_models_summary: vec![],
    }
}

#[test]
fn list_capabilities_returns_all_section_kinds() {
    let service = make_service();
    let caps = service.list_section_capabilities();
    assert!(!caps.is_empty());
    let kinds: Vec<_> = caps.iter().map(|c| c.kind.clone()).collect();
    assert!(kinds.contains(&ReportSectionKind::ProjectInfo));
    assert!(kinds.contains(&ReportSectionKind::SchematicSummary));
}

#[test]
fn generate_project_summary_without_project_returns_empty_sections() {
    let service = make_service();
    let request = make_request(AdvancedReportType::ProjectSummary, vec![
        ReportSectionKind::ProjectInfo,
        ReportSectionKind::SchematicSummary,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();

    assert_eq!(report.id, "test-report-1");
    assert_eq!(report.title, "Test Report");
    assert_eq!(report.report_type, AdvancedReportType::ProjectSummary);
    assert!(!report.sections.is_empty());
}

#[test]
fn generate_full_project_report_with_no_context_does_not_panic() {
    let service = make_service();
    let request = make_request(AdvancedReportType::FullProjectReport, vec![
        ReportSectionKind::ProjectInfo,
        ReportSectionKind::SchematicSummary,
        ReportSectionKind::FormulaCalculations,
        ReportSectionKind::SimulationResults,
        ReportSectionKind::DcdcCalculations,
        ReportSectionKind::SelectedRegionAnalysis,
        ReportSectionKind::ExportHistory,
        ReportSectionKind::ImportedModels,
        ReportSectionKind::ComponentSummary,
        ReportSectionKind::SpiceNetlist,
        ReportSectionKind::Bom,
        ReportSectionKind::NotebookCalculations,
        ReportSectionKind::ESeriesSelections,
        ReportSectionKind::WarningsAndAssumptions,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();

    // Without project data, most sections should be Empty or Unavailable
    for section in &report.sections {
        assert!(
            matches!(
                section.status,
                ReportSectionStatus::Included | ReportSectionStatus::Empty | ReportSectionStatus::Unavailable
            ),
            "Section {:?} had unexpected status {:?}",
            section.kind,
            section.status
        );
    }
}

#[test]
fn render_markdown_produces_non_empty_output() {
    let service = make_service();
    let request = make_request(AdvancedReportType::ProjectSummary, vec![
        ReportSectionKind::ProjectInfo,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();
    let markdown = service.render_report_markdown(&report).unwrap();

    assert!(!markdown.is_empty());
    assert!(markdown.contains("#") || markdown.contains("Test Report"));
}

#[test]
fn render_html_produces_non_empty_output() {
    let service = make_service();
    let request = make_request(AdvancedReportType::ProjectSummary, vec![
        ReportSectionKind::ProjectInfo,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();
    let html = service.render_report_html(&report).unwrap();

    assert!(!html.is_empty());
    assert!(html.contains("<html>") || html.contains("<body>") || html.contains("<h1>"));
}

#[test]
fn render_json_produces_valid_json() {
    let service = make_service();
    let request = make_request(AdvancedReportType::ProjectSummary, vec![
        ReportSectionKind::ProjectInfo,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();
    let json = service.render_report_json(&report).unwrap();

    assert!(!json.is_empty());
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON should be valid");
    assert!(parsed.get("id").is_some() || parsed.get("sections").is_some());
}

#[test]
fn render_csv_summary_produces_csv_lines() {
    let service = make_service();
    let request = make_request(AdvancedReportType::ProjectSummary, vec![
        ReportSectionKind::ProjectInfo,
        ReportSectionKind::SchematicSummary,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();
    let csv = service.render_report_csv_summary(&report).unwrap();

    assert!(!csv.is_empty());
    let lines: Vec<&str> = csv.lines().collect();
    assert!(lines.len() >= 2); // header + at least one data row
    assert!(lines[0].contains("section_id") || lines[0].contains("section_title"));
}

#[test]
fn section_status_is_unavailable_for_missing_data() {
    let service = make_service();
    let request = make_request(AdvancedReportType::SimulationReport, vec![
        ReportSectionKind::SimulationResults,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();

    let sim_section = report
        .sections
        .iter()
        .find(|s| s.kind == ReportSectionKind::SimulationResults)
        .expect("SimulationResults section should exist");

    assert_eq!(sim_section.status, ReportSectionStatus::Unavailable);
}

#[test]
fn included_sections_are_respected() {
    let service = make_service();
    let request = make_request(AdvancedReportType::ProjectSummary, vec![
        ReportSectionKind::ProjectInfo,
    ]);
    let context = empty_context();
    let report = service.generate_report(request, &context).unwrap();

    assert_eq!(report.sections.len(), 1);
    assert_eq!(report.sections[0].kind, ReportSectionKind::ProjectInfo);
}
