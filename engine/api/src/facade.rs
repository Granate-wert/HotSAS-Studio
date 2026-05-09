use crate::{
    AcSweepSettingsDto, AddComponentRequestDto, ApiError, AppDiagnosticsReportDto,
    ApplyNotebookValueRequestDto, AssignComponentRequestDto, CircuitValidationIssueDto,
    CircuitValidationReportDto, ComponentDetailsDto, ComponentLibraryDto,
    ComponentModelAssignmentDto, ComponentParameterDto, ComponentSearchRequestDto,
    ComponentSearchResultDto, ComponentSummaryDto, ConnectPinsRequestDto,
    DeleteComponentRequestDto, DeleteWireRequestDto, ExportCapabilityDto, ExportHistoryEntryDto,
    ExportRequestDto, ExportResultDto, FootprintDto, FormulaCalculationRequestDto,
    FormulaDetailsDto, FormulaEvaluationResultDto, FormulaOutputValueDto, FormulaPackDto,
    FormulaResultDto, FormulaSummaryDto, KeyValueDto, MoveComponentRequestDto, NetlistPreviewDto,
    NgspiceAvailabilityDto, NgspiceDiagnosticsDto, NotebookEvaluationRequestDto,
    NotebookEvaluationResultDto, NotebookStateDto, OperatingPointSettingsDto,
    PlaceComponentRequestDto, PlaceableComponentDto, PreferredValueDto, ProductWorkflowStatusDto,
    ProjectDto, ProjectOpenRequestDto, ProjectOpenResultDto, ProjectPackageManifestDto,
    ProjectPackageValidationReportDto, ProjectPersistenceWarningDto, ProjectSaveResultDto,
    ProjectSessionStateDto, RecentProjectEntryDto, RenameNetRequestDto, SaveProjectDto,
    SchematicEditResultDto, SchematicEditableFieldDto, SchematicSelectionDetailsDto,
    SchematicSelectionRequestDto, SchematicToolCapabilityDto, SelectedComponentDto,
    SelectedRegionAnalysisRequestDto, SelectedRegionAnalysisResultDto, SelectedRegionPreviewDto,
    SimulationDiagnosticMessageDto, SimulationGraphViewDto, SimulationMeasurementDto,
    SimulationModelDto, SimulationPointDto, SimulationPreflightResultDto, SimulationProbeDto,
    SimulationProbeTargetDto, SimulationResultDto, SimulationRunHistoryEntryDto,
    SimulationRunRequestDto, SimulationSeriesDto, SimulationWorkflowErrorDto,
    SimulationWorkflowWarningDto, SymbolDto, TransientSettingsDto, UndoRedoStateDto,
    UpdateQuickParameterRequestDto, UserCircuitSimulationProfileDto,
    UserCircuitSimulationResultDto, UserCircuitSimulationRunDto, ValueDto, VerticalSliceDto,
};
use hotsas_application::{
    AppDiagnosticsService, AppServices, FormulaRegistryService, ProductWorkflowService,
};
use hotsas_core::{
    rc_low_pass_formula, CircuitProject, ComponentAssignment, ComponentDefinition,
    ComponentLibrary, ComponentLibraryQuery, EngineeringNotebook, EngineeringUnit,
    FormulaDefinition, FormulaPack, NotebookBlock, NotebookEvaluationStatus, NotebookHistoryEntry,
    ValueWithUnit,
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
    app_diagnostics: AppDiagnosticsService,
    product_workflow: ProductWorkflowService,
    last_advanced_report: Mutex<Option<hotsas_core::advanced_report::AdvancedReportModel>>,
    last_advanced_report_project_id: Mutex<Option<String>>,
    // v2.8 undo/redo foundation
    undo_stack: Mutex<Vec<(CircuitProject, String)>>,
    redo_stack: Mutex<Vec<(CircuitProject, String)>>,
}

impl HotSasApi {
    pub fn services_mut(&mut self) -> &mut AppServices {
        &mut self.services
    }

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
            app_diagnostics: AppDiagnosticsService::new(),
            product_workflow: ProductWorkflowService::new(),
            last_advanced_report: Mutex::new(None),
            last_advanced_report_project_id: Mutex::new(None),
            undo_stack: Mutex::new(Vec::new()),
            redo_stack: Mutex::new(Vec::new()),
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
        self.services
            .project_session_service()
            .set_current_project(&project, None);
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

    pub fn check_ngspice_availability(&self) -> Result<NgspiceAvailabilityDto, ApiError> {
        let availability = self.services.check_ngspice_availability()?;
        Ok(NgspiceAvailabilityDto::from(&availability))
    }

    pub fn run_simulation(
        &self,
        request: SimulationRunRequestDto,
    ) -> Result<SimulationResultDto, ApiError> {
        let project = self.current_project()?;
        let result =
            self.services
                .run_simulation(&project, &request.engine, &request.analysis_kind)?;
        Ok(SimulationResultDto::from(&result))
    }

