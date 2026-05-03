use crate::{
    ApiError, CircuitValidationReportDto, ComponentParameterDto, FormulaCalculationRequestDto,
    FormulaDetailsDto, FormulaEvaluationResultDto, FormulaOutputValueDto, FormulaPackDto,
    FormulaResultDto, FormulaSummaryDto, PreferredValueDto, ProjectDto, ProjectPackageManifestDto,
    ProjectPackageValidationReportDto, SaveProjectDto, SelectedComponentDto, SimulationResultDto,
    SymbolDto, ValueDto, VerticalSliceDto,
};
use hotsas_application::{AppServices, FormulaRegistryService};
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, EngineeringUnit, FormulaDefinition, FormulaPack,
    ValueWithUnit,
};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Mutex;

pub struct HotSasApi {
    services: AppServices,
    current_project: Mutex<Option<CircuitProject>>,
    formula_registry: Mutex<FormulaRegistryService>,
}

impl HotSasApi {
    pub fn new(services: AppServices) -> Self {
        Self {
            services,
            current_project: Mutex::new(None),
            formula_registry: Mutex::new(fallback_formula_registry()),
        }
    }

    pub fn load_formula_packs(
        &self,
        packs: Vec<FormulaPack>,
    ) -> Result<Vec<FormulaPackDto>, ApiError> {
        let registry = FormulaRegistryService::new(packs)?;
        let metadata = registry
            .get_pack_metadata()
            .iter()
            .map(FormulaPackDto::from)
            .collect();
        let mut guard = self
            .formula_registry
            .lock()
            .map_err(|_| ApiError::State("formula registry lock poisoned".to_string()))?;
        *guard = registry;
        Ok(metadata)
    }

    pub fn list_formulas(&self) -> Result<Vec<FormulaSummaryDto>, ApiError> {
        let registry = self.formula_registry()?;
        Ok(registry
            .list_formulas()
            .iter()
            .map(FormulaSummaryDto::from)
            .collect())
    }

    pub fn list_formula_categories(&self) -> Result<Vec<String>, ApiError> {
        Ok(self.formula_registry()?.list_categories())
    }

    pub fn get_formula(&self, id: String) -> Result<FormulaDetailsDto, ApiError> {
        let registry = self.formula_registry()?;
        Ok(FormulaDetailsDto::from(&registry.get_formula(&id)?))
    }

    pub fn get_formula_pack_metadata(&self) -> Result<Vec<FormulaPackDto>, ApiError> {
        let registry = self.formula_registry()?;
        Ok(registry
            .get_pack_metadata()
            .iter()
            .map(FormulaPackDto::from)
            .collect())
    }

