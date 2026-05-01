use hotsas_core::{
    CircuitProject, CircuitQueryService, EngineeringUnit, GraphPoint, GraphSeries, ReportModel,
    SimulationProfile, SimulationResult, SimulationStatus, ValueWithUnit,
};
use hotsas_ports::{
    FormulaEnginePort, NetlistExporterPort, PortError, ReportExporterPort, SimulationEnginePort,
    StoragePort,
};
use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct JsonProjectStorage;

impl StoragePort for JsonProjectStorage {
    fn save_project(&self, path: &Path, project: &CircuitProject) -> Result<(), PortError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| PortError::Storage(error.to_string()))?;
        }
        let json = serde_json::to_string_pretty(project)
            .map_err(|error| PortError::Storage(error.to_string()))?;
        fs::write(path, json).map_err(|error| PortError::Storage(error.to_string()))
    }

    fn load_project(&self, path: &Path) -> Result<CircuitProject, PortError> {
        let json =
            fs::read_to_string(path).map_err(|error| PortError::Storage(error.to_string()))?;
        serde_json::from_str(&json).map_err(|error| PortError::Storage(error.to_string()))
    }
}

#[derive(Debug, Default)]
pub struct SimpleFormulaEngine;

impl FormulaEnginePort for SimpleFormulaEngine {
    fn calculate_rc_low_pass_cutoff(
        &self,
        resistance: &ValueWithUnit,
        capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError> {
        if resistance.unit != EngineeringUnit::Ohm {
            return Err(PortError::Formula("R must be expressed in Ohm".to_string()));
        }
        if capacitance.unit != EngineeringUnit::Farad {
            return Err(PortError::Formula("C must be expressed in F".to_string()));
        }
        if resistance.si_value() <= 0.0 || capacitance.si_value() <= 0.0 {
            return Err(PortError::Formula("R and C must be positive".to_string()));
        }

        let cutoff = 1.0 / (2.0 * PI * resistance.si_value() * capacitance.si_value());
        Ok(ValueWithUnit::new_si(cutoff, EngineeringUnit::Hertz))
    }
}

#[derive(Debug, Default)]
pub struct SpiceNetlistExporter;

impl NetlistExporterPort for SpiceNetlistExporter {
    fn export_spice_netlist(&self, project: &CircuitProject) -> Result<String, PortError> {
        let resistance = require_component_parameter(project, "R1", "resistance")?;
        let capacitance = require_component_parameter(project, "C1", "capacitance")?;

        Ok(format!(
            "* HotSAS Studio - RC Low-Pass Demo\n* Source of truth: CircuitModel\nV1 net_in 0 AC 1\nR1 net_in net_out {}\nC1 net_out 0 {}\n.ac dec 100 10 1e6\n.end",
            format_si(resistance.si_value()),
            format_si(capacitance.si_value())
        ))
    }
}

#[derive(Debug, Default)]
pub struct MockSimulationEngine;

impl SimulationEnginePort for MockSimulationEngine {
    fn run_ac_sweep(
        &self,
        project: &CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        let resistance = require_component_parameter(project, "R1", "resistance")?;
        let capacitance = require_component_parameter(project, "C1", "capacitance")?;
        let cutoff = 1.0 / (2.0 * PI * resistance.si_value() * capacitance.si_value());
        let start = profile
            .parameters
            .get("start")
            .map(ValueWithUnit::si_value)
            .unwrap_or(10.0);
        let stop = profile
            .parameters
            .get("stop")
            .map(ValueWithUnit::si_value)
            .unwrap_or(1_000_000.0);
        let mut gain_points = Vec::new();
        let mut phase_points = Vec::new();

        for index in 0..80 {
            let t = index as f64 / 79.0;
            let frequency = 10_f64.powf(start.log10() + t * (stop.log10() - start.log10()));
            let normalized = frequency / cutoff;
            let gain = 1.0 / (1.0 + normalized.powi(2)).sqrt();
            let gain_db = 20.0 * gain.log10();
            let phase_deg = -normalized.atan().to_degrees();
            gain_points.push(GraphPoint {
                x: frequency,
                y: gain_db,
            });
            phase_points.push(GraphPoint {
                x: frequency,
                y: phase_deg,
            });
        }

        let mut measurements = BTreeMap::new();
        measurements.insert(
            "fc".to_string(),
            ValueWithUnit::new_si(cutoff, EngineeringUnit::Hertz),
        );

        Ok(SimulationResult {
            id: "mock-ac-rc-low-pass".to_string(),
            profile_id: profile.id.clone(),
            status: SimulationStatus::Completed,
            graph_series: vec![
                GraphSeries {
                    name: "Gain".to_string(),
                    x_unit: EngineeringUnit::Hertz,
                    y_unit: EngineeringUnit::Unitless,
                    points: gain_points,
                    metadata: BTreeMap::from([("quantity".to_string(), "dB".to_string())]),
                },
                GraphSeries {
                    name: "Phase".to_string(),
                    x_unit: EngineeringUnit::Hertz,
                    y_unit: EngineeringUnit::Unitless,
                    points: phase_points,
                    metadata: BTreeMap::from([("quantity".to_string(), "degrees".to_string())]),
                },
            ],
            measurements,
            warnings: vec![
                "Mock simulation result; ngspice integration is a later adapter.".to_string(),
            ],
            errors: vec![],
            raw_data_path: None,
        })
    }
}

#[derive(Debug, Default)]
pub struct MarkdownReportExporter;

impl ReportExporterPort for MarkdownReportExporter {
    fn export_markdown(&self, report: &ReportModel) -> Result<String, PortError> {
        let mut markdown = format!("# {}\n\n", report.title);
        for section in &report.sections {
            markdown.push_str(&format!(
                "## {}\n\n{}\n\n",
                section.title, section.body_markdown
            ));
        }
        if !report.included_bom.is_empty() {
            markdown.push_str(
                "## BOM\n\n| Designator | Quantity | Value | Description |\n|---|---:|---|---|\n",
            );
            for line in &report.included_bom {
                let value = line
                    .value
                    .as_ref()
                    .map(|value| format!("{:.6} {}", value.si_value(), value.unit.symbol()))
                    .unwrap_or_else(|| "-".to_string());
                markdown.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    line.designator, line.quantity, value, line.description
                ));
            }
            markdown.push('\n');
        }
        Ok(markdown)
    }

    fn export_html(&self, report: &ReportModel) -> Result<String, PortError> {
        let markdown = self.export_markdown(report)?;
        let escaped = markdown
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        Ok(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body><pre>{}</pre></body></html>",
            report.title, escaped
        ))
    }
}

