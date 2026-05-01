use crate::{
    ApiError, FormulaResultDto, PreferredValueDto, ProjectDto, SaveProjectDto, SimulationResultDto,
    ValueDto, VerticalSliceDto,
};
use hotsas_application::AppServices;
use hotsas_core::{CircuitProject, EngineeringUnit, ValueWithUnit};
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
        Ok(Self::formula_result_dto(&result))
    }

    pub fn nearest_e24_for_resistor(&self) -> Result<PreferredValueDto, ApiError> {
        let project = self.current_project()?;
        Ok(PreferredValueDto::from(
            &self.services.nearest_e24_for_resistor(&project)?,
        ))
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
        let nearest = self.services.nearest_e24_for_resistor(&project)?;
        let netlist = self.services.generate_spice_netlist(&project)?;
        let simulation = self.services.run_mock_ac_simulation(&project)?;
        let report =
            self.services
                .build_report_model(&project, &cutoff, &nearest, &netlist, &simulation);
        let markdown_report = self.services.export_markdown_report(&report)?;
        let html_report = self.services.export_html_report(&report)?;

        Ok(VerticalSliceDto {
            project: ProjectDto::from(&project),
            cutoff_frequency: Self::formula_result_dto(&cutoff),
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
        let nearest = self.services.nearest_e24_for_resistor(&project)?;
        let netlist = self.services.generate_spice_netlist(&project)?;
        let simulation = self.services.run_mock_ac_simulation(&project)?;
        Ok(self
            .services
            .build_report_model(&project, &cutoff, &nearest, &netlist, &simulation))
    }

    fn formula_result_dto(result: &ValueWithUnit) -> FormulaResultDto {
        FormulaResultDto {
            formula_id: "rc_low_pass_cutoff".to_string(),
            output_name: "fc".to_string(),
            value: ValueDto::from(result),
            expression: "fc = 1 / (2*pi*R*C)".to_string(),
        }
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
