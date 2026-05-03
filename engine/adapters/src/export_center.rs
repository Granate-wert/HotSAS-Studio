use hotsas_core::{
    CircuitProject, CircuitQueryService, ComponentLibrary, GraphSeries, SimulationResult,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, PortError, SchematicExporterPort,
    SimulationDataExporterPort,
};
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct BomCsvExporter;

impl BomExporterPort for BomCsvExporter {
    fn export_bom_csv(&self, project: &CircuitProject) -> Result<String, PortError> {
        let mut csv = "Designator,Quantity,Value,Unit,Description\n".to_string();
        for component in &project.schematic.components {
            let designator = &component.instance_id;
            let (value_str, unit_str) = if let Ok(value) =
                CircuitQueryService::require_component_parameter(project, designator, "resistance")
            {
                (format_si(value.si_value()), value.unit.symbol().to_string())
            } else if let Ok(value) =
                CircuitQueryService::require_component_parameter(project, designator, "capacitance")
            {
                (format_si(value.si_value()), value.unit.symbol().to_string())
            } else {
                ("-".to_string(), "-".to_string())
            };
            let description = format!("{} component", component.definition_id);
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                designator, 1, value_str, unit_str, description
            ));
        }
        Ok(csv)
    }

    fn export_bom_json(&self, project: &CircuitProject) -> Result<String, PortError> {
        let mut lines = Vec::new();
        for component in &project.schematic.components {
            let designator = &component.instance_id;
            let (value_str, unit_str) = if let Ok(value) =
                CircuitQueryService::require_component_parameter(project, designator, "resistance")
            {
                (format_si(value.si_value()), value.unit.symbol().to_string())
            } else if let Ok(value) =
                CircuitQueryService::require_component_parameter(project, designator, "capacitance")
            {
                (format_si(value.si_value()), value.unit.symbol().to_string())
            } else {
                ("-".to_string(), "-".to_string())
            };
            let mut line = BTreeMap::new();
            line.insert("designator".to_string(), designator.clone());
            line.insert("quantity".to_string(), "1".to_string());
            line.insert("value".to_string(), value_str);
            line.insert("unit".to_string(), unit_str);
            line.insert(
                "description".to_string(),
                format!("{} component", component.definition_id),
            );
            lines.push(line);
        }
        serde_json::to_string_pretty(&lines).map_err(|e| PortError::Export(e.to_string()))
    }
}

#[derive(Debug, Default)]
pub struct CsvSimulationDataExporter;

impl SimulationDataExporterPort for CsvSimulationDataExporter {
    fn export_simulation_csv(&self, simulation: &SimulationResult) -> Result<String, PortError> {
        if simulation.graph_series.is_empty() {
            return Ok("No simulation data available.\n".to_string());
        }
        let x_series = &simulation.graph_series[0];
        let mut headers = vec!["frequency".to_string()];
        let mut series_data: Vec<&GraphSeries> = Vec::new();
        for series in &simulation.graph_series {
            headers.push(series.name.clone());
            series_data.push(series);
        }
        let mut csv = headers.join(",") + "\n";
        let point_count = x_series.points.len();
        for i in 0..point_count {
            let mut row = vec![format!("{:.9}", x_series.points[i].x)];
            for series in &series_data {
                let y = series.points.get(i).map(|p| p.y).unwrap_or(f64::NAN);
                row.push(format!("{:.9}", y));
            }
            csv.push_str(&(row.join(",") + "\n"));
        }
        Ok(csv)
    }
}

#[derive(Debug, Default)]
pub struct ComponentLibraryJsonExporter;

impl ComponentLibraryExporterPort for ComponentLibraryJsonExporter {
    fn export_component_library_json(
        &self,
        library: &ComponentLibrary,
    ) -> Result<String, PortError> {
        serde_json::to_string_pretty(library).map_err(|e| PortError::Export(e.to_string()))
    }
}

#[derive(Debug, Default)]
pub struct SvgSchematicExporter;

impl SchematicExporterPort for SvgSchematicExporter {
    fn export_svg_schematic(&self, project: &CircuitProject) -> Result<String, PortError> {
        let mut svg = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"600\" viewBox=\"0 0 800 600\">\n\
  <rect width=\"800\" height=\"600\" fill=\"#1a1a1a\"/>\n\
  <text x=\"400\" y=\"40\" text-anchor=\"middle\" fill=\"#e0e0e0\" font-size=\"20\" font-family=\"sans-serif\">HotSAS Studio — Schematic Placeholder</text>\n"
            .to_string();
        let mut y = 80.0;
        for component in &project.schematic.components {
            svg.push_str(&format!(
                "  <rect x=\"100\" y=\"{y:.0}\" width=\"120\" height=\"40\" fill=\"#2a2a2a\" stroke=\"#888\" stroke-width=\"1\"/>\n\
  <text x=\"110\" y=\"{text_y:.0}\" fill=\"#e0e0e0\" font-size=\"14\" font-family=\"sans-serif\">{}</text>\n",
                component.instance_id,
                y = y,
                text_y = y + 25.0
            ));
            y += 60.0;
        }
        svg.push_str("</svg>\n");
        Ok(svg)
    }
}

#[derive(Debug, Default)]
pub struct AltiumWorkflowPackageExporter;

impl AltiumWorkflowPackageExporter {
    pub fn export_placeholder_description(
        &self,
        project: &CircuitProject,
    ) -> Result<String, PortError> {
        let content = format!(
            r#"# Altium Designer Workflow Package — Placeholder

Project: {}
Generated by: HotSAS Studio v1.7

## Contents (Placeholder)

- `README.md` — This file
- `Schematic.SchDoc` — Placeholder (not a real Altium file)
- `PCB.PcbDoc` — Placeholder (not a real Altium file)
- `BOM.xlsx` — Placeholder BOM

## Usage

This package is a workflow placeholder. In a future version, it may contain
real Altium Designer primitives or an output job script.
"#,
            project.name
        );
        Ok(content)
    }
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