#[derive(Debug, Default)]
pub struct PdfReportExporterPlaceholder;

fn require_component_parameter(
    project: &CircuitProject,
    component_id: &str,
    parameter: &str,
) -> Result<ValueWithUnit, PortError> {
    CircuitQueryService::require_component_parameter(project, component_id, parameter)
        .map_err(|error| PortError::Export(error.to_string()))
}

fn format_si(value: f64) -> String {
    if value.abs() >= 1e4 || value.abs() < 1e-3 {
        format!("{value:.9e}")
    } else {
        format!("{value:.9}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hotsas_core::{rc_low_pass_project, EngineeringUnit};

    #[test]
    fn calculates_rc_low_pass_formula() {
        let engine = SimpleFormulaEngine;
        let r = ValueWithUnit::parse_with_default("10k", EngineeringUnit::Ohm).unwrap();
        let c = ValueWithUnit::parse_with_default("100n", EngineeringUnit::Farad).unwrap();

        let result = engine.calculate_rc_low_pass_cutoff(&r, &c).unwrap();

        assert_eq!(result.unit, EngineeringUnit::Hertz);
        assert!((result.si_value() - 159.154943).abs() < 0.001);
    }

    #[test]
    fn exports_rc_low_pass_spice_netlist() {
        let exporter = SpiceNetlistExporter;
        let project = rc_low_pass_project();

        let netlist = exporter.export_spice_netlist(&project).unwrap();

        assert!(netlist.contains("V1 net_in 0 AC 1"));
        assert!(netlist.contains("R1 net_in net_out"));
        assert!(netlist.contains("C1 net_out 0"));
        assert!(netlist.contains(".ac dec 100 10 1e6"));
    }

    #[test]
    fn exports_markdown_report() {
        let exporter = MarkdownReportExporter;
        let report = ReportModel {
            id: "report".to_string(),
            title: "RC Report".to_string(),
            sections: vec![hotsas_core::ReportSection {
                title: "Formula".to_string(),
                body_markdown: "fc calculation".to_string(),
            }],
            included_schematic_images: vec![],
            included_formulas: vec![],
            included_simulation_results: vec![],
            included_bom: vec![],
            export_settings: BTreeMap::new(),
        };

        let markdown = exporter.export_markdown(&report).unwrap();

        assert!(markdown.starts_with("# RC Report"));
        assert!(markdown.contains("## Formula"));
    }
}
