use hotsas_application::{component_parameter, AppServices, ApplicationError};
use hotsas_core::{
    CircuitProject, EngineeringUnit, GraphSeries, PreferredValueResult, SimulationResult,
    ValueWithUnit,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use std::sync::Mutex;

pub struct HotSasApi {
    services: AppServices,
    current_project: Mutex<Option<CircuitProject>>,
}

impl HotSasApi {
    pub fn new(services: AppServices) -> Self {
        Self {
            services,
            current_project: Mutex::new(None),
        }
    }

    pub fn create_rc_low_pass_demo_project(&self) -> Result<ProjectDto, ApiError> {
        let project = self.services.create_rc_low_pass_demo_project();
        self.replace_current_project(project.clone())?;
        Ok(ProjectDto::from(&project))
    }

    pub fn calculate_rc_low_pass(&self) -> Result<FormulaResultDto, ApiError> {
        let project = self.current_project()?;
        let result = self.services.calculate_rc_low_pass_cutoff(&project)?;
        Ok(FormulaResultDto {
            formula_id: "rc_low_pass_cutoff".to_string(),
            output_name: "fc".to_string(),
            value: ValueDto::from(&result),
            expression: "fc = 1 / (2*pi*R*C)".to_string(),
        })
    }

    pub fn nearest_e24_for_resistor(&self) -> Result<PreferredValueDto, ApiError> {
        let project = self.current_project()?;
        let resistance = component_parameter(&project, "R1", "resistance")?;
        let result = self.services.nearest_e24(resistance)?;
        Ok(PreferredValueDto::from(&result))
    }

    pub fn nearest_e24(
        &self,
        value: String,
        unit: Option<String>,
    ) -> Result<PreferredValueDto, ApiError> {
        let unit = match unit.as_deref() {
            Some("Ohm") => EngineeringUnit::Ohm,
            Some("F") => EngineeringUnit::Farad,
            Some("Hz") => EngineeringUnit::Hertz,
            Some("") | None => EngineeringUnit::Unitless,
            Some(other) => {
                return Err(ApiError::InvalidInput(format!("unsupported unit: {other}")))
            }
        };
        let parsed = ValueWithUnit::parse_with_default(&value, unit)?;
        Ok(PreferredValueDto::from(&self.services.nearest_e24(parsed)?))
    }

    pub fn generate_spice_netlist(&self) -> Result<String, ApiError> {
        let project = self.current_project()?;
        Ok(self.services.generate_spice_netlist(&project)?)
    }

    pub fn run_mock_ac_simulation(&self) -> Result<SimulationResultDto, ApiError> {
        let project = self.current_project()?;
        let result = self.services.run_mock_ac_simulation(&project)?;
        Ok(SimulationResultDto::from(&result))
    }

    pub fn export_markdown_report(&self) -> Result<String, ApiError> {
        let report = self.current_report_model()?;
        Ok(self.services.export_markdown_report(&report)?)
    }

    pub fn export_html_report(&self) -> Result<String, ApiError> {
        let report = self.current_report_model()?;
        Ok(self.services.export_html_report(&report)?)
    }

    pub fn save_project_json(&self, path: String) -> Result<SaveProjectDto, ApiError> {
        let project = self.current_project()?;
        self.services.save_project(Path::new(&path), &project)?;
        Ok(SaveProjectDto { path })
    }

    pub fn run_vertical_slice_preview(&self) -> Result<VerticalSliceDto, ApiError> {
        let project = self.services.create_rc_low_pass_demo_project();
        self.replace_current_project(project.clone())?;
        let cutoff = self.services.calculate_rc_low_pass_cutoff(&project)?;
        let resistance = component_parameter(&project, "R1", "resistance")?;
        let nearest = self.services.nearest_e24(resistance)?;
        let netlist = self.services.generate_spice_netlist(&project)?;
        let simulation = self.services.run_mock_ac_simulation(&project)?;
        let report =
            self.services
                .build_report_model(&project, &cutoff, &nearest, &netlist, &simulation);
        let markdown_report = self.services.export_markdown_report(&report)?;
        let html_report = self.services.export_html_report(&report)?;

        Ok(VerticalSliceDto {
            project: ProjectDto::from(&project),
            cutoff_frequency: FormulaResultDto {
                formula_id: "rc_low_pass_cutoff".to_string(),
                output_name: "fc".to_string(),
                value: ValueDto::from(&cutoff),
                expression: "fc = 1 / (2*pi*R*C)".to_string(),
            },
            nearest_e24: PreferredValueDto::from(&nearest),
            spice_netlist: netlist,
            simulation: SimulationResultDto::from(&simulation),
            markdown_report,
            html_report,
        })
    }

    fn current_report_model(&self) -> Result<hotsas_core::ReportModel, ApiError> {
        let project = self.current_project()?;
        let cutoff = self.services.calculate_rc_low_pass_cutoff(&project)?;
        let resistance = component_parameter(&project, "R1", "resistance")?;
        let nearest = self.services.nearest_e24(resistance)?;
        let netlist = self.services.generate_spice_netlist(&project)?;
        let simulation = self.services.run_mock_ac_simulation(&project)?;
        Ok(self
            .services
            .build_report_model(&project, &cutoff, &nearest, &netlist, &simulation))
    }

    fn replace_current_project(&self, project: CircuitProject) -> Result<(), ApiError> {
        let mut guard = self
            .current_project
            .lock()
            .map_err(|_| ApiError::State("current project lock poisoned".to_string()))?;
        *guard = Some(project);
        Ok(())
    }

    fn current_project(&self) -> Result<CircuitProject, ApiError> {
        self.current_project
            .lock()
            .map_err(|_| ApiError::State("current project lock poisoned".to_string()))?
            .clone()
            .ok_or_else(|| ApiError::State("create or open a project first".to_string()))
    }
}

#[derive(Debug)]
pub enum ApiError {
    Application(ApplicationError),
    Core(hotsas_core::CoreError),
    InvalidInput(String),
    State(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Application(error) => write!(f, "{error}"),
            Self::Core(error) => write!(f, "{error}"),
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::State(message) => write!(f, "state error: {message}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<ApplicationError> for ApiError {
    fn from(value: ApplicationError) -> Self {
        Self::Application(value)
    }
}

impl From<hotsas_core::CoreError> for ApiError {
    fn from(value: hotsas_core::CoreError) -> Self {
        Self::Core(value)
    }
}

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
