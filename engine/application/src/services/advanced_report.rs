use crate::ApplicationError;
use hotsas_core::advanced_report::{
    AdvancedReportContext, AdvancedReportModel, AdvancedReportRequest,
    ReportContentBlock, ReportKeyValueRow, ReportSection, ReportSectionCapability,
    ReportSectionKind, ReportSectionStatus, ReportSourceReference, ReportWarning,
    ReportWarningSeverity, default_section_capabilities,
};

#[derive(Clone)]
pub struct AdvancedReportService;

impl AdvancedReportService {
    pub fn new() -> Self {
        Self
    }

    pub fn list_section_capabilities(&self) -> Vec<ReportSectionCapability> {
        default_section_capabilities()
    }

    pub fn generate_report(
        &self,
        request: AdvancedReportRequest,
        context: &AdvancedReportContext,
    ) -> Result<AdvancedReportModel, ApplicationError> {
        let generated_at = Some(format_timestamp());

        let (project_id, project_name) = context
            .project
            .as_ref()
            .map(|p| (Some(p.id.clone()), Some(p.name.clone())))
            .unwrap_or((None, None));

        let mut sections: Vec<ReportSection> = Vec::new();
        let mut report_warnings: Vec<ReportWarning> = Vec::new();
        let mut assumptions: Vec<String> = Vec::new();
        let mut source_refs: Vec<ReportSourceReference> = Vec::new();

        if context.project.is_none() {
            report_warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Warning,
                code: "NO_PROJECT".to_string(),
                message: "No project loaded. Most sections will be unavailable.".to_string(),
                section_kind: None,
            });
            assumptions.push("Report generated without active project.".to_string());
        }

        for kind in &request.included_sections {
            let section = self.build_section(kind, context, &mut report_warnings, &mut source_refs);
            sections.push(section);
        }

        assumptions.push("Calculations use ideal first-order approximations.".to_string());
        assumptions.push("Component values may be nominal; tolerances not included.".to_string());

        Ok(AdvancedReportModel {
            id: request.report_id.clone(),
            title: request.title.clone(),
            report_type: request.report_type.clone(),
            generated_at,
            project_id,
            project_name,
            sections,
            warnings: report_warnings,
            assumptions,
            source_references: source_refs,
            metadata: request.metadata.clone(),
        })
    }

    pub fn render_report_markdown(&self, report: &AdvancedReportModel) -> Result<String, ApplicationError> {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", report.title));
        md.push_str(&format!(
            "**Report Type:** `{}` | **Generated:** {}\n\n",
            report.report_type,
            report.generated_at.as_deref().unwrap_or("unknown")
        ));

        if let Some(ref name) = report.project_name {
            md.push_str(&format!("**Project:** `{}`\n\n", name));
        }

        for section in &report.sections {
            md.push_str(&format!("## {}\n\n", section.title));
            md.push_str(&format!("**Status:** `{}`\n\n", section.status));

            for block in &section.blocks {
                Self::render_block_markdown(&mut md, block);
            }
        }

        if !report.warnings.is_empty() {
            md.push_str("## Report Warnings\n\n");
            for w in &report.warnings {
                md.push_str(&format!("- [{}] {:?}: {}\n", w.code, w.severity, w.message));
            }
            md.push('\n');
        }

        if !report.assumptions.is_empty() {
            md.push_str("## Assumptions\n\n");
            for a in &report.assumptions {
                md.push_str(&format!("- {}\n", a));
            }
            md.push('\n');
        }

        if !report.source_references.is_empty() {
            md.push_str("## Source References\n\n");
            for sr in &report.source_references {
                md.push_str(&format!("- `{}` ({}): {}\n", sr.source_id, sr.source_type, sr.description));
            }
            md.push('\n');
        }

        Ok(md)
    }

    fn render_block_markdown(md: &mut String, block: &ReportContentBlock) {
        match block {
            ReportContentBlock::Paragraph { text } => {
                md.push_str(&format!("{}\n\n", text));
            }
            ReportContentBlock::KeyValueTable { title, rows } => {
                md.push_str(&format!("### {}\n\n", title));
                for row in rows {
                    let unit_str = row.unit.as_deref().unwrap_or("");
                    if unit_str.is_empty() {
                        md.push_str(&format!("- **{}:** {}\n", row.key, row.value));
                    } else {
                        md.push_str(&format!("- **{}:** {} {}\n", row.key, row.value, unit_str));
                    }
                }
                md.push('\n');
            }
            ReportContentBlock::DataTable { title, columns, rows } => {
                md.push_str(&format!("### {}\n\n", title));
                md.push_str(&format!("| {} |\n", columns.join(" | ")));
                md.push_str(&format!("|{}|\n", columns.iter().map(|_| " --- ").collect::<String>()));
                for row in rows {
                    md.push_str(&format!("| {} |\n", row.join(" | ")));
                }
                md.push('\n');
            }
            ReportContentBlock::FormulaBlock { title, equation, substituted_values, result } => {
                md.push_str(&format!("### {}\n\n", title));
                md.push_str(&format!("Equation: `{}`\n\n", equation));
                if !substituted_values.is_empty() {
                    md.push_str("Substituted values:\n\n");
                    for row in substituted_values {
                        let unit_str = row.unit.as_deref().unwrap_or("");
                        if unit_str.is_empty() {
                            md.push_str(&format!("- **{}:** {}\n", row.key, row.value));
                        } else {
                            md.push_str(&format!("- **{}:** {} {}\n", row.key, row.value, unit_str));
                        }
                    }
                    md.push('\n');
                }
                if let Some(ref r) = result {
                    md.push_str(&format!("**Result:** {}\n\n", r));
                }
            }
            ReportContentBlock::CodeBlock { title, language, content } => {
                md.push_str(&format!("### {}\n\n", title));
                md.push_str(&format!("```{language}\n{content}\n```\n\n"));
            }
            ReportContentBlock::GraphReference { title, series_names, x_unit, y_unit } => {
                md.push_str(&format!("### {}\n\n", title));
                md.push_str(&format!("Series: `{}`\n\n", series_names.join(", ")));
                if let Some(ref x) = x_unit {
                    md.push_str(&format!("X-axis unit: `{}`\n\n", x));
                }
                if let Some(ref y) = y_unit {
                    md.push_str(&format!("Y-axis unit: `{}`\n\n", y));
                }
            }
            ReportContentBlock::WarningList { items } => {
                if !items.is_empty() {
                    md.push_str("**Warnings:**\n\n");
                    for w in items {
                        md.push_str(&format!("- [{}] {:?}: {}\n", w.code, w.severity, w.message));
                    }
                    md.push('\n');
                }
            }
        }
    }

    pub fn render_report_html(&self, report: &AdvancedReportModel) -> Result<String, ApplicationError> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>");
        html.push_str(&Self::escape_html(&report.title));
        html.push_str("</title></head><body style=\"font-family:sans-serif;max-width:800px;margin:2rem auto;\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", Self::escape_html(&report.title)));
        html.push_str(&format!("<p><strong>Report Type:</strong> {} | <strong>Generated:</strong> {}</p>\n",
            Self::escape_html(&report.report_type.to_string()),
            Self::escape_html(report.generated_at.as_deref().unwrap_or("unknown"))
        ));

        if let Some(ref name) = report.project_name {
            html.push_str(&format!("<p><strong>Project:</strong> {}</p>\n", Self::escape_html(name)));
        }

        for section in &report.sections {
            html.push_str(&format!("<h2>{}</h2>\n", Self::escape_html(&section.title)));
            html.push_str(&format!("<p><strong>Status:</strong> {}</p>\n", Self::escape_html(&section.status.to_string())));

            for block in &section.blocks {
                Self::render_block_html(&mut html, block);
            }
        }

        if !report.warnings.is_empty() {
            html.push_str("<h2>Report Warnings</h2>\n<ul>\n");
            for w in &report.warnings {
                html.push_str(&format!("<li>[{}] {:?}: {}</li>\n",
                    Self::escape_html(&w.code),
                    w.severity,
                    Self::escape_html(&w.message)
                ));
            }
            html.push_str("</ul>\n");
        }

        if !report.assumptions.is_empty() {
            html.push_str("<h2>Assumptions</h2>\n<ul>\n");
            for a in &report.assumptions {
                html.push_str(&format!("<li>{}</li>\n", Self::escape_html(a)));
            }
            html.push_str("</ul>\n");
        }

        if !report.source_references.is_empty() {
            html.push_str("<h2>Source References</h2>\n<ul>\n");
            for sr in &report.source_references {
                html.push_str(&format!("<li><code>{}</code> ({}): {}</li>\n",
                    Self::escape_html(&sr.source_id),
                    Self::escape_html(&sr.source_type),
                    Self::escape_html(&sr.description)
                ));
            }
            html.push_str("</ul>\n");
        }

        html.push_str("</body></html>");
        Ok(html)
    }

    fn render_block_html(html: &mut String, block: &ReportContentBlock) {
        match block {
            ReportContentBlock::Paragraph { text } => {
                html.push_str(&format!("<p>{}</p>\n", Self::escape_html(text)));
            }
            ReportContentBlock::KeyValueTable { title, rows } => {
                html.push_str(&format!("<h3>{}</h3>\n<table border=\"1\" cellpadding=\"4\" cellspacing=\"0\">\n", Self::escape_html(title)));
                for row in rows {
                    let unit_str = row.unit.as_deref().unwrap_or("");
                    let val = if unit_str.is_empty() { row.value.clone() } else { format!("{} {}", row.value, unit_str) };
                    html.push_str(&format!("<tr><th>{}</th><td>{}</td></tr>\n", Self::escape_html(&row.key), Self::escape_html(&val)));
                }
                html.push_str("</table>\n");
            }
            ReportContentBlock::DataTable { title, columns, rows } => {
                html.push_str(&format!("<h3>{}</h3>\n<table border=\"1\" cellpadding=\"4\" cellspacing=\"0\">\n<thead><tr>\n", Self::escape_html(title)));
                for col in columns {
                    html.push_str(&format!("<th>{}</th>", Self::escape_html(col)));
                }
                html.push_str("</tr></thead>\n<tbody>\n");
                for row in rows {
                    html.push_str("<tr>");
                    for cell in row {
                        html.push_str(&format!("<td>{}</td>", Self::escape_html(cell)));
                    }
                    html.push_str("</tr>\n");
                }
                html.push_str("</tbody></table>\n");
            }
            ReportContentBlock::FormulaBlock { title, equation, substituted_values, result } => {
                html.push_str(&format!("<h3>{}</h3>\n", Self::escape_html(title)));
                html.push_str(&format!("<p>Equation: <code>{}</code></p>\n", Self::escape_html(equation)));
                if !substituted_values.is_empty() {
                    html.push_str("<ul>\n");
                    for row in substituted_values {
                        let unit_str = row.unit.as_deref().unwrap_or("");
                        let val = if unit_str.is_empty() { row.value.clone() } else { format!("{} {}", row.value, unit_str) };
                        html.push_str(&format!("<li><strong>{}:</strong> {}</li>\n", Self::escape_html(&row.key), Self::escape_html(&val)));
                    }
                    html.push_str("</ul>\n");
                }
                if let Some(ref r) = result {
                    html.push_str(&format!("<p><strong>Result:</strong> {}</p>\n", Self::escape_html(r)));
                }
            }
            ReportContentBlock::CodeBlock { title, language, content } => {
                html.push_str(&format!("<h3>{}</h3>\n", Self::escape_html(title)));
                html.push_str(&format!("<pre><code class=\"language-{}\">{}</code></pre>\n",
                    Self::escape_html(language),
                    Self::escape_html(content)
                ));
            }
            ReportContentBlock::GraphReference { title, series_names, x_unit, y_unit } => {
                html.push_str(&format!("<h3>{}</h3>\n", Self::escape_html(title)));
                html.push_str(&format!("<p>Series: <code>{}</code></p>\n", Self::escape_html(&series_names.join(", "))));
                if let Some(ref x) = x_unit {
                    html.push_str(&format!("<p>X-axis unit: <code>{}</code></p>\n", Self::escape_html(x)));
                }
                if let Some(ref y) = y_unit {
                    html.push_str(&format!("<p>Y-axis unit: <code>{}</code></p>\n", Self::escape_html(y)));
                }
            }
            ReportContentBlock::WarningList { items } => {
                if !items.is_empty() {
                    html.push_str("<ul>\n");
                    for w in items {
                        html.push_str(&format!("<li>[{}] {:?}: {}</li>\n",
                            Self::escape_html(&w.code),
                            w.severity,
                            Self::escape_html(&w.message)
                        ));
                    }
                    html.push_str("</ul>\n");
                }
            }
        }
    }

    pub fn render_report_json(&self, report: &AdvancedReportModel) -> Result<String, ApplicationError> {
        serde_json::to_string_pretty(report)
            .map_err(|e| ApplicationError::Export(format!("JSON serialization failed: {e}")))
    }

    pub fn render_report_csv_summary(&self, report: &AdvancedReportModel) -> Result<String, ApplicationError> {
        let mut csv = String::new();
        csv.push_str("section_id,section_title,section_kind,status,warnings_count,blocks_count\n");
        for section in &report.sections {
            csv.push_str(&format!("{},{},{},{},{},{}\n",
                Self::escape_csv(&section.title),
                Self::escape_csv(&section.title),
                Self::escape_csv(&section.kind.to_string()),
                Self::escape_csv(&section.status.to_string()),
                section.warnings.len(),
                section.blocks.len()
            ));
        }
        Ok(csv)
    }

    fn escape_html(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn escape_csv(input: &str) -> String {
        if input.contains(',') || input.contains('"') || input.contains('\n') {
            format!("\"{}\"", input.replace('"', "\"\""))
        } else {
            input.to_string()
        }
    }

    fn build_section(
        &self,
        kind: &ReportSectionKind,
        context: &AdvancedReportContext,
        report_warnings: &mut Vec<ReportWarning>,
        source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        match kind {
            ReportSectionKind::ProjectInfo => self.build_project_info(context, report_warnings, source_refs),
            ReportSectionKind::SchematicSummary => self.build_schematic_summary(context, report_warnings, source_refs),
            ReportSectionKind::ComponentSummary => self.build_component_summary(context, report_warnings, source_refs),
            ReportSectionKind::FormulaCalculations => self.build_formula_calculations(context, report_warnings, source_refs),
            ReportSectionKind::NotebookCalculations => self.build_notebook_calculations(context, report_warnings, source_refs),
            ReportSectionKind::DcdcCalculations => self.build_dcdc_calculations(context, report_warnings, source_refs),
            ReportSectionKind::SelectedRegionAnalysis => self.build_selected_region(context, report_warnings, source_refs),
            ReportSectionKind::SimulationResults => self.build_simulation_results(context, report_warnings, source_refs),
            ReportSectionKind::SpiceNetlist => self.build_spice_netlist(context, report_warnings, source_refs),
            ReportSectionKind::ESeriesSelections => self.build_e_series(context, report_warnings, source_refs),
            ReportSectionKind::Bom => self.build_bom(context, report_warnings, source_refs),
            ReportSectionKind::ImportedModels => self.build_imported_models(context, report_warnings, source_refs),
            ReportSectionKind::ExportHistory => self.build_export_history(context, report_warnings, source_refs),
            ReportSectionKind::WarningsAndAssumptions => self.build_warnings_and_assumptions(context, report_warnings, source_refs),
        }
    }

    fn build_project_info(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref project) = context.project {
            blocks.push(ReportContentBlock::KeyValueTable {
                title: "Project Metadata".to_string(),
                rows: vec![
                    ReportKeyValueRow { key: "ID".to_string(), value: project.id.clone(), unit: None },
                    ReportKeyValueRow { key: "Name".to_string(), value: project.name.clone(), unit: None },
                    ReportKeyValueRow { key: "Format Version".to_string(), value: project.format_version.clone(), unit: None },
                    ReportKeyValueRow { key: "Engine Version".to_string(), value: project.engine_version.clone(), unit: None },
                    ReportKeyValueRow { key: "Project Type".to_string(), value: project.project_type.clone(), unit: None },
                ],
            });
            source_refs.push(ReportSourceReference {
                source_id: project.id.clone(),
                source_type: "CircuitProject".to_string(),
                description: "Current project".to_string(),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Warning,
                code: "PROJECT_UNAVAILABLE".to_string(),
                message: "No project loaded.".to_string(),
                section_kind: Some(ReportSectionKind::ProjectInfo),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No project information available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::ProjectInfo,
            title: "Project Info".to_string(),
            status: if context.project.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings: Vec::new(),
        }
    }

    fn build_schematic_summary(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref project) = context.project {
            let component_count = project.schematic.components.len();
            let net_count = project.schematic.nets.len();
            let wire_count = project.schematic.wires.len();
            blocks.push(ReportContentBlock::KeyValueTable {
                title: "Schematic Overview".to_string(),
                rows: vec![
                    ReportKeyValueRow { key: "Components".to_string(), value: component_count.to_string(), unit: None },
                    ReportKeyValueRow { key: "Nets".to_string(), value: net_count.to_string(), unit: None },
                    ReportKeyValueRow { key: "Wires".to_string(), value: wire_count.to_string(), unit: None },
                ],
            });
            source_refs.push(ReportSourceReference {
                source_id: project.schematic.id.clone(),
                source_type: "Schematic".to_string(),
                description: "Project schematic".to_string(),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Warning,
                code: "SCHEMATIC_UNAVAILABLE".to_string(),
                message: "No schematic data available.".to_string(),
                section_kind: Some(ReportSectionKind::SchematicSummary),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No schematic summary available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::SchematicSummary,
            title: "Schematic Summary".to_string(),
            status: if context.project.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_component_summary(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref project) = context.project {
            let mut rows: Vec<Vec<String>> = Vec::new();
            for component in &project.schematic.components {
                let params = component.overridden_parameters.iter()
                    .map(|(k, v)| format!("{}={}", k, v.original()))
                    .collect::<Vec<_>>()
                    .join(", ");
                rows.push(vec![
                    component.instance_id.clone(),
                    component.definition_id.clone(),
                    params,
                ]);
            }
            blocks.push(ReportContentBlock::DataTable {
                title: "Components".to_string(),
                columns: vec!["Instance".to_string(), "Definition".to_string(), "Parameters".to_string()],
                rows,
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Warning,
                code: "COMPONENTS_UNAVAILABLE".to_string(),
                message: "No component data available.".to_string(),
                section_kind: Some(ReportSectionKind::ComponentSummary),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No component summary available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::ComponentSummary,
            title: "Component Summary".to_string(),
            status: if context.project.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_formula_calculations(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref project) = context.project {
            blocks.push(ReportContentBlock::Paragraph {
                text: format!("Formula calculations for project `{}`.", project.name),
            });
            blocks.push(ReportContentBlock::FormulaBlock {
                title: "RC Low-Pass Cutoff".to_string(),
                equation: "fc = 1 / (2*pi*R*C)".to_string(),
                substituted_values: vec![],
                result: None,
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Warning,
                code: "FORMULAS_UNAVAILABLE".to_string(),
                message: "No project loaded for formula calculations.".to_string(),
                section_kind: Some(ReportSectionKind::FormulaCalculations),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No formula calculations available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::FormulaCalculations,
            title: "Formula Calculations".to_string(),
            status: if context.project.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_notebook_calculations(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref notebook) = context.notebook {
            blocks.push(ReportContentBlock::Paragraph {
                text: format!("Notebook `{}` has {} history entries.", notebook.id, notebook.history.len()),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "NOTEBOOK_UNAVAILABLE".to_string(),
                message: "No notebook data available.".to_string(),
                section_kind: Some(ReportSectionKind::NotebookCalculations),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No notebook calculations available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::NotebookCalculations,
            title: "Notebook Calculations".to_string(),
            status: if context.notebook.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_dcdc_calculations(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref dcdc) = context.dcdc_result {
            let mut rows: Vec<ReportKeyValueRow> = vec![
                ReportKeyValueRow { key: "Topology".to_string(), value: dcdc.topology.title().to_string(), unit: None },
                ReportKeyValueRow { key: "Operating Mode".to_string(), value: dcdc.operating_mode.to_string(), unit: None },
            ];
            for value in &dcdc.values {
                rows.push(ReportKeyValueRow {
                    key: value.label.clone(),
                    value: format!("{:.6}", value.value.si_value()),
                    unit: Some(value.value.unit.symbol().to_string()),
                });
            }
            blocks.push(ReportContentBlock::KeyValueTable {
                title: "DC-DC Design Parameters".to_string(),
                rows,
            });
            if !dcdc.warnings.is_empty() {
                let items: Vec<ReportWarning> = dcdc.warnings.iter().map(|w| ReportWarning {
                    severity: match w.severity {
                        hotsas_core::DcdcWarningSeverity::Info => ReportWarningSeverity::Info,
                        hotsas_core::DcdcWarningSeverity::Warning => ReportWarningSeverity::Warning,
                        hotsas_core::DcdcWarningSeverity::Error => ReportWarningSeverity::Error,
                    },
                    code: w.code.clone(),
                    message: w.message.clone(),
                    section_kind: Some(ReportSectionKind::DcdcCalculations),
                }).collect();
                blocks.push(ReportContentBlock::WarningList { items });
            }
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "DCDC_UNAVAILABLE".to_string(),
                message: "No DC-DC calculation data available.".to_string(),
                section_kind: Some(ReportSectionKind::DcdcCalculations),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No DC-DC calculations available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::DcdcCalculations,
            title: "DC-DC Calculations".to_string(),
            status: if context.dcdc_result.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_selected_region(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref region) = context.selected_region_result {
            blocks.push(ReportContentBlock::Paragraph {
                text: format!("Selected region analysis status: {:?}.", region.status),
            });
            blocks.push(ReportContentBlock::Paragraph {
                text: region.summary.clone(),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "REGION_UNAVAILABLE".to_string(),
                message: "No selected region analysis available.".to_string(),
                section_kind: Some(ReportSectionKind::SelectedRegionAnalysis),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No selected region analysis available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::SelectedRegionAnalysis,
            title: "Selected Region Analysis".to_string(),
            status: if context.selected_region_result.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_simulation_results(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref sim) = context.simulation_result {
            blocks.push(ReportContentBlock::KeyValueTable {
                title: "Simulation Overview".to_string(),
                rows: vec![
                    ReportKeyValueRow { key: "Status".to_string(), value: format!("{:?}", sim.status), unit: None },
                    ReportKeyValueRow { key: "Engine".to_string(), value: sim.engine.clone(), unit: None },
                    ReportKeyValueRow { key: "Series Count".to_string(), value: sim.graph_series.len().to_string(), unit: None },
                ],
            });
            let series_names: Vec<String> = sim.graph_series.iter().map(|s| s.name.clone()).collect();
            if !series_names.is_empty() {
                blocks.push(ReportContentBlock::GraphReference {
                    title: "Simulation Graphs".to_string(),
                    series_names,
                    x_unit: sim.graph_series.first().map(|s| s.x_unit.symbol().to_string()),
                    y_unit: sim.graph_series.first().map(|s| s.y_unit.symbol().to_string()),
                });
            }
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "SIMULATION_UNAVAILABLE".to_string(),
                message: "No simulation results available.".to_string(),
                section_kind: Some(ReportSectionKind::SimulationResults),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No simulation results available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::SimulationResults,
            title: "Simulation Results".to_string(),
            status: if context.simulation_result.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_spice_netlist(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref netlist) = context.netlist {
            blocks.push(ReportContentBlock::CodeBlock {
                title: "SPICE Netlist".to_string(),
                language: "spice".to_string(),
                content: netlist.clone(),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "NETLIST_UNAVAILABLE".to_string(),
                message: "No SPICE netlist available.".to_string(),
                section_kind: Some(ReportSectionKind::SpiceNetlist),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No SPICE netlist available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::SpiceNetlist,
            title: "SPICE Netlist".to_string(),
            status: if context.netlist.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_e_series(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref project) = context.project {
            blocks.push(ReportContentBlock::Paragraph {
                text: format!("E-Series selections for project `{}`.", project.name),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "ESERIES_UNAVAILABLE".to_string(),
                message: "No project loaded for E-Series selections.".to_string(),
                section_kind: Some(ReportSectionKind::ESeriesSelections),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No E-Series selections available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::ESeriesSelections,
            title: "E-Series Selections".to_string(),
            status: if context.project.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_bom(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if let Some(ref project) = context.project {
            let mut rows: Vec<Vec<String>> = Vec::new();
            for component in &project.schematic.components {
                let value_str = component.overridden_parameters.iter()
                    .map(|(k, v)| format!("{}={}", k, v.original()))
                    .collect::<Vec<_>>()
                    .join(", ");
                rows.push(vec![
                    component.instance_id.clone(),
                    component.definition_id.clone(),
                    value_str,
                    "1".to_string(),
                ]);
            }
            blocks.push(ReportContentBlock::DataTable {
                title: "Bill of Materials".to_string(),
                columns: vec!["Designator".to_string(), "Part".to_string(), "Value".to_string(), "Qty".to_string()],
                rows,
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "BOM_UNAVAILABLE".to_string(),
                message: "No project loaded for BOM generation.".to_string(),
                section_kind: Some(ReportSectionKind::Bom),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No BOM available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::Bom,
            title: "Bill of Materials".to_string(),
            status: if context.project.is_some() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_imported_models(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if !context.imported_models_summary.is_empty() {
            blocks.push(ReportContentBlock::DataTable {
                title: "Imported Models".to_string(),
                columns: vec!["Model".to_string()],
                rows: context.imported_models_summary.iter().map(|m| vec![m.clone()]).collect(),
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "IMPORTS_UNAVAILABLE".to_string(),
                message: "No imported models available.".to_string(),
                section_kind: Some(ReportSectionKind::ImportedModels),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No imported models available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::ImportedModels,
            title: "Imported Models".to_string(),
            status: if !context.imported_models_summary.is_empty() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_export_history(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut warnings: Vec<ReportWarning> = Vec::new();

        if !context.export_history.is_empty() {
            let mut rows: Vec<Vec<String>> = Vec::new();
            for entry in &context.export_history {
                rows.push(vec![
                    entry.timestamp.clone(),
                    entry.format.clone(),
                    entry.file_path.clone().unwrap_or_else(|| "N/A".to_string()),
                    if entry.success { "Success".to_string() } else { "Failed".to_string() },
                ]);
            }
            blocks.push(ReportContentBlock::DataTable {
                title: "Export History".to_string(),
                columns: vec!["Timestamp".to_string(), "Format".to_string(), "File".to_string(), "Status".to_string()],
                rows,
            });
        } else {
            warnings.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "HISTORY_UNAVAILABLE".to_string(),
                message: "No export history available.".to_string(),
                section_kind: Some(ReportSectionKind::ExportHistory),
            });
            blocks.push(ReportContentBlock::Paragraph { text: "No export history available.".to_string() });
        }

        ReportSection {
            kind: ReportSectionKind::ExportHistory,
            title: "Export History".to_string(),
            status: if !context.export_history.is_empty() { ReportSectionStatus::Included } else { ReportSectionStatus::Unavailable },
            blocks,
            warnings,
        }
    }

    fn build_warnings_and_assumptions(
        &self,
        context: &AdvancedReportContext,
        _report_warnings: &mut Vec<ReportWarning>,
        _source_refs: &mut Vec<ReportSourceReference>,
    ) -> ReportSection {
        let mut blocks: Vec<ReportContentBlock> = Vec::new();
        let mut items: Vec<ReportWarning> = Vec::new();
        if context.project.is_none() {
            items.push(ReportWarning {
                severity: ReportWarningSeverity::Warning,
                code: "NO_PROJECT".to_string(),
                message: "No project is currently loaded.".to_string(),
                section_kind: None,
            });
        }
        if context.simulation_result.is_none() {
            items.push(ReportWarning {
                severity: ReportWarningSeverity::Info,
                code: "NO_SIMULATION".to_string(),
                message: "No simulation results in context.".to_string(),
                section_kind: None,
            });
        }
        items.push(ReportWarning {
            severity: ReportWarningSeverity::Info,
            code: "IDEAL_MODELS".to_string(),
            message: "All calculations use ideal component models.".to_string(),
            section_kind: None,
        });
        blocks.push(ReportContentBlock::WarningList { items });

        ReportSection {
            kind: ReportSectionKind::WarningsAndAssumptions,
            title: "Warnings and Assumptions".to_string(),
            status: ReportSectionStatus::Included,
            blocks,
            warnings: Vec::new(),
        }
    }
}

fn format_timestamp() -> String {
    use std::time::SystemTime;
    let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    let seconds_of_day = secs % 86400;
    let hours = seconds_of_day / 3600;
    let minutes = (seconds_of_day % 3600) / 60;
    let seconds = seconds_of_day % 60;
    format!("{} UTC ({}h:{}m:{}s since midnight)", secs, hours, minutes, seconds)
}
