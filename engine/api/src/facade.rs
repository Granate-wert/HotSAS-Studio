use crate::{
    ApiError, ApplyNotebookValueRequestDto, AssignComponentRequestDto, CircuitValidationReportDto,
    ComponentDetailsDto, ComponentLibraryDto, ComponentParameterDto, ComponentSearchRequestDto,
    ComponentSearchResultDto, ComponentSummaryDto, FootprintDto, FormulaCalculationRequestDto,
    FormulaDetailsDto, FormulaEvaluationResultDto, FormulaOutputValueDto, FormulaPackDto,
    FormulaResultDto, FormulaSummaryDto, KeyValueDto, NotebookEvaluationRequestDto,
    NotebookEvaluationResultDto, NotebookStateDto, PreferredValueDto, ProjectDto,
    ProjectPackageManifestDto, ProjectPackageValidationReportDto, SaveProjectDto,
    SelectedComponentDto, SimulationModelDto, SimulationResultDto, SymbolDto, ValueDto,
    VerticalSliceDto,
};
use hotsas_application::{AppServices, FormulaRegistryService};
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, ComponentAssignment, ComponentLibrary,
    ComponentLibraryQuery, EngineeringNotebook, EngineeringUnit, FormulaDefinition, FormulaPack,
    NotebookBlock, NotebookEvaluationStatus, NotebookHistoryEntry, ValueWithUnit,
};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Mutex;

pub struct HotSasApi {
    services: AppServices,
    current_project: Mutex<Option<CircuitProject>>,
    formula_registry: Mutex<FormulaRegistryService>,
    notebook: Mutex<EngineeringNotebook>,
    component_library: Mutex<ComponentLibrary>,
}