    pub fn simulation_history(&self) -> Result<Vec<SimulationResultDto>, ApiError> {
        Ok(self
            .services
            .simulation_history()
            .iter()
            .map(SimulationResultDto::from)
            .collect())
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
        self.services
            .project_session_service()
            .set_current_project(&project, Some(package_dir));
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

    pub fn preview_selected_region(
        &self,
        component_ids: Vec<String>,
    ) -> Result<SelectedRegionPreviewDto, ApiError> {
        let project = self.current_project()?;
        let preview = self
            .services
            .selected_region_analysis_service()
            .preview_selected_region(&project.schematic, component_ids)
            .map_err(ApiError::Application)?;
        Ok(SelectedRegionPreviewDto::from(&preview))
    }

    pub fn analyze_selected_region(
        &self,
        request: SelectedRegionAnalysisRequestDto,
    ) -> Result<SelectedRegionAnalysisResultDto, ApiError> {
        let project = self.current_project()?;
        let direction = match request.analysis_direction.as_str() {
            "LeftToRight" => hotsas_core::RegionAnalysisDirection::LeftToRight,
            "RightToLeft" => hotsas_core::RegionAnalysisDirection::RightToLeft,
            _ => hotsas_core::RegionAnalysisDirection::Custom,
        };
        let mode = match request.analysis_mode.as_str() {
            "Structural" => hotsas_core::RegionAnalysisMode::Structural,
            "TemplateBased" => hotsas_core::RegionAnalysisMode::TemplateBased,
            "NumericMock" => hotsas_core::RegionAnalysisMode::NumericMock,
            _ => hotsas_core::RegionAnalysisMode::AllAvailable,
        };
        let core_request = hotsas_core::SelectedRegionAnalysisRequest {
            component_ids: request.component_ids,
            input_port: request.input_port.map(|p| hotsas_core::RegionPort {
                positive_net: p.positive_net,
                negative_net: p.negative_net,
                label: p.label,
            }),
            output_port: request.output_port.map(|p| hotsas_core::RegionPort {
                positive_net: p.positive_net,
                negative_net: p.negative_net,
                label: p.label,
            }),
            reference_node: request.reference_node,
            analysis_direction: direction,
            analysis_mode: mode,
        };
        let result = self
            .services
            .selected_region_analysis_service()
            .analyze_selected_region(&project.schematic, core_request)
            .map_err(ApiError::Application)?;
        Ok(SelectedRegionAnalysisResultDto::from(&result))
    }

    pub fn validate_selected_region(
        &self,
        request: SelectedRegionAnalysisRequestDto,
    ) -> Result<Vec<crate::SelectedRegionIssueDto>, ApiError> {
        let project = self.current_project()?;
        let direction = match request.analysis_direction.as_str() {
            "LeftToRight" => hotsas_core::RegionAnalysisDirection::LeftToRight,
            "RightToLeft" => hotsas_core::RegionAnalysisDirection::RightToLeft,
            _ => hotsas_core::RegionAnalysisDirection::Custom,
        };
        let mode = match request.analysis_mode.as_str() {
            "Structural" => hotsas_core::RegionAnalysisMode::Structural,
            "TemplateBased" => hotsas_core::RegionAnalysisMode::TemplateBased,
            "NumericMock" => hotsas_core::RegionAnalysisMode::NumericMock,
            _ => hotsas_core::RegionAnalysisMode::AllAvailable,
        };
        let core_request = hotsas_core::SelectedRegionAnalysisRequest {
            component_ids: request.component_ids,
            input_port: request.input_port.map(|p| hotsas_core::RegionPort {
                positive_net: p.positive_net,
                negative_net: p.negative_net,
                label: p.label,
            }),
            output_port: request.output_port.map(|p| hotsas_core::RegionPort {
                positive_net: p.positive_net,
                negative_net: p.negative_net,
                label: p.label,
            }),
            reference_node: request.reference_node,
            analysis_direction: direction,
            analysis_mode: mode,
        };
        let issues = self
            .services
            .selected_region_analysis_service()
            .validate_selected_region(&project.schematic, &core_request);
        Ok(issues
            .iter()
            .map(crate::SelectedRegionIssueDto::from)
            .collect())
    }

    fn replace_current_project(&self, project: CircuitProject) -> Result<(), ApiError> {
        let old_id = self
            .current_project
            .lock()
            .map_err(|_| ApiError::State("current project lock poisoned".to_string()))?
            .as_ref()
            .map(|p| p.id.clone());
        let new_id = project.id.clone();
        let same_project = old_id == Some(new_id.clone());
        if !same_project {
            if let Ok(mut guard) = self.last_advanced_report.lock() {
                *guard = None;
            }
            if let Ok(mut guard) = self.last_advanced_report_project_id.lock() {
                *guard = None;
            }
        }
        let mut guard = self
            .current_project
            .lock()
            .map_err(|_| ApiError::State("current project lock poisoned".to_string()))?;
        *guard = Some(project);
        drop(guard);
        if same_project {
            self.services
                .project_session_service()
                .mark_dirty("project mutated");
        }
        Ok(())
    }

    pub fn get_current_project(&self) -> Result<ProjectDto, ApiError> {
        self.current_project().map(|p| ProjectDto::from(&p))
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

    pub fn list_export_capabilities(&self) -> Result<Vec<ExportCapabilityDto>, ApiError> {
        Ok(self
            .services
            .export_center_service()
            .list_capabilities()
            .iter()
            .map(ExportCapabilityDto::from)
            .collect())
    }

    pub fn export(&self, request: ExportRequestDto) -> Result<ExportResultDto, ApiError> {
        let format = parse_export_format(&request.format)?;
        let project = self.current_project()?;
        let report = match format {
            hotsas_core::ExportFormat::MarkdownReport | hotsas_core::ExportFormat::HtmlReport => {
                Some(self.current_report_model()?)
            }
            _ => None,
        };
        let simulation = match format {
            hotsas_core::ExportFormat::CsvSimulationData => {
                Some(self.services.run_mock_ac_simulation(&project)?)
            }
            _ => None,
        };
        let library = match format {
            hotsas_core::ExportFormat::ComponentLibraryJson => {
                Some(self.current_component_library()?)
            }
            _ => None,
        };
        let result = if request.write_to_file {
            let output_dir = request
                .output_dir
                .as_deref()
                .map(Path::new)
                .unwrap_or_else(|| Path::new("."));
            self.services.export_center_service().export_to_file(
                format,
                &project,
                report.as_ref(),
                simulation.as_ref(),
                library.as_ref(),
                output_dir,
            )?
        } else {
            self.services.export_center_service().export_to_string(
                format,
                &project,
                report.as_ref(),
                simulation.as_ref(),
                library.as_ref(),
            )?
        };
        Ok(ExportResultDto::from(&result))
    }

    pub fn export_history(&self) -> Result<Vec<ExportHistoryEntryDto>, ApiError> {
        Ok(self
            .services
            .export_center_service()
            .list_history()?
            .iter()
            .map(ExportHistoryEntryDto::from)
            .collect())
    }

    pub fn import_spice_model(
        &self,
        request: crate::SpiceImportRequestDto,
    ) -> Result<crate::SpiceImportReportDto, ApiError> {
        let report = self
            .services
            .model_import_service()
            .import_spice_from_text(request.source_name, request.content)?;
        Ok(crate::SpiceImportReportDto::from(&report))
    }

    pub fn import_touchstone_model(
        &self,
        request: crate::TouchstoneImportRequestDto,
    ) -> Result<crate::TouchstoneImportReportDto, ApiError> {
        let report = self
            .services
            .model_import_service()
            .import_touchstone_from_text(request.source_name, request.content)?;
        Ok(crate::TouchstoneImportReportDto::from(&report))
    }

    pub fn list_imported_models(&self) -> Result<Vec<crate::ImportedModelSummaryDto>, ApiError> {
        Ok(self
            .services
            .model_import_service()
            .list_imported_models()?
            .iter()
            .map(crate::ImportedModelSummaryDto::from)
            .collect())
    }

    pub fn get_imported_model(
        &self,
        model_id: String,
    ) -> Result<crate::ImportedModelDetailsDto, ApiError> {
        let details = self
            .services
            .model_import_service()
            .get_imported_model(model_id)?;
        Ok(crate::ImportedModelDetailsDto::from(&details))
    }

    pub fn validate_spice_pin_mapping(
        &self,
        request: crate::SpicePinMappingRequestDto,
    ) -> Result<crate::SpicePinMappingValidationReportDto, ApiError> {
        let core_request = hotsas_core::SpicePinMappingRequest {
            model_id: request.model_id,
            component_definition_id: request.component_definition_id,
            mappings: request
                .mappings
                .into_iter()
                .map(|m| hotsas_core::SpicePinMappingEntry {
                    model_pin: m.model_pin,
                    component_pin: m.component_pin,
                    role_hint: m.role_hint,
                })
                .collect(),
        };
        let report = self
            .services
            .model_import_service()
            .validate_spice_pin_mapping(core_request)?;
        Ok(crate::SpicePinMappingValidationReportDto::from(&report))
    }

    pub fn attach_imported_model_to_component(
        &self,
        request: crate::AttachImportedModelRequestDto,
    ) -> Result<crate::ComponentDetailsDto, ApiError> {
        let mut library = self.current_component_library()?;
        let component = library
            .components
            .iter_mut()
            .find(|c| c.id == request.component_definition_id)
            .ok_or_else(|| {
                ApiError::Application(hotsas_application::ApplicationError::NotFound(format!(
                    "component definition {}",
                    request.component_definition_id
                )))
            })?;
        let core_request = hotsas_core::AttachImportedModelRequest {
            model_id: request.model_id,
            component_definition_id: request.component_definition_id,
            pin_mapping: request
                .pin_mapping
                .map(|pm| hotsas_core::SpicePinMappingRequest {
                    model_id: pm.model_id,
                    component_definition_id: pm.component_definition_id,
                    mappings: pm
                        .mappings
                        .into_iter()
                        .map(|m| hotsas_core::SpicePinMappingEntry {
                            model_pin: m.model_pin,
                            component_pin: m.component_pin,
                            role_hint: m.role_hint,
                        })
                        .collect(),
                }),
        };
        self.services
            .model_import_service()
            .attach_imported_model_to_component(core_request, component)?;
        Ok(crate::ComponentDetailsDto::from(&*component))
    }

    pub fn get_app_diagnostics(&self) -> Result<AppDiagnosticsReportDto, ApiError> {
        let report = self.app_diagnostics.get_app_diagnostics(&self.services);
        Ok(AppDiagnosticsReportDto::from(&report))
    }

    pub fn run_readiness_self_check(&self) -> Result<AppDiagnosticsReportDto, ApiError> {
        let report = self
            .app_diagnostics
            .run_readiness_self_check(&self.services);
        Ok(AppDiagnosticsReportDto::from(&report))
    }

    pub fn get_product_workflow_status(&self) -> Result<ProductWorkflowStatusDto, ApiError> {
        let project_opt = self
            .current_project
            .lock()
            .map_err(|_| ApiError::State("current project lock poisoned".to_string()))?
            .clone();
        let status = self
            .product_workflow
            .get_product_workflow_status(&self.services, project_opt.as_ref());
        Ok(ProductWorkflowStatusDto::from(&status))
    }

    pub fn run_product_beta_self_check(&self) -> Result<ProductWorkflowStatusDto, ApiError> {
        let status = self
            .product_workflow
            .run_product_beta_self_check(&self.services);
        Ok(ProductWorkflowStatusDto::from(&status))
    }

    pub fn create_integrated_demo_project(&self) -> Result<ProjectDto, ApiError> {
        let project = self
            .product_workflow
            .create_integrated_demo_project(&self.services);
        self.replace_current_project(project.clone())?;
        self.services
            .project_session_service()
            .set_current_project(&project, None);
        Ok(ProjectDto::from(&project))
    }

    pub fn calculate_dcdc(
        &self,
        request: crate::DcdcInputDto,
    ) -> Result<crate::DcdcCalculationResultDto, ApiError> {
        let topology = request
            .topology
            .parse::<hotsas_core::DcdcTopology>()
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        let input = hotsas_core::DcdcInput {
            topology,
            vin: parse_value(&request.vin, EngineeringUnit::Volt)?,
            vout: parse_value(&request.vout, EngineeringUnit::Volt)?,
            iout: parse_value(&request.iout, EngineeringUnit::Ampere)?,
            switching_frequency: parse_value(&request.switching_frequency, EngineeringUnit::Hertz)?,
            inductor: request
                .inductor
                .as_ref()
                .map(|s| ValueWithUnit::parse_with_default(s, EngineeringUnit::Henry))
                .transpose()
                .map_err(|e| ApiError::InvalidInput(e.to_string()))?,
            output_capacitor: request
                .output_capacitor
                .as_ref()
                .map(|s| ValueWithUnit::parse_with_default(s, EngineeringUnit::Farad))
                .transpose()
                .map_err(|e| ApiError::InvalidInput(e.to_string()))?,
            target_inductor_ripple_percent: request.target_inductor_ripple_percent,
            estimated_efficiency_percent: request.estimated_efficiency_percent,
        };
        let result = self
            .services
            .dcdc_calculator_service()
            .calculate_dcdc(input)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        Ok(crate::DcdcCalculationResultDto::from(&result))
    }

    pub fn list_dcdc_templates(&self) -> Result<Vec<crate::DcdcTemplateDto>, ApiError> {
        Ok(self
            .services
            .dcdc_calculator_service()
            .list_dcdc_templates()
            .iter()
            .map(crate::DcdcTemplateDto::from)
            .collect())
    }

    pub fn generate_dcdc_netlist_preview(
        &self,
        request: crate::DcdcNetlistPreviewRequestDto,
    ) -> Result<String, ApiError> {
        let topology = request
            .topology
            .parse::<hotsas_core::DcdcTopology>()
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        let input = hotsas_core::DcdcInput {
            topology,
            vin: parse_value(&request.vin, EngineeringUnit::Volt)?,
            vout: parse_value(&request.vout, EngineeringUnit::Volt)?,
            iout: parse_value(&request.iout, EngineeringUnit::Ampere)?,
            switching_frequency: parse_value(&request.switching_frequency, EngineeringUnit::Hertz)?,
            inductor: None,
            output_capacitor: None,
            target_inductor_ripple_percent: None,
            estimated_efficiency_percent: None,
        };
        self.services
            .dcdc_calculator_service()
            .generate_dcdc_netlist_preview(topology, &input)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))
    }

