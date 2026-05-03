use hotsas_core::{
    CircuitProject, EngineeringNotebook, FormulaDefinition, FormulaEquation, FormulaOutput,
    FormulaPackMetadata, FormulaVariable, GraphSeries, NotebookEvaluationResult,
    NotebookHistoryEntry, PreferredValueResult, ProjectPackageManifest,
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
pub struct FormulaDetailsDto {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub variables: Vec<FormulaVariableDto>,
    pub equations: Vec<FormulaEquationDto>,
    pub outputs: Vec<FormulaOutputDto>,
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
