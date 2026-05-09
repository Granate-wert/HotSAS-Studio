use crate::ApplicationError;
use hotsas_core::{
    calculate_error_percent, rc_low_pass_formula, BomLine, CircuitProject, CircuitQueryService,
    PreferredValueResult, ReportModel, ReportSection, SimulationResult, ValueWithUnit,
};
use hotsas_ports::ReportExporterPort;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::ComponentModelMappingService;

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
                    title: "Model Mapping Readiness".to_string(),
                    body_markdown: build_model_mapping_markdown(project),
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

fn build_model_mapping_markdown(project: &CircuitProject) -> String {
    let readiness = ComponentModelMappingService::new()
        .evaluate_project_simulation_readiness(project, &hotsas_core::built_in_component_library());
    let mut body = String::new();
    body.push_str(&format!(
        "Can simulate: `{}`. Ready: `{}`, placeholder: `{}`, missing: `{}`, blocking: `{}`, warnings: `{}`.\n\n",
        readiness.can_simulate,
        readiness.ready_count,
        readiness.placeholder_count,
        readiness.missing_count,
        readiness.blocking_count,
        readiness.warning_count
    ));
    body.push_str(
        "| Component | Model status | Model source | Pin mapping | Readiness | Diagnostics |\n",
    );
    body.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for assignment in readiness.components {
        let component = assignment
            .component_instance_id
            .unwrap_or_else(|| assignment.component_definition_id.clone());
        let source = assignment
            .model_ref
            .as_ref()
            .map(|model| format!("{:?}", model.source))
            .unwrap_or_else(|| "None".to_string());
        let pin_mapping = if assignment.pin_mappings.is_empty() {
            "None".to_string()
        } else {
            assignment
                .pin_mappings
                .iter()
                .map(|mapping| format!("{}->{}", mapping.component_pin_id, mapping.model_pin_name))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let diagnostics = if assignment.diagnostics.is_empty() {
            "None".to_string()
        } else {
            assignment
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.clone())
                .collect::<Vec<_>>()
                .join(", ")
        };
        body.push_str(&format!(
            "| {} | `{:?}` | `{}` | {} | {} | {} |\n",
            component,
            assignment.status,
            source,
            pin_mapping,
            assignment.readiness.status_label,
            diagnostics
        ));
    }
    body
}
