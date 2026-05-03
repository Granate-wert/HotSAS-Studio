use crate::ApplicationError;
use hotsas_core::{
    CircuitProject, ComponentLibrary, ExportCapability, ExportFormat, ExportHistoryEntry,
    ExportResult, ReportModel, SimulationResult,
};
use hotsas_ports::{
    BomExporterPort, ComponentLibraryExporterPort, ExportHistoryPort, NetlistExporterPort,
    ReportExporterPort, SchematicExporterPort, SimulationDataExporterPort,
};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExportCenterService {
    report_exporter: Arc<dyn ReportExporterPort>,
    netlist_exporter: Arc<dyn NetlistExporterPort>,
    bom_exporter: Arc<dyn BomExporterPort>,
    simulation_exporter: Arc<dyn SimulationDataExporterPort>,
    library_exporter: Arc<dyn ComponentLibraryExporterPort>,
    schematic_exporter: Arc<dyn SchematicExporterPort>,
    history_port: Option<Arc<dyn ExportHistoryPort>>,
}

impl ExportCenterService {
    pub fn new(
        report_exporter: Arc<dyn ReportExporterPort>,
        netlist_exporter: Arc<dyn NetlistExporterPort>,
        bom_exporter: Arc<dyn BomExporterPort>,
        simulation_exporter: Arc<dyn SimulationDataExporterPort>,
        library_exporter: Arc<dyn ComponentLibraryExporterPort>,
        schematic_exporter: Arc<dyn SchematicExporterPort>,
    ) -> Self {
        Self {
            report_exporter,
            netlist_exporter,
            bom_exporter,
            simulation_exporter,
            library_exporter,
            schematic_exporter,
            history_port: None,
        }
    }

    pub fn with_history_port(mut self, history_port: Arc<dyn ExportHistoryPort>) -> Self {
        self.history_port = Some(history_port);
        self
    }

    pub fn list_capabilities(&self) -> Vec<ExportCapability> {
        hotsas_core::default_export_capabilities()
    }

    pub fn export_to_string(
        &self,
        format: ExportFormat,
        project: &CircuitProject,
        report: Option<&ReportModel>,
        simulation: Option<&SimulationResult>,
        library: Option<&ComponentLibrary>,
    ) -> Result<ExportResult, ApplicationError> {
        let (content, message) = match format {
            ExportFormat::MarkdownReport => {
                let report = report.ok_or_else(|| {
                    ApplicationError::Port(hotsas_ports::PortError::Export(
                        "report required for markdown export".to_string(),
                    ))
                })?;
                let content = self.report_exporter.export_markdown(report)?;
                (content, "Markdown report generated.".to_string())
            }
            ExportFormat::HtmlReport => {
                let report = report.ok_or_else(|| {
                    ApplicationError::Port(hotsas_ports::PortError::Export(
                        "report required for HTML export".to_string(),
                    ))
                })?;
                let content = self.report_exporter.export_html(report)?;
                (content, "HTML report generated.".to_string())
            }
            ExportFormat::SpiceNetlist => {
                let content = self.netlist_exporter.export_spice_netlist(project)?;
                (content, "SPICE netlist generated.".to_string())
            }
            ExportFormat::CsvSimulationData => {
                let simulation = simulation.ok_or_else(|| {
                    ApplicationError::Port(hotsas_ports::PortError::Export(
                        "simulation required for CSV export".to_string(),
                    ))
                })?;
                let content = self.simulation_exporter.export_simulation_csv(simulation)?;
                (content, "Simulation CSV generated.".to_string())
            }
            ExportFormat::BomCsv => {
                let content = self.bom_exporter.export_bom_csv(project)?;
                (content, "BOM CSV generated.".to_string())
            }
            ExportFormat::BomJson => {
                let content = self.bom_exporter.export_bom_json(project)?;
                (content, "BOM JSON generated.".to_string())
            }
            ExportFormat::ComponentLibraryJson => {
                let library = library.ok_or_else(|| {
                    ApplicationError::Port(hotsas_ports::PortError::Export(
                        "component library required for JSON export".to_string(),
                    ))
                })?;
                let content = self
                    .library_exporter
                    .export_component_library_json(library)?;
                (content, "Component library JSON generated.".to_string())
            }
            ExportFormat::SvgSchematic => {
                let content = self.schematic_exporter.export_svg_schematic(project)?;
                (content, "SVG schematic generated.".to_string())
            }
            ExportFormat::AltiumWorkflowPackage => {
                let content = format!(
                    "# Altium Workflow Placeholder\n\nProject: {}\n",
                    project.name
                );
                (
                    content,
                    "Altium workflow placeholder generated.".to_string(),
                )
            }
        };
        Ok(ExportResult {
            format: format.id().to_string(),
            content,
            file_path: None,
            success: true,
            message,
        })
    }

    pub fn export_to_file(
        &self,
        format: ExportFormat,
        project: &CircuitProject,
        report: Option<&ReportModel>,
        simulation: Option<&SimulationResult>,
        library: Option<&ComponentLibrary>,
        output_dir: &Path,
    ) -> Result<ExportResult, ApplicationError> {
        let mut result = self.export_to_string(format, project, report, simulation, library)?;
        std::fs::create_dir_all(output_dir)
            .map_err(|e| ApplicationError::Port(hotsas_ports::PortError::Export(e.to_string())))?;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let file_name = format!(
            "hotsas_export_{}_{}.{}",
            format.id(),
            timestamp,
            format.file_extension()
        );
        let file_path = output_dir.join(&file_name);
        std::fs::write(&file_path, &result.content)
            .map_err(|e| ApplicationError::Port(hotsas_ports::PortError::Export(e.to_string())))?;
        result.file_path = Some(file_path.to_string_lossy().to_string());
        result.message = format!("Saved to {}", file_path.display());
        if let Some(history) = &self.history_port {
            let _ = history.record_export(&result);
        }
        Ok(result)
    }

    pub fn record_export(&self, result: &ExportResult) -> Result<(), ApplicationError> {
        if let Some(history) = &self.history_port {
            history
                .record_export(result)
                .map_err(ApplicationError::Port)?;
        }
        Ok(())
    }

    pub fn list_history(&self) -> Result<Vec<ExportHistoryEntry>, ApplicationError> {
        if let Some(history) = &self.history_port {
            history.list_history().map_err(ApplicationError::Port)
        } else {
            Ok(vec![])
        }
    }
}
