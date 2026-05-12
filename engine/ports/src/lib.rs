use hotsas_core::{
    CircuitProject, ComponentLibrary, ExportResult, FormulaDefinition, FormulaEvaluationResult,
    FormulaExpressionValidationResult, FormulaOutput, NgspiceAvailability, ProjectPackageManifest,
    ProjectPackageValidationReport, ReportModel, SimulationProfile, SimulationResult,
    ValueWithUnit,
};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum PortError {
    Storage(String),
    Formula(String),
    Export(String),
    Simulation(String),
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(message) => write!(f, "storage error: {message}"),
            Self::Formula(message) => write!(f, "formula error: {message}"),
            Self::Export(message) => write!(f, "export error: {message}"),
            Self::Simulation(message) => write!(f, "simulation error: {message}"),
        }
    }
}

impl std::error::Error for PortError {}

pub trait StoragePort: Send + Sync {
    fn save_project(&self, path: &Path, project: &CircuitProject) -> Result<(), PortError>;
    fn load_project(&self, path: &Path) -> Result<CircuitProject, PortError>;
}

pub trait FormulaEnginePort: Send + Sync {
    fn calculate_rc_low_pass_cutoff(
        &self,
        resistance: &ValueWithUnit,
        capacitance: &ValueWithUnit,
    ) -> Result<ValueWithUnit, PortError>;

    fn evaluate_formula(
        &self,
        formula: &FormulaDefinition,
        variables: &BTreeMap<String, ValueWithUnit>,
    ) -> Result<FormulaEvaluationResult, PortError> {
        let equation = formula
            .equations
            .first()
            .ok_or_else(|| PortError::Formula("formula has no equations".to_string()))?;
        let outputs =
            self.evaluate_expression(&equation.expression, variables, &formula.outputs)?;
        Ok(FormulaEvaluationResult {
            formula_id: formula.id.clone(),
            equation_id: equation.id.clone(),
            expression: equation.expression.clone(),
            inputs: variables.clone(),
            outputs,
            warnings: vec![],
        })
    }

    fn evaluate_expression(
        &self,
        expression: &str,
        _variables: &BTreeMap<String, ValueWithUnit>,
        _expected_outputs: &BTreeMap<String, FormulaOutput>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, PortError> {
        Err(PortError::Formula(format!(
            "unsupported expression: {expression}"
        )))
    }

    fn validate_expression(
        &self,
        expression: &str,
    ) -> Result<FormulaExpressionValidationResult, PortError> {
        Ok(FormulaExpressionValidationResult {
            expression: expression.to_string(),
            supported: false,
            reason: Some(format!("unsupported expression: {expression}")),
        })
    }
}

pub trait NetlistExporterPort: Send + Sync {
    fn export_spice_netlist(&self, project: &CircuitProject) -> Result<String, PortError>;
}

pub trait SimulationEnginePort: Send + Sync {
    fn engine_name(&self) -> &str;

    fn check_availability(&self) -> Result<NgspiceAvailability, PortError> {
        Ok(NgspiceAvailability {
            available: false,
            executable_path: None,
            version: None,
            message: Some(format!(
                "availability check not implemented for {}",
                self.engine_name()
            )),
            warnings: vec![],
        })
    }

    fn run_ac_sweep(
        &self,
        project: &CircuitProject,
        profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError>;

    fn run_operating_point(
        &self,
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation(format!(
            "operating point not supported by {}",
            self.engine_name()
        )))
    }

    fn run_transient(
        &self,
        _project: &CircuitProject,
        _profile: &SimulationProfile,
    ) -> Result<SimulationResult, PortError> {
        Err(PortError::Simulation(format!(
            "transient not supported by {}",
            self.engine_name()
        )))
    }

    fn stop_simulation(&self, _run_id: String) -> Result<(), PortError> {
        Ok(())
    }

    fn get_result(&self, _run_id: String) -> Result<Option<SimulationResult>, PortError> {
        Ok(None)
    }
}

pub trait ReportExporterPort: Send + Sync {
    fn export_markdown(&self, report: &ReportModel) -> Result<String, PortError>;
    fn export_html(&self, report: &ReportModel) -> Result<String, PortError>;
}

pub trait ProjectPackageStoragePort: Send + Sync {
    fn save_project_package(
        &self,
        package_dir: &Path,
        project: &CircuitProject,
    ) -> Result<ProjectPackageManifest, PortError>;

    fn load_project_package(&self, package_dir: &Path) -> Result<CircuitProject, PortError>;

    fn validate_project_package(
        &self,
        package_dir: &Path,
    ) -> Result<ProjectPackageValidationReport, PortError>;

    fn save_model_catalog(
        &self,
        package_dir: &Path,
        catalog: &hotsas_core::PersistedModelCatalog,
    ) -> Result<(), PortError>;

    fn load_model_catalog(
        &self,
        package_dir: &Path,
    ) -> Result<hotsas_core::PersistedModelCatalog, PortError>;

    fn save_model_assignments(
        &self,
        package_dir: &Path,
        assignments: &[hotsas_core::PersistedInstanceModelAssignment],
    ) -> Result<(), PortError>;

    fn load_model_assignments(
        &self,
        package_dir: &Path,
    ) -> Result<Vec<hotsas_core::PersistedInstanceModelAssignment>, PortError>;
}

pub trait BomExporterPort: Send + Sync {
    fn export_bom_csv(&self, project: &CircuitProject) -> Result<String, PortError>;
    fn export_bom_json(&self, project: &CircuitProject) -> Result<String, PortError>;
}

pub trait SimulationDataExporterPort: Send + Sync {
    fn export_simulation_csv(&self, simulation: &SimulationResult) -> Result<String, PortError>;
}

pub trait ComponentLibraryExporterPort: Send + Sync {
    fn export_component_library_json(
        &self,
        library: &ComponentLibrary,
    ) -> Result<String, PortError>;
}

pub trait SchematicExporterPort: Send + Sync {
    fn export_svg_schematic(&self, project: &CircuitProject) -> Result<String, PortError>;
}

pub trait ExportHistoryPort: Send + Sync {
    fn record_export(&self, result: &ExportResult) -> Result<(), PortError>;
    fn list_history(&self) -> Result<Vec<hotsas_core::ExportHistoryEntry>, PortError>;
}

pub trait ComponentLibraryPort: Send + Sync {
    fn load_builtin_library(&self) -> Result<ComponentLibrary, PortError>;

    fn load_library_from_path(&self, path: &Path) -> Result<ComponentLibrary, PortError>;

    fn save_library_to_path(
        &self,
        path: &Path,
        library: &ComponentLibrary,
    ) -> Result<(), PortError>;
}

pub trait SpiceModelParserPort: Send + Sync {
    fn parse_spice_models_from_str(
        &self,
        source_name: Option<String>,
        content: &str,
    ) -> Result<hotsas_core::SpiceImportReport, PortError>;
}

pub trait TouchstoneParserPort: Send + Sync {
    fn parse_touchstone_from_str(
        &self,
        source_name: Option<String>,
        content: &str,
    ) -> Result<hotsas_core::TouchstoneImportReport, PortError>;
}
