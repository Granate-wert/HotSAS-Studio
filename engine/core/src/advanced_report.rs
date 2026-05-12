use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdvancedReportType {
    ProjectSummary,
    CalculationReport,
    SimulationReport,
    SelectedRegionReport,
    DcdcDesignReport,
    FullProjectReport,
}

impl std::fmt::Display for AdvancedReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancedReportType::ProjectSummary => write!(f, "ProjectSummary"),
            AdvancedReportType::CalculationReport => write!(f, "CalculationReport"),
            AdvancedReportType::SimulationReport => write!(f, "SimulationReport"),
            AdvancedReportType::SelectedRegionReport => write!(f, "SelectedRegionReport"),
            AdvancedReportType::DcdcDesignReport => write!(f, "DcdcDesignReport"),
            AdvancedReportType::FullProjectReport => write!(f, "FullProjectReport"),
        }
    }
}

impl std::str::FromStr for AdvancedReportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProjectSummary" => Ok(AdvancedReportType::ProjectSummary),
            "CalculationReport" => Ok(AdvancedReportType::CalculationReport),
            "SimulationReport" => Ok(AdvancedReportType::SimulationReport),
            "SelectedRegionReport" => Ok(AdvancedReportType::SelectedRegionReport),
            "DcdcDesignReport" => Ok(AdvancedReportType::DcdcDesignReport),
            "FullProjectReport" => Ok(AdvancedReportType::FullProjectReport),
            other => Err(format!("unknown report type: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportSectionKind {
    ProjectInfo,
    SchematicSummary,
    ComponentSummary,
    FormulaCalculations,
    NotebookCalculations,
    DcdcCalculations,
    SelectedRegionAnalysis,
    SimulationResults,
    SpiceNetlist,
    ESeriesSelections,
    Bom,
    ImportedModels,
    ModelMappingReadiness,
    ModelPersistence,
    ExportHistory,
    WarningsAndAssumptions,
}

impl std::fmt::Display for ReportSectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportSectionKind::ProjectInfo => write!(f, "ProjectInfo"),
            ReportSectionKind::SchematicSummary => write!(f, "SchematicSummary"),
            ReportSectionKind::ComponentSummary => write!(f, "ComponentSummary"),
            ReportSectionKind::FormulaCalculations => write!(f, "FormulaCalculations"),
            ReportSectionKind::NotebookCalculations => write!(f, "NotebookCalculations"),
            ReportSectionKind::DcdcCalculations => write!(f, "DcdcCalculations"),
            ReportSectionKind::SelectedRegionAnalysis => write!(f, "SelectedRegionAnalysis"),
            ReportSectionKind::SimulationResults => write!(f, "SimulationResults"),
            ReportSectionKind::SpiceNetlist => write!(f, "SpiceNetlist"),
            ReportSectionKind::ESeriesSelections => write!(f, "ESeriesSelections"),
            ReportSectionKind::Bom => write!(f, "Bom"),
            ReportSectionKind::ImportedModels => write!(f, "ImportedModels"),
            ReportSectionKind::ModelMappingReadiness => write!(f, "ModelMappingReadiness"),
            ReportSectionKind::ModelPersistence => write!(f, "ModelPersistence"),
            ReportSectionKind::ExportHistory => write!(f, "ExportHistory"),
            ReportSectionKind::WarningsAndAssumptions => write!(f, "WarningsAndAssumptions"),
        }
    }
}

