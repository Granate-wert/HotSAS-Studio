use hotsas_core::advanced_report::{
    default_section_capabilities, AdvancedReportModel, AdvancedReportRequest, AdvancedReportType,
    ReportContentBlock, ReportExportOptions, ReportKeyValueRow, ReportSection,
    ReportSectionCapability, ReportSectionKind, ReportSectionStatus, ReportSourceReference,
    ReportWarning, ReportWarningSeverity,
};

#[test]
fn default_section_capabilities_contains_all_kinds() {
    let caps = default_section_capabilities();
    let kinds: Vec<_> = caps.iter().map(|c| c.kind.clone()).collect();

    assert!(kinds.contains(&ReportSectionKind::ProjectInfo));
    assert!(kinds.contains(&ReportSectionKind::SchematicSummary));
    assert!(kinds.contains(&ReportSectionKind::ComponentSummary));
    assert!(kinds.contains(&ReportSectionKind::FormulaCalculations));
    assert!(kinds.contains(&ReportSectionKind::NotebookCalculations));
    assert!(kinds.contains(&ReportSectionKind::DcdcCalculations));
    assert!(kinds.contains(&ReportSectionKind::SelectedRegionAnalysis));
    assert!(kinds.contains(&ReportSectionKind::SimulationResults));
    assert!(kinds.contains(&ReportSectionKind::SpiceNetlist));
    assert!(kinds.contains(&ReportSectionKind::ESeriesSelections));
    assert!(kinds.contains(&ReportSectionKind::Bom));
    assert!(kinds.contains(&ReportSectionKind::ImportedModels));
    assert!(kinds.contains(&ReportSectionKind::ModelPersistence));
    assert!(kinds.contains(&ReportSectionKind::ExportHistory));
    assert!(kinds.contains(&ReportSectionKind::WarningsAndAssumptions));
}

#[test]
fn report_section_status_display() {
    assert_eq!(format!("{}", ReportSectionStatus::Included), "Included");
    assert_eq!(format!("{}", ReportSectionStatus::Empty), "Empty");
    assert_eq!(
        format!("{}", ReportSectionStatus::Unavailable),
        "Unavailable"
    );
    assert_eq!(format!("{}", ReportSectionStatus::Error), "Error");
}

#[test]
fn report_section_kind_display() {
    assert_eq!(format!("{}", ReportSectionKind::ProjectInfo), "ProjectInfo");
    assert_eq!(format!("{}", ReportSectionKind::Bom), "Bom");
    assert_eq!(
        format!("{}", ReportSectionKind::WarningsAndAssumptions),
        "WarningsAndAssumptions"
    );
}

#[test]
fn advanced_report_type_display() {
    assert_eq!(
        format!("{}", AdvancedReportType::ProjectSummary),
        "ProjectSummary"
    );
    assert_eq!(
        format!("{}", AdvancedReportType::FullProjectReport),
        "FullProjectReport"
    );
}

#[test]
fn report_section_can_be_constructed() {
    let section = ReportSection {
        kind: ReportSectionKind::ProjectInfo,
        title: "Project Information".to_string(),
        status: ReportSectionStatus::Included,
        blocks: vec![ReportContentBlock::Paragraph {
            text: "Test paragraph".to_string(),
        }],
        warnings: vec![],
    };

    assert_eq!(section.kind, ReportSectionKind::ProjectInfo);
    assert_eq!(section.title, "Project Information");
    assert_eq!(section.status, ReportSectionStatus::Included);
    assert_eq!(section.blocks.len(), 1);
    assert!(section.warnings.is_empty());
}

