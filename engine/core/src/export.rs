use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    MarkdownReport,
    HtmlReport,
    SpiceNetlist,
    CsvSimulationData,
    BomCsv,
    BomJson,
    ComponentLibraryJson,
    SvgSchematic,
    AltiumWorkflowPackage,
}

impl ExportFormat {
    pub fn id(&self) -> &'static str {
        match self {
            Self::MarkdownReport => "markdown_report",
            Self::HtmlReport => "html_report",
            Self::SpiceNetlist => "spice_netlist",
            Self::CsvSimulationData => "csv_simulation_data",
            Self::BomCsv => "bom_csv",
            Self::BomJson => "bom_json",
            Self::ComponentLibraryJson => "component_library_json",
            Self::SvgSchematic => "svg_schematic",
            Self::AltiumWorkflowPackage => "altium_workflow_package",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::MarkdownReport => "Markdown Report",
            Self::HtmlReport => "HTML Report",
            Self::SpiceNetlist => "SPICE Netlist",
            Self::CsvSimulationData => "CSV Simulation Data",
            Self::BomCsv => "BOM (CSV)",
            Self::BomJson => "BOM (JSON)",
            Self::ComponentLibraryJson => "Component Library (JSON)",
            Self::SvgSchematic => "SVG Schematic",
            Self::AltiumWorkflowPackage => "Altium Workflow Package",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::MarkdownReport => "Human-readable report in Markdown format.",
            Self::HtmlReport => "Self-contained HTML report with escaped content.",
            Self::SpiceNetlist => "SPICE-compatible netlist for circuit simulation.",
            Self::CsvSimulationData => "Simulation graph series exported as CSV.",
            Self::BomCsv => "Bill of Materials in CSV format.",
            Self::BomJson => "Bill of Materials in JSON format.",
            Self::ComponentLibraryJson => "Full component library as JSON.",
            Self::SvgSchematic => "Placeholder SVG schematic image.",
            Self::AltiumWorkflowPackage => "Placeholder Altium Designer workflow package.",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::MarkdownReport => "md",
            Self::HtmlReport => "html",
            Self::SpiceNetlist => "cir",
            Self::CsvSimulationData => "csv",
            Self::BomCsv => "csv",
            Self::BomJson => "json",
            Self::ComponentLibraryJson => "json",
            Self::SvgSchematic => "svg",
            Self::AltiumWorkflowPackage => "zip",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportCapability {
    pub format: String,
    pub label: String,
    pub description: String,
    pub file_extension: String,
    pub available: bool,
}

impl From<ExportFormat> for ExportCapability {
    fn from(format: ExportFormat) -> Self {
        Self {
            format: format.id().to_string(),
            label: format.label().to_string(),
            description: format.description().to_string(),
            file_extension: format.file_extension().to_string(),
            available: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportResult {
    pub format: String,
    pub content: String,
    pub file_path: Option<String>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportHistoryEntry {
    pub timestamp: String,
    pub format: String,
    pub file_path: Option<String>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportJobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportJob {
    pub id: String,
    pub format: String,
    pub status: ExportJobStatus,
    pub requested_at: String,
}

pub fn all_export_formats() -> Vec<ExportFormat> {
    vec![
        ExportFormat::MarkdownReport,
        ExportFormat::HtmlReport,
        ExportFormat::SpiceNetlist,
        ExportFormat::CsvSimulationData,
        ExportFormat::BomCsv,
        ExportFormat::BomJson,
        ExportFormat::ComponentLibraryJson,
        ExportFormat::SvgSchematic,
        ExportFormat::AltiumWorkflowPackage,
    ]
}

pub fn default_export_capabilities() -> Vec<ExportCapability> {
    all_export_formats()
        .into_iter()
        .map(ExportCapability::from)
        .collect()
}
