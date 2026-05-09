use crate::ApplicationError;
use hotsas_core::{
    CircuitProject, ComponentDefinition, ComponentInstance, ComponentLibrary,
    ComponentModelAssignment, ComponentModelAssignmentStatus, ComponentPinMapping,
    ComponentPinRole, ImportedModelDetails, ModelMappingDiagnostic, ModelMappingSeverity,
    ModelParameterBinding, ProjectSimulationReadiness, SimulationModel, SimulationModelKind,
    SimulationReadiness, SpiceModelReference, SpiceModelReferenceKind, SpiceModelSource,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ComponentModelMappingService;

impl ComponentModelMappingService {
    pub fn new() -> Self {
        Self
    }

    /// List all available SPICE models for a given component definition.
    /// Includes builtin models from the definition and imported models that could match.
    pub fn list_available_models_for_component(
        &self,
        definition: &ComponentDefinition,
        imported_models: &[ImportedModelDetails],
    ) -> Vec<SpiceModelReference> {
        let mut refs = Vec::new();

        for model in &definition.simulation_models {
            let (status, _, _) = evaluate_single_model(model, &definition.id);
            let mut model_ref = model_ref_for_model(model, status);
            model_ref.display_name =
                format!("{} ({})", model.id, format_kind_label(model_ref.model_kind));
            if status == ComponentModelAssignmentStatus::Placeholder {
                model_ref
                    .limitations
                    .push("Placeholder model - not production-accurate".to_string());
            }
            if model.raw_model.is_none()
                && model.raw_model_id.is_none()
                && model.kind != SimulationModelKind::Primitive
            {
                model_ref
                    .warnings
                    .push("Model has no raw SPICE data".to_string());
            }
            refs.push(model_ref);
        }

        for imported in imported_models {
            let already_included = refs.iter().any(|r| r.id == imported.id);
            if !already_included {
                let (kind, status) = match imported.kind {
                    hotsas_core::ImportedModelKind::SpiceModel => (
                        SpiceModelReferenceKind::PrimitiveModel,
                        ComponentModelAssignmentStatus::AssignedImported,
                    ),
                    hotsas_core::ImportedModelKind::SpiceSubcircuit => (
                        SpiceModelReferenceKind::Subcircuit,
                        ComponentModelAssignmentStatus::AssignedImported,
                    ),
                    hotsas_core::ImportedModelKind::TouchstoneNetwork => (
                        SpiceModelReferenceKind::PrimitiveModel,
                        ComponentModelAssignmentStatus::AssignedImported,
                    ),
                    hotsas_core::ImportedModelKind::Unknown => (
                        SpiceModelReferenceKind::Placeholder,
                        ComponentModelAssignmentStatus::Invalid,
                    ),
                };

                refs.push(SpiceModelReference {
                    id: imported.id.clone(),
                    display_name: format!(
                        "Imported: {} ({})",
                        imported.name,
                        format_kind_label(kind)
                    ),
                    model_kind: kind,
                    source: SpiceModelSource::ImportedFile,
                    status,
                    limitations: vec![],
                    warnings: vec![],
                });
            }
        }

        refs
    }

    /// Evaluate the model assignment for a component definition.
    pub fn get_component_model_assignment(
        &self,
        definition: &ComponentDefinition,
    ) -> ComponentModelAssignment {
        evaluate_definition_assignment(definition, None)
    }

    /// Evaluate the model assignment for a placed component instance.
    pub fn get_instance_model_assignment(
        &self,
        instance: &ComponentInstance,
        definition: &ComponentDefinition,
    ) -> ComponentModelAssignment {
        let definition = definition_with_instance_parameters(definition, instance);
        evaluate_instance_assignment(instance, &definition)
    }

    /// Assign a model to a component instance by ID.
    pub fn assign_model_to_instance(
        &self,
        instance: &mut ComponentInstance,
        model_id: &str,
        definition: &ComponentDefinition,
    ) -> Result<ComponentModelAssignment, ApplicationError> {
        let synthetic_id = synthetic_primitive_model(definition).id;
        let model_exists = definition
            .simulation_models
            .iter()
            .any(|m| m.id == model_id)
            || model_id == synthetic_id;
        if !model_exists {
            return Err(ApplicationError::InvalidInput(format!(
                "model {} not available for component {}",
                model_id, definition.id
            )));
        }
        instance.selected_simulation_model_id = Some(model_id.to_string());
        Ok(self.get_instance_model_assignment(instance, definition))
    }

    /// Validate a model assignment and return diagnostics.
    pub fn validate_model_assignment(
        &self,
        assignment: &ComponentModelAssignment,
    ) -> Vec<ModelMappingDiagnostic> {
        assignment.diagnostics.clone()
    }

    /// Evaluate simulation readiness for the entire project.
    pub fn evaluate_project_simulation_readiness(
        &self,
        project: &CircuitProject,
        library: &ComponentLibrary,
    ) -> ProjectSimulationReadiness {
        let mut components = Vec::new();
        let mut ready_count = 0usize;
        let mut placeholder_count = 0usize;
        let mut missing_count = 0usize;
        let mut invalid_count = 0usize;
        let mut blocking_count = 0usize;
        let mut warning_count = 0usize;

        for instance in &project.schematic.components {
            let definition = library
                .components
                .iter()
                .find(|d| d.id == instance.definition_id)
                .cloned()
                .unwrap_or_else(|| default_placeholder_definition(&instance.definition_id));

            let assignment = self.get_instance_model_assignment(instance, &definition);

            match assignment.status {
                ComponentModelAssignmentStatus::AssignedBuiltin
                | ComponentModelAssignmentStatus::AssignedImported
                | ComponentModelAssignmentStatus::AssignedManual => ready_count += 1,
                ComponentModelAssignmentStatus::Placeholder => placeholder_count += 1,
                ComponentModelAssignmentStatus::Missing => missing_count += 1,
                ComponentModelAssignmentStatus::Invalid => invalid_count += 1,
            }

            blocking_count += assignment.readiness.blocking_count;
            warning_count += assignment.readiness.warning_count;
            components.push(assignment);
        }

        ProjectSimulationReadiness {
            project_id: project.id.clone(),
            can_simulate: blocking_count == 0 && missing_count == 0 && invalid_count == 0,
            component_count: components.len(),
            ready_count,
            placeholder_count,
            missing_count,
            invalid_count,
            blocking_count,
            warning_count,
            components,
        }
    }
}

fn format_kind_label(kind: SpiceModelReferenceKind) -> String {
    match kind {
        SpiceModelReferenceKind::PrimitiveModel => "primitive",
        SpiceModelReferenceKind::Subcircuit => "subcircuit",
        SpiceModelReferenceKind::Behavioral => "behavioral",
        SpiceModelReferenceKind::Placeholder => "placeholder",
    }
    .to_string()
}

fn is_primitive_component(definition_id: &str) -> bool {
    definition_id.contains("resistor")
        || definition_id.contains("capacitor")
        || definition_id.contains("inductor")
        || definition_id.contains("voltage_source")
        || definition_id.contains("ground")
}

fn evaluate_definition_assignment(
    definition: &ComponentDefinition,
    instance_id: Option<String>,
) -> ComponentModelAssignment {
    if definition.simulation_models.is_empty() {
        if is_primitive_component(&definition.id) {
            let synthetic = synthetic_primitive_model(definition);
            return build_assignment_for_model(definition, instance_id, &synthetic);
        }
        return ComponentModelAssignment {
            component_definition_id: definition.id.clone(),
            component_instance_id: instance_id,
            model_ref: None,
            pin_mappings: vec![],
            parameter_bindings: vec![],
            status: ComponentModelAssignmentStatus::Missing,
            readiness: SimulationReadiness::missing(),
            diagnostics: vec![ModelMappingDiagnostic {
                code: "MISSING_MODEL".to_string(),
                severity: ModelMappingSeverity::Blocking,
                title: "No SPICE model assigned".to_string(),
                message: format!(
                    "Component '{}' has no SPICE model. Simulation will use generic fallback or fail.",
                    definition.id
                ),
                suggested_fix: Some("Assign a builtin or imported SPICE model.".to_string()),
                related_component_id: Some(definition.id.clone()),
                related_model_id: None,
            }],
        };
    }

    let model = definition
        .simulation_models
        .iter()
        .find(|m| m.kind != SimulationModelKind::Placeholder)
        .or_else(|| definition.simulation_models.first())
        .expect("simulation_models is not empty");

    build_assignment_for_model(definition, instance_id, model)
}

fn evaluate_instance_assignment(
    instance: &ComponentInstance,
    definition: &ComponentDefinition,
) -> ComponentModelAssignment {
    if let Some(ref selected_id) = instance.selected_simulation_model_id {
        if let Some(model) = definition
            .simulation_models
            .iter()
            .find(|m| m.id == *selected_id)
        {
            return build_assignment_for_model(
                definition,
                Some(instance.instance_id.clone()),
                model,
            );
        }

        if is_primitive_component(&definition.id) {
            let synthetic = synthetic_primitive_model(definition);
            if synthetic.id == *selected_id {
                return build_assignment_for_model(
                    definition,
                    Some(instance.instance_id.clone()),
                    &synthetic,
                );
            }
        }

        return invalid_selected_model_assignment(instance, definition, selected_id);
    }

    evaluate_definition_assignment(definition, Some(instance.instance_id.clone()))
}

fn evaluate_single_model(
    model: &SimulationModel,
    definition_id: &str,
) -> (
    ComponentModelAssignmentStatus,
    SimulationReadiness,
    Vec<ModelMappingDiagnostic>,
) {
    match model.kind {
        SimulationModelKind::Primitive => (
            ComponentModelAssignmentStatus::AssignedBuiltin,
            SimulationReadiness::ready(),
            vec![],
        ),
        SimulationModelKind::Placeholder => (
            ComponentModelAssignmentStatus::Placeholder,
            SimulationReadiness::placeholder(),
            vec![ModelMappingDiagnostic {
                code: "PLACEHOLDER_MODEL".to_string(),
                severity: ModelMappingSeverity::Warning,
                title: "Placeholder model assigned".to_string(),
                message: format!(
                    "Component '{}' uses a placeholder model. Results are not production-accurate.",
                    definition_id
                ),
                suggested_fix: Some("Replace with a real SPICE model when available.".to_string()),
                related_component_id: Some(definition_id.to_string()),
                related_model_id: Some(model.id.clone()),
            }],
        ),
        SimulationModelKind::Model | SimulationModelKind::Subcircuit => {
            if model.raw_model.is_none() && model.raw_model_id.is_none() {
                (
                    ComponentModelAssignmentStatus::Placeholder,
                    SimulationReadiness::placeholder(),
                    vec![ModelMappingDiagnostic {
                        code: "MODEL_NO_RAW_DATA".to_string(),
                        severity: ModelMappingSeverity::Warning,
                        title: "Model has no raw SPICE data".to_string(),
                        message: format!(
                            "Model '{}' for component '{}' has no raw_model or raw_model_id.",
                            model.id, definition_id
                        ),
                        suggested_fix: Some(
                            "Re-import the SPICE model with full data.".to_string(),
                        ),
                        related_component_id: Some(definition_id.to_string()),
                        related_model_id: Some(model.id.clone()),
                    }],
                )
            } else {
                (
                    ComponentModelAssignmentStatus::AssignedImported,
                    SimulationReadiness::ready(),
                    vec![],
                )
            }
        }
        SimulationModelKind::Touchstone => (
            ComponentModelAssignmentStatus::AssignedImported,
            SimulationReadiness::ready(),
            vec![ModelMappingDiagnostic {
                code: "TOUCHSTONE_NOT_SPICE".to_string(),
                severity: ModelMappingSeverity::Warning,
                title: "Touchstone model cannot be used in SPICE simulation".to_string(),
                message: format!(
                    "Model '{}' is a Touchstone network and cannot be directly simulated in SPICE.",
                    model.id
                ),
                suggested_fix: Some("Use a SPICE-compatible model for simulation.".to_string()),
                related_component_id: Some(definition_id.to_string()),
                related_model_id: Some(model.id.clone()),
            }],
        ),
        SimulationModelKind::Unknown => (
            ComponentModelAssignmentStatus::Invalid,
            SimulationReadiness::invalid(),
            vec![ModelMappingDiagnostic {
                code: "UNKNOWN_MODEL_KIND".to_string(),
                severity: ModelMappingSeverity::Blocking,
                title: "Unknown model kind".to_string(),
                message: format!("Model '{}' has an unrecognized kind.", model.id),
                suggested_fix: Some("Re-assign a known model type.".to_string()),
                related_component_id: Some(definition_id.to_string()),
                related_model_id: Some(model.id.clone()),
            }],
        ),
    }
}

fn build_assignment_for_model(
    definition: &ComponentDefinition,
    instance_id: Option<String>,
    model: &SimulationModel,
) -> ComponentModelAssignment {
    let (base_status, _base_readiness, mut diagnostics) =
        evaluate_single_model(model, &definition.id);
    let pin_mappings = build_pin_mappings(definition, model);
    let parameter_bindings = build_parameter_bindings(definition);

    diagnostics.extend(validate_pin_mappings(definition, model, &pin_mappings));
    diagnostics.extend(validate_parameter_bindings(
        definition,
        model,
        &parameter_bindings,
    ));

    let status = if diagnostics.iter().any(|diagnostic| {
        matches!(
            diagnostic.severity,
            ModelMappingSeverity::Blocking | ModelMappingSeverity::Error
        )
    }) {
        ComponentModelAssignmentStatus::Invalid
    } else {
        base_status
    };
    let readiness = readiness_from_status_and_diagnostics(status, &diagnostics);

    ComponentModelAssignment {
        component_definition_id: definition.id.clone(),
        component_instance_id: instance_id,
        model_ref: Some(model_ref_for_model(model, status)),
        pin_mappings,
        parameter_bindings,
        status,
        readiness,
        diagnostics,
    }
}

fn invalid_selected_model_assignment(
    instance: &ComponentInstance,
    definition: &ComponentDefinition,
    selected_id: &str,
) -> ComponentModelAssignment {
    ComponentModelAssignment {
        component_definition_id: definition.id.clone(),
        component_instance_id: Some(instance.instance_id.clone()),
        model_ref: None,
        pin_mappings: vec![],
        parameter_bindings: vec![],
        status: ComponentModelAssignmentStatus::Invalid,
        readiness: SimulationReadiness::invalid(),
        diagnostics: vec![ModelMappingDiagnostic {
            code: "INVALID_MODEL_SELECTION".to_string(),
            severity: ModelMappingSeverity::Blocking,
            title: "Selected model is not available".to_string(),
            message: format!(
                "Instance '{}' selects model '{}' but component '{}' does not provide it.",
                instance.instance_id, selected_id, definition.id
            ),
            suggested_fix: Some(
                "Choose an available model or clear the instance-level override.".to_string(),
            ),
            related_component_id: Some(definition.id.clone()),
            related_model_id: Some(selected_id.to_string()),
        }],
    }
}

fn build_pin_mappings(
    definition: &ComponentDefinition,
    model: &SimulationModel,
) -> Vec<ComponentPinMapping> {
    let symbol = symbol_for_definition(definition);
    let pin_order = symbol
        .as_ref()
        .map(|symbol| {
            symbol
                .pins
                .iter()
                .enumerate()
                .map(|(index, pin)| (pin.id.clone(), index))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();

    if !model.pin_mapping.is_empty() {
        let model_order = model_pin_order(model);
        let mut entries = model
            .pin_mapping
            .iter()
            .map(|(model_pin, component_pin)| (model_pin.clone(), component_pin.clone()))
            .collect::<Vec<_>>();
        entries.sort_by(
            |(left_model_pin, left_component_pin), (right_model_pin, right_component_pin)| {
                let left_model_order = model_order
                    .get(left_model_pin)
                    .copied()
                    .unwrap_or(usize::MAX);
                let right_model_order = model_order
                    .get(right_model_pin)
                    .copied()
                    .unwrap_or(usize::MAX);
                let left_order = pin_order
                    .get(left_component_pin)
                    .copied()
                    .unwrap_or(usize::MAX);
                let right_order = pin_order
                    .get(right_component_pin)
                    .copied()
                    .unwrap_or(usize::MAX);
                left_model_order
                    .cmp(&right_model_order)
                    .then_with(|| left_order.cmp(&right_order))
                    .then_with(|| left_model_pin.cmp(right_model_pin))
            },
        );

        return entries
            .into_iter()
            .enumerate()
            .map(|(index, (model_pin, component_pin))| ComponentPinMapping {
                role: infer_pin_role(&component_pin, &model_pin),
                component_pin_id: component_pin,
                model_pin_name: model_pin,
                model_pin_index: Some(index),
                required: true,
            })
            .collect();
    }

    symbol
        .map(|symbol| {
            symbol
                .pins
                .iter()
                .enumerate()
                .map(|(index, pin)| ComponentPinMapping {
                    component_pin_id: pin.id.clone(),
                    model_pin_name: pin.id.clone(),
                    model_pin_index: Some(index),
                    role: infer_pin_role(&pin.id, &pin.name),
                    required: true,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn model_pin_order(model: &SimulationModel) -> BTreeMap<String, usize> {
    let Some(raw_model) = model.raw_model.as_deref() else {
        return BTreeMap::new();
    };
    raw_model
        .lines()
        .find_map(|line| {
            let trimmed = line.trim();
            if !trimmed.to_ascii_lowercase().starts_with(".subckt ") {
                return None;
            }
            Some(
                trimmed
                    .split_whitespace()
                    .skip(2)
                    .enumerate()
                    .map(|(index, pin)| (pin.to_string(), index))
                    .collect::<BTreeMap<_, _>>(),
            )
        })
        .unwrap_or_default()
}

fn build_parameter_bindings(definition: &ComponentDefinition) -> Vec<ModelParameterBinding> {
    required_component_parameter_ids(definition)
        .map(|parameter_id| ModelParameterBinding {
            model_parameter_name: parameter_id.to_string(),
            component_parameter_id: parameter_id.to_string(),
            value_expression: definition
                .parameters
                .get(parameter_id)
                .map(|value| value.original().to_string()),
            required: true,
        })
        .into_iter()
        .collect()
}

fn validate_pin_mappings(
    definition: &ComponentDefinition,
    model: &SimulationModel,
    pin_mappings: &[ComponentPinMapping],
) -> Vec<ModelMappingDiagnostic> {
    let Some(symbol) = symbol_for_definition(definition) else {
        return vec![];
    };
    pin_mappings
        .iter()
        .filter(|mapping| symbol.find_pin(&mapping.component_pin_id).is_none())
        .map(|mapping| ModelMappingDiagnostic {
            code: "INVALID_PIN_MAPPING".to_string(),
            severity: ModelMappingSeverity::Blocking,
            title: "Pin mapping references an unknown component pin".to_string(),
            message: format!(
                "Model '{}' maps pin '{}' to unknown component pin '{}' on component '{}'.",
                model.id, mapping.model_pin_name, mapping.component_pin_id, definition.id
            ),
            suggested_fix: Some("Remap the model pin to an existing component pin.".to_string()),
            related_component_id: Some(definition.id.clone()),
            related_model_id: Some(model.id.clone()),
        })
        .collect()
}

fn validate_parameter_bindings(
    definition: &ComponentDefinition,
    model: &SimulationModel,
    bindings: &[ModelParameterBinding],
) -> Vec<ModelMappingDiagnostic> {
    bindings
        .iter()
        .filter(|binding| binding.required)
        .filter(|binding| {
            !definition
                .parameters
                .contains_key(&binding.component_parameter_id)
        })
        .map(|binding| ModelMappingDiagnostic {
            code: "MISSING_MODEL_PARAMETER".to_string(),
            severity: ModelMappingSeverity::Blocking,
            title: "Required model parameter is missing".to_string(),
            message: format!(
                "Model '{}' requires component parameter '{}' on component '{}'.",
                model.id, binding.component_parameter_id, definition.id
            ),
            suggested_fix: Some(
                "Add the required parameter or adjust the model binding.".to_string(),
            ),
            related_component_id: Some(definition.id.clone()),
            related_model_id: Some(model.id.clone()),
        })
        .collect()
}

fn readiness_from_status_and_diagnostics(
    status: ComponentModelAssignmentStatus,
    diagnostics: &[ModelMappingDiagnostic],
) -> SimulationReadiness {
    let blocking_count = diagnostics
        .iter()
        .filter(|diagnostic| {
            matches!(
                diagnostic.severity,
                ModelMappingSeverity::Blocking | ModelMappingSeverity::Error
            )
        })
        .count();
    let warning_count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == ModelMappingSeverity::Warning)
        .count();

    if blocking_count > 0 || status == ComponentModelAssignmentStatus::Invalid {
        return SimulationReadiness {
            can_simulate: false,
            can_export_netlist: false,
            uses_placeholder: false,
            blocking_count: blocking_count.max(1),
            warning_count,
            status_label: "Invalid model assignment".to_string(),
        };
    }

    match status {
        ComponentModelAssignmentStatus::Missing => SimulationReadiness::missing(),
        ComponentModelAssignmentStatus::Placeholder => SimulationReadiness {
            warning_count: warning_count.max(1),
            ..SimulationReadiness::placeholder()
        },
        _ => SimulationReadiness {
            warning_count,
            ..SimulationReadiness::ready()
        },
    }
}

fn model_ref_for_model(
    model: &SimulationModel,
    status: ComponentModelAssignmentStatus,
) -> SpiceModelReference {
    SpiceModelReference {
        id: model.id.clone(),
        display_name: model.id.clone(),
        model_kind: match model.kind {
            SimulationModelKind::Primitive => SpiceModelReferenceKind::PrimitiveModel,
            SimulationModelKind::Model => SpiceModelReferenceKind::PrimitiveModel,
            SimulationModelKind::Subcircuit => SpiceModelReferenceKind::Subcircuit,
            SimulationModelKind::Placeholder => SpiceModelReferenceKind::Placeholder,
            SimulationModelKind::Touchstone => SpiceModelReferenceKind::PrimitiveModel,
            SimulationModelKind::Unknown => SpiceModelReferenceKind::Placeholder,
        },
        source: if model.raw_model.is_some()
            || model.raw_model_id.is_some()
            || model.source_path.is_some()
        {
            SpiceModelSource::ImportedFile
        } else {
            SpiceModelSource::Builtin
        },
        status,
        limitations: if status == ComponentModelAssignmentStatus::Placeholder {
            vec!["Placeholder model".to_string()]
        } else {
            vec![]
        },
        warnings: vec![],
    }
}

fn synthetic_primitive_model(definition: &ComponentDefinition) -> SimulationModel {
    SimulationModel {
        id: format!(
            "builtin_{}_primitive",
            primitive_kind_for_definition(definition).unwrap_or("component")
        ),
        model_type: "spice".to_string(),
        source_path: None,
        raw_model: None,
        raw_model_id: None,
        pin_mapping: BTreeMap::new(),
        kind: SimulationModelKind::Primitive,
    }
}

fn symbol_for_definition(
    definition: &ComponentDefinition,
) -> Option<hotsas_core::SymbolDefinition> {
    definition
        .symbol_ids
        .first()
        .and_then(|symbol_id| hotsas_core::seed_symbol_for_kind(symbol_id))
        .or_else(|| hotsas_core::seed_symbol_for_kind(&definition.category))
        .or_else(|| {
            primitive_kind_for_definition(definition).and_then(hotsas_core::seed_symbol_for_kind)
        })
}

fn primitive_kind_for_definition(definition: &ComponentDefinition) -> Option<&'static str> {
    let text = format!(
        "{} {} {:?}",
        definition.id, definition.category, definition.symbol_ids
    )
    .to_ascii_lowercase();
    if text.contains("resistor") {
        Some("resistor")
    } else if text.contains("capacitor") {
        Some("capacitor")
    } else if text.contains("inductor") {
        Some("inductor")
    } else if text.contains("voltage_source") || text.contains("voltage source") {
        Some("voltage_source")
    } else if text.contains("ground") {
        Some("ground")
    } else if text.contains("diode") {
        Some("diode")
    } else if text.contains("op_amp") || text.contains("op amp") {
        Some("op_amp")
    } else {
        None
    }
}

fn required_component_parameter_ids(definition: &ComponentDefinition) -> Option<&'static str> {
    match primitive_kind_for_definition(definition) {
        Some("resistor") => Some("resistance"),
        Some("capacitor") => Some("capacitance"),
        Some("inductor") => Some("inductance"),
        Some("voltage_source") => {
            if definition.parameters.contains_key("voltage") {
                Some("voltage")
            } else if definition.parameters.contains_key("ac_magnitude") {
                Some("ac_magnitude")
            } else {
                Some("voltage")
            }
        }
        _ => None,
    }
}

fn definition_with_instance_parameters(
    definition: &ComponentDefinition,
    instance: &ComponentInstance,
) -> ComponentDefinition {
    let mut merged = definition.clone();
    for (key, value) in &instance.overridden_parameters {
        merged.parameters.insert(key.clone(), value.clone());
    }
    merged
}

fn infer_pin_role(component_pin: &str, model_pin: &str) -> Option<ComponentPinRole> {
    let text = format!("{component_pin} {model_pin}").to_ascii_lowercase();
    if text.contains("non_inverting") || text == "1 1" || text.contains("positive") {
        Some(ComponentPinRole::Positive)
    } else if text.contains("inverting") || text == "2 2" || text.contains("negative") {
        Some(ComponentPinRole::Negative)
    } else if text.contains("out") {
        Some(ComponentPinRole::Output)
    } else if text.contains("vcc") || text.contains("vdd") {
        Some(ComponentPinRole::SupplyPositive)
    } else if text.contains("vee") || text.contains("vss") {
        Some(ComponentPinRole::SupplyNegative)
    } else if text.contains("anode") {
        Some(ComponentPinRole::Anode)
    } else if text.contains("cathode") {
        Some(ComponentPinRole::Cathode)
    } else {
        Some(ComponentPinRole::Other)
    }
}

fn default_placeholder_definition(definition_id: &str) -> ComponentDefinition {
    ComponentDefinition {
        id: definition_id.to_string(),
        name: definition_id.to_string(),
        category: "unknown".to_string(),
        manufacturer: None,
        part_number: None,
        description: None,
        parameters: BTreeMap::new(),
        ratings: BTreeMap::new(),
        symbol_ids: vec![],
        footprint_ids: vec![],
        simulation_models: vec![],
        datasheets: vec![],
        tags: vec![],
        metadata: BTreeMap::new(),
    }
}
