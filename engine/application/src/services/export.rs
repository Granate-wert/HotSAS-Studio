use crate::ApplicationError;
use hotsas_core::{
    calculate_error_percent, rc_low_pass_formula, BomLine, CircuitProject, CircuitQueryService,
    PreferredValueResult, ReportModel, ReportSection, SimulationResult, ValueWithUnit,
};
use hotsas_ports::ReportExporterPort;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExportService {
    report_exporter: Arc<dyn ReportExporterPort>,
}

impl ExportService {
    pub fn new(report_exporter: Arc<dyn ReportExporterPort>) -> Self {
        Self { report_exporter }
    }

    pub fn build_report_model(
        &self,
        project: &CircuitProject,
        cutoff_frequency: &ValueWithUnit,
        nearest_e24: &PreferredValueResult,
        netlist: &str,
        simulation_result: &SimulationResult,
    ) -> ReportModel {
        let formula = rc_low_pass_formula();
        let resistance =
            CircuitQueryService::require_component_parameter(project, "R1", "resistance").ok();
        let capacitance =
            CircuitQueryService::require_component_parameter(project, "C1", "capacitance").ok();
        let mut export_settings = BTreeMap::new();
        export_settings.insert("primary_format".to_string(), "markdown".to_string());
        export_settings.insert("html_available".to_string(), "true".to_string());
        export_settings.insert("pdf_status".to_string(), "placeholder".to_string());

        ReportModel {
            id: "rc-low-pass-report".to_string(),
            title: format!("{} Report", project.name),
            sections: vec![
                ReportSection {
                    title: "Project".to_string(),
                    body_markdown: format!(
                        "Project `{}` uses format version `{}` and engine version `{}`.",
                        project.name, project.format_version, project.engine_version
                    ),
                },
                ReportSection {
                    title: "Formula".to_string(),
                    body_markdown: format!(
                        "`fc = 1 / (2*pi*R*C)` = `{:.6} Hz`.",
                        cutoff_frequency.si_value()
                    ),
                },
                ReportSection {
                    title: "Preferred Value".to_string(),
                    body_markdown: format!(
                        "Nearest {} value for `{:.6} {}` is `{:.6} {}` with error `{:.4}%`.",
                        nearest_e24.series.label(),
                        nearest_e24.requested_value.si_value(),
                        nearest_e24.requested_value.unit.symbol(),
                        nearest_e24.nearest.si_value(),
                        nearest_e24.nearest.unit.symbol(),
                        calculate_error_percent(
                            nearest_e24.requested_value.si_value(),
                            nearest_e24.nearest.si_value()
                        )
                    ),
                },
                ReportSection {
                    title: "SPICE Netlist".to_string(),
                    body_markdown: format!("```spice\n{netlist}\n```"),
                },
                ReportSection {
                    title: "Mock AC Simulation".to_string(),
                    body_markdown: format!(
                        "Simulation status: `{:?}`. Series count: `{}`.",
                        simulation_result.status,
                        simulation_result.graph_series.len()
                    ),
                },
            ],
            included_schematic_images: vec![],
            included_formulas: vec![formula],
            included_simulation_results: vec![simulation_result.clone()],
            included_bom: vec![
                BomLine {
                    designator: "R1".to_string(),
                    quantity: 1,
                    value: resistance,
                    description: "RC low-pass resistor".to_string(),
                },
                BomLine {
                    designator: "C1".to_string(),
                    quantity: 1,
                    value: capacitance,
                    description: "RC low-pass capacitor".to_string(),
                },
            ],
            export_settings,
        }
    }

    pub fn export_markdown_report(&self, report: &ReportModel) -> Result<String, ApplicationError> {
        Ok(self.report_exporter.export_markdown(report)?)
    }

    pub fn export_html_report(&self, report: &ReportModel) -> Result<String, ApplicationError> {
        Ok(self.report_exporter.export_html(report)?)
    }
}