impl HotSasApi {
    pub fn new(services: AppServices) -> Self {
        let library = services
            .component_library_service()
            .load_builtin_library()
            .unwrap_or_else(|_| ComponentLibrary {
                id: "fallback".to_string(),
                title: "Fallback Library".to_string(),
                version: "0.0.0".to_string(),
                components: vec![],
                symbols: vec![],
                footprints: vec![],
                simulation_models: vec![],
                metadata: std::collections::BTreeMap::new(),
            });
        Self {
            services,
            current_project: Mutex::new(None),
            formula_registry: Mutex::new(fallback_formula_registry()),
            notebook: Mutex::new(EngineeringNotebook::new("default", "Engineering Notebook")),
            component_library: Mutex::new(library),
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

    pub fn evaluate_notebook_input(
        &self,
        request: NotebookEvaluationRequestDto,
    ) -> Result<NotebookEvaluationResultDto, ApiError> {
        let registry = self.formula_registry()?;
        let mut notebook = self
            .notebook
            .lock()
            .map_err(|_| ApiError::State("notebook lock poisoned".to_string()))?;
        let result = self
            .services
            .engineering_notebook_service()
            .evaluate_input(
                &request.input,
                &notebook.variables,
                &registry,
                self.services.preferred_values_service(),
                self.services.formula_service(),
            )?;
        if !result.variables.is_empty() {
            notebook.variables = result.variables.clone();
        }
        let summary = if result.outputs.is_empty() {
            result.message.clone().unwrap_or_default()
        } else {
            result
                .outputs
                .iter()
                .map(|(name, value)| format!("{}={}", name, value.original()))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let hist_id = format!("hist-{}", notebook.history.len());
        notebook.history.push(NotebookHistoryEntry {
            id: hist_id,
            input: request.input.clone(),
            result_summary: summary,
            status: result.status.clone(),
        });
        let block_id = format!("blk-{}", notebook.blocks.len());
        notebook.blocks.push(NotebookBlock {
            id: block_id,
            kind: result.kind.clone(),
            input: request.input.clone(),
            result: Some(result.clone()),
            created_at: None,
            updated_at: None,
        });
        Ok(NotebookEvaluationResultDto::from(&result))
    }

    pub fn get_notebook_state(&self) -> Result<NotebookStateDto, ApiError> {
        let notebook = self
            .notebook
            .lock()
            .map_err(|_| ApiError::State("notebook lock poisoned".to_string()))?;
        Ok(NotebookStateDto::from(&*notebook))
    }

    pub fn clear_notebook(&self) -> Result<NotebookStateDto, ApiError> {
        let mut notebook = self
            .notebook
            .lock()
            .map_err(|_| ApiError::State("notebook lock poisoned".to_string()))?;
        *notebook = EngineeringNotebook::new("default", "Engineering Notebook");
        Ok(NotebookStateDto::from(&*notebook))
    }

    pub fn apply_notebook_output_to_component(
        &self,
        request: ApplyNotebookValueRequestDto,
    ) -> Result<ProjectDto, ApiError> {
        let mut project = self.current_project()?;
        let notebook = self
            .notebook
            .lock()
            .map_err(|_| ApiError::State("notebook lock poisoned".to_string()))?;
        let last_result = notebook
            .history
            .iter()
            .filter(|h| h.status == NotebookEvaluationStatus::Success)
            .last()
            .and_then(|h| {
                notebook
                    .blocks
                    .iter()
                    .find(|b| b.input == h.input)
                    .and_then(|b| b.result.clone())
            });
        let value = last_result
            .and_then(|r| r.outputs.get(&request.output_name).cloned())
            .ok_or_else(|| {
                ApiError::InvalidInput(format!(
                    "output '{}' not found in last notebook result",
                    request.output_name
                ))
            })?;
        self.services
            .engineering_notebook_service()
            .apply_result_to_component(
                &mut project,
                &request.instance_id,
                &request.parameter_name,
                value,
            )?;
        self.replace_current_project(project.clone())?;
        Ok(ProjectDto::from(&project))
    }

    pub fn load_builtin_component_library(&self) -> Result<ComponentLibraryDto, ApiError> {
        let library = self
            .component_library
            .lock()
            .map_err(|_| ApiError::State("component library lock poisoned".to_string()))?;
        Ok(ComponentLibraryDto {
            id: library.id.clone(),
            title: library.title.clone(),
            version: library.version.clone(),
            components: library
                .components
                .iter()
                .map(ComponentSummaryDto::from)
                .collect(),
            categories: library
                .components
                .iter()
                .map(|c| c.category.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect(),
            tags: library
                .components
                .iter()
                .flat_map(|c| c.tags.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect(),
        })
    }

    pub fn list_components(&self) -> Result<Vec<ComponentSummaryDto>, ApiError> {
        let library = self
            .component_library
            .lock()
            .map_err(|_| ApiError::State("component library lock poisoned".to_string()))?;
        Ok(library
            .components
            .iter()
            .map(ComponentSummaryDto::from)
            .collect())
    }

    pub fn search_components(
        &self,
        request: ComponentSearchRequestDto,
    ) -> Result<ComponentSearchResultDto, ApiError> {
        let library = self
            .component_library
            .lock()
            .map_err(|_| ApiError::State("component library lock poisoned".to_string()))?;
        let query = ComponentLibraryQuery {
            search: request.search,
            category: request.category,
            tags: request.tags,
            manufacturer: request.manufacturer,
            has_symbol: request.has_symbol,
            has_footprint: request.has_footprint,
            has_simulation_model: request.has_simulation_model,
        };
        let result = self
            .services
            .component_library_service()
            .search_components(&library, query);
        Ok(ComponentSearchResultDto {
            components: result
                .components
                .iter()
                .map(ComponentSummaryDto::from)
                .collect(),
            total_count: result.total_count,
            categories: result.categories,
            tags: result.tags,
        })
    }

    pub fn get_component_details(
        &self,
        component_id: String,
    ) -> Result<ComponentDetailsDto, ApiError> {
        let library = self
            .component_library
            .lock()
            .map_err(|_| ApiError::State("component library lock poisoned".to_string()))?;
        let component = self
            .services
            .component_library_service()
            .get_component(&library, &component_id)?;
        let symbol_preview = self
            .services
            .component_library_service()
            .get_symbol_for_component(&library, &component_id)
            .map(|s| SymbolDto::from(&s));
        let footprint_previews = self
            .services
            .component_library_service()
            .get_footprints_for_component(&library, &component_id)
            .iter()
            .map(FootprintDto::from)
            .collect();
        Ok(ComponentDetailsDto {
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
            symbol_preview,
            footprint_previews,
        })
    }

    pub fn assign_component_to_selected_instance(
        &self,
        request: AssignComponentRequestDto,
    ) -> Result<ProjectDto, ApiError> {
        let mut project = self.current_project()?;
        let library = self
            .component_library
            .lock()
            .map_err(|_| ApiError::State("component library lock poisoned".to_string()))?;
        // Verify component exists
        self.services
            .component_library_service()
            .get_component(&library, &request.component_definition_id)?;
        self.services
            .component_library_service()
            .assign_component_to_instance(
                &mut project,
                ComponentAssignment {
                    instance_id: request.instance_id,
                    component_definition_id: request.component_definition_id,
                    selected_symbol_id: request.selected_symbol_id,
                    selected_footprint_id: request.selected_footprint_id,
                    selected_simulation_model_id: request.selected_simulation_model_id,
                },
            )?;
        self.replace_current_project(project.clone())?;
        Ok(ProjectDto::from(&project))
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