    pub fn run_dcdc_mock_transient_preview(
        &self,
        request: crate::DcdcMockTransientRequestDto,
    ) -> Result<SimulationResultDto, ApiError> {
        let topology = request
            .topology
            .parse::<hotsas_core::DcdcTopology>()
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        let input = hotsas_core::DcdcInput {
            topology,
            vin: parse_value(&request.vin, EngineeringUnit::Volt)?,
            vout: parse_value(&request.vout, EngineeringUnit::Volt)?,
            iout: parse_value(&request.iout, EngineeringUnit::Ampere)?,
            switching_frequency: parse_value(&request.switching_frequency, EngineeringUnit::Hertz)?,
            inductor: request
                .inductor
                .as_ref()
                .map(|s| ValueWithUnit::parse_with_default(s, EngineeringUnit::Henry))
                .transpose()
                .map_err(|e| ApiError::InvalidInput(e.to_string()))?,
            output_capacitor: request
                .output_capacitor
                .as_ref()
                .map(|s| ValueWithUnit::parse_with_default(s, EngineeringUnit::Farad))
                .transpose()
                .map_err(|e| ApiError::InvalidInput(e.to_string()))?,
            target_inductor_ripple_percent: request.target_inductor_ripple_percent,
            estimated_efficiency_percent: request.estimated_efficiency_percent,
        };
        let calc_result = self
            .services
            .dcdc_calculator_service()
            .calculate_dcdc(input)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        let sim_result = self
            .services
            .dcdc_calculator_service()
            .create_dcdc_mock_transient_preview(&calc_result)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        Ok(SimulationResultDto::from(&sim_result))
    }

    pub fn list_report_section_capabilities(
        &self,
    ) -> Result<Vec<crate::ReportSectionCapabilityDto>, ApiError> {
        let caps = self
            .services
            .advanced_report_service()
            .list_section_capabilities();
        Ok(caps
            .into_iter()
            .map(|c| crate::ReportSectionCapabilityDto {
                kind: c.kind.to_string(),
                title: c.title,
                description: c.description,
                default_enabled: c.default_enabled,
                supported_report_types: c
                    .supported_report_types
                    .iter()
                    .map(|t| t.to_string())
                    .collect(),
            })
            .collect())
    }

    pub fn generate_advanced_report(
        &self,
        request: crate::AdvancedReportRequestDto,
    ) -> Result<crate::AdvancedReportDto, ApiError> {
        let report_type = request
            .report_type
            .parse::<hotsas_core::advanced_report::AdvancedReportType>()
            .map_err(|e| ApiError::InvalidInput(e))?;
        let included_sections: Result<Vec<_>, _> = request
            .included_sections
            .iter()
            .map(|s| s.parse::<hotsas_core::advanced_report::ReportSectionKind>())
            .collect();
        let included_sections = included_sections.map_err(|e| ApiError::InvalidInput(e))?;
        let report_request = hotsas_core::advanced_report::AdvancedReportRequest {
            report_id: request.report_id.clone(),
            title: request.title,
            report_type,
            included_sections,
            export_options: hotsas_core::advanced_report::ReportExportOptions {
                include_source_references: request.export_options.include_source_references,
                include_graph_references: request.export_options.include_graph_references,
                include_assumptions: request.export_options.include_assumptions,
                max_table_rows: request.export_options.max_table_rows,
            },
            metadata: request.metadata,
        };
        let project = self.current_project().ok();
        let project_id = project.as_ref().map(|p| p.id.clone());
        let context = hotsas_core::advanced_report::AdvancedReportContext {
            project,
            notebook: Some(
                self.notebook
                    .lock()
                    .map_err(|_| ApiError::State("notebook lock poisoned".to_string()))?
                    .clone(),
            ),
            simulation_result: None,
            dcdc_result: None,
            selected_region_result: None,
            export_history: vec![],
            netlist: None,
            imported_models_summary: vec![],
        };
        let report = self
            .services
            .advanced_report_service()
            .generate_report(report_request, &context)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        let dto = Self::advanced_report_to_dto(&report);
        if let Ok(mut guard) = self.last_advanced_report.lock() {
            *guard = Some(report);
        }
        if let Ok(mut guard) = self.last_advanced_report_project_id.lock() {
            *guard = project_id;
        }
        Ok(dto)
    }

    pub fn export_advanced_report(
        &self,
        request: crate::AdvancedReportExportRequestDto,
    ) -> Result<crate::AdvancedReportExportResultDto, ApiError> {
        let current_project_id = self.current_project().ok().map(|p| p.id);
        let cached_project_id = self
            .last_advanced_report_project_id
            .lock()
            .map_err(|_| ApiError::State("report project id lock poisoned".to_string()))?
            .clone();
        if current_project_id != cached_project_id {
            return Err(ApiError::State(
                "report cache invalidated due to project change".to_string(),
            ));
        }
        let report = self
            .last_advanced_report
            .lock()
            .map_err(|_| ApiError::State("report lock poisoned".to_string()))?
            .clone()
            .ok_or_else(|| ApiError::State("no report generated yet".to_string()))?;
        let (content, message) = match request.format.as_str() {
            "markdown" => {
                let md = self
                    .services
                    .advanced_report_service()
                    .render_report_markdown(&report)
                    .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
                (md, "Markdown report exported.".to_string())
            }
            "html" => {
                let html = self
                    .services
                    .advanced_report_service()
                    .render_report_html(&report)
                    .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
                (html, "HTML report exported.".to_string())
            }
            "json" => {
                let json = self
                    .services
                    .advanced_report_service()
                    .render_report_json(&report)
                    .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
                (json, "JSON report exported.".to_string())
            }
            "csv_summary" => {
                let csv = self
                    .services
                    .advanced_report_service()
                    .render_report_csv_summary(&report)
                    .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
                (csv, "CSV summary exported.".to_string())
            }
            other => {
                return Err(ApiError::InvalidInput(format!(
                    "unknown export format: {other}"
                )))
            }
        };
        Ok(crate::AdvancedReportExportResultDto {
            report_id: report.id,
            format: request.format,
            content,
            output_path: request.output_path,
            success: true,
            message,
        })
    }

    pub fn get_last_advanced_report(&self) -> Result<Option<crate::AdvancedReportDto>, ApiError> {
        let current_project_id = self.current_project().ok().map(|p| p.id);
        let cached_project_id = self
            .last_advanced_report_project_id
            .lock()
            .map_err(|_| ApiError::State("report project id lock poisoned".to_string()))?
            .clone();
        if current_project_id != cached_project_id {
            return Ok(None);
        }
        let report = self
            .last_advanced_report
            .lock()
            .map_err(|_| ApiError::State("report lock poisoned".to_string()))?
            .clone();
        Ok(report.as_ref().map(Self::advanced_report_to_dto))
    }

