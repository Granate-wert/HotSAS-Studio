use hotsas_core::{
    CircuitProject, EngineeringNotebook, FormulaDefinition, FormulaEquation, FormulaOutput,
    FormulaPackMetadata, FormulaVariable, GraphSeries, ModelAssetValidationDiagnostic,
    NotebookEvaluationResult, NotebookHistoryEntry, PersistedModelAsset, PersistedModelCatalog,
    PreferredValueResult, ProjectModelPersistenceSummary, ProjectPackageManifest,
    ProjectPackageValidationReport, SimulationResult, ValueWithUnit,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub id: String,
    pub name: String,
    pub format_version: String,
    pub engine_version: String,
    pub project_type: String,
    pub schematic: CircuitDto,
}

impl From<&CircuitProject> for ProjectDto {
    fn from(project: &CircuitProject) -> Self {
        Self {
            id: project.id.clone(),
            name: project.name.clone(),
            format_version: project.format_version.clone(),
            engine_version: project.engine_version.clone(),
            project_type: project.project_type.clone(),
            schematic: CircuitDto {
                id: project.schematic.id.clone(),
                title: project.schematic.title.clone(),
                components: project
                    .schematic
                    .components
                    .iter()
                    .map(|component| {
                        let symbol = hotsas_core::seed_symbol_for_kind(&component.definition_id)
                            .as_ref()
                            .map(SymbolDto::from);
                        let pins = symbol.as_ref().map(|s| s.pins.clone()).unwrap_or_default();
                        ComponentDto {
                            instance_id: component.instance_id.clone(),
                            definition_id: component.definition_id.clone(),
                            component_kind: component.definition_id.clone(),
                            display_label: component.instance_id.clone(),
                            x: component.position.x,
                            y: component.position.y,
                            rotation_degrees: component.rotation_degrees,
                            parameters: component
                                .overridden_parameters
                                .iter()
                                .map(|(key, value)| ParameterDto {
                                    name: key.clone(),
                                    value: ValueDto::from(value),
                                })
                                .collect(),
                            symbol,
                            pins,
                            connected_nets: component
                                .connected_nets
                                .iter()
                                .map(|cn| ConnectedPinDto {
                                    component_id: cn.component_id.clone(),
                                    pin_id: cn.pin_id.clone(),
                                    net_id: cn.net_id.clone(),
                                })
                                .collect(),
                        }
                    })
                    .collect(),
                wires: project
                    .schematic
                    .wires
                    .iter()
                    .map(|wire| WireDto {
                        id: wire.id.clone(),
                        from_component_id: wire.from.component_id.clone(),
                        to_component_id: wire.to.component_id.clone(),
                        net_id: wire.net_id.clone(),
                    })
                    .collect(),
                nets: project
                    .schematic
                    .nets
                    .iter()
                    .map(|net| NetDto {
                        id: net.id.clone(),
                        name: net.name.clone(),
                    })
                    .collect(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitDto {
    pub id: String,
    pub title: String,
    pub components: Vec<ComponentDto>,
    pub wires: Vec<WireDto>,
    pub nets: Vec<NetDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDto {
    pub instance_id: String,
    pub definition_id: String,
    pub component_kind: String,
    pub display_label: String,
    pub x: f64,
    pub y: f64,
    pub rotation_degrees: f64,
    pub parameters: Vec<ParameterDto>,
    pub symbol: Option<SymbolDto>,
    pub pins: Vec<PinDto>,
    pub connected_nets: Vec<ConnectedPinDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDto {
    pub name: String,
    pub value: ValueDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinDto {
    pub id: String,
    pub name: String,
    pub number: String,
    pub electrical_type: String,
    pub x: f64,
    pub y: f64,
    pub side: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolDto {
    pub id: String,
    pub title: String,
    pub component_kind: String,
    pub width: f64,
    pub height: f64,
    pub pins: Vec<PinDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedPinDto {
    #[serde(default)]
    pub component_id: String,
    pub pin_id: String,
    pub net_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentParameterDto {
    pub name: String,
    pub value: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedComponentDto {
    pub instance_id: String,
    pub component_kind: String,
    pub title: String,
    pub parameters: Vec<ComponentParameterDto>,
    pub symbol: Option<SymbolDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitValidationIssueDto {
    pub code: String,
    pub message: String,
    pub component_id: Option<String>,
    pub net_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitValidationReportDto {
    pub valid: bool,
    pub warnings: Vec<CircuitValidationIssueDto>,
    pub errors: Vec<CircuitValidationIssueDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireDto {
    pub id: String,
    pub from_component_id: Option<String>,
    pub to_component_id: Option<String>,
    pub net_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueDto {
    pub original: String,
    pub si_value: f64,
    pub unit: String,
    pub display: String,
}

impl From<&hotsas_core::SymbolDefinition> for SymbolDto {
    fn from(symbol: &hotsas_core::SymbolDefinition) -> Self {
        Self {
            id: symbol.id.clone(),
            title: symbol.title.clone(),
            component_kind: symbol.component_kind.clone(),
            width: symbol.width,
            height: symbol.height,
            pins: symbol.pins.iter().map(PinDto::from).collect(),
        }
    }
}

impl From<&hotsas_core::PinDefinition> for PinDto {
    fn from(pin: &hotsas_core::PinDefinition) -> Self {
        Self {
            id: pin.id.clone(),
            name: pin.name.clone(),
            number: pin.number.clone(),
            electrical_type: pin.electrical_type.to_string(),
            x: pin.position.x,
            y: pin.position.y,
            side: pin.position.side.to_string(),
        }
    }
}

impl From<&hotsas_core::CircuitValidationReport> for CircuitValidationReportDto {
    fn from(report: &hotsas_core::CircuitValidationReport) -> Self {
        Self {
            valid: report.valid,
            warnings: report
                .warnings
                .iter()
                .map(CircuitValidationIssueDto::from)
                .collect(),
            errors: report
                .errors
                .iter()
                .map(CircuitValidationIssueDto::from)
                .collect(),
        }
    }
}

impl From<&hotsas_core::CircuitValidationIssue> for CircuitValidationIssueDto {
    fn from(issue: &hotsas_core::CircuitValidationIssue) -> Self {
        Self {
            code: issue.code.clone(),
            message: issue.message.clone(),
            component_id: issue.component_id.clone(),
            net_id: issue.net_id.clone(),
        }
    }
}

impl From<&ValueWithUnit> for ValueDto {
    fn from(value: &ValueWithUnit) -> Self {
        let unit = value.unit.symbol().to_string();
        Self {
            original: value.value.original.clone(),
            si_value: value.si_value(),
            unit: unit.clone(),
            display: if unit.is_empty() {
                format!("{:.6}", value.si_value())
            } else {
                format!("{:.6} {unit}", value.si_value())
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaResultDto {
    pub formula_id: String,
    pub output_name: String,
    pub value: ValueDto,
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaVariableInputDto {
    pub name: String,
    pub value: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaCalculationRequestDto {
    pub formula_id: String,
    pub variables: Vec<FormulaVariableInputDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaOutputValueDto {
    pub name: String,
    pub value: ValueDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaEvaluationResultDto {
    pub formula_id: String,
    pub equation_id: String,
    pub expression: String,
    pub outputs: Vec<FormulaOutputValueDto>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaPackDto {
    pub pack_id: String,
    pub title: String,
    pub version: String,
    pub formula_count: usize,
    pub categories: Vec<String>,
}

impl From<&FormulaPackMetadata> for FormulaPackDto {
    fn from(metadata: &FormulaPackMetadata) -> Self {
        Self {
            pack_id: metadata.pack_id.clone(),
            title: metadata.title.clone(),
            version: metadata.version.clone(),
            formula_count: metadata.formula_count,
            categories: metadata.categories.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaSummaryDto {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub linked_circuit_template_id: Option<String>,
}

impl From<&FormulaDefinition> for FormulaSummaryDto {
    fn from(formula: &FormulaDefinition) -> Self {
        Self {
            id: formula.id.clone(),
            title: formula.title.clone(),
            category: formula.category.clone(),
            description: formula.description.clone(),
            linked_circuit_template_id: formula.linked_circuit_template_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaExampleDto {
    pub title: String,
    pub inputs: Vec<FormulaExampleValueDto>,
    pub expected_outputs: Vec<FormulaExampleValueDto>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaExampleValueDto {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaDetailsDto {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub variables: Vec<FormulaVariableDto>,
    pub equations: Vec<FormulaEquationDto>,
    pub outputs: Vec<FormulaOutputDto>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub examples: Vec<FormulaExampleDto>,
    pub linked_circuit_template_id: Option<String>,
    pub mapping: Option<BTreeMap<String, String>>,
    pub default_simulation: Option<String>,
}

impl From<&FormulaDefinition> for FormulaDetailsDto {
    fn from(formula: &FormulaDefinition) -> Self {
        Self {
            id: formula.id.clone(),
            title: formula.title.clone(),
            category: formula.category.clone(),
            description: formula.description.clone(),
            variables: formula
                .variables
                .iter()
                .map(|(name, variable)| FormulaVariableDto::from_pair(name, variable))
                .collect(),
            equations: formula
                .equations
                .iter()
                .map(FormulaEquationDto::from)
                .collect(),
            outputs: formula
                .outputs
                .iter()
                .map(|(name, output)| FormulaOutputDto::from_pair(name, output))
                .collect(),
            assumptions: formula.assumptions.clone(),
            limitations: formula.limitations.clone(),
            examples: formula
                .examples
                .iter()
                .map(|e| FormulaExampleDto {
                    title: e.title.clone(),
                    inputs: e
                        .inputs
                        .iter()
                        .map(|(k, v)| FormulaExampleValueDto {
                            name: k.clone(),
                            value: v.clone(),
                        })
                        .collect(),
                    expected_outputs: e
                        .expected_outputs
                        .iter()
                        .map(|(k, v)| FormulaExampleValueDto {
                            name: k.clone(),
                            value: v.clone(),
                        })
                        .collect(),
                    notes: e.notes.clone(),
                })
                .collect(),
            linked_circuit_template_id: formula.linked_circuit_template_id.clone(),
            mapping: formula.mapping.clone(),
            default_simulation: formula
                .default_simulation_profile
                .as_ref()
                .map(|profile| format!("{:?}", profile.simulation_type)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaVariableDto {
    pub name: String,
    pub unit: String,
    pub description: String,
    pub default: Option<ValueDto>,
}

impl FormulaVariableDto {
    fn from_pair(name: &str, variable: &FormulaVariable) -> Self {
        Self {
            name: name.to_string(),
            unit: variable.unit.symbol().to_string(),
            description: variable.description.clone(),
            default: variable.default.as_ref().map(ValueDto::from),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaEquationDto {
    pub id: String,
    pub latex: String,
    pub expression: String,
    pub solve_for: Vec<String>,
}

impl From<&FormulaEquation> for FormulaEquationDto {
    fn from(equation: &FormulaEquation) -> Self {
        Self {
            id: equation.id.clone(),
            latex: equation.latex.clone(),
            expression: equation.expression.clone(),
            solve_for: equation.solve_for.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaOutputDto {
    pub name: String,
    pub unit: String,
    pub description: String,
}

impl FormulaOutputDto {
    fn from_pair(name: &str, output: &FormulaOutput) -> Self {
        Self {
            name: name.to_string(),
            unit: output.unit.symbol().to_string(),
            description: output.description.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredValueDto {
    pub requested_value: ValueDto,
    pub series: String,
    pub lower: Option<ValueDto>,
    pub nearest: ValueDto,
    pub higher: Option<ValueDto>,
    pub error_percent: f64,
}

impl From<&PreferredValueResult> for PreferredValueDto {
    fn from(result: &PreferredValueResult) -> Self {
        Self {
            requested_value: ValueDto::from(&result.requested_value),
            series: result.series.label().to_string(),
            lower: result.lower.as_ref().map(ValueDto::from),
            nearest: ValueDto::from(&result.nearest),
            higher: result.higher.as_ref().map(ValueDto::from),
            error_percent: result.error_percent,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResultDto {
    pub id: String,
    pub profile_id: String,
    pub status: String,
    pub engine: String,
    pub graph_series: Vec<GraphSeriesDto>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl From<&SimulationResult> for SimulationResultDto {
    fn from(result: &SimulationResult) -> Self {
        Self {
            id: result.id.clone(),
            profile_id: result.profile_id.clone(),
            status: format!("{:?}", result.status),
            engine: result.engine.clone(),
            graph_series: result
                .graph_series
                .iter()
                .map(GraphSeriesDto::from)
                .collect(),
            warnings: result.warnings.clone(),
            errors: result.errors.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSeriesDto {
    pub name: String,
    pub x_unit: String,
    pub y_unit: String,
    pub points: Vec<[f64; 2]>,
}

impl From<&GraphSeries> for GraphSeriesDto {
    fn from(series: &GraphSeries) -> Self {
        Self {
            name: series.name.clone(),
            x_unit: series.x_unit.symbol().to_string(),
            y_unit: series
                .metadata
                .get("quantity")
                .cloned()
                .unwrap_or_else(|| series.y_unit.symbol().to_string()),
            points: series
                .points
                .iter()
                .map(|point| [point.x, point.y])
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAssetDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub source: String,
    pub source_file_name: Option<String>,
    pub content_hash: Option<String>,
    pub package_asset_path: Option<String>,
    pub status: String,
    pub warnings: Vec<String>,
}

impl From<&PersistedModelAsset> for ModelAssetDto {
    fn from(asset: &PersistedModelAsset) -> Self {
        Self {
            id: asset.id.clone(),
            name: asset.name.clone(),
            kind: format!("{:?}", asset.kind).to_lowercase(),
            source: format!("{:?}", asset.source).to_lowercase(),
            source_file_name: asset.source_file_name.clone(),
            content_hash: asset.content_hash.clone(),
            package_asset_path: asset.package_asset_path.clone(),
            status: format!("{:?}", asset.status).to_lowercase(),
            warnings: asset.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCatalogDto {
    pub assets: Vec<ModelAssetDto>,
}

impl From<&PersistedModelCatalog> for ModelCatalogDto {
    fn from(catalog: &PersistedModelCatalog) -> Self {
        Self {
            assets: catalog.assets.iter().map(ModelAssetDto::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPersistenceDiagnosticDto {
    pub code: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub asset_id: Option<String>,
    pub assignment_id: Option<String>,
}

impl From<&ModelAssetValidationDiagnostic> for ModelPersistenceDiagnosticDto {
    fn from(d: &ModelAssetValidationDiagnostic) -> Self {
        Self {
            code: d.code.clone(),
            severity: d.severity.clone(),
            title: d.title.clone(),
            message: d.message.clone(),
            asset_id: d.asset_id.clone(),
            assignment_id: d.assignment_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectModelPersistenceSummaryDto {
    pub asset_count: usize,
    pub spice_model_count: usize,
    pub subcircuit_count: usize,
    pub touchstone_dataset_count: usize,
    pub component_assignment_count: usize,
    pub instance_assignment_count: usize,
    pub missing_asset_reference_count: usize,
    pub stale_assignment_count: usize,
    pub diagnostics: Vec<ModelPersistenceDiagnosticDto>,
    pub ready: bool,
}

impl From<&ProjectModelPersistenceSummary> for ProjectModelPersistenceSummaryDto {
    fn from(summary: &ProjectModelPersistenceSummary) -> Self {
        Self {
            asset_count: summary.asset_count,
            spice_model_count: summary.spice_model_count,
            subcircuit_count: summary.subcircuit_count,
            touchstone_dataset_count: summary.touchstone_dataset_count,
            component_assignment_count: summary.component_assignment_count,
            instance_assignment_count: summary.instance_assignment_count,
            missing_asset_reference_count: summary.missing_asset_reference_count,
            stale_assignment_count: summary.stale_assignment_count,
            diagnostics: summary.diagnostics.iter().map(|d| d.into()).collect(),
            ready: summary.ready,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveProjectDto {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalSliceDto {
    pub project: ProjectDto,
    pub cutoff_frequency: FormulaResultDto,
    pub nearest_e24: PreferredValueDto,
    pub spice_netlist: String,
    pub simulation: SimulationResultDto,
    pub markdown_report: String,
    pub html_report: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPackageFilesDto {
    pub schematic: String,
    pub components: String,
    pub formulas: String,
    pub simulation_profiles: String,
    pub reports_index: String,
    pub results_index: String,
}

impl From<&hotsas_core::ProjectPackageFiles> for ProjectPackageFilesDto {
    fn from(files: &hotsas_core::ProjectPackageFiles) -> Self {
        Self {
            schematic: files.schematic.clone(),
            components: files.components.clone(),
            formulas: files.formulas.clone(),
            simulation_profiles: files.simulation_profiles.clone(),
            reports_index: files.reports_index.clone(),
            results_index: files.results_index.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPackageManifestDto {
    pub format_version: String,
    pub engine_version: String,
    pub project_id: String,
    pub project_name: String,
    pub project_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub files: ProjectPackageFilesDto,
}

impl From<&ProjectPackageManifest> for ProjectPackageManifestDto {
    fn from(manifest: &ProjectPackageManifest) -> Self {
        Self {
            format_version: manifest.format_version.clone(),
            engine_version: manifest.engine_version.clone(),
            project_id: manifest.project_id.clone(),
            project_name: manifest.project_name.clone(),
            project_type: format!("{:?}", manifest.project_type),
            created_at: manifest.created_at.clone(),
            updated_at: manifest.updated_at.clone(),
            files: ProjectPackageFilesDto::from(&manifest.files),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPackageValidationReportDto {
    pub valid: bool,
    pub package_dir: String,
    pub missing_files: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl From<&ProjectPackageValidationReport> for ProjectPackageValidationReportDto {
    fn from(report: &ProjectPackageValidationReport) -> Self {
        Self {
            valid: report.valid,
            package_dir: report.package_dir.clone(),
            missing_files: report.missing_files.clone(),
            warnings: report.warnings.clone(),
            errors: report.errors.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookVariableDto {
    pub name: String,
    pub value: ValueDto,
}

impl From<(&String, &ValueWithUnit)> for NotebookVariableDto {
    fn from((name, value): (&String, &ValueWithUnit)) -> Self {
        Self {
            name: name.clone(),
            value: ValueDto::from(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookOutputDto {
    pub name: String,
    pub value: ValueDto,
}

impl From<(&String, &ValueWithUnit)> for NotebookOutputDto {
    fn from((name, value): (&String, &ValueWithUnit)) -> Self {
        Self {
            name: name.clone(),
            value: ValueDto::from(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookEvaluationRequestDto {
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookEvaluationResultDto {
    pub input: String,
    pub status: String,
    pub kind: String,
    pub outputs: Vec<NotebookOutputDto>,
    pub variables: Vec<NotebookVariableDto>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
}

impl From<&NotebookEvaluationResult> for NotebookEvaluationResultDto {
    fn from(result: &NotebookEvaluationResult) -> Self {
        Self {
            input: result.input.clone(),
            status: result.status.as_str().to_string(),
            kind: format!("{:?}", result.kind),
            outputs: result.outputs.iter().map(NotebookOutputDto::from).collect(),
            variables: result
                .variables
                .iter()
                .map(NotebookVariableDto::from)
                .collect(),
            message: result.message.clone(),
            warnings: result.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookHistoryEntryDto {
    pub id: String,
    pub input: String,
    pub result_summary: String,
    pub status: String,
}

impl From<&NotebookHistoryEntry> for NotebookHistoryEntryDto {
    fn from(entry: &NotebookHistoryEntry) -> Self {
        Self {
            id: entry.id.clone(),
            input: entry.input.clone(),
            result_summary: entry.result_summary.clone(),
            status: entry.status.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookStateDto {
    pub variables: Vec<NotebookVariableDto>,
    pub history: Vec<NotebookHistoryEntryDto>,
}

impl From<&EngineeringNotebook> for NotebookStateDto {
    fn from(notebook: &EngineeringNotebook) -> Self {
        Self {
            variables: notebook
                .variables
                .iter()
                .map(NotebookVariableDto::from)
                .collect(),
            history: notebook
                .history
                .iter()
                .map(NotebookHistoryEntryDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyNotebookValueRequestDto {
    pub instance_id: String,
    pub parameter_name: String,
    pub output_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLibraryDto {
    pub id: String,
    pub title: String,
    pub version: String,
    pub components: Vec<ComponentSummaryDto>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSummaryDto {
    pub id: String,
    pub name: String,
    pub category: String,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub has_symbol: bool,
    pub has_footprint: bool,
    pub has_simulation_model: bool,
}

impl From<&hotsas_core::ComponentDefinition> for ComponentSummaryDto {
    fn from(def: &hotsas_core::ComponentDefinition) -> Self {
        Self {
            id: def.id.clone(),
            name: def.name.clone(),
            category: def.category.clone(),
            manufacturer: def.manufacturer.clone(),
            part_number: def.part_number.clone(),
            description: def.description.clone(),
            tags: def.tags.clone(),
            has_symbol: !def.symbol_ids.is_empty(),
            has_footprint: !def.footprint_ids.is_empty(),
            has_simulation_model: !def.simulation_models.is_empty(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDetailsDto {
    pub id: String,
    pub name: String,
    pub category: String,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<ComponentParameterDto>,
    pub ratings: Vec<ComponentParameterDto>,
    pub symbol_ids: Vec<String>,
    pub footprint_ids: Vec<String>,
    pub simulation_models: Vec<SimulationModelDto>,
    pub datasheets: Vec<String>,
    pub tags: Vec<String>,
    pub metadata: Vec<KeyValueDto>,
    pub symbol_preview: Option<SymbolDto>,
    pub footprint_previews: Vec<FootprintDto>,
}

impl From<&hotsas_core::ComponentDefinition> for ComponentDetailsDto {
    fn from(component: &hotsas_core::ComponentDefinition) -> Self {
        Self {
            id: component.id.clone(),
            name: component.name.clone(),
            category: component.category.clone(),
            manufacturer: component.manufacturer.clone(),
            part_number: component.part_number.clone(),
            description: component.description.clone(),
            parameters: component
                .parameters
                .iter()
                .map(|(name, value)| ComponentParameterDto {
                    name: name.clone(),
                    value: value.value.original.clone(),
                    unit: Some(value.unit.symbol().to_string()),
                })
                .collect(),
            ratings: component
                .ratings
                .iter()
                .map(|(name, value)| ComponentParameterDto {
                    name: name.clone(),
                    value: value.value.original.clone(),
                    unit: Some(value.unit.symbol().to_string()),
                })
                .collect(),
            symbol_ids: component.symbol_ids.clone(),
            footprint_ids: component.footprint_ids.clone(),
            simulation_models: component
                .simulation_models
                .iter()
                .map(SimulationModelDto::from)
                .collect(),
            datasheets: component.datasheets.clone(),
            tags: component.tags.clone(),
            metadata: component
                .metadata
                .iter()
                .map(|(k, v)| KeyValueDto {
                    key: k.clone(),
                    value: v.clone(),
                })
                .collect(),
            symbol_preview: None,
            footprint_previews: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSearchRequestDto {
    pub search: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub manufacturer: Option<String>,
    pub has_symbol: Option<bool>,
    pub has_footprint: Option<bool>,
    pub has_simulation_model: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSearchResultDto {
    pub components: Vec<ComponentSummaryDto>,
    pub total_count: usize,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignComponentRequestDto {
    pub instance_id: String,
    pub component_definition_id: String,
    pub selected_symbol_id: Option<String>,
    pub selected_footprint_id: Option<String>,
    pub selected_simulation_model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintDto {
    pub id: String,
    pub name: String,
    pub package_name: String,
    pub pad_count: usize,
    pub metadata: Vec<KeyValueDto>,
}

impl From<&hotsas_core::FootprintDefinition> for FootprintDto {
    fn from(fp: &hotsas_core::FootprintDefinition) -> Self {
        Self {
            id: fp.id.clone(),
            name: fp.name.clone(),
            package_name: fp.package_name.clone(),
            pad_count: fp.pads.len(),
            metadata: fp
                .metadata
                .iter()
                .map(|(k, v)| KeyValueDto {
                    key: k.clone(),
                    value: v.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationModelDto {
    pub id: String,
    pub model_type: String,
    pub source_path: Option<String>,
}

impl From<&hotsas_core::SimulationModel> for SimulationModelDto {
    fn from(model: &hotsas_core::SimulationModel) -> Self {
        Self {
            id: model.id.clone(),
            model_type: model.model_type.clone(),
            source_path: model.source_path.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueDto {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionPortDto {
    pub positive_net: String,
    pub negative_net: Option<String>,
    pub label: Option<String>,
}

impl From<&hotsas_core::RegionPort> for RegionPortDto {
    fn from(port: &hotsas_core::RegionPort) -> Self {
        Self {
            positive_net: port.positive_net.clone(),
            negative_net: port.negative_net.clone(),
            label: port.label.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedRegionAnalysisRequestDto {
    pub component_ids: Vec<String>,
    pub input_port: Option<RegionPortDto>,
    pub output_port: Option<RegionPortDto>,
    pub reference_node: Option<String>,
    pub analysis_direction: String,
    pub analysis_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedCircuitRegionDto {
    pub id: String,
    pub title: String,
    pub component_ids: Vec<String>,
    pub internal_nets: Vec<String>,
    pub boundary_nets: Vec<String>,
    pub input_port: Option<RegionPortDto>,
    pub output_port: Option<RegionPortDto>,
    pub reference_node: Option<String>,
    pub analysis_direction: String,
    pub analysis_mode: String,
}

impl From<&hotsas_core::SelectedCircuitRegion> for SelectedCircuitRegionDto {
    fn from(region: &hotsas_core::SelectedCircuitRegion) -> Self {
        Self {
            id: region.id.clone(),
            title: region.title.clone(),
            component_ids: region.component_ids.clone(),
            internal_nets: region.internal_nets.clone(),
            boundary_nets: region.boundary_nets.clone(),
            input_port: region.input_port.as_ref().map(RegionPortDto::from),
            output_port: region.output_port.as_ref().map(RegionPortDto::from),
            reference_node: region.reference_node.clone(),
            analysis_direction: format!("{:?}", region.analysis_direction),
            analysis_mode: format!("{:?}", region.analysis_mode),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionComponentSummaryDto {
    pub instance_id: String,
    pub definition_id: Option<String>,
    pub component_kind: String,
    pub display_label: String,
    pub connected_nets: Vec<String>,
}

impl From<&hotsas_core::RegionComponentSummary> for RegionComponentSummaryDto {
    fn from(summary: &hotsas_core::RegionComponentSummary) -> Self {
        Self {
            instance_id: summary.instance_id.clone(),
            definition_id: summary.definition_id.clone(),
            component_kind: summary.component_kind.clone(),
            display_label: summary.display_label.clone(),
            connected_nets: summary.connected_nets.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionNetSummaryDto {
    pub net_id: String,
    pub net_name: String,
    pub connected_selected_components: Vec<String>,
    pub connected_external_components: Vec<String>,
    pub is_ground: bool,
    pub role_hint: Option<String>,
}

impl From<&hotsas_core::RegionNetSummary> for RegionNetSummaryDto {
    fn from(summary: &hotsas_core::RegionNetSummary) -> Self {
        Self {
            net_id: summary.net_id.clone(),
            net_name: summary.net_name.clone(),
            connected_selected_components: summary.connected_selected_components.clone(),
            connected_external_components: summary.connected_external_components.clone(),
            is_ground: summary.is_ground,
            role_hint: summary.role_hint.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedRegionIssueDto {
    pub code: String,
    pub severity: String,
    pub message: String,
    pub component_id: Option<String>,
    pub net_id: Option<String>,
}

impl From<&hotsas_core::SelectedRegionIssue> for SelectedRegionIssueDto {
    fn from(issue: &hotsas_core::SelectedRegionIssue) -> Self {
        Self {
            code: issue.code.clone(),
            severity: format!("{:?}", issue.severity),
            message: issue.message.clone(),
            component_id: issue.component_id.clone(),
            net_id: issue.net_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedRegionPreviewDto {
    pub region: SelectedCircuitRegionDto,
    pub selected_components: Vec<RegionComponentSummaryDto>,
    pub detected_internal_nets: Vec<RegionNetSummaryDto>,
    pub detected_boundary_nets: Vec<RegionNetSummaryDto>,
    pub suggested_input_nets: Vec<String>,
    pub suggested_output_nets: Vec<String>,
    pub suggested_reference_nodes: Vec<String>,
    pub warnings: Vec<SelectedRegionIssueDto>,
    pub errors: Vec<SelectedRegionIssueDto>,
}

impl From<&hotsas_core::SelectedRegionPreview> for SelectedRegionPreviewDto {
    fn from(preview: &hotsas_core::SelectedRegionPreview) -> Self {
        Self {
            region: SelectedCircuitRegionDto::from(&preview.region),
            selected_components: preview
                .selected_components
                .iter()
                .map(RegionComponentSummaryDto::from)
                .collect(),
            detected_internal_nets: preview
                .detected_internal_nets
                .iter()
                .map(RegionNetSummaryDto::from)
                .collect(),
            detected_boundary_nets: preview
                .detected_boundary_nets
                .iter()
                .map(RegionNetSummaryDto::from)
                .collect(),
            suggested_input_nets: preview.suggested_input_nets.clone(),
            suggested_output_nets: preview.suggested_output_nets.clone(),
            suggested_reference_nodes: preview.suggested_reference_nodes.clone(),
            warnings: preview
                .warnings
                .iter()
                .map(SelectedRegionIssueDto::from)
                .collect(),
            errors: preview
                .errors
                .iter()
                .map(SelectedRegionIssueDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedRegionTemplateDto {
    pub template_id: String,
    pub title: String,
    pub confidence: f64,
    pub formula_ids: Vec<String>,
    pub explanation: String,
}

impl From<&hotsas_core::MatchedRegionTemplate> for MatchedRegionTemplateDto {
    fn from(template: &hotsas_core::MatchedRegionTemplate) -> Self {
        Self {
            template_id: template.template_id.clone(),
            title: template.title.clone(),
            confidence: template.confidence,
            formula_ids: template.formula_ids.clone(),
            explanation: template.explanation.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquivalentCircuitSummaryDto {
    pub title: String,
    pub description: String,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}

impl From<&hotsas_core::EquivalentCircuitSummary> for EquivalentCircuitSummaryDto {
    fn from(summary: &hotsas_core::EquivalentCircuitSummary) -> Self {
        Self {
            title: summary.title.clone(),
            description: summary.description.clone(),
            assumptions: summary.assumptions.clone(),
            limitations: summary.limitations.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionTransferFunctionDto {
    pub expression: String,
    pub latex: Option<String>,
    pub output_name: String,
    pub unit: Option<String>,
    pub availability_note: Option<String>,
}

impl From<&hotsas_core::RegionTransferFunction> for RegionTransferFunctionDto {
    fn from(tf: &hotsas_core::RegionTransferFunction) -> Self {
        Self {
            expression: tf.expression.clone(),
            latex: tf.latex.clone(),
            output_name: tf.output_name.clone(),
            unit: tf.unit.map(|u| u.symbol().to_string()),
            availability_note: tf.availability_note.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionMeasurementDto {
    pub name: String,
    pub value: Option<ValueDto>,
    pub description: String,
    pub source: String,
}

impl From<&hotsas_core::RegionMeasurement> for RegionMeasurementDto {
    fn from(m: &hotsas_core::RegionMeasurement) -> Self {
        Self {
            name: m.name.clone(),
            value: m.value.as_ref().map(ValueDto::from),
            description: m.description.clone(),
            source: m.source.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionGraphSpecDto {
    pub id: String,
    pub title: String,
    pub x_unit: Option<String>,
    pub y_unit: Option<String>,
    pub description: String,
    pub available: bool,
    pub unavailable_reason: Option<String>,
}

impl From<&hotsas_core::RegionGraphSpec> for RegionGraphSpecDto {
    fn from(spec: &hotsas_core::RegionGraphSpec) -> Self {
        Self {
            id: spec.id.clone(),
            title: spec.title.clone(),
            x_unit: spec.x_unit.map(|u| u.symbol().to_string()),
            y_unit: spec.y_unit.map(|u| u.symbol().to_string()),
            description: spec.description.clone(),
            available: spec.available,
            unavailable_reason: spec.unavailable_reason.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionNetlistFragmentDto {
    pub title: String,
    pub format: String,
    pub content: String,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::RegionNetlistFragment> for RegionNetlistFragmentDto {
    fn from(fragment: &hotsas_core::RegionNetlistFragment) -> Self {
        Self {
            title: fragment.title.clone(),
            format: fragment.format.clone(),
            content: fragment.content.clone(),
            warnings: fragment.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedRegionAnalysisResultDto {
    pub region: SelectedCircuitRegionDto,
    pub status: String,
    pub summary: String,
    pub matched_template: Option<MatchedRegionTemplateDto>,
    pub equivalent_circuit: Option<EquivalentCircuitSummaryDto>,
    pub transfer_function: Option<RegionTransferFunctionDto>,
    pub measurements: Vec<RegionMeasurementDto>,
    pub graph_specs: Vec<RegionGraphSpecDto>,
    pub netlist_fragment: Option<RegionNetlistFragmentDto>,
    pub warnings: Vec<SelectedRegionIssueDto>,
    pub errors: Vec<SelectedRegionIssueDto>,
    pub report_section_markdown: Option<String>,
}

impl From<&hotsas_core::SelectedRegionAnalysisResult> for SelectedRegionAnalysisResultDto {
    fn from(result: &hotsas_core::SelectedRegionAnalysisResult) -> Self {
        Self {
            region: SelectedCircuitRegionDto::from(&result.region),
            status: format!("{:?}", result.status),
            summary: result.summary.clone(),
            matched_template: result
                .matched_template
                .as_ref()
                .map(MatchedRegionTemplateDto::from),
            equivalent_circuit: result
                .equivalent_circuit
                .as_ref()
                .map(EquivalentCircuitSummaryDto::from),
            transfer_function: result
                .transfer_function
                .as_ref()
                .map(RegionTransferFunctionDto::from),
            measurements: result
                .measurements
                .iter()
                .map(RegionMeasurementDto::from)
                .collect(),
            graph_specs: result
                .graph_specs
                .iter()
                .map(RegionGraphSpecDto::from)
                .collect(),
            netlist_fragment: result
                .netlist_fragment
                .as_ref()
                .map(RegionNetlistFragmentDto::from),
            warnings: result
                .warnings
                .iter()
                .map(SelectedRegionIssueDto::from)
                .collect(),
            errors: result
                .errors
                .iter()
                .map(SelectedRegionIssueDto::from)
                .collect(),
            report_section_markdown: result.report_section_markdown.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCapabilityDto {
    pub format: String,
    pub label: String,
    pub description: String,
    pub file_extension: String,
    pub available: bool,
}

impl From<&hotsas_core::ExportCapability> for ExportCapabilityDto {
    fn from(cap: &hotsas_core::ExportCapability) -> Self {
        Self {
            format: cap.format.clone(),
            label: cap.label.clone(),
            description: cap.description.clone(),
            file_extension: cap.file_extension.clone(),
            available: cap.available,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResultDto {
    pub format: String,
    pub content: String,
    pub file_path: Option<String>,
    pub success: bool,
    pub message: String,
}

impl From<&hotsas_core::ExportResult> for ExportResultDto {
    fn from(result: &hotsas_core::ExportResult) -> Self {
        Self {
            format: result.format.clone(),
            content: result.content.clone(),
            file_path: result.file_path.clone(),
            success: result.success,
            message: result.message.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportHistoryEntryDto {
    pub timestamp: String,
    pub format: String,
    pub file_path: Option<String>,
    pub success: bool,
    pub message: String,
}

impl From<&hotsas_core::ExportHistoryEntry> for ExportHistoryEntryDto {
    fn from(entry: &hotsas_core::ExportHistoryEntry) -> Self {
        Self {
            timestamp: entry.timestamp.clone(),
            format: entry.format.clone(),
            file_path: entry.file_path.clone(),
            success: entry.success,
            message: entry.message.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequestDto {
    pub format: String,
    pub write_to_file: bool,
    pub output_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgspiceAvailabilityDto {
    pub available: bool,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::NgspiceAvailability> for NgspiceAvailabilityDto {
    fn from(a: &hotsas_core::NgspiceAvailability) -> Self {
        Self {
            available: a.available,
            executable_path: a.executable_path.clone(),
            version: a.version.clone(),
            message: a.message.clone(),
            warnings: a.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRunRequestDto {
    pub engine: String,
    pub analysis_kind: String,
    pub profile_id: Option<String>,
    pub output_variables: Vec<String>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRunMetadataDto {
    pub run_id: String,
    pub engine: String,
    pub status: String,
    pub netlist_path: Option<String>,
    pub stdout_path: Option<String>,
    pub stderr_path: Option<String>,
    pub raw_output_path: Option<String>,
    pub parsed_output_path: Option<String>,
    pub exit_code: Option<i32>,
    pub elapsed_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceImportRequestDto {
    pub source_name: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceModelParameterDto {
    pub name: String,
    pub value: String,
    pub unit_hint: Option<String>,
}

impl From<&hotsas_core::SpiceModelParameter> for SpiceModelParameterDto {
    fn from(p: &hotsas_core::SpiceModelParameter) -> Self {
        Self {
            name: p.name.clone(),
            value: p.value.clone(),
            unit_hint: p.unit_hint.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceModelDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub parameters: Vec<SpiceModelParameterDto>,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::SpiceModelDefinition> for SpiceModelDto {
    fn from(m: &hotsas_core::SpiceModelDefinition) -> Self {
        Self {
            id: m.id.clone(),
            name: m.name.clone(),
            kind: format!("{:?}", m.kind),
            parameters: m
                .parameters
                .iter()
                .map(SpiceModelParameterDto::from)
                .collect(),
            warnings: m.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceSubcircuitDto {
    pub id: String,
    pub name: String,
    pub pins: Vec<String>,
    pub detected_kind: String,
    pub parameters: Vec<SpiceModelParameterDto>,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::SpiceSubcircuitDefinition> for SpiceSubcircuitDto {
    fn from(s: &hotsas_core::SpiceSubcircuitDefinition) -> Self {
        Self {
            id: s.id.clone(),
            name: s.name.clone(),
            pins: s.pins.clone(),
            detected_kind: format!("{:?}", s.detected_kind),
            parameters: s
                .parameters
                .iter()
                .map(SpiceModelParameterDto::from)
                .collect(),
            warnings: s.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceImportReportDto {
    pub status: String,
    pub models: Vec<SpiceModelDto>,
    pub subcircuits: Vec<SpiceSubcircuitDto>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl From<&hotsas_core::SpiceImportReport> for SpiceImportReportDto {
    fn from(r: &hotsas_core::SpiceImportReport) -> Self {
        Self {
            status: format!("{:?}", r.status),
            models: r.models.iter().map(SpiceModelDto::from).collect(),
            subcircuits: r.subcircuits.iter().map(SpiceSubcircuitDto::from).collect(),
            warnings: r.warnings.clone(),
            errors: r.errors.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchstoneImportRequestDto {
    pub source_name: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchstoneSummaryDto {
    pub id: String,
    pub name: String,
    pub port_count: usize,
    pub point_count: usize,
    pub start_frequency_hz: Option<f64>,
    pub stop_frequency_hz: Option<f64>,
    pub parameter_format: String,
    pub reference_impedance_ohm: f64,
}

impl From<&hotsas_core::TouchstoneNetworkData> for TouchstoneSummaryDto {
    fn from(n: &hotsas_core::TouchstoneNetworkData) -> Self {
        let freqs: Vec<f64> = n.points.iter().map(|p| p.frequency_hz).collect();
        Self {
            id: n.id.clone(),
            name: n.name.clone(),
            port_count: n.port_count,
            point_count: n.points.len(),
            start_frequency_hz: freqs.first().copied(),
            stop_frequency_hz: freqs.last().copied(),
            parameter_format: format!("{:?}", n.parameter_format),
            reference_impedance_ohm: n.reference_impedance_ohm,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TouchstoneImportReportDto {
    pub status: String,
    pub summary: Option<TouchstoneSummaryDto>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl From<&hotsas_core::TouchstoneImportReport> for TouchstoneImportReportDto {
    fn from(r: &hotsas_core::TouchstoneImportReport) -> Self {
        Self {
            status: format!("{:?}", r.status),
            summary: r.network.as_ref().map(TouchstoneSummaryDto::from),
            warnings: r.warnings.clone(),
            errors: r.errors.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpicePinMappingEntryDto {
    pub model_pin: String,
    pub component_pin: String,
    pub role_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpicePinMappingRequestDto {
    pub model_id: String,
    pub component_definition_id: String,
    pub mappings: Vec<SpicePinMappingEntryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpicePinMappingValidationReportDto {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl From<&hotsas_core::SpicePinMappingValidationReport> for SpicePinMappingValidationReportDto {
    fn from(r: &hotsas_core::SpicePinMappingValidationReport) -> Self {
        Self {
            valid: r.valid,
            warnings: r.warnings.clone(),
            errors: r.errors.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachImportedModelRequestDto {
    pub model_id: String,
    pub component_definition_id: String,
    pub pin_mapping: Option<SpicePinMappingRequestDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedModelSummaryDto {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub source_format: String,
}

impl From<&hotsas_core::ImportedModelSummary> for ImportedModelSummaryDto {
    fn from(s: &hotsas_core::ImportedModelSummary) -> Self {
        Self {
            id: s.id.clone(),
            kind: format!("{:?}", s.kind),
            name: s.name.clone(),
            source_format: s.source_format.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedModelDetailsDto {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub source_format: String,
    pub spice_model: Option<SpiceModelDto>,
    pub spice_subcircuit: Option<SpiceSubcircuitDto>,
    pub touchstone_summary: Option<TouchstoneSummaryDto>,
}

impl From<&hotsas_core::ImportedModelDetails> for ImportedModelDetailsDto {
    fn from(d: &hotsas_core::ImportedModelDetails) -> Self {
        Self {
            id: d.id.clone(),
            kind: format!("{:?}", d.kind),
            name: d.name.clone(),
            source_format: d.source.source_format.clone(),
            spice_model: d.spice_model.as_ref().map(SpiceModelDto::from),
            spice_subcircuit: d.spice_subcircuit.as_ref().map(SpiceSubcircuitDto::from),
            touchstone_summary: d
                .touchstone_network
                .as_ref()
                .map(TouchstoneSummaryDto::from),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDiagnosticsDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub summary: String,
    pub details: std::collections::BTreeMap<String, String>,
}

impl From<&hotsas_core::ModuleDiagnostics> for ModuleDiagnosticsDto {
    fn from(m: &hotsas_core::ModuleDiagnostics) -> Self {
        Self {
            id: m.id.clone(),
            title: m.title.clone(),
            status: m.status.as_str().to_string(),
            summary: m.summary.clone(),
            details: m.details.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessCheckDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub message: String,
}

impl From<&hotsas_core::ReadinessCheck> for ReadinessCheckDto {
    fn from(c: &hotsas_core::ReadinessCheck) -> Self {
        Self {
            id: c.id.clone(),
            title: c.title.clone(),
            status: c.status.as_str().to_string(),
            message: c.message.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDiagnosticsReportDto {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub build_profile: String,
    pub modules: Vec<ModuleDiagnosticsDto>,
    pub checks: Vec<ReadinessCheckDto>,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::AppDiagnosticsReport> for AppDiagnosticsReportDto {
    fn from(r: &hotsas_core::AppDiagnosticsReport) -> Self {
        Self {
            app_name: r.app_name.clone(),
            app_version: r.app_version.clone(),
            roadmap_stage: r.roadmap_stage.clone(),
            build_profile: r.build_profile.clone(),
            modules: r.modules.iter().map(ModuleDiagnosticsDto::from).collect(),
            checks: r.checks.iter().map(ReadinessCheckDto::from).collect(),
            warnings: r.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWorkflowStatusDto {
    pub app_name: String,
    pub app_version: String,
    pub roadmap_stage: String,
    pub build_profile: String,
    pub current_project: Option<ProjectSummaryDto>,
    pub workflow_steps: Vec<WorkflowStepStatusDto>,
    pub module_statuses: Vec<WorkflowModuleStatusDto>,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::ProductWorkflowStatus> for ProductWorkflowStatusDto {
    fn from(s: &hotsas_core::ProductWorkflowStatus) -> Self {
        Self {
            app_name: s.app_name.clone(),
            app_version: s.app_version.clone(),
            roadmap_stage: s.roadmap_stage.clone(),
            build_profile: s.build_profile.clone(),
            current_project: s.current_project.as_ref().map(ProjectSummaryDto::from),
            workflow_steps: s
                .workflow_steps
                .iter()
                .map(WorkflowStepStatusDto::from)
                .collect(),
            module_statuses: s
                .module_statuses
                .iter()
                .map(WorkflowModuleStatusDto::from)
                .collect(),
            blockers: s.blockers.clone(),
            warnings: s.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummaryDto {
    pub project_id: String,
    pub project_name: String,
    pub format_version: String,
    pub component_count: usize,
    pub net_count: usize,
    pub simulation_profile_count: usize,
}

impl From<&hotsas_core::ProjectSummary> for ProjectSummaryDto {
    fn from(p: &hotsas_core::ProjectSummary) -> Self {
        Self {
            project_id: p.project_id.clone(),
            project_name: p.project_name.clone(),
            format_version: p.format_version.clone(),
            component_count: p.component_count,
            net_count: p.net_count,
            simulation_profile_count: p.simulation_profile_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepStatusDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub screen_id: String,
    pub description: String,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::WorkflowStepStatus> for WorkflowStepStatusDto {
    fn from(s: &hotsas_core::WorkflowStepStatus) -> Self {
        Self {
            id: s.id.clone(),
            title: s.title.clone(),
            status: s.status.as_str().to_string(),
            screen_id: s.screen_id.clone(),
            description: s.description.clone(),
            warnings: s.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowModuleStatusDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub details: Vec<KeyValueDto>,
}

impl From<&hotsas_core::WorkflowModuleStatus> for WorkflowModuleStatusDto {
    fn from(m: &hotsas_core::WorkflowModuleStatus) -> Self {
        Self {
            id: m.id.clone(),
            title: m.title.clone(),
            status: m.status.as_str().to_string(),
            details: m
                .details
                .iter()
                .map(|(k, v)| KeyValueDto {
                    key: k.clone(),
                    value: v.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcInputDto {
    pub topology: String,
    pub vin: String,
    pub vout: String,
    pub iout: String,
    pub switching_frequency: String,
    pub inductor: Option<String>,
    pub output_capacitor: Option<String>,
    pub target_inductor_ripple_percent: Option<f64>,
    pub estimated_efficiency_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcComputedValueDto {
    pub id: String,
    pub label: String,
    pub value: ValueDto,
    pub formula: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcWarningDto {
    pub code: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcSimulationPlanDto {
    pub id: String,
    pub title: String,
    pub profile_type: String,
    pub recommended_stop_time: ValueDto,
    pub recommended_time_step: Option<ValueDto>,
    pub signals: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcCalculationResultDto {
    pub topology: String,
    pub operating_mode: String,
    pub values: Vec<DcdcComputedValueDto>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
    pub warnings: Vec<DcdcWarningDto>,
    pub simulation_plan: Option<DcdcSimulationPlanDto>,
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcTemplateDto {
    pub id: String,
    pub title: String,
    pub topology: String,
    pub description: String,
    pub supported_outputs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDcdcDemoProjectRequestDto {
    pub topology: String,
    pub vin: String,
    pub vout: String,
    pub iout: String,
    pub switching_frequency: String,
    pub inductor: Option<String>,
    pub output_capacitor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcNetlistPreviewRequestDto {
    pub topology: String,
    pub vin: String,
    pub vout: String,
    pub iout: String,
    pub switching_frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcdcMockTransientRequestDto {
    pub topology: String,
    pub vin: String,
    pub vout: String,
    pub iout: String,
    pub switching_frequency: String,
    pub inductor: Option<String>,
    pub output_capacitor: Option<String>,
    pub target_inductor_ripple_percent: Option<f64>,
    pub estimated_efficiency_percent: Option<f64>,
}

impl From<&hotsas_core::DcdcComputedValue> for DcdcComputedValueDto {
    fn from(v: &hotsas_core::DcdcComputedValue) -> Self {
        Self {
            id: v.id.clone(),
            label: v.label.clone(),
            value: ValueDto::from(&v.value),
            formula: v.formula.clone(),
            description: v.description.clone(),
        }
    }
}

impl From<&hotsas_core::DcdcWarning> for DcdcWarningDto {
    fn from(w: &hotsas_core::DcdcWarning) -> Self {
        Self {
            code: w.code.clone(),
            message: w.message.clone(),
            severity: w.severity.to_string(),
        }
    }
}

impl From<&hotsas_core::DcdcSimulationPlan> for DcdcSimulationPlanDto {
    fn from(p: &hotsas_core::DcdcSimulationPlan) -> Self {
        Self {
            id: p.id.clone(),
            title: p.title.clone(),
            profile_type: p.profile_type.clone(),
            recommended_stop_time: ValueDto::from(&p.recommended_stop_time),
            recommended_time_step: p.recommended_time_step.as_ref().map(ValueDto::from),
            signals: p.signals.clone(),
            notes: p.notes.clone(),
        }
    }
}

impl From<&hotsas_core::DcdcCalculationResult> for DcdcCalculationResultDto {
    fn from(r: &hotsas_core::DcdcCalculationResult) -> Self {
        Self {
            topology: r.topology.id().to_string(),
            operating_mode: r.operating_mode.to_string(),
            values: r.values.iter().map(DcdcComputedValueDto::from).collect(),
            assumptions: r.assumptions.clone(),
            limitations: r.limitations.clone(),
            warnings: r.warnings.iter().map(DcdcWarningDto::from).collect(),
            simulation_plan: r.simulation_plan.as_ref().map(DcdcSimulationPlanDto::from),
            template_id: r.template_id.clone(),
        }
    }
}

impl From<&hotsas_core::DcdcTemplateDefinition> for DcdcTemplateDto {
    fn from(t: &hotsas_core::DcdcTemplateDefinition) -> Self {
        Self {
            id: t.id.clone(),
            title: t.title.clone(),
            topology: t.topology.id().to_string(),
            description: t.description.clone(),
            supported_outputs: t.supported_outputs.clone(),
            limitations: t.limitations.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReportRequestDto {
    pub report_id: String,
    pub title: String,
    pub report_type: String,
    pub included_sections: Vec<String>,
    pub export_options: ReportExportOptionsDto,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportExportOptionsDto {
    pub include_source_references: bool,
    pub include_graph_references: bool,
    pub include_assumptions: bool,
    pub max_table_rows: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReportDto {
    pub id: String,
    pub title: String,
    pub report_type: String,
    pub generated_at: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub sections: Vec<ReportSectionDto>,
    pub warnings: Vec<ReportWarningDto>,
    pub assumptions: Vec<String>,
    pub source_references: Vec<ReportSourceReferenceDto>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSectionDto {
    pub kind: String,
    pub title: String,
    pub status: String,
    pub blocks: Vec<ReportContentBlockDto>,
    pub warnings: Vec<ReportWarningDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportContentBlockDto {
    pub block_type: String,
    pub title: Option<String>,
    pub text: Option<String>,
    pub rows: Option<Vec<ReportKeyValueRowDto>>,
    pub columns: Option<Vec<String>>,
    pub data_rows: Option<Vec<Vec<String>>>,
    pub equation: Option<String>,
    pub substituted_values: Option<Vec<ReportKeyValueRowDto>>,
    pub result: Option<String>,
    pub language: Option<String>,
    pub content: Option<String>,
    pub series_names: Option<Vec<String>>,
    pub x_unit: Option<String>,
    pub y_unit: Option<String>,
    pub items: Option<Vec<ReportWarningDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportKeyValueRowDto {
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportWarningDto {
    pub severity: String,
    pub code: String,
    pub message: String,
    pub section_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSourceReferenceDto {
    pub source_id: String,
    pub source_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSectionCapabilityDto {
    pub kind: String,
    pub title: String,
    pub description: String,
    pub default_enabled: bool,
    pub supported_report_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReportExportRequestDto {
    pub report_id: String,
    pub format: String,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReportExportResultDto {
    pub report_id: String,
    pub format: String,
    pub content: String,
    pub output_path: Option<String>,
    pub success: bool,
    pub message: String,
}

// ---------------------------------------------------------------------------
// v2.4 Typed Component Parameter DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentParameterSchemaDto {
    pub category: String,
    pub groups: Vec<ComponentParameterGroupDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentParameterGroupDto {
    pub name: String,
    pub key: String,
    pub parameters: Vec<ComponentParameterDefinitionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentParameterDefinitionDto {
    pub name: String,
    pub key: String,
    pub description: Option<String>,
    pub unit: String,
    pub kind: String,
    pub required: bool,
    pub editable: bool,
    pub value_range: Option<(f64, f64)>,
    pub default_value: Option<ValueDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentParameterIssueDto {
    pub key: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedComponentParametersDto {
    pub component_id: String,
    pub category: String,
    pub bundle: ParameterBundleDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ParameterBundleDto {
    Resistor {
        resistance: ValueDto,
        power_rating: Option<ValueDto>,
    },
    Capacitor {
        capacitance: ValueDto,
        voltage_rating: Option<ValueDto>,
    },
    Inductor {
        inductance: ValueDto,
        current_rating: Option<ValueDto>,
    },
    Diode {
        forward_voltage: Option<ValueDto>,
        reverse_voltage: Option<ValueDto>,
    },
    Bjt {
        vce_max: Option<ValueDto>,
        ic_max: Option<ValueDto>,
    },
    Mosfet {
        vds_max: Option<ValueDto>,
        id_max: Option<ValueDto>,
        rds_on: Option<ValueDto>,
    },
    OpAmp {
        gbw: Option<ValueDto>,
        input_offset_voltage: Option<ValueDto>,
    },
    Regulator {
        output_voltage: Option<ValueDto>,
        max_current: Option<ValueDto>,
    },
    Generic,
}

impl From<&hotsas_core::ComponentParameterSchema> for ComponentParameterSchemaDto {
    fn from(schema: &hotsas_core::ComponentParameterSchema) -> Self {
        Self {
            category: schema.category.clone(),
            groups: schema
                .groups
                .iter()
                .map(ComponentParameterGroupDto::from)
                .collect(),
        }
    }
}

impl From<&hotsas_core::ComponentParameterGroup> for ComponentParameterGroupDto {
    fn from(group: &hotsas_core::ComponentParameterGroup) -> Self {
        Self {
            name: group.name.clone(),
            key: group.key.clone(),
            parameters: group
                .parameters
                .iter()
                .map(ComponentParameterDefinitionDto::from)
                .collect(),
        }
    }
}

impl From<&hotsas_core::ComponentParameterDefinition> for ComponentParameterDefinitionDto {
    fn from(def: &hotsas_core::ComponentParameterDefinition) -> Self {
        Self {
            name: def.name.clone(),
            key: def.key.clone(),
            description: def.description.clone(),
            unit: def.unit.symbol().to_string(),
            kind: format!("{:?}", def.kind),
            required: def.required,
            editable: def.editable,
            value_range: def.value_range,
            default_value: def.default_value.as_ref().map(ValueDto::from),
        }
    }
}

impl From<&hotsas_application::ParameterIssue> for ComponentParameterIssueDto {
    fn from(issue: &hotsas_application::ParameterIssue) -> Self {
        Self {
            key: issue.key.clone(),
            message: issue.message.clone(),
            severity: format!("{:?}", issue.severity),
        }
    }
}

// v2.5 Schematic Editor Hardening DTOs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddComponentRequestDto {
    pub component_kind: String,
    pub component_definition_id: Option<String>,
    pub instance_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub rotation_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveComponentRequestDto {
    pub instance_id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteComponentRequestDto {
    pub instance_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectPinsRequestDto {
    pub from_component_id: String,
    pub from_pin_id: String,
    pub to_component_id: String,
    pub to_pin_id: String,
    pub net_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameNetRequestDto {
    pub net_id: String,
    pub new_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicEditResultDto {
    pub project: ProjectDto,
    pub validation_warnings: Vec<CircuitValidationIssueDto>,
    pub validation_errors: Vec<CircuitValidationIssueDto>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicToolCapabilityDto {
    pub tool_id: String,
    pub label: String,
    pub available: bool,
    pub limitation: Option<String>,
}

// v2.8 Interactive Schematic Editing MVP DTOs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceableComponentDto {
    pub definition_id: String,
    pub name: String,
    pub category: String,
    pub component_kind: String,
    pub has_symbol: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceComponentRequestDto {
    pub component_definition_id: String,
    pub x: f64,
    pub y: f64,
    pub rotation_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinRefDto {
    pub component_id: String,
    pub pin_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectWireRequestDto {
    pub from: PinRefDto,
    pub to: PinRefDto,
    pub net_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWireRequestDto {
    pub wire_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuickParameterRequestDto {
    pub component_id: String,
    pub parameter_id: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicSelectionRequestDto {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicEditableFieldDto {
    pub field_id: String,
    pub label: String,
    pub current_value: String,
    pub editable: bool,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicSelectionDetailsDto {
    pub kind: String,
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub editable_fields: Vec<SchematicEditableFieldDto>,
    pub model_assignment: Option<ComponentModelAssignmentDto>,
    pub model_assignment_origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicEditActionDto {
    pub id: String,
    pub label: String,
    pub timestamp: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoRedoStateDto {
    pub can_undo: bool,
    pub can_redo: bool,
    pub last_action_label: Option<String>,
    pub next_redo_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetlistPreviewDto {
    pub netlist: String,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

// v2.6 Project Persistence DTOs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSessionStateDto {
    pub current_project_id: Option<String>,
    pub current_project_name: Option<String>,
    pub current_project_path: Option<String>,
    pub dirty: bool,
    pub last_saved_at: Option<String>,
    pub last_loaded_at: Option<String>,
    pub last_error: Option<String>,
}

impl From<&hotsas_core::ProjectSessionState> for ProjectSessionStateDto {
    fn from(state: &hotsas_core::ProjectSessionState) -> Self {
        Self {
            current_project_id: state.current_project_id.clone(),
            current_project_name: state.current_project_name.clone(),
            current_project_path: state.current_project_path.clone(),
            dirty: state.dirty,
            last_saved_at: state.last_saved_at.clone(),
            last_loaded_at: state.last_loaded_at.clone(),
            last_error: state.last_error.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProjectEntryDto {
    pub path: String,
    pub display_name: String,
    pub last_opened_at: String,
    pub exists: bool,
}

impl From<&hotsas_core::RecentProjectEntry> for RecentProjectEntryDto {
    fn from(entry: &hotsas_core::RecentProjectEntry) -> Self {
        Self {
            path: entry.path.clone(),
            display_name: entry.display_name.clone(),
            last_opened_at: entry.last_opened_at.clone(),
            exists: entry.exists,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPersistenceWarningDto {
    pub code: String,
    pub message: String,
    pub severity: String,
}

impl From<&hotsas_core::ProjectPersistenceWarning> for ProjectPersistenceWarningDto {
    fn from(w: &hotsas_core::ProjectPersistenceWarning) -> Self {
        Self {
            code: w.code.clone(),
            message: w.message.clone(),
            severity: format!("{:?}", w.severity),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSaveResultDto {
    pub project_id: String,
    pub path: String,
    pub saved_at: String,
    pub warnings: Vec<ProjectPersistenceWarningDto>,
}

impl From<&hotsas_core::ProjectSaveResult> for ProjectSaveResultDto {
    fn from(result: &hotsas_core::ProjectSaveResult) -> Self {
        Self {
            project_id: result.project_id.clone(),
            path: result.path.clone(),
            saved_at: result.saved_at.clone(),
            warnings: result
                .warnings
                .iter()
                .map(ProjectPersistenceWarningDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOpenRequestDto {
    pub path: String,
    pub confirm_discard_unsaved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOpenResultDto {
    pub project: ProjectDto,
    pub path: String,
    pub opened_at: String,
    pub validation_warnings: Vec<ProjectPersistenceWarningDto>,
}

// v2.9 User-Circuit Simulation Workflow DTOs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCircuitSimulationProfileDto {
    pub id: String,
    pub name: String,
    pub analysis_type: String,
    pub engine: String,
    pub probes: Vec<SimulationProbeDto>,
    pub ac: Option<AcSweepSettingsDto>,
    pub transient: Option<TransientSettingsDto>,
    pub op: Option<OperatingPointSettingsDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationProbeDto {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub target: SimulationProbeTargetDto,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationProbeTargetDto {
    pub net_id: Option<String>,
    pub component_id: Option<String>,
    pub pin_id: Option<String>,
    pub positive_net_id: Option<String>,
    pub negative_net_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcSweepSettingsDto {
    pub start_hz: f64,
    pub stop_hz: f64,
    pub points_per_decade: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransientSettingsDto {
    pub step_seconds: f64,
    pub stop_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingPointSettingsDto {
    pub include_node_voltages: bool,
    pub include_branch_currents: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationPreflightResultDto {
    pub can_run: bool,
    pub blocking_errors: Vec<SimulationWorkflowErrorDto>,
    pub warnings: Vec<SimulationWorkflowWarningDto>,
    pub generated_netlist_preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationWorkflowErrorDto {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationWorkflowWarningDto {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCircuitSimulationRunDto {
    pub id: String,
    pub project_id: String,
    pub profile: UserCircuitSimulationProfileDto,
    pub generated_netlist: String,
    pub status: String,
    pub engine_used: String,
    pub warnings: Vec<SimulationWorkflowWarningDto>,
    pub errors: Vec<SimulationWorkflowErrorDto>,
    pub result: Option<UserCircuitSimulationResultDto>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCircuitSimulationResultDto {
    pub summary: Vec<SimulationMeasurementDto>,
    pub series: Vec<SimulationSeriesDto>,
    pub raw_output_excerpt: Option<String>,
    pub netlist_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationMeasurementDto {
    pub name: String,
    pub si_value: f64,
    pub unit: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationSeriesDto {
    pub id: String,
    pub label: String,
    pub x_unit: Option<String>,
    pub y_unit: Option<String>,
    pub points: Vec<SimulationPointDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationPointDto {
    pub x: f64,
    pub y: f64,
}

// ---------------------------------------------------------------------------
// v3.0 Simulation Diagnostics, History & Graph DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgspiceDiagnosticsDto {
    pub availability: NgspiceAvailabilityDto,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub checked_at: String,
    pub warnings: Vec<SimulationDiagnosticMessageDto>,
    pub errors: Vec<SimulationDiagnosticMessageDto>,
}

impl From<&hotsas_core::NgspiceDiagnostics> for NgspiceDiagnosticsDto {
    fn from(d: &hotsas_core::NgspiceDiagnostics) -> Self {
        Self {
            availability: NgspiceAvailabilityDto::from(&d.availability),
            executable_path: d.executable_path.clone(),
            version: d.version.clone(),
            checked_at: d.checked_at.clone(),
            warnings: d
                .warnings
                .iter()
                .map(SimulationDiagnosticMessageDto::from)
                .collect(),
            errors: d
                .errors
                .iter()
                .map(SimulationDiagnosticMessageDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDiagnosticMessageDto {
    pub code: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub related_entity: Option<SimulationDiagnosticEntityRefDto>,
    pub related_model_id: Option<String>,
    pub suggested_fix: Option<String>,
}

impl From<&hotsas_core::SimulationDiagnosticMessage> for SimulationDiagnosticMessageDto {
    fn from(m: &hotsas_core::SimulationDiagnosticMessage) -> Self {
        Self {
            code: m.code.clone(),
            severity: format!("{:?}", m.severity),
            title: m.title.clone(),
            message: m.message.clone(),
            related_entity: m
                .related_entity
                .as_ref()
                .map(SimulationDiagnosticEntityRefDto::from),
            related_model_id: m.related_model_id.clone(),
            suggested_fix: m.suggested_fix.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDiagnosticEntityRefDto {
    pub kind: String,
    pub id: String,
}

impl From<&hotsas_core::SimulationDiagnosticEntityRef> for SimulationDiagnosticEntityRefDto {
    fn from(e: &hotsas_core::SimulationDiagnosticEntityRef) -> Self {
        Self {
            kind: format!("{:?}", e.kind),
            id: e.id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRunHistoryEntryDto {
    pub run_id: String,
    pub profile_id: String,
    pub profile_name: String,
    pub analysis_type: String,
    pub engine_used: String,
    pub status: String,
    pub created_at: String,
    pub warnings_count: usize,
    pub errors_count: usize,
    pub series_count: usize,
    pub measurements_count: usize,
}

impl From<&hotsas_core::SimulationRunHistoryEntry> for SimulationRunHistoryEntryDto {
    fn from(e: &hotsas_core::SimulationRunHistoryEntry) -> Self {
        Self {
            run_id: e.run_id.clone(),
            profile_id: e.profile_id.clone(),
            profile_name: e.profile_name.clone(),
            analysis_type: e.analysis_type.clone(),
            engine_used: e.engine_used.clone(),
            status: e.status.clone(),
            created_at: e.created_at.clone(),
            warnings_count: e.warnings_count,
            errors_count: e.errors_count,
            series_count: e.series_count,
            measurements_count: e.measurements_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationGraphViewDto {
    pub run_id: String,
    pub title: String,
    pub x_axis: SimulationAxisDto,
    pub y_axis: SimulationAxisDto,
    pub series: Vec<SimulationGraphSeriesDto>,
}

impl From<&hotsas_core::SimulationGraphView> for SimulationGraphViewDto {
    fn from(v: &hotsas_core::SimulationGraphView) -> Self {
        Self {
            run_id: v.run_id.clone(),
            title: v.title.clone(),
            x_axis: SimulationAxisDto::from(&v.x_axis),
            y_axis: SimulationAxisDto::from(&v.y_axis),
            series: v
                .series
                .iter()
                .map(SimulationGraphSeriesDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationAxisDto {
    pub label: String,
    pub unit: Option<String>,
    pub scale: String,
}

impl From<&hotsas_core::SimulationAxis> for SimulationAxisDto {
    fn from(a: &hotsas_core::SimulationAxis) -> Self {
        Self {
            label: a.label.clone(),
            unit: a.unit.map(|u| u.symbol().to_string()),
            scale: format!("{:?}", a.scale),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationGraphSeriesDto {
    pub id: String,
    pub label: String,
    pub visible_by_default: bool,
    pub points_count: usize,
}

impl From<&hotsas_core::SimulationGraphSeries> for SimulationGraphSeriesDto {
    fn from(s: &hotsas_core::SimulationGraphSeries) -> Self {
        Self {
            id: s.id.clone(),
            label: s.label.clone(),
            visible_by_default: s.visible_by_default,
            points_count: s.points_count,
        }
    }
}

// ─── v3.1 Component Model Mapping DTOs ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiceModelReferenceDto {
    pub id: String,
    pub display_name: String,
    pub model_kind: String,
    pub source: String,
    pub status: String,
    pub limitations: Vec<String>,
    pub warnings: Vec<String>,
}

impl From<&hotsas_core::SpiceModelReference> for SpiceModelReferenceDto {
    fn from(m: &hotsas_core::SpiceModelReference) -> Self {
        Self {
            id: m.id.clone(),
            display_name: m.display_name.clone(),
            model_kind: spice_model_reference_kind_to_string(m.model_kind),
            source: spice_model_source_to_string(m.source),
            status: component_model_assignment_status_to_string(m.status),
            limitations: m.limitations.clone(),
            warnings: m.warnings.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentPinMappingDto {
    pub component_pin_id: String,
    pub model_pin_name: String,
    pub model_pin_index: Option<usize>,
    pub role: Option<String>,
    pub required: bool,
}

impl From<&hotsas_core::ComponentPinMapping> for ComponentPinMappingDto {
    fn from(m: &hotsas_core::ComponentPinMapping) -> Self {
        Self {
            component_pin_id: m.component_pin_id.clone(),
            model_pin_name: m.model_pin_name.clone(),
            model_pin_index: m.model_pin_index,
            role: m.role.map(component_pin_role_to_string),
            required: m.required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameterBindingDto {
    pub model_parameter_name: String,
    pub component_parameter_id: String,
    pub value_expression: Option<String>,
    pub required: bool,
}

impl From<&hotsas_core::ModelParameterBinding> for ModelParameterBindingDto {
    fn from(m: &hotsas_core::ModelParameterBinding) -> Self {
        Self {
            model_parameter_name: m.model_parameter_name.clone(),
            component_parameter_id: m.component_parameter_id.clone(),
            value_expression: m.value_expression.clone(),
            required: m.required,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReadinessDto {
    pub can_simulate: bool,
    pub can_export_netlist: bool,
    pub uses_placeholder: bool,
    pub blocking_count: usize,
    pub warning_count: usize,
    pub status_label: String,
}

impl From<&hotsas_core::SimulationReadiness> for SimulationReadinessDto {
    fn from(r: &hotsas_core::SimulationReadiness) -> Self {
        Self {
            can_simulate: r.can_simulate,
            can_export_netlist: r.can_export_netlist,
            uses_placeholder: r.uses_placeholder,
            blocking_count: r.blocking_count,
            warning_count: r.warning_count,
            status_label: r.status_label.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMappingDiagnosticDto {
    pub code: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub related_component_id: Option<String>,
    pub related_model_id: Option<String>,
}

impl From<&hotsas_core::ModelMappingDiagnostic> for ModelMappingDiagnosticDto {
    fn from(d: &hotsas_core::ModelMappingDiagnostic) -> Self {
        Self {
            code: d.code.clone(),
            severity: model_mapping_severity_to_string(d.severity),
            title: d.title.clone(),
            message: d.message.clone(),
            suggested_fix: d.suggested_fix.clone(),
            related_component_id: d.related_component_id.clone(),
            related_model_id: d.related_model_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentModelAssignmentDto {
    pub component_definition_id: String,
    pub component_instance_id: Option<String>,
    pub model_ref: Option<SpiceModelReferenceDto>,
    pub pin_mappings: Vec<ComponentPinMappingDto>,
    pub parameter_bindings: Vec<ModelParameterBindingDto>,
    pub status: String,
    pub readiness: SimulationReadinessDto,
    pub diagnostics: Vec<ModelMappingDiagnosticDto>,
}

impl From<&hotsas_core::ComponentModelAssignment> for ComponentModelAssignmentDto {
    fn from(a: &hotsas_core::ComponentModelAssignment) -> Self {
        Self {
            component_definition_id: a.component_definition_id.clone(),
            component_instance_id: a.component_instance_id.clone(),
            model_ref: a.model_ref.as_ref().map(SpiceModelReferenceDto::from),
            pin_mappings: a
                .pin_mappings
                .iter()
                .map(ComponentPinMappingDto::from)
                .collect(),
            parameter_bindings: a
                .parameter_bindings
                .iter()
                .map(ModelParameterBindingDto::from)
                .collect(),
            status: component_model_assignment_status_to_string(a.status),
            readiness: SimulationReadinessDto::from(&a.readiness),
            diagnostics: a
                .diagnostics
                .iter()
                .map(ModelMappingDiagnosticDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSimulationReadinessDto {
    pub project_id: String,
    pub can_simulate: bool,
    pub component_count: usize,
    pub ready_count: usize,
    pub placeholder_count: usize,
    pub missing_count: usize,
    pub invalid_count: usize,
    pub blocking_count: usize,
    pub warning_count: usize,
    pub components: Vec<ComponentModelAssignmentDto>,
}

impl From<&hotsas_core::ProjectSimulationReadiness> for ProjectSimulationReadinessDto {
    fn from(p: &hotsas_core::ProjectSimulationReadiness) -> Self {
        Self {
            project_id: p.project_id.clone(),
            can_simulate: p.can_simulate,
            component_count: p.component_count,
            ready_count: p.ready_count,
            placeholder_count: p.placeholder_count,
            missing_count: p.missing_count,
            invalid_count: p.invalid_count,
            blocking_count: p.blocking_count,
            warning_count: p.warning_count,
            components: p
                .components
                .iter()
                .map(ComponentModelAssignmentDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignModelRequestDto {
    pub instance_id: String,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAvailableModelsResponseDto {
    pub definition_id: String,
    pub models: Vec<SpiceModelReferenceDto>,
}

fn component_model_assignment_status_to_string(
    status: hotsas_core::ComponentModelAssignmentStatus,
) -> String {
    match status {
        hotsas_core::ComponentModelAssignmentStatus::Missing => "missing",
        hotsas_core::ComponentModelAssignmentStatus::Placeholder => "placeholder",
        hotsas_core::ComponentModelAssignmentStatus::AssignedBuiltin => "assigned_builtin",
        hotsas_core::ComponentModelAssignmentStatus::AssignedImported => "assigned_imported",
        hotsas_core::ComponentModelAssignmentStatus::AssignedManual => "assigned_manual",
        hotsas_core::ComponentModelAssignmentStatus::Invalid => "invalid",
    }
    .to_string()
}

fn spice_model_reference_kind_to_string(kind: hotsas_core::SpiceModelReferenceKind) -> String {
    match kind {
        hotsas_core::SpiceModelReferenceKind::PrimitiveModel => "primitive_model",
        hotsas_core::SpiceModelReferenceKind::Subcircuit => "subcircuit",
        hotsas_core::SpiceModelReferenceKind::Behavioral => "behavioral",
        hotsas_core::SpiceModelReferenceKind::Placeholder => "placeholder",
    }
    .to_string()
}

fn spice_model_source_to_string(source: hotsas_core::SpiceModelSource) -> String {
    match source {
        hotsas_core::SpiceModelSource::Builtin => "builtin",
        hotsas_core::SpiceModelSource::ImportedFile => "imported_file",
        hotsas_core::SpiceModelSource::UserAssigned => "user_assigned",
        hotsas_core::SpiceModelSource::GeneratedFallback => "generated_fallback",
        hotsas_core::SpiceModelSource::Unknown => "unknown",
    }
    .to_string()
}

fn component_pin_role_to_string(role: hotsas_core::ComponentPinRole) -> String {
    match role {
        hotsas_core::ComponentPinRole::Positive => "positive",
        hotsas_core::ComponentPinRole::Negative => "negative",
        hotsas_core::ComponentPinRole::Input => "input",
        hotsas_core::ComponentPinRole::Output => "output",
        hotsas_core::ComponentPinRole::SupplyPositive => "supply_positive",
        hotsas_core::ComponentPinRole::SupplyNegative => "supply_negative",
        hotsas_core::ComponentPinRole::Gate => "gate",
        hotsas_core::ComponentPinRole::Drain => "drain",
        hotsas_core::ComponentPinRole::Source => "source",
        hotsas_core::ComponentPinRole::Base => "base",
        hotsas_core::ComponentPinRole::Collector => "collector",
        hotsas_core::ComponentPinRole::Emitter => "emitter",
        hotsas_core::ComponentPinRole::Anode => "anode",
        hotsas_core::ComponentPinRole::Cathode => "cathode",
        hotsas_core::ComponentPinRole::Reference => "reference",
        hotsas_core::ComponentPinRole::Other => "other",
    }
    .to_string()
}

fn model_mapping_severity_to_string(severity: hotsas_core::ModelMappingSeverity) -> String {
    match severity {
        hotsas_core::ModelMappingSeverity::Info => "info",
        hotsas_core::ModelMappingSeverity::Warning => "warning",
        hotsas_core::ModelMappingSeverity::Error => "error",
        hotsas_core::ModelMappingSeverity::Blocking => "blocking",
    }
    .to_string()
}

// Two-Port / Filter Network Analysis DTOs

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAnalysisScopeDto {
    WholeCircuit,
    SelectedRegion,
}

impl From<hotsas_core::FilterAnalysisScope> for FilterAnalysisScopeDto {
    fn from(v: hotsas_core::FilterAnalysisScope) -> Self {
        match v {
            hotsas_core::FilterAnalysisScope::WholeCircuit => Self::WholeCircuit,
            hotsas_core::FilterAnalysisScope::SelectedRegion => Self::SelectedRegion,
        }
    }
}

impl From<FilterAnalysisScopeDto> for hotsas_core::FilterAnalysisScope {
    fn from(v: FilterAnalysisScopeDto) -> Self {
        match v {
            FilterAnalysisScopeDto::WholeCircuit => Self::WholeCircuit,
            FilterAnalysisScopeDto::SelectedRegion => Self::SelectedRegion,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitAnalysisPortDto {
    pub label: String,
    pub positive_net_id: String,
    pub negative_net_id: Option<String>,
    pub reference_node_id: Option<String>,
    pub nominal_impedance_ohm: Option<f64>,
}

impl From<&hotsas_core::CircuitAnalysisPort> for CircuitAnalysisPortDto {
    fn from(p: &hotsas_core::CircuitAnalysisPort) -> Self {
        Self {
            label: p.label.clone(),
            positive_net_id: p.positive_net_id.clone(),
            negative_net_id: p.negative_net_id.clone(),
            reference_node_id: p.reference_node_id.clone(),
            nominal_impedance_ohm: p.nominal_impedance_ohm,
        }
    }
}

impl From<CircuitAnalysisPortDto> for hotsas_core::CircuitAnalysisPort {
    fn from(p: CircuitAnalysisPortDto) -> Self {
        Self {
            label: p.label,
            positive_net_id: p.positive_net_id,
            negative_net_id: p.negative_net_id,
            reference_node_id: p.reference_node_id,
            nominal_impedance_ohm: p.nominal_impedance_ohm,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrequencySweepScaleDto {
    Linear,
    Logarithmic,
}

impl From<hotsas_core::FrequencySweepScale> for FrequencySweepScaleDto {
    fn from(v: hotsas_core::FrequencySweepScale) -> Self {
        match v {
            hotsas_core::FrequencySweepScale::Linear => Self::Linear,
            hotsas_core::FrequencySweepScale::Logarithmic => Self::Logarithmic,
        }
    }
}

impl From<FrequencySweepScaleDto> for hotsas_core::FrequencySweepScale {
    fn from(v: FrequencySweepScaleDto) -> Self {
        match v {
            FrequencySweepScaleDto::Linear => Self::Linear,
            FrequencySweepScaleDto::Logarithmic => Self::Logarithmic,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencySweepSettingsDto {
    pub start_hz: f64,
    pub stop_hz: f64,
    pub points: usize,
    pub points_per_decade: Option<usize>,
    pub scale: FrequencySweepScaleDto,
}

impl From<&hotsas_core::FrequencySweepSettings> for FrequencySweepSettingsDto {
    fn from(s: &hotsas_core::FrequencySweepSettings) -> Self {
        Self {
            start_hz: s.start_hz,
            stop_hz: s.stop_hz,
            points: s.points,
            points_per_decade: s.points_per_decade,
            scale: s.scale.into(),
        }
    }
}

impl From<FrequencySweepSettingsDto> for hotsas_core::FrequencySweepSettings {
    fn from(s: FrequencySweepSettingsDto) -> Self {
        Self {
            start_hz: s.start_hz,
            stop_hz: s.stop_hz,
            points: s.points,
            points_per_decade: s.points_per_decade,
            scale: s.scale.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAnalysisMethodDto {
    Auto,
    TemplateAnalytic,
    Mock,
    Ngspice,
}

impl From<hotsas_core::FilterAnalysisMethod> for FilterAnalysisMethodDto {
    fn from(v: hotsas_core::FilterAnalysisMethod) -> Self {
        match v {
            hotsas_core::FilterAnalysisMethod::Auto => Self::Auto,
            hotsas_core::FilterAnalysisMethod::TemplateAnalytic => Self::TemplateAnalytic,
            hotsas_core::FilterAnalysisMethod::Mock => Self::Mock,
            hotsas_core::FilterAnalysisMethod::Ngspice => Self::Ngspice,
        }
    }
}

impl From<FilterAnalysisMethodDto> for hotsas_core::FilterAnalysisMethod {
    fn from(v: FilterAnalysisMethodDto) -> Self {
        match v {
            FilterAnalysisMethodDto::Auto => Self::Auto,
            FilterAnalysisMethodDto::TemplateAnalytic => Self::TemplateAnalytic,
            FilterAnalysisMethodDto::Mock => Self::Mock,
            FilterAnalysisMethodDto::Ngspice => Self::Ngspice,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectedFilterKindDto {
    LowPass,
    HighPass,
    BandPass,
    BandStop,
    AllPass,
    Unknown,
}

impl From<hotsas_core::DetectedFilterKind> for DetectedFilterKindDto {
    fn from(v: hotsas_core::DetectedFilterKind) -> Self {
        match v {
            hotsas_core::DetectedFilterKind::LowPass => Self::LowPass,
            hotsas_core::DetectedFilterKind::HighPass => Self::HighPass,
            hotsas_core::DetectedFilterKind::BandPass => Self::BandPass,
            hotsas_core::DetectedFilterKind::BandStop => Self::BandStop,
            hotsas_core::DetectedFilterKind::AllPass => Self::AllPass,
            hotsas_core::DetectedFilterKind::Unknown => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterMetricKindDto {
    CutoffFrequency,
    LowerCutoffFrequency,
    UpperCutoffFrequency,
    Bandwidth,
    PeakGain,
    PassbandRipple,
    StopbandAttenuation,
    AttenuationAtFrequency,
    InputImpedance,
    OutputImpedance,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterMetricConfidenceDto {
    Exact,
    Estimated,
    Approximate,
    NotAvailable,
}

impl From<hotsas_core::FilterMetricConfidence> for FilterMetricConfidenceDto {
    fn from(v: hotsas_core::FilterMetricConfidence) -> Self {
        match v {
            hotsas_core::FilterMetricConfidence::Exact => Self::Exact,
            hotsas_core::FilterMetricConfidence::Estimated => Self::Estimated,
            hotsas_core::FilterMetricConfidence::Approximate => Self::Approximate,
            hotsas_core::FilterMetricConfidence::NotAvailable => Self::NotAvailable,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterMetricValueDto {
    pub kind: FilterMetricKindDto,
    pub label: String,
    pub value: Option<f64>,
    pub unit: String,
    pub frequency_hz: Option<f64>,
    pub confidence: FilterMetricConfidenceDto,
    pub note: Option<String>,
}

impl From<&hotsas_core::FilterMetricValue> for FilterMetricValueDto {
    fn from(m: &hotsas_core::FilterMetricValue) -> Self {
        Self {
            kind: match m.kind {
                hotsas_core::FilterMetricKind::CutoffFrequency => {
                    FilterMetricKindDto::CutoffFrequency
                }
                hotsas_core::FilterMetricKind::LowerCutoffFrequency => {
                    FilterMetricKindDto::LowerCutoffFrequency
                }
                hotsas_core::FilterMetricKind::UpperCutoffFrequency => {
                    FilterMetricKindDto::UpperCutoffFrequency
                }
                hotsas_core::FilterMetricKind::Bandwidth => FilterMetricKindDto::Bandwidth,
                hotsas_core::FilterMetricKind::PeakGain => FilterMetricKindDto::PeakGain,
                hotsas_core::FilterMetricKind::PassbandRipple => {
                    FilterMetricKindDto::PassbandRipple
                }
                hotsas_core::FilterMetricKind::StopbandAttenuation => {
                    FilterMetricKindDto::StopbandAttenuation
                }
                hotsas_core::FilterMetricKind::AttenuationAtFrequency => {
                    FilterMetricKindDto::AttenuationAtFrequency
                }
                hotsas_core::FilterMetricKind::InputImpedance => {
                    FilterMetricKindDto::InputImpedance
                }
                hotsas_core::FilterMetricKind::OutputImpedance => {
                    FilterMetricKindDto::OutputImpedance
                }
            },
            label: m.label.clone(),
            value: m.value,
            unit: m.unit.clone(),
            frequency_hz: m.frequency_hz,
            confidence: m.confidence.clone().into(),
            note: m.note.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSweepPointDto {
    pub frequency_hz: f64,
    pub vin_magnitude: Option<f64>,
    pub vout_magnitude: Option<f64>,
    pub transfer_magnitude: Option<f64>,
    pub gain_db: Option<f64>,
    pub attenuation_db: Option<f64>,
    pub phase_deg: Option<f64>,
    pub zin_magnitude_ohm: Option<f64>,
    pub zin_phase_deg: Option<f64>,
    pub zout_magnitude_ohm: Option<f64>,
    pub zout_phase_deg: Option<f64>,
}

impl From<&hotsas_core::FilterSweepPoint> for FilterSweepPointDto {
    fn from(p: &hotsas_core::FilterSweepPoint) -> Self {
        Self {
            frequency_hz: p.frequency_hz,
            vin_magnitude: p.vin_magnitude,
            vout_magnitude: p.vout_magnitude,
            transfer_magnitude: p.transfer_magnitude,
            gain_db: p.gain_db,
            attenuation_db: p.attenuation_db,
            phase_deg: p.phase_deg,
            zin_magnitude_ohm: p.zin_magnitude_ohm,
            zin_phase_deg: p.zin_phase_deg,
            zout_magnitude_ohm: p.zout_magnitude_ohm,
            zout_phase_deg: p.zout_phase_deg,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAnalysisSeverityDto {
    Info,
    Warning,
    Error,
    Blocking,
}

impl From<hotsas_core::FilterAnalysisSeverity> for FilterAnalysisSeverityDto {
    fn from(v: hotsas_core::FilterAnalysisSeverity) -> Self {
        match v {
            hotsas_core::FilterAnalysisSeverity::Info => Self::Info,
            hotsas_core::FilterAnalysisSeverity::Warning => Self::Warning,
            hotsas_core::FilterAnalysisSeverity::Error => Self::Error,
            hotsas_core::FilterAnalysisSeverity::Blocking => Self::Blocking,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterAnalysisDiagnosticDto {
    pub code: String,
    pub severity: FilterAnalysisSeverityDto,
    pub title: String,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub related_component_id: Option<String>,
    pub related_net_id: Option<String>,
    pub related_model_id: Option<String>,
}

impl From<&hotsas_core::FilterAnalysisDiagnostic> for FilterAnalysisDiagnosticDto {
    fn from(d: &hotsas_core::FilterAnalysisDiagnostic) -> Self {
        Self {
            code: d.code.clone(),
            severity: d.severity.clone().into(),
            title: d.title.clone(),
            message: d.message.clone(),
            suggested_fix: d.suggested_fix.clone(),
            related_component_id: d.related_component_id.clone(),
            related_net_id: d.related_net_id.clone(),
            related_model_id: d.related_model_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterNetworkAnalysisRequestDto {
    pub project_id: String,
    pub scope: FilterAnalysisScopeDto,
    pub selected_component_ids: Vec<String>,
    pub input_port: CircuitAnalysisPortDto,
    pub output_port: CircuitAnalysisPortDto,
    pub sweep: FrequencySweepSettingsDto,
    pub method: FilterAnalysisMethodDto,
    pub source_amplitude_v: Option<f64>,
    pub requested_metrics: Vec<FilterMetricKindDto>,
}

impl From<FilterNetworkAnalysisRequestDto> for hotsas_core::FilterNetworkAnalysisRequest {
    fn from(r: FilterNetworkAnalysisRequestDto) -> Self {
        Self {
            project_id: r.project_id,
            scope: r.scope.into(),
            selected_component_ids: r.selected_component_ids,
            input_port: r.input_port.into(),
            output_port: r.output_port.into(),
            sweep: r.sweep.into(),
            method: r.method.into(),
            source_amplitude_v: r.source_amplitude_v,
            requested_metrics: r
                .requested_metrics
                .into_iter()
                .map(|k| match k {
                    FilterMetricKindDto::CutoffFrequency => {
                        hotsas_core::FilterMetricKind::CutoffFrequency
                    }
                    FilterMetricKindDto::LowerCutoffFrequency => {
                        hotsas_core::FilterMetricKind::LowerCutoffFrequency
                    }
                    FilterMetricKindDto::UpperCutoffFrequency => {
                        hotsas_core::FilterMetricKind::UpperCutoffFrequency
                    }
                    FilterMetricKindDto::Bandwidth => hotsas_core::FilterMetricKind::Bandwidth,
                    FilterMetricKindDto::PeakGain => hotsas_core::FilterMetricKind::PeakGain,
                    FilterMetricKindDto::PassbandRipple => {
                        hotsas_core::FilterMetricKind::PassbandRipple
                    }
                    FilterMetricKindDto::StopbandAttenuation => {
                        hotsas_core::FilterMetricKind::StopbandAttenuation
                    }
                    FilterMetricKindDto::AttenuationAtFrequency => {
                        hotsas_core::FilterMetricKind::AttenuationAtFrequency
                    }
                    FilterMetricKindDto::InputImpedance => {
                        hotsas_core::FilterMetricKind::InputImpedance
                    }
                    FilterMetricKindDto::OutputImpedance => {
                        hotsas_core::FilterMetricKind::OutputImpedance
                    }
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterNetworkAnalysisResultDto {
    pub analysis_id: String,
    pub project_id: String,
    pub request: FilterNetworkAnalysisRequestDto,
    pub method_used: FilterAnalysisMethodDto,
    pub detected_filter_kind: DetectedFilterKindDto,
    pub can_trust_as_engineering_estimate: bool,
    pub points: Vec<FilterSweepPointDto>,
    pub metrics: Vec<FilterMetricValueDto>,
    pub diagnostics: Vec<FilterAnalysisDiagnosticDto>,
    pub generated_netlist_preview: Option<String>,
    pub created_at: String,
}

impl From<&hotsas_core::FilterNetworkAnalysisResult> for FilterNetworkAnalysisResultDto {
    fn from(r: &hotsas_core::FilterNetworkAnalysisResult) -> Self {
        Self {
            analysis_id: r.analysis_id.clone(),
            project_id: r.project_id.clone(),
            request: FilterNetworkAnalysisRequestDto {
                project_id: r.request.project_id.clone(),
                scope: r.request.scope.into(),
                selected_component_ids: r.request.selected_component_ids.clone(),
                input_port: CircuitAnalysisPortDto::from(&r.request.input_port),
                output_port: CircuitAnalysisPortDto::from(&r.request.output_port),
                sweep: FrequencySweepSettingsDto::from(&r.request.sweep),
                method: r.request.method.into(),
                source_amplitude_v: r.request.source_amplitude_v,
                requested_metrics: r
                    .request
                    .requested_metrics
                    .iter()
                    .map(|k| match k {
                        hotsas_core::FilterMetricKind::CutoffFrequency => {
                            FilterMetricKindDto::CutoffFrequency
                        }
                        hotsas_core::FilterMetricKind::LowerCutoffFrequency => {
                            FilterMetricKindDto::LowerCutoffFrequency
                        }
                        hotsas_core::FilterMetricKind::UpperCutoffFrequency => {
                            FilterMetricKindDto::UpperCutoffFrequency
                        }
                        hotsas_core::FilterMetricKind::Bandwidth => FilterMetricKindDto::Bandwidth,
                        hotsas_core::FilterMetricKind::PeakGain => FilterMetricKindDto::PeakGain,
                        hotsas_core::FilterMetricKind::PassbandRipple => {
                            FilterMetricKindDto::PassbandRipple
                        }
                        hotsas_core::FilterMetricKind::StopbandAttenuation => {
                            FilterMetricKindDto::StopbandAttenuation
                        }
                        hotsas_core::FilterMetricKind::AttenuationAtFrequency => {
                            FilterMetricKindDto::AttenuationAtFrequency
                        }
                        hotsas_core::FilterMetricKind::InputImpedance => {
                            FilterMetricKindDto::InputImpedance
                        }
                        hotsas_core::FilterMetricKind::OutputImpedance => {
                            FilterMetricKindDto::OutputImpedance
                        }
                    })
                    .collect(),
            },
            method_used: r.method_used.into(),
            detected_filter_kind: r.detected_filter_kind.into(),
            can_trust_as_engineering_estimate: r.can_trust_as_engineering_estimate,
            points: r.points.iter().map(FilterSweepPointDto::from).collect(),
            metrics: r.metrics.iter().map(FilterMetricValueDto::from).collect(),
            diagnostics: r
                .diagnostics
                .iter()
                .map(FilterAnalysisDiagnosticDto::from)
                .collect(),
            generated_netlist_preview: r.generated_netlist_preview.clone(),
            created_at: r.created_at.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// v3.3 S-Parameter / Touchstone Workflow DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SParameterAnalysisSourceDto {
    ImportedTouchstone,
    TwoPortFilterAnalysis,
    ManualDataset,
    SimulatedFoundation,
}

impl From<hotsas_core::SParameterAnalysisSource> for SParameterAnalysisSourceDto {
    fn from(v: hotsas_core::SParameterAnalysisSource) -> Self {
        match v {
            hotsas_core::SParameterAnalysisSource::ImportedTouchstone => Self::ImportedTouchstone,
            hotsas_core::SParameterAnalysisSource::TwoPortFilterAnalysis => {
                Self::TwoPortFilterAnalysis
            }
            hotsas_core::SParameterAnalysisSource::ManualDataset => Self::ManualDataset,
            hotsas_core::SParameterAnalysisSource::SimulatedFoundation => Self::SimulatedFoundation,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexValueDto {
    pub re: f64,
    pub im: f64,
}

impl From<&hotsas_core::ComplexValue> for ComplexValueDto {
    fn from(c: &hotsas_core::ComplexValue) -> Self {
        Self { re: c.re, im: c.im }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SParameterDataPointDto {
    pub frequency_hz: f64,
    pub s11: Option<ComplexValueDto>,
    pub s21: Option<ComplexValueDto>,
    pub s12: Option<ComplexValueDto>,
    pub s22: Option<ComplexValueDto>,
}

impl From<&hotsas_core::SParameterDataPoint> for SParameterDataPointDto {
    fn from(p: &hotsas_core::SParameterDataPoint) -> Self {
        Self {
            frequency_hz: p.frequency_hz,
            s11: p.s11.as_ref().map(ComplexValueDto::from),
            s21: p.s21.as_ref().map(ComplexValueDto::from),
            s12: p.s12.as_ref().map(ComplexValueDto::from),
            s22: p.s22.as_ref().map(ComplexValueDto::from),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SParameterDatasetDto {
    pub id: String,
    pub name: String,
    pub source: SParameterAnalysisSourceDto,
    pub port_count: usize,
    pub reference_impedance_ohm: f64,
    pub frequency_unit: String,
    pub parameter_format: String,
    pub points: Vec<SParameterDataPointDto>,
    pub warnings: Vec<SParameterDiagnosticDto>,
}

impl From<&hotsas_core::SParameterDataset> for SParameterDatasetDto {
    fn from(d: &hotsas_core::SParameterDataset) -> Self {
        Self {
            id: d.id.clone(),
            name: d.name.clone(),
            source: d.source.clone().into(),
            port_count: d.port_count,
            reference_impedance_ohm: d.reference_impedance_ohm,
            frequency_unit: d.frequency_unit.clone(),
            parameter_format: d.parameter_format.clone(),
            points: d.points.iter().map(SParameterDataPointDto::from).collect(),
            warnings: d
                .warnings
                .iter()
                .map(SParameterDiagnosticDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SParameterCurvePointDto {
    pub frequency_hz: f64,
    pub s11_db: Option<f64>,
    pub s21_db: Option<f64>,
    pub s12_db: Option<f64>,
    pub s22_db: Option<f64>,
    pub s11_phase_deg: Option<f64>,
    pub s21_phase_deg: Option<f64>,
    pub s12_phase_deg: Option<f64>,
    pub s22_phase_deg: Option<f64>,
    pub return_loss_s11_db: Option<f64>,
    pub return_loss_s22_db: Option<f64>,
    pub insertion_loss_s21_db: Option<f64>,
    pub vswr_s11: Option<f64>,
    pub vswr_s22: Option<f64>,
}

impl From<&hotsas_core::SParameterCurvePoint> for SParameterCurvePointDto {
    fn from(p: &hotsas_core::SParameterCurvePoint) -> Self {
        Self {
            frequency_hz: p.frequency_hz,
            s11_db: p.s11_db,
            s21_db: p.s21_db,
            s12_db: p.s12_db,
            s22_db: p.s22_db,
            s11_phase_deg: p.s11_phase_deg,
            s21_phase_deg: p.s21_phase_deg,
            s12_phase_deg: p.s12_phase_deg,
            s22_phase_deg: p.s22_phase_deg,
            return_loss_s11_db: p.return_loss_s11_db,
            return_loss_s22_db: p.return_loss_s22_db,
            insertion_loss_s21_db: p.insertion_loss_s21_db,
            vswr_s11: p.vswr_s11,
            vswr_s22: p.vswr_s22,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SParameterMetricConfidenceDto {
    High,
    Medium,
    Low,
}

impl From<hotsas_core::SParameterMetricConfidence> for SParameterMetricConfidenceDto {
    fn from(v: hotsas_core::SParameterMetricConfidence) -> Self {
        match v {
            hotsas_core::SParameterMetricConfidence::High => Self::High,
            hotsas_core::SParameterMetricConfidence::Medium => Self::Medium,
            hotsas_core::SParameterMetricConfidence::Low => Self::Low,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SParameterMetricDto {
    pub id: String,
    pub label: String,
    pub value: f64,
    pub unit: String,
    pub frequency_hz: Option<f64>,
    pub confidence: SParameterMetricConfidenceDto,
    pub notes: Vec<String>,
}

impl From<&hotsas_core::SParameterMetric> for SParameterMetricDto {
    fn from(m: &hotsas_core::SParameterMetric) -> Self {
        Self {
            id: m.id.clone(),
            label: m.label.clone(),
            value: m.value,
            unit: m.unit.clone(),
            frequency_hz: m.frequency_hz,
            confidence: m.confidence.clone().into(),
            notes: m.notes.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SParameterSeverityDto {
    Info,
    Warning,
    Error,
    Blocking,
}

impl From<hotsas_core::SParameterSeverity> for SParameterSeverityDto {
    fn from(v: hotsas_core::SParameterSeverity) -> Self {
        match v {
            hotsas_core::SParameterSeverity::Info => Self::Info,
            hotsas_core::SParameterSeverity::Warning => Self::Warning,
            hotsas_core::SParameterSeverity::Error => Self::Error,
            hotsas_core::SParameterSeverity::Blocking => Self::Blocking,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SParameterDiagnosticDto {
    pub code: String,
    pub severity: SParameterSeverityDto,
    pub title: String,
    pub message: String,
    pub suggested_fix: Option<String>,
}

impl From<&hotsas_core::SParameterDiagnostic> for SParameterDiagnosticDto {
    fn from(d: &hotsas_core::SParameterDiagnostic) -> Self {
        Self {
            code: d.code.clone(),
            severity: d.severity.clone().into(),
            title: d.title.clone(),
            message: d.message.clone(),
            suggested_fix: d.suggested_fix.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SParameterAnalysisResultDto {
    pub id: String,
    pub dataset: SParameterDatasetDto,
    pub curve_points: Vec<SParameterCurvePointDto>,
    pub metrics: Vec<SParameterMetricDto>,
    pub diagnostics: Vec<SParameterDiagnosticDto>,
    pub can_plot_s11: bool,
    pub can_plot_s21: bool,
    pub can_plot_s12: bool,
    pub can_plot_s22: bool,
    pub summary: String,
}

impl From<&hotsas_core::SParameterAnalysisResult> for SParameterAnalysisResultDto {
    fn from(r: &hotsas_core::SParameterAnalysisResult) -> Self {
        Self {
            id: r.id.clone(),
            dataset: SParameterDatasetDto::from(&r.dataset),
            curve_points: r
                .curve_points
                .iter()
                .map(SParameterCurvePointDto::from)
                .collect(),
            metrics: r.metrics.iter().map(SParameterMetricDto::from).collect(),
            diagnostics: r
                .diagnostics
                .iter()
                .map(SParameterDiagnosticDto::from)
                .collect(),
            can_plot_s11: r.can_plot_s11,
            can_plot_s21: r.can_plot_s21,
            can_plot_s12: r.can_plot_s12,
            can_plot_s22: r.can_plot_s22,
            summary: r.summary.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeTouchstoneRequestDto {
    pub source_name: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSParameterCsvRequestDto {
    pub result_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSParameterAnalysisToReportRequestDto {
    pub result_id: String,
}