#[test]
fn report_content_block_variants() {
    let paragraph = ReportContentBlock::Paragraph {
        text: "Hello".to_string(),
    };
    let table = ReportContentBlock::KeyValueTable {
        title: "Params".to_string(),
        rows: vec![ReportKeyValueRow {
            key: "R1".to_string(),
            value: "10k".to_string(),
            unit: Some("Ohm".to_string()),
        }],
    };
    let formula = ReportContentBlock::FormulaBlock {
        title: "Calc".to_string(),
        equation: "f = 1/(2πRC)".to_string(),
        substituted_values: vec![],
        result: Some("159.15 Hz".to_string()),
    };
    let code = ReportContentBlock::CodeBlock {
        title: "Netlist".to_string(),
        language: "spice".to_string(),
        content: "V1 1 0 DC 5".to_string(),
    };
    let graph = ReportContentBlock::GraphReference {
        title: "AC Sweep".to_string(),
        series_names: vec!["V(out)".to_string()],
        x_unit: Some("Hz".to_string()),
        y_unit: Some("dB".to_string()),
    };
    let warnings = ReportContentBlock::WarningList {
        items: vec![ReportWarning {
            severity: ReportWarningSeverity::Warning,
            code: "W001".to_string(),
            message: "Check values".to_string(),
            section_kind: Some(ReportSectionKind::ProjectInfo),
        }],
    };

    // Just verify they construct without panic
    drop(paragraph);
    drop(table);
    drop(formula);
    drop(code);
    drop(graph);
    drop(warnings);
}

#[test]
fn advanced_report_model_construction() {
    let report = AdvancedReportModel {
        id: "rpt-1".to_string(),
        title: "Test Report".to_string(),
        report_type: AdvancedReportType::ProjectSummary,
        generated_at: Some("2026-05-05T00:00:00Z".to_string()),
        project_id: Some("proj-1".to_string()),
        project_name: Some("Demo".to_string()),
        sections: vec![],
        warnings: vec![],
        assumptions: vec!["Assumed 25°C".to_string()],
        source_references: vec![ReportSourceReference {
            source_id: "src-1".to_string(),
            source_type: "project".to_string(),
            description: "Main project".to_string(),
        }],
        metadata: Default::default(),
    };

    assert_eq!(report.id, "rpt-1");
    assert_eq!(report.assumptions.len(), 1);
    assert_eq!(report.source_references.len(), 1);
}

#[test]
fn report_capability_has_expected_fields() {
    let cap = ReportSectionCapability {
        kind: ReportSectionKind::ProjectInfo,
        title: "Project Information".to_string(),
        description: "Basic project metadata".to_string(),
        default_enabled: true,
        supported_report_types: vec![
            AdvancedReportType::ProjectSummary,
            AdvancedReportType::FullProjectReport,
        ],
    };

    assert_eq!(cap.kind, ReportSectionKind::ProjectInfo);
    assert!(cap.default_enabled);
    assert_eq!(cap.supported_report_types.len(), 2);
}

#[test]
fn report_warning_severity_variants_exist() {
    let _ = ReportWarningSeverity::Info;
    let _ = ReportWarningSeverity::Warning;
    let _ = ReportWarningSeverity::Error;
}

#[test]
fn report_export_options_default() {
    let opts = ReportExportOptions {
        include_source_references: true,
        include_graph_references: false,
        include_assumptions: true,
        max_table_rows: Some(100),
    };

    assert!(opts.include_source_references);
    assert!(!opts.include_graph_references);
    assert!(opts.include_assumptions);
    assert_eq!(opts.max_table_rows, Some(100));
}

#[test]
fn advanced_report_request_construction() {
    let request = AdvancedReportRequest {
        report_id: "req-1".to_string(),
        title: "My Report".to_string(),
        report_type: AdvancedReportType::CalculationReport,
        included_sections: vec![ReportSectionKind::FormulaCalculations],
        export_options: ReportExportOptions {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        },
        metadata: {
            let mut m = std::collections::BTreeMap::new();
            m.insert("author".to_string(), "test".to_string());
            m
        },
    };

    assert_eq!(request.report_id, "req-1");
    assert_eq!(request.report_type, AdvancedReportType::CalculationReport);
    assert_eq!(request.included_sections.len(), 1);
    assert_eq!(request.metadata.get("author"), Some(&"test".to_string()));
}