impl std::str::FromStr for ReportSectionKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProjectInfo" => Ok(ReportSectionKind::ProjectInfo),
            "SchematicSummary" => Ok(ReportSectionKind::SchematicSummary),
            "ComponentSummary" => Ok(ReportSectionKind::ComponentSummary),
            "FormulaCalculations" => Ok(ReportSectionKind::FormulaCalculations),
            "NotebookCalculations" => Ok(ReportSectionKind::NotebookCalculations),
            "DcdcCalculations" => Ok(ReportSectionKind::DcdcCalculations),
            "SelectedRegionAnalysis" => Ok(ReportSectionKind::SelectedRegionAnalysis),
            "SimulationResults" => Ok(ReportSectionKind::SimulationResults),
            "SpiceNetlist" => Ok(ReportSectionKind::SpiceNetlist),
            "ESeriesSelections" => Ok(ReportSectionKind::ESeriesSelections),
            "Bom" => Ok(ReportSectionKind::Bom),
            "ImportedModels" => Ok(ReportSectionKind::ImportedModels),
            "ModelMappingReadiness" => Ok(ReportSectionKind::ModelMappingReadiness),
            "ModelPersistence" => Ok(ReportSectionKind::ModelPersistence),
            "ExportHistory" => Ok(ReportSectionKind::ExportHistory),
            "WarningsAndAssumptions" => Ok(ReportSectionKind::WarningsAndAssumptions),
            other => Err(format!("unknown section kind: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedReportRequest {
    pub report_id: String,
    pub title: String,
    pub report_type: AdvancedReportType,
    pub included_sections: Vec<ReportSectionKind>,
    pub export_options: ReportExportOptions,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportExportOptions {
    pub include_source_references: bool,
    pub include_graph_references: bool,
    pub include_assumptions: bool,
    pub max_table_rows: Option<usize>,
}

impl Default for ReportExportOptions {
    fn default() -> Self {
        Self {
            include_source_references: true,
            include_graph_references: true,
            include_assumptions: true,
            max_table_rows: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedReportModel {
    pub id: String,
    pub title: String,
    pub report_type: AdvancedReportType,
    pub generated_at: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub sections: Vec<ReportSection>,
    pub warnings: Vec<ReportWarning>,
    pub assumptions: Vec<String>,
    pub source_references: Vec<ReportSourceReference>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportSection {
    pub kind: ReportSectionKind,
    pub title: String,
    pub status: ReportSectionStatus,
    pub blocks: Vec<ReportContentBlock>,
    pub warnings: Vec<ReportWarning>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportSectionStatus {
    Included,
    Empty,
    Unavailable,
    Error,
}

impl std::fmt::Display for ReportSectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportSectionStatus::Included => write!(f, "Included"),
            ReportSectionStatus::Empty => write!(f, "Empty"),
            ReportSectionStatus::Unavailable => write!(f, "Unavailable"),
            ReportSectionStatus::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportContentBlock {
    Paragraph {
        text: String,
    },
    KeyValueTable {
        title: String,
        rows: Vec<ReportKeyValueRow>,
    },
    DataTable {
        title: String,
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    FormulaBlock {
        title: String,
        equation: String,
        substituted_values: Vec<ReportKeyValueRow>,
        result: Option<String>,
    },
    CodeBlock {
        title: String,
        language: String,
        content: String,
    },
    GraphReference {
        title: String,
        series_names: Vec<String>,
        x_unit: Option<String>,
        y_unit: Option<String>,
    },
    WarningList {
        items: Vec<ReportWarning>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportKeyValueRow {
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportWarning {
    pub severity: ReportWarningSeverity,
    pub code: String,
    pub message: String,
    pub section_kind: Option<ReportSectionKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportWarningSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportSourceReference {
    pub source_id: String,
    pub source_type: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportSectionCapability {
    pub kind: ReportSectionKind,
    pub title: String,
    pub description: String,
    pub default_enabled: bool,
    pub supported_report_types: Vec<AdvancedReportType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvancedReportContext {
    pub project: Option<crate::CircuitProject>,
    pub notebook: Option<crate::EngineeringNotebook>,
    pub simulation_result: Option<crate::SimulationResult>,
    pub dcdc_result: Option<crate::DcdcCalculationResult>,
    pub selected_region_result: Option<crate::SelectedRegionAnalysisResult>,
    pub export_history: Vec<crate::ExportHistoryEntry>,
    pub netlist: Option<String>,
    pub imported_models_summary: Vec<String>,
    pub model_persistence_summary: Option<crate::ProjectModelPersistenceSummary>,
}

pub fn default_section_capabilities() -> Vec<ReportSectionCapability> {
    vec![
        ReportSectionCapability {
            kind: ReportSectionKind::ProjectInfo,
            title: "Project Info".to_string(),
            description: "Basic project metadata".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::SchematicSummary,
            title: "Schematic Summary".to_string(),
            description: "Components and nets overview".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::ComponentSummary,
            title: "Component Summary".to_string(),
            description: "Component list with parameters".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::FormulaCalculations,
            title: "Formula Calculations".to_string(),
            description: "Formula-based calculations".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::CalculationReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::NotebookCalculations,
            title: "Notebook Calculations".to_string(),
            description: "Engineering notebook entries".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::CalculationReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::DcdcCalculations,
            title: "DC-DC Calculations".to_string(),
            description: "DC-DC converter design calculations".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::DcdcDesignReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::SelectedRegionAnalysis,
            title: "Selected Region Analysis".to_string(),
            description: "Selected region template matching".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::SelectedRegionReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::SimulationResults,
            title: "Simulation Results".to_string(),
            description: "Simulation graphs and measurements".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::SimulationReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::SpiceNetlist,
            title: "SPICE Netlist".to_string(),
            description: "Generated SPICE netlist".to_string(),
            default_enabled: false,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::ESeriesSelections,
            title: "E-Series Selections".to_string(),
            description: "Preferred value selections".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::CalculationReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::Bom,
            title: "Bill of Materials".to_string(),
            description: "Project BOM".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::ImportedModels,
            title: "Imported Models".to_string(),
            description: "SPICE/Touchstone imported models".to_string(),
            default_enabled: false,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::ModelMappingReadiness,
            title: "Model Mapping Readiness".to_string(),
            description: "Component model assignment and simulation readiness".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::SimulationReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::ModelPersistence,
            title: "Model Persistence & Package Integrity".to_string(),
            description: "Persisted model catalog, assignments, and package diagnostics"
                .to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::ExportHistory,
            title: "Export History".to_string(),
            description: "Recent export operations".to_string(),
            default_enabled: false,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::FullProjectReport,
            ],
        },
        ReportSectionCapability {
            kind: ReportSectionKind::WarningsAndAssumptions,
            title: "Warnings and Assumptions".to_string(),
            description: "Report warnings and design assumptions".to_string(),
            default_enabled: true,
            supported_report_types: vec![
                AdvancedReportType::ProjectSummary,
                AdvancedReportType::CalculationReport,
                AdvancedReportType::SimulationReport,
                AdvancedReportType::SelectedRegionReport,
                AdvancedReportType::DcdcDesignReport,
                AdvancedReportType::FullProjectReport,
            ],
        },
    ]
}