    fn advanced_report_to_dto(
        report: &hotsas_core::advanced_report::AdvancedReportModel,
    ) -> crate::AdvancedReportDto {
        crate::AdvancedReportDto {
            id: report.id.clone(),
            title: report.title.clone(),
            report_type: report.report_type.to_string(),
            generated_at: report.generated_at.clone(),
            project_id: report.project_id.clone(),
            project_name: report.project_name.clone(),
            sections: report
                .sections
                .iter()
                .map(|s| crate::ReportSectionDto {
                    kind: s.kind.to_string(),
                    title: s.title.clone(),
                    status: s.status.to_string(),
                    blocks: s
                        .blocks
                        .iter()
                        .map(|b| Self::report_block_to_dto(b))
                        .collect(),
                    warnings: s
                        .warnings
                        .iter()
                        .map(|w| crate::ReportWarningDto {
                            severity: format!("{:?}", w.severity),
                            code: w.code.clone(),
                            message: w.message.clone(),
                            section_kind: w.section_kind.as_ref().map(|k| k.to_string()),
                        })
                        .collect(),
                })
                .collect(),
            warnings: report
                .warnings
                .iter()
                .map(|w| crate::ReportWarningDto {
                    severity: format!("{:?}", w.severity),
                    code: w.code.clone(),
                    message: w.message.clone(),
                    section_kind: w.section_kind.as_ref().map(|k| k.to_string()),
                })
                .collect(),
            assumptions: report.assumptions.clone(),
            source_references: report
                .source_references
                .iter()
                .map(|sr| crate::ReportSourceReferenceDto {
                    source_id: sr.source_id.clone(),
                    source_type: sr.source_type.clone(),
                    description: sr.description.clone(),
                })
                .collect(),
            metadata: report.metadata.clone(),
        }
    }

