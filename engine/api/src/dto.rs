use hotsas_core::{
    CircuitProject, GraphSeries, PreferredValueResult, SimulationResult, ValueWithUnit,
};
use serde::{Deserialize, Serialize};

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
                    .map(|component| ComponentDto {
                        instance_id: component.instance_id.clone(),
                        definition_id: component.definition_id.clone(),
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
    pub x: f64,
    pub y: f64,
    pub rotation_degrees: f64,
    pub parameters: Vec<ParameterDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDto {
    pub name: String,
    pub value: ValueDto,
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