    pub fn calculate_formula(
        &self,
        request: FormulaCalculationRequestDto,
    ) -> Result<FormulaEvaluationResultDto, ApiError> {
        let registry = self.formula_registry()?;
        let formula = registry.get_formula(&request.formula_id)?;
        let variables = Self::parse_formula_variables(&formula, request.variables)?;
        let result = self
            .services
            .formula_service()
            .calculate_formula_from_definition(&formula, variables)?;

        Ok(FormulaEvaluationResultDto {
            formula_id: result.formula_id,
            equation_id: result.equation_id,
            expression: result.expression,
            outputs: result
                .outputs
                .iter()
                .map(|(name, value)| FormulaOutputValueDto {
                    name: name.clone(),
                    value: ValueDto::from(value),
                })
                .collect(),
            warnings: result.warnings,
        })
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

    pub fn save_project_package(
        &self,
        package_dir: String,
    ) -> Result<ProjectPackageManifestDto, ApiError> {
        let project = self.current_project()?;
        let manifest = self
            .services
            .save_project_package(Path::new(&package_dir), &project)?;
        Ok(ProjectPackageManifestDto::from(&manifest))
    }

    pub fn load_project_package(&self, package_dir: String) -> Result<ProjectDto, ApiError> {
        let project = self
            .services
            .load_project_package(Path::new(&package_dir))?;
        self.replace_current_project(project.clone())?;
        Ok(ProjectDto::from(&project))
    }

    pub fn validate_project_package(
        &self,
        package_dir: String,
    ) -> Result<ProjectPackageValidationReportDto, ApiError> {
        let report = self
            .services
            .validate_project_package(Path::new(&package_dir))?;
        Ok(ProjectPackageValidationReportDto::from(&report))
    }

    pub fn get_selected_component(
        &self,
        instance_id: String,
    ) -> Result<SelectedComponentDto, ApiError> {
        let project = self.current_project()?;
        let component = project
            .schematic
            .components
            .iter()
            .find(|c| c.instance_id == instance_id)
            .ok_or_else(|| {
                ApiError::InvalidInput(format!("component '{}' not found", instance_id))
            })?;
        let symbol = hotsas_core::seed_symbol_for_kind(&component.definition_id);
        let parameters: Vec<ComponentParameterDto> = component
            .overridden_parameters
            .iter()
            .map(|(name, value)| ComponentParameterDto {
                name: name.clone(),
                value: value.value.original.clone(),
                unit: Some(value.unit.symbol().to_string()),
            })
            .collect();
        Ok(SelectedComponentDto {
            instance_id: component.instance_id.clone(),
            component_kind: component.definition_id.clone(),
            title: symbol
                .as_ref()
                .map(|s| s.title.clone())
                .unwrap_or_else(|| component.definition_id.clone()),
            parameters,
            symbol: symbol.as_ref().map(SymbolDto::from),
        })
    }

    pub fn update_component_parameter(
        &self,
        instance_id: String,
        parameter_name: String,
        value: String,
        unit: Option<String>,
    ) -> Result<ProjectDto, ApiError> {
        let mut project = self.current_project()?;
        let component = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == instance_id)
            .ok_or_else(|| {
                ApiError::InvalidInput(format!("component '{}' not found", instance_id))
            })?;
        let engineering_unit = match unit.as_deref() {
            Some("Ohm") => EngineeringUnit::Ohm,
            Some("F") => EngineeringUnit::Farad,
            Some("Hz") => EngineeringUnit::Hertz,
            Some("V") => EngineeringUnit::Volt,
            Some("") | None => EngineeringUnit::Unitless,
            Some(other) => {
                return Err(ApiError::InvalidInput(format!("unsupported unit: {other}")))
            }
        };
        let parsed = ValueWithUnit::parse_with_default(&value, engineering_unit)?;
        component
            .overridden_parameters
            .insert(parameter_name, parsed);
        self.replace_current_project(project.clone())?;
        Ok(ProjectDto::from(&project))
    }

    pub fn validate_current_circuit(&self) -> Result<CircuitValidationReportDto, ApiError> {
        let project = self.current_project()?;
        let report = self.services.validate_circuit(&project);
        Ok(CircuitValidationReportDto::from(&report))
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

    fn parse_formula_variables(
        formula: &FormulaDefinition,
        inputs: Vec<crate::FormulaVariableInputDto>,
    ) -> Result<BTreeMap<String, ValueWithUnit>, ApiError> {
        let inputs_by_name = inputs
            .into_iter()
            .map(|input| (input.name.clone(), input))
            .collect::<BTreeMap<_, _>>();
        let mut variables = BTreeMap::new();

        for (name, variable) in &formula.variables {
            let input = inputs_by_name
                .get(name)
                .ok_or_else(|| ApiError::InvalidInput(format!("missing variable: {name}")))?;
            let unit = match input.unit.as_deref() {
                Some(unit) => EngineeringUnit::parse(unit)?,
                None => variable.unit,
            };
            variables.insert(
                name.clone(),
                ValueWithUnit::parse_with_default(&input.value, unit)?,
            );
        }

        for name in inputs_by_name.keys() {
            if !formula.variables.contains_key(name) {
                return Err(ApiError::InvalidInput(format!(
                    "unknown variable for formula {}: {name}",
                    formula.id
                )));
            }
        }

        Ok(variables)
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

    fn formula_registry(&self) -> Result<FormulaRegistryService, ApiError> {
        self.formula_registry
            .lock()
            .map_err(|_| ApiError::State("formula registry lock poisoned".to_string()))
            .map(|guard| guard.clone())
    }
}

fn fallback_formula_registry() -> FormulaRegistryService {
    FormulaRegistryService::new(vec![FormulaPack {
        pack_id: "fallback_filters".to_string(),
        title: "Fallback Filters".to_string(),
        version: "0.1.0".to_string(),
        formulas: vec![rc_low_pass_formula()],
    }])
    .expect("fallback formula registry must be valid")
}