    fn report_block_to_dto(
        block: &hotsas_core::advanced_report::ReportContentBlock,
    ) -> crate::ReportContentBlockDto {
        match block {
            hotsas_core::advanced_report::ReportContentBlock::Paragraph { text } => {
                crate::ReportContentBlockDto {
                    block_type: "paragraph".to_string(),
                    title: None,
                    text: Some(text.clone()),
                    rows: None,
                    columns: None,
                    data_rows: None,
                    equation: None,
                    substituted_values: None,
                    result: None,
                    language: None,
                    content: None,
                    series_names: None,
                    x_unit: None,
                    y_unit: None,
                    items: None,
                }
            }
            hotsas_core::advanced_report::ReportContentBlock::KeyValueTable { title, rows } => {
                crate::ReportContentBlockDto {
                    block_type: "key_value_table".to_string(),
                    title: Some(title.clone()),
                    text: None,
                    rows: Some(
                        rows.iter()
                            .map(|r| crate::ReportKeyValueRowDto {
                                key: r.key.clone(),
                                value: r.value.clone(),
                                unit: r.unit.clone(),
                            })
                            .collect(),
                    ),
                    columns: None,
                    data_rows: None,
                    equation: None,
                    substituted_values: None,
                    result: None,
                    language: None,
                    content: None,
                    series_names: None,
                    x_unit: None,
                    y_unit: None,
                    items: None,
                }
            }
            hotsas_core::advanced_report::ReportContentBlock::DataTable {
                title,
                columns,
                rows,
            } => crate::ReportContentBlockDto {
                block_type: "data_table".to_string(),
                title: Some(title.clone()),
                text: None,
                rows: None,
                columns: Some(columns.clone()),
                data_rows: Some(rows.clone()),
                equation: None,
                substituted_values: None,
                result: None,
                language: None,
                content: None,
                series_names: None,
                x_unit: None,
                y_unit: None,
                items: None,
            },
            hotsas_core::advanced_report::ReportContentBlock::FormulaBlock {
                title,
                equation,
                substituted_values,
                result,
            } => crate::ReportContentBlockDto {
                block_type: "formula_block".to_string(),
                title: Some(title.clone()),
                text: None,
                rows: None,
                columns: None,
                data_rows: None,
                equation: Some(equation.clone()),
                substituted_values: Some(
                    substituted_values
                        .iter()
                        .map(|r| crate::ReportKeyValueRowDto {
                            key: r.key.clone(),
                            value: r.value.clone(),
                            unit: r.unit.clone(),
                        })
                        .collect(),
                ),
                result: result.clone(),
                language: None,
                content: None,
                series_names: None,
                x_unit: None,
                y_unit: None,
                items: None,
            },
            hotsas_core::advanced_report::ReportContentBlock::CodeBlock {
                title,
                language,
                content,
            } => crate::ReportContentBlockDto {
                block_type: "code_block".to_string(),
                title: Some(title.clone()),
                text: None,
                rows: None,
                columns: None,
                data_rows: None,
                equation: None,
                substituted_values: None,
                result: None,
                language: Some(language.clone()),
                content: Some(content.clone()),
                series_names: None,
                x_unit: None,
                y_unit: None,
                items: None,
            },
            hotsas_core::advanced_report::ReportContentBlock::GraphReference {
                title,
                series_names,
                x_unit,
                y_unit,
            } => crate::ReportContentBlockDto {
                block_type: "graph_reference".to_string(),
                title: Some(title.clone()),
                text: None,
                rows: None,
                columns: None,
                data_rows: None,
                equation: None,
                substituted_values: None,
                result: None,
                language: None,
                content: None,
                series_names: Some(series_names.clone()),
                x_unit: x_unit.clone(),
                y_unit: y_unit.clone(),
                items: None,
            },
            hotsas_core::advanced_report::ReportContentBlock::WarningList { items } => {
                crate::ReportContentBlockDto {
                    block_type: "warning_list".to_string(),
                    title: None,
                    text: None,
                    rows: None,
                    columns: None,
                    data_rows: None,
                    equation: None,
                    substituted_values: None,
                    result: None,
                    language: None,
                    content: None,
                    series_names: None,
                    x_unit: None,
                    y_unit: None,
                    items: Some(
                        items
                            .iter()
                            .map(|w| crate::ReportWarningDto {
                                severity: format!("{:?}", w.severity),
                                code: w.code.clone(),
                                message: w.message.clone(),
                                section_kind: w.section_kind.as_ref().map(|k| k.to_string()),
                            })
                            .collect(),
                    ),
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // v2.4 Typed Component Parameters
    // ------------------------------------------------------------------

    pub fn get_component_parameter_schema(
        &self,
        category: String,
    ) -> Result<Option<crate::ComponentParameterSchemaDto>, ApiError> {
        let svc = self.services.component_parameter_service();
        Ok(svc
            .schema_for_category(&category)
            .as_ref()
            .map(crate::ComponentParameterSchemaDto::from))
    }

    pub fn validate_component_parameters(
        &self,
        component_id: String,
    ) -> Result<Vec<crate::ComponentParameterIssueDto>, ApiError> {
        let library = self.current_component_library()?;
        let component = library
            .components
            .iter()
            .find(|c| c.id == component_id)
            .ok_or_else(|| {
                ApiError::InvalidInput(format!("component '{}' not found", component_id))
            })?;
        let svc = self.services.component_parameter_service();
        let issues = svc.validate_component(component);
        Ok(issues
            .iter()
            .map(crate::ComponentParameterIssueDto::from)
            .collect())
    }

    pub fn get_typed_component_parameters(
        &self,
        component_id: String,
    ) -> Result<crate::TypedComponentParametersDto, ApiError> {
        let library = self.current_component_library()?;
        let component = library
            .components
            .iter()
            .find(|c| c.id == component_id)
            .ok_or_else(|| {
                ApiError::InvalidInput(format!("component '{}' not found", component_id))
            })?;
        let svc = self.services.component_parameter_service();
        let bundle = match component.category.as_str() {
            "resistor" => {
                if let Some(p) = svc.resistor_parameters(component) {
                    crate::ParameterBundleDto::Resistor {
                        resistance: ValueDto::from(&p.resistance),
                        power_rating: p.power_rating.as_ref().map(ValueDto::from),
                    }
                } else {
                    crate::ParameterBundleDto::Generic
                }
            }
            "capacitor" => {
                if let Some(p) = svc.capacitor_parameters(component) {
                    crate::ParameterBundleDto::Capacitor {
                        capacitance: ValueDto::from(&p.capacitance),
                        voltage_rating: p.voltage_rating.as_ref().map(ValueDto::from),
                    }
                } else {
                    crate::ParameterBundleDto::Generic
                }
            }
            "inductor" => {
                if let Some(p) = svc.inductor_parameters(component) {
                    crate::ParameterBundleDto::Inductor {
                        inductance: ValueDto::from(&p.inductance),
                        current_rating: p.current_rating.as_ref().map(ValueDto::from),
                    }
                } else {
                    crate::ParameterBundleDto::Generic
                }
            }
            "diode" | "led" => {
                let p = svc.diode_parameters(component);
                crate::ParameterBundleDto::Diode {
                    forward_voltage: p.forward_voltage.as_ref().map(ValueDto::from),
                    reverse_voltage: p.reverse_voltage.as_ref().map(ValueDto::from),
                }
            }
            "bjt" => {
                let p = svc.bjt_parameters(component);
                crate::ParameterBundleDto::Bjt {
                    vce_max: p.vce_max.as_ref().map(ValueDto::from),
                    ic_max: p.ic_max.as_ref().map(ValueDto::from),
                }
            }
            "mosfet" => {
                let p = svc.mosfet_parameters(component);
                crate::ParameterBundleDto::Mosfet {
                    vds_max: p.vds_max.as_ref().map(ValueDto::from),
                    id_max: p.id_max.as_ref().map(ValueDto::from),
                    rds_on: p.rds_on.as_ref().map(ValueDto::from),
                }
            }
            "op_amp" => {
                let p = svc.op_amp_parameters(component);
                crate::ParameterBundleDto::OpAmp {
                    gbw: p.gbw.as_ref().map(ValueDto::from),
                    input_offset_voltage: p.input_offset_voltage.as_ref().map(ValueDto::from),
                }
            }
            "voltage_regulator" => {
                let p = svc.regulator_parameters(component);
                crate::ParameterBundleDto::Regulator {
                    output_voltage: p.output_voltage.as_ref().map(ValueDto::from),
                    max_current: p.max_current.as_ref().map(ValueDto::from),
                }
            }
            _ => crate::ParameterBundleDto::Generic,
        };
        Ok(crate::TypedComponentParametersDto {
            component_id: component.id.clone(),
            category: component.category.clone(),
            bundle,
        })
    }

    // ------------------------------------------------------------------
    // v2.5 Schematic Editor Hardening
    // ------------------------------------------------------------------

    // v2.8 undo/redo helpers

    fn push_undo_snapshot(&self, project: CircuitProject, label: String) -> Result<(), ApiError> {
        let mut stack = self
            .undo_stack
            .lock()
            .map_err(|_| ApiError::State("undo stack lock poisoned".to_string()))?;
        if stack.len() >= 50 {
            stack.remove(0);
        }
        stack.push((project, label));
        Ok(())
    }

    fn clear_redo_stack(&self) -> Result<(), ApiError> {
        let mut stack = self
            .redo_stack
            .lock()
            .map_err(|_| ApiError::State("redo stack lock poisoned".to_string()))?;
        stack.clear();
        Ok(())
    }

    fn pop_undo_snapshot(&self) -> Result<Option<(CircuitProject, String)>, ApiError> {
        let mut stack = self
            .undo_stack
            .lock()
            .map_err(|_| ApiError::State("undo stack lock poisoned".to_string()))?;
        Ok(stack.pop())
    }

    fn push_redo_snapshot(&self, project: CircuitProject, label: String) -> Result<(), ApiError> {
        let mut stack = self
            .redo_stack
            .lock()
            .map_err(|_| ApiError::State("redo stack lock poisoned".to_string()))?;
        if stack.len() >= 50 {
            stack.remove(0);
        }
        stack.push((project, label));
        Ok(())
    }

    fn pop_redo_snapshot(&self) -> Result<Option<(CircuitProject, String)>, ApiError> {
        let mut stack = self
            .redo_stack
            .lock()
            .map_err(|_| ApiError::State("redo stack lock poisoned".to_string()))?;
        Ok(stack.pop())
    }

    fn make_schematic_edit_result(
        &self,
        result: hotsas_core::SchematicEditResult,
    ) -> SchematicEditResultDto {
        SchematicEditResultDto {
            project: ProjectDto::from(&result.project),
            validation_warnings: result
                .validation_warnings
                .iter()
                .map(CircuitValidationIssueDto::from)
                .collect(),
            validation_errors: result
                .validation_errors
                .iter()
                .map(CircuitValidationIssueDto::from)
                .collect(),
            message: result.message,
        }
    }

    pub fn list_schematic_editor_capabilities(
        &self,
    ) -> Result<Vec<SchematicToolCapabilityDto>, ApiError> {
        Ok(vec![
            SchematicToolCapabilityDto {
                tool_id: "add_component".to_string(),
                label: "Add Component".to_string(),
                available: true,
                limitation: None,
            },
            SchematicToolCapabilityDto {
                tool_id: "move_component".to_string(),
                label: "Move Component".to_string(),
                available: true,
                limitation: Some("Position updates via backend; drag is UI-only".to_string()),
            },
            SchematicToolCapabilityDto {
                tool_id: "delete_component".to_string(),
                label: "Delete Component".to_string(),
                available: true,
                limitation: None,
            },
            SchematicToolCapabilityDto {
                tool_id: "connect_pins".to_string(),
                label: "Connect Pins".to_string(),
                available: true,
                limitation: Some("Logical connection only; no interactive wire drag".to_string()),
            },
            SchematicToolCapabilityDto {
                tool_id: "rename_net".to_string(),
                label: "Rename Net".to_string(),
                available: true,
                limitation: None,
            },
        ])
    }

    pub fn add_schematic_component(
        &self,
        request: AddComponentRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Add component".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .add_component(
                &mut project,
                hotsas_core::AddComponentRequest {
                    component_kind: request.component_kind,
                    component_definition_id: request.component_definition_id,
                    instance_id: request.instance_id,
                    position: hotsas_core::Point::new(request.x, request.y),
                    rotation_deg: request.rotation_deg,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn move_schematic_component(
        &self,
        request: MoveComponentRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Move component".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .move_component(
                &mut project,
                hotsas_core::MoveComponentRequest {
                    instance_id: request.instance_id,
                    position: hotsas_core::Point::new(request.x, request.y),
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn delete_schematic_component(
        &self,
        request: DeleteComponentRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Delete component".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .delete_component(
                &mut project,
                hotsas_core::DeleteComponentRequest {
                    instance_id: request.instance_id,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn connect_schematic_pins(
        &self,
        request: ConnectPinsRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Connect pins".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .connect_pins(
                &mut project,
                hotsas_core::ConnectPinsRequest {
                    from_component_id: request.from_component_id,
                    from_pin_id: request.from_pin_id,
                    to_component_id: request.to_component_id,
                    to_pin_id: request.to_pin_id,
                    net_name: request.net_name,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn rename_schematic_net(
        &self,
        request: RenameNetRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Rename net".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .rename_net(
                &mut project,
                hotsas_core::RenameNetRequest {
                    net_id: request.net_id,
                    new_name: request.new_name,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn list_placeable_components(&self) -> Result<Vec<PlaceableComponentDto>, ApiError> {
        let library = self.current_component_library()?;
        let components: Vec<PlaceableComponentDto> = library
            .components
            .iter()
            .map(|c| PlaceableComponentDto {
                definition_id: c.id.clone(),
                name: c.name.clone(),
                category: c.category.clone(),
                component_kind: c.id.clone(),
                has_symbol: !c.symbol_ids.is_empty(),
            })
            .collect();
        Ok(components)
    }

    pub fn place_schematic_component(
        &self,
        request: PlaceComponentRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Place component".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .add_component(
                &mut project,
                hotsas_core::AddComponentRequest {
                    component_kind: request.component_definition_id.clone(),
                    component_definition_id: Some(request.component_definition_id),
                    instance_id: None,
                    position: hotsas_core::Point::new(request.x, request.y),
                    rotation_deg: request.rotation_deg,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn delete_schematic_wire(
        &self,
        request: DeleteWireRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Delete wire".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .delete_wire(
                &mut project,
                hotsas_core::DeleteWireRequest {
                    wire_id: request.wire_id,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn update_schematic_quick_parameter(
        &self,
        request: UpdateQuickParameterRequestDto,
    ) -> Result<SchematicEditResultDto, ApiError> {
        let mut project = self.current_project()?;
        self.push_undo_snapshot(project.clone(), "Update parameter".to_string())?;
        let result = self
            .services
            .schematic_editing_service()
            .update_component_quick_parameter(
                &mut project,
                hotsas_core::UpdateQuickParameterRequest {
                    component_id: request.component_id,
                    parameter_id: request.parameter_id,
                    value: request.value,
                },
            )
            .map_err(|e| ApiError::InvalidInput(e))?;
        self.replace_current_project(project)?;
        self.clear_redo_stack()?;
        Ok(self.make_schematic_edit_result(result))
    }

    pub fn get_schematic_selection_details(
        &self,
        request: SchematicSelectionRequestDto,
    ) -> Result<SchematicSelectionDetailsDto, ApiError> {
        let project = self.current_project()?;
        match request.kind.as_str() {
            "component" => {
                let comp = project
                    .schematic
                    .components
                    .iter()
                    .find(|c| c.instance_id == request.id)
                    .ok_or_else(|| {
                        ApiError::InvalidInput(format!("component '{}' not found", request.id))
                    })?;
                let mut fields = vec![SchematicEditableFieldDto {
                    field_id: "instance_id".to_string(),
                    label: "Instance ID".to_string(),
                    current_value: comp.instance_id.clone(),
                    editable: false,
                }];
                for (key, value) in &comp.overridden_parameters {
                    fields.push(SchematicEditableFieldDto {
                        field_id: key.clone(),
                        label: key.clone(),
                        current_value: format!("{}", value),
                        editable: true,
                    });
                }
                let library = hotsas_core::built_in_component_library();
                let definition = library
                    .components
                    .iter()
                    .find(|d| d.id == comp.definition_id)
                    .cloned()
                    .unwrap_or_else(|| ComponentDefinition {
                        id: comp.definition_id.clone(),
                        name: comp.definition_id.clone(),
                        category: "unknown".to_string(),
                        manufacturer: None,
                        part_number: None,
                        description: None,
                        parameters: std::collections::BTreeMap::new(),
                        ratings: std::collections::BTreeMap::new(),
                        symbol_ids: vec![],
                        footprint_ids: vec![],
                        simulation_models: vec![],
                        datasheets: vec![],
                        tags: vec![],
                        metadata: std::collections::BTreeMap::new(),
                    });
                let assignment = self
                    .services
                    .component_model_mapping_service()
                    .get_instance_model_assignment(comp, &definition);
                Ok(SchematicSelectionDetailsDto {
                    kind: "component".to_string(),
                    id: Some(comp.instance_id.clone()),
                    display_name: Some(comp.instance_id.clone()),
                    editable_fields: fields,
                    model_assignment: Some(ComponentModelAssignmentDto::from(&assignment)),
                    model_assignment_origin: Some(
                        if comp.selected_simulation_model_id.is_some() {
                            "override"
                        } else {
                            "inherited"
                        }
                        .to_string(),
                    ),
                })
            }
            "wire" => {
                let wire = project
                    .schematic
                    .wires
                    .iter()
                    .find(|w| w.id == request.id)
                    .ok_or_else(|| {
                        ApiError::InvalidInput(format!("wire '{}' not found", request.id))
                    })?;
                Ok(SchematicSelectionDetailsDto {
                    kind: "wire".to_string(),
                    id: Some(wire.id.clone()),
                    display_name: Some(format!("Wire {}", wire.id)),
                    editable_fields: vec![],
                    model_assignment: None,
                    model_assignment_origin: None,
                })
            }
            "net" => {
                let net = project
                    .schematic
                    .nets
                    .iter()
                    .find(|n| n.id == request.id)
                    .ok_or_else(|| {
                        ApiError::InvalidInput(format!("net '{}' not found", request.id))
                    })?;
                Ok(SchematicSelectionDetailsDto {
                    kind: "net".to_string(),
                    id: Some(net.id.clone()),
                    display_name: Some(net.name.clone()),
                    editable_fields: vec![SchematicEditableFieldDto {
                        field_id: "name".to_string(),
                        label: "Net Name".to_string(),
                        current_value: net.name.clone(),
                        editable: true,
                    }],
                    model_assignment: None,
                    model_assignment_origin: None,
                })
            }
            _ => Err(ApiError::InvalidInput(format!(
                "unknown selection kind: {}",
                request.kind
            ))),
        }
    }

    pub fn undo_schematic_edit(&self) -> Result<ProjectDto, ApiError> {
        let current = self.current_project()?;
        if let Some((previous_project, label)) = self.pop_undo_snapshot()? {
            self.push_redo_snapshot(current, label)?;
            self.replace_current_project(previous_project.clone())?;
            Ok(ProjectDto::from(&previous_project))
        } else {
            Err(ApiError::State("nothing to undo".to_string()))
        }
    }

    pub fn redo_schematic_edit(&self) -> Result<ProjectDto, ApiError> {
        let current = self.current_project()?;
        if let Some((next_project, label)) = self.pop_redo_snapshot()? {
            self.push_undo_snapshot(current, label)?;
            self.replace_current_project(next_project.clone())?;
            Ok(ProjectDto::from(&next_project))
        } else {
            Err(ApiError::State("nothing to redo".to_string()))
        }
    }

    pub fn get_schematic_undo_redo_state(&self) -> Result<UndoRedoStateDto, ApiError> {
        let undo = self
            .undo_stack
            .lock()
            .map_err(|_| ApiError::State("undo stack lock poisoned".to_string()))?;
        let redo = self
            .redo_stack
            .lock()
            .map_err(|_| ApiError::State("redo stack lock poisoned".to_string()))?;
        Ok(UndoRedoStateDto {
            can_undo: !undo.is_empty(),
            can_redo: !redo.is_empty(),
            last_action_label: undo.last().map(|(_, label)| label.clone()),
            next_redo_label: redo.last().map(|(_, label)| label.clone()),
        })
    }

    pub fn generate_current_schematic_netlist_preview(
        &self,
    ) -> Result<NetlistPreviewDto, ApiError> {
        let project = self.current_project()?;
        let netlist = self
            .services
            .generate_spice_netlist(&project)
            .map_err(|e| ApiError::State(format!("netlist generation failed: {e}")))?;
        let validation = self.services.validate_circuit(&project);
        Ok(NetlistPreviewDto {
            netlist,
            warnings: validation
                .warnings
                .iter()
                .map(|w| w.message.clone())
                .collect(),
            errors: validation
                .errors
                .iter()
                .map(|e| e.message.clone())
                .collect(),
        })
    }

    fn current_component_library(&self) -> Result<ComponentLibrary, ApiError> {
        self.component_library
            .lock()
            .map_err(|_| ApiError::State("component library lock poisoned".to_string()))
            .map(|guard| guard.clone())
    }

    pub fn get_project_session_state(&self) -> Result<ProjectSessionStateDto, ApiError> {
        let state = self.services.project_session_service().get_state();
        Ok(ProjectSessionStateDto::from(&state))
    }

    pub fn save_current_project(&self) -> Result<ProjectSaveResultDto, ApiError> {
        let project = self.current_project()?;
        let result = self
            .services
            .project_session_service()
            .save_current_project(&project)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        Ok(ProjectSaveResultDto::from(&result))
    }

    pub fn save_project_as(&self, path: String) -> Result<ProjectSaveResultDto, ApiError> {
        let project = self.current_project()?;
        let result = self
            .services
            .project_session_service()
            .save_project_as(&project, path)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        Ok(ProjectSaveResultDto::from(&result))
    }

    pub fn open_project_package(
        &self,
        request: ProjectOpenRequestDto,
    ) -> Result<ProjectOpenResultDto, ApiError> {
        let result = self
            .services
            .project_session_service()
            .open_project_package(request.path, request.confirm_discard_unsaved)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))?;
        self.replace_current_project(result.project.clone())?;
        Ok(ProjectOpenResultDto {
            project: ProjectDto::from(&result.project),
            path: result.path,
            opened_at: result.opened_at,
            validation_warnings: result
                .validation_warnings
                .iter()
                .map(ProjectPersistenceWarningDto::from)
                .collect(),
        })
    }

    pub fn list_recent_projects(&self) -> Result<Vec<RecentProjectEntryDto>, ApiError> {
        Ok(self
            .services
            .project_session_service()
            .list_recent_projects()
            .iter()
            .map(RecentProjectEntryDto::from)
            .collect())
    }

    pub fn remove_recent_project(&self, path: String) -> Result<(), ApiError> {
        self.services
            .project_session_service()
            .remove_recent_project(path)
            .map_err(|e| ApiError::InvalidInput(e.to_string()))
    }

    pub fn clear_missing_recent_projects(&self) -> Result<usize, ApiError> {
        self.services
            .project_session_service()
            .clear_missing_recent_projects()
            .map_err(|e| ApiError::InvalidInput(e.to_string()))
    }

    // v2.9 User-Circuit Simulation Workflow

    pub fn list_user_circuit_simulation_profiles(
        &self,
    ) -> Result<Vec<UserCircuitSimulationProfileDto>, ApiError> {
        let project = self.current_project()?;
        let profiles = self
            .services
            .simulation_workflow_service()
            .list_default_simulation_profiles(&project)?;
        Ok(profiles.into_iter().map(into_profile_dto).collect())
    }

    pub fn suggest_user_circuit_simulation_probes(
        &self,
    ) -> Result<Vec<SimulationProbeDto>, ApiError> {
        let project = self.current_project()?;
        let probes = self
            .services
            .simulation_workflow_service()
            .suggest_simulation_probes(&project)?;
        Ok(probes.into_iter().map(into_probe_dto).collect())
    }

    pub fn validate_current_circuit_for_simulation(
        &self,
        profile: UserCircuitSimulationProfileDto,
    ) -> Result<SimulationPreflightResultDto, ApiError> {
        let project = self.current_project()?;
        let core_profile = from_profile_dto(profile)?;
        let result = self
            .services
            .simulation_workflow_service()
            .validate_circuit_for_simulation(&project, &core_profile)?;
        Ok(SimulationPreflightResultDto {
            can_run: result.can_run,
            blocking_errors: result
                .blocking_errors
                .into_iter()
                .map(|e| SimulationWorkflowErrorDto {
                    code: e.code,
                    message: e.message,
                })
                .collect(),
            warnings: result
                .warnings
                .into_iter()
                .map(|w| SimulationWorkflowWarningDto {
                    code: w.code,
                    message: w.message,
                })
                .collect(),
            generated_netlist_preview: result.generated_netlist_preview,
        })
    }

    pub fn run_current_circuit_simulation(
        &self,
        profile: UserCircuitSimulationProfileDto,
    ) -> Result<UserCircuitSimulationRunDto, ApiError> {
        let project = self.current_project()?;
        let core_profile = from_profile_dto(profile)?;
        let run = self
            .services
            .simulation_workflow_service()
            .run_user_circuit_simulation(&project, core_profile)?;
        Ok(into_run_dto(run))
    }

    pub fn get_last_user_circuit_simulation(
        &self,
    ) -> Result<Option<UserCircuitSimulationRunDto>, ApiError> {
        let project = self.current_project()?;
        Ok(self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .map(into_run_dto))
    }

    pub fn clear_last_user_circuit_simulation(&self) -> Result<(), ApiError> {
        let project = self.current_project()?;
        self.services
            .simulation_workflow_service()
            .clear_last_user_circuit_simulation(&project.id)
            .map_err(ApiError::Application)
    }

    pub fn add_last_simulation_to_advanced_report(&self) -> Result<ProjectDto, ApiError> {
        let project = self.current_project()?;
        let run = self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .ok_or_else(|| ApiError::State("no simulation run found".to_string()))?;
        let _section = self
            .services
            .simulation_workflow_service()
            .simulation_result_to_report_section(&run)
            .map_err(ApiError::Application)?;
        // Report integration is session-only in v2.9; project DTO returned for UI refresh
        Ok(ProjectDto::from(&project))
    }

    // ------------------------------------------------------------------
    // v3.0 Simulation Diagnostics, History & Graph
    // ------------------------------------------------------------------

    pub fn check_ngspice_diagnostics(&self) -> Result<NgspiceDiagnosticsDto, ApiError> {
        let diagnostics = self
            .services
            .simulation_diagnostics_service()
            .check_ngspice_diagnostics()?;
        Ok(NgspiceDiagnosticsDto::from(&diagnostics))
    }

    pub fn diagnose_simulation_preflight(
        &self,
        profile: UserCircuitSimulationProfileDto,
    ) -> Result<Vec<SimulationDiagnosticMessageDto>, ApiError> {
        let project = self.current_project()?;
        let core_profile = from_profile_dto(profile)?;
        let diagnostics = self
            .services
            .simulation_diagnostics_service()
            .diagnose_simulation_preflight(&project, &core_profile)?;
        Ok(diagnostics
            .iter()
            .map(SimulationDiagnosticMessageDto::from)
            .collect())
    }

    pub fn diagnose_last_simulation_run(
        &self,
    ) -> Result<Vec<SimulationDiagnosticMessageDto>, ApiError> {
        let project = self.current_project()?;
        let run = self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .ok_or_else(|| ApiError::State("no simulation run found".to_string()))?;
        let diagnostics = self
            .services
            .simulation_diagnostics_service()
            .diagnose_failed_run(&run)?;
        Ok(diagnostics
            .iter()
            .map(SimulationDiagnosticMessageDto::from)
            .collect())
    }

    pub fn add_run_to_history(&self) -> Result<(), ApiError> {
        let project = self.current_project()?;
        let run = self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .ok_or_else(|| ApiError::State("no simulation run found".to_string()))?;
        self.services
            .simulation_history_service()
            .add_run(&run)
            .map_err(ApiError::Application)
    }

    pub fn list_simulation_history(&self) -> Result<Vec<SimulationRunHistoryEntryDto>, ApiError> {
        let project = self.current_project()?;
        let entries = self
            .services
            .simulation_history_service()
            .list_runs(&project.id)?;
        Ok(entries
            .iter()
            .map(SimulationRunHistoryEntryDto::from)
            .collect())
    }

    pub fn delete_simulation_history_run(&self, run_id: String) -> Result<(), ApiError> {
        let project = self.current_project()?;
        self.services
            .simulation_history_service()
            .delete_run(&project.id, &run_id)
            .map_err(ApiError::Application)
    }

    pub fn clear_simulation_history(&self) -> Result<(), ApiError> {
        let project = self.current_project()?;
        self.services
            .simulation_history_service()
            .clear_runs(&project.id)
            .map_err(ApiError::Application)
    }

    pub fn build_simulation_graph_view(&self) -> Result<SimulationGraphViewDto, ApiError> {
        let project = self.current_project()?;
        let run = self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .ok_or_else(|| ApiError::State("no simulation run found".to_string()))?;
        let view = self
            .services
            .simulation_graph_service()
            .build_graph_view(&run)?;
        Ok(SimulationGraphViewDto::from(&view))
    }

    pub fn export_run_series_csv(&self) -> Result<String, ApiError> {
        let project = self.current_project()?;
        let run = self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .ok_or_else(|| ApiError::State("no simulation run found".to_string()))?;
        self.services
            .simulation_graph_service()
            .export_run_series_csv(&run)
            .map_err(ApiError::Application)
    }

    pub fn export_run_series_json(&self) -> Result<String, ApiError> {
        let project = self.current_project()?;
        let run = self
            .services
            .simulation_workflow_service()
            .get_last_user_circuit_simulation(&project.id)
            .ok_or_else(|| ApiError::State("no simulation run found".to_string()))?;
        self.services
            .simulation_graph_service()
            .export_run_series_json(&run)
            .map_err(ApiError::Application)
    }

    // ─── v3.1 Component Model Mapping ───

    pub fn list_available_models_for_component(
        &self,
        definition_id: String,
    ) -> Result<Vec<crate::SpiceModelReferenceDto>, ApiError> {
        let library = hotsas_core::built_in_component_library();
        let definition = library
            .components
            .iter()
            .find(|d| d.id == definition_id)
            .cloned()
            .ok_or_else(|| {
                ApiError::InvalidInput(format!(
                    "component definition '{}' not found",
                    definition_id
                ))
            })?;
        let imported_models = self
            .services
            .model_import_service()
            .list_imported_model_details()
            .unwrap_or_default();
        let models = self
            .services
            .component_model_mapping_service()
            .list_available_models_for_component(&definition, &imported_models);
        Ok(models
            .iter()
            .map(crate::SpiceModelReferenceDto::from)
            .collect())
    }

    pub fn get_component_model_assignment(
        &self,
        instance_id: String,
    ) -> Result<crate::ComponentModelAssignmentDto, ApiError> {
        let project = self.current_project()?;
        let library = hotsas_core::built_in_component_library();
        let instance = project
            .schematic
            .components
            .iter()
            .find(|c| c.instance_id == instance_id)
            .ok_or_else(|| {
                ApiError::InvalidInput(format!("component instance '{}' not found", instance_id))
            })?;
        let definition = library
            .components
            .iter()
            .find(|d| d.id == instance.definition_id)
            .cloned()
            .unwrap_or_else(|| ComponentDefinition {
                id: instance.definition_id.clone(),
                name: instance.definition_id.clone(),
                category: "unknown".to_string(),
                manufacturer: None,
                part_number: None,
                description: None,
                parameters: std::collections::BTreeMap::new(),
                ratings: std::collections::BTreeMap::new(),
                symbol_ids: vec![],
                footprint_ids: vec![],
                simulation_models: vec![],
                datasheets: vec![],
                tags: vec![],
                metadata: std::collections::BTreeMap::new(),
            });
        let assignment = self
            .services
            .component_model_mapping_service()
            .get_instance_model_assignment(instance, &definition);
        Ok(crate::ComponentModelAssignmentDto::from(&assignment))
    }

    pub fn assign_model_to_instance(
        &self,
        request: crate::AssignModelRequestDto,
    ) -> Result<crate::ComponentModelAssignmentDto, ApiError> {
        let mut project = self.current_project()?;
        let library = hotsas_core::built_in_component_library();
        let instance = project
            .schematic
            .components
            .iter_mut()
            .find(|c| c.instance_id == request.instance_id)
            .ok_or_else(|| {
                ApiError::InvalidInput(format!(
                    "component instance '{}' not found",
                    request.instance_id
                ))
            })?;
        let definition = library
            .components
            .iter()
            .find(|d| d.id == instance.definition_id)
            .cloned()
            .unwrap_or_else(|| ComponentDefinition {
                id: instance.definition_id.clone(),
                name: instance.definition_id.clone(),
                category: "unknown".to_string(),
                manufacturer: None,
                part_number: None,
                description: None,
                parameters: std::collections::BTreeMap::new(),
                ratings: std::collections::BTreeMap::new(),
                symbol_ids: vec![],
                footprint_ids: vec![],
                simulation_models: vec![],
                datasheets: vec![],
                tags: vec![],
                metadata: std::collections::BTreeMap::new(),
            });
        let assignment = self
            .services
            .component_model_mapping_service()
            .assign_model_to_instance(instance, &request.model_id, &definition)
            .map_err(ApiError::Application)?;
        self.replace_current_project(project)?;
        Ok(crate::ComponentModelAssignmentDto::from(&assignment))
    }

    pub fn evaluate_project_simulation_readiness(
        &self,
    ) -> Result<crate::ProjectSimulationReadinessDto, ApiError> {
        let project = self.current_project()?;
        let library = hotsas_core::built_in_component_library();
        let readiness = self
            .services
            .component_model_mapping_service()
            .evaluate_project_simulation_readiness(&project, &library);
        Ok(crate::ProjectSimulationReadinessDto::from(&readiness))
    }
}

fn parse_export_format(format: &str) -> Result<hotsas_core::ExportFormat, ApiError> {
    match format {
        "markdown_report" => Ok(hotsas_core::ExportFormat::MarkdownReport),
        "html_report" => Ok(hotsas_core::ExportFormat::HtmlReport),
        "spice_netlist" => Ok(hotsas_core::ExportFormat::SpiceNetlist),
        "csv_simulation_data" => Ok(hotsas_core::ExportFormat::CsvSimulationData),
        "bom_csv" => Ok(hotsas_core::ExportFormat::BomCsv),
        "bom_json" => Ok(hotsas_core::ExportFormat::BomJson),
        "component_library_json" => Ok(hotsas_core::ExportFormat::ComponentLibraryJson),
        "svg_schematic" => Ok(hotsas_core::ExportFormat::SvgSchematic),
        "altium_workflow_package" => Ok(hotsas_core::ExportFormat::AltiumWorkflowPackage),
        other => Err(ApiError::InvalidInput(format!(
            "unknown export format: {other}"
        ))),
    }
}

fn parse_value(value: &str, unit: EngineeringUnit) -> Result<ValueWithUnit, ApiError> {
    ValueWithUnit::parse_with_default(value, unit)
        .map_err(|e| ApiError::InvalidInput(e.to_string()))
}

fn into_probe_dto(probe: hotsas_core::SimulationProbe) -> SimulationProbeDto {
    SimulationProbeDto {
        id: probe.id,
        label: probe.label,
        kind: format!("{:?}", probe.kind),
        target: match probe.target {
            hotsas_core::SimulationProbeTarget::Net { net_id } => SimulationProbeTargetDto {
                net_id: Some(net_id),
                component_id: None,
                pin_id: None,
                positive_net_id: None,
                negative_net_id: None,
            },
            hotsas_core::SimulationProbeTarget::ComponentPin {
                component_id,
                pin_id,
            } => SimulationProbeTargetDto {
                net_id: None,
                component_id: Some(component_id),
                pin_id: Some(pin_id),
                positive_net_id: None,
                negative_net_id: None,
            },
            hotsas_core::SimulationProbeTarget::Component { component_id } => {
                SimulationProbeTargetDto {
                    net_id: None,
                    component_id: Some(component_id),
                    pin_id: None,
                    positive_net_id: None,
                    negative_net_id: None,
                }
            }
            hotsas_core::SimulationProbeTarget::NetPair {
                positive_net_id,
                negative_net_id,
            } => SimulationProbeTargetDto {
                net_id: None,
                component_id: None,
                pin_id: None,
                positive_net_id: Some(positive_net_id),
                negative_net_id: Some(negative_net_id),
            },
        },
        unit: probe.unit.map(|u| u.symbol().to_string()),
    }
}

fn into_profile_dto(
    profile: hotsas_core::UserCircuitSimulationProfile,
) -> UserCircuitSimulationProfileDto {
    UserCircuitSimulationProfileDto {
        id: profile.id,
        name: profile.name,
        analysis_type: format!("{:?}", profile.analysis_type),
        engine: format!("{:?}", profile.engine),
        probes: profile.probes.into_iter().map(into_probe_dto).collect(),
        ac: profile.ac.map(|ac| AcSweepSettingsDto {
            start_hz: ac.start_hz,
            stop_hz: ac.stop_hz,
            points_per_decade: ac.points_per_decade,
        }),
        transient: profile.transient.map(|tr| TransientSettingsDto {
            step_seconds: tr.step_seconds,
            stop_seconds: tr.stop_seconds,
        }),
        op: profile.op.map(|op| OperatingPointSettingsDto {
            include_node_voltages: op.include_node_voltages,
            include_branch_currents: op.include_branch_currents,
        }),
    }
}

fn from_profile_dto(
    dto: UserCircuitSimulationProfileDto,
) -> Result<hotsas_core::UserCircuitSimulationProfile, ApiError> {
    Ok(hotsas_core::UserCircuitSimulationProfile {
        id: dto.id,
        name: dto.name,
        analysis_type: match dto.analysis_type.as_str() {
            "OperatingPoint" => hotsas_core::UserCircuitAnalysisType::OperatingPoint,
            "AcSweep" => hotsas_core::UserCircuitAnalysisType::AcSweep,
            "Transient" => hotsas_core::UserCircuitAnalysisType::Transient,
            other => {
                return Err(ApiError::InvalidInput(format!(
                    "unknown analysis type: {other}"
                )))
            }
        },
        engine: match dto.engine.as_str() {
            "Mock" => hotsas_core::UserCircuitSimulationEngine::Mock,
            "Ngspice" => hotsas_core::UserCircuitSimulationEngine::Ngspice,
            "Auto" => hotsas_core::UserCircuitSimulationEngine::Auto,
            other => return Err(ApiError::InvalidInput(format!("unknown engine: {other}"))),
        },
        probes: dto
            .probes
            .into_iter()
            .map(|p| {
                let kind = match p.kind.as_str() {
                    "NodeVoltage" => hotsas_core::SimulationProbeKind::NodeVoltage,
                    "ComponentCurrent" => hotsas_core::SimulationProbeKind::ComponentCurrent,
                    "DifferentialVoltage" => hotsas_core::SimulationProbeKind::DifferentialVoltage,
                    _ => hotsas_core::SimulationProbeKind::NodeVoltage,
                };
                let target = if let Some(net_id) = p.target.net_id {
                    hotsas_core::SimulationProbeTarget::Net { net_id }
                } else if let (Some(cid), Some(pid)) =
                    (p.target.component_id.clone(), p.target.pin_id.clone())
                {
                    hotsas_core::SimulationProbeTarget::ComponentPin {
                        component_id: cid,
                        pin_id: pid,
                    }
                } else if let Some(cid) = p.target.component_id.clone() {
                    hotsas_core::SimulationProbeTarget::Component { component_id: cid }
                } else if let (Some(pos), Some(neg)) = (
                    p.target.positive_net_id.clone(),
                    p.target.negative_net_id.clone(),
                ) {
                    hotsas_core::SimulationProbeTarget::NetPair {
                        positive_net_id: pos,
                        negative_net_id: neg,
                    }
                } else {
                    hotsas_core::SimulationProbeTarget::Net {
                        net_id: "unknown".to_string(),
                    }
                };
                hotsas_core::SimulationProbe {
                    id: p.id,
                    label: p.label,
                    kind,
                    target,
                    unit: p
                        .unit
                        .and_then(|u| hotsas_core::EngineeringUnit::parse(&u).ok()),
                }
            })
            .collect(),
        ac: dto.ac.map(|ac| hotsas_core::AcSweepSettings {
            start_hz: ac.start_hz,
            stop_hz: ac.stop_hz,
            points_per_decade: ac.points_per_decade,
        }),
        transient: dto.transient.map(|tr| hotsas_core::TransientSettings {
            step_seconds: tr.step_seconds,
            stop_seconds: tr.stop_seconds,
        }),
        op: dto.op.map(|op| hotsas_core::OperatingPointSettings {
            include_node_voltages: op.include_node_voltages,
            include_branch_currents: op.include_branch_currents,
        }),
    })
}

fn into_run_dto(run: hotsas_core::UserCircuitSimulationRun) -> UserCircuitSimulationRunDto {
    UserCircuitSimulationRunDto {
        id: run.id,
        project_id: run.project_id,
        profile: into_profile_dto(run.profile),
        generated_netlist: run.generated_netlist,
        status: format!("{:?}", run.status),
        engine_used: run.engine_used,
        warnings: run
            .warnings
            .into_iter()
            .map(|w| SimulationWorkflowWarningDto {
                code: w.code,
                message: w.message,
            })
            .collect(),
        errors: run
            .errors
            .into_iter()
            .map(|e| SimulationWorkflowErrorDto {
                code: e.code,
                message: e.message,
            })
            .collect(),
        result: run.result.map(|r| UserCircuitSimulationResultDto {
            summary: r
                .summary
                .into_iter()
                .map(|m| SimulationMeasurementDto {
                    name: m.name,
                    si_value: m.value.si_value(),
                    unit: m.value.unit.symbol().to_string(),
                    display: format!("{:.6} {}", m.value.si_value(), m.value.unit.symbol()),
                })
                .collect(),
            series: r
                .series
                .into_iter()
                .map(|s| SimulationSeriesDto {
                    id: s.id,
                    label: s.label,
                    x_unit: s.x_unit.map(|u| u.symbol().to_string()),
                    y_unit: s.y_unit.map(|u| u.symbol().to_string()),
                    points: s
                        .points
                        .into_iter()
                        .map(|p| SimulationPointDto { x: p.x, y: p.y })
                        .collect(),
                })
                .collect(),
            raw_output_excerpt: r.raw_output_excerpt,
            netlist_hash: r.netlist_hash,
        }),
        created_at: run.created_at,
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
