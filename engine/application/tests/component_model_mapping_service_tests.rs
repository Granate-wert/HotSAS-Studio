use std::collections::BTreeMap;

use hotsas_application::ComponentModelMappingService;
use hotsas_core::{
    built_in_component_library, CircuitModel, CircuitProject, ComponentInstance,
    ComponentModelAssignmentStatus, ModelMappingSeverity, Net, Point, SimulationModel,
    SimulationModelKind, Wire,
};

fn instance(instance_id: &str, definition_id: &str) -> ComponentInstance {
    ComponentInstance {
        instance_id: instance_id.to_string(),
        definition_id: definition_id.to_string(),
        selected_symbol_id: None,
        selected_footprint_id: None,
        selected_simulation_model_id: None,
        position: Point::new(0.0, 0.0),
        rotation_degrees: 0.0,
        connected_nets: vec![],
        overridden_parameters: BTreeMap::new(),
        notes: None,
    }
}

fn project_with_components(components: Vec<ComponentInstance>) -> CircuitProject {
    CircuitProject {
        id: "mapping-readiness-project".to_string(),
        name: "Mapping Readiness Project".to_string(),
        format_version: "3.1".to_string(),
        engine_version: "test".to_string(),
        project_type: "circuit".to_string(),
        created_at: "2026-05-09T00:00:00Z".to_string(),
        updated_at: "2026-05-09T00:00:00Z".to_string(),
        schematic: CircuitModel {
            id: "schematic".to_string(),
            title: "Schematic".to_string(),
            components,
            wires: Vec::<Wire>::new(),
            nets: Vec::<Net>::new(),
            labels: vec![],
            probes: vec![],
            annotations: vec![],
        },
        simulation_profiles: vec![],
        linked_libraries: vec![],
        reports: vec![],
    }
}

#[test]
fn primitive_assignment_includes_pin_mappings_and_parameter_binding() {
    let service = ComponentModelMappingService::new();
    let library = built_in_component_library();
    let definition = library
        .components
        .iter()
        .find(|component| component.id == "generic_resistor")
        .unwrap();

    let assignment = service.get_component_model_assignment(definition);

    assert_eq!(
        assignment.status,
        ComponentModelAssignmentStatus::AssignedBuiltin
    );
    assert_eq!(
        assignment.model_ref.as_ref().map(|model| model.id.as_str()),
        Some("builtin_resistor_primitive")
    );
    assert_eq!(assignment.pin_mappings.len(), 2);
    assert_eq!(assignment.pin_mappings[0].component_pin_id, "1");
    assert_eq!(assignment.pin_mappings[0].model_pin_name, "1");
    assert_eq!(assignment.pin_mappings[0].model_pin_index, Some(0));
    assert_eq!(assignment.pin_mappings[1].component_pin_id, "2");
    assert_eq!(assignment.pin_mappings[1].model_pin_name, "2");
    assert_eq!(assignment.pin_mappings[1].model_pin_index, Some(1));
    assert_eq!(assignment.parameter_bindings.len(), 1);
    assert_eq!(
        assignment.parameter_bindings[0].model_parameter_name,
        "resistance"
    );
    assert_eq!(
        assignment.parameter_bindings[0].component_parameter_id,
        "resistance"
    );
    assert!(assignment.parameter_bindings[0].required);
    assert!(assignment.diagnostics.is_empty());
}

#[test]
fn imported_subcircuit_assignment_preserves_explicit_pin_order() {
    let service = ComponentModelMappingService::new();
    let library = built_in_component_library();
    let mut definition = library
        .components
        .iter()
        .find(|component| component.id == "generic_op_amp")
        .unwrap()
        .clone();
    definition.simulation_models = vec![SimulationModel {
        id: "lm358_subckt".to_string(),
        model_type: "spice".to_string(),
        source_path: Some("models/lm358.lib".to_string()),
        raw_model: Some(".subckt LM358 non_inverting inverting output vcc vee".to_string()),
        raw_model_id: Some("LM358".to_string()),
        pin_mapping: BTreeMap::from([
            ("non_inverting".to_string(), "non_inverting".to_string()),
            ("inverting".to_string(), "inverting".to_string()),
            ("output".to_string(), "output".to_string()),
            ("vcc".to_string(), "vcc".to_string()),
            ("vee".to_string(), "vee".to_string()),
        ]),
        kind: SimulationModelKind::Subcircuit,
    }];

    let assignment = service.get_component_model_assignment(&definition);

    assert_eq!(
        assignment.status,
        ComponentModelAssignmentStatus::AssignedImported
    );
    assert_eq!(
        assignment.model_ref.as_ref().map(|model| model.id.as_str()),
        Some("lm358_subckt")
    );
    let ordered_model_pins = assignment
        .pin_mappings
        .iter()
        .map(|mapping| mapping.model_pin_name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ordered_model_pins,
        vec!["non_inverting", "inverting", "output", "vcc", "vee"]
    );
    assert!(assignment.diagnostics.is_empty());
}

#[test]
fn missing_required_component_parameter_is_blocking_diagnostic() {
    let service = ComponentModelMappingService::new();
    let library = built_in_component_library();
    let mut definition = library
        .components
        .iter()
        .find(|component| component.id == "generic_resistor")
        .unwrap()
        .clone();
    definition.parameters.remove("resistance");

    let assignment = service.get_component_model_assignment(&definition);

    assert_eq!(assignment.status, ComponentModelAssignmentStatus::Invalid);
    let diagnostic = assignment
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "MISSING_MODEL_PARAMETER")
        .unwrap();
    assert_eq!(diagnostic.severity, ModelMappingSeverity::Blocking);
    assert_eq!(
        diagnostic.related_model_id.as_deref(),
        Some("builtin_resistor_primitive")
    );
    assert_eq!(assignment.readiness.blocking_count, 1);
}

#[test]
fn invalid_selected_model_id_is_blocking_diagnostic() {
    let service = ComponentModelMappingService::new();
    let library = built_in_component_library();
    let definition = library
        .components
        .iter()
        .find(|component| component.id == "generic_resistor")
        .unwrap();
    let mut selected = instance("R1", "generic_resistor");
    selected.selected_simulation_model_id = Some("missing_model_id".to_string());

    let assignment = service.get_instance_model_assignment(&selected, definition);

    assert_eq!(assignment.status, ComponentModelAssignmentStatus::Invalid);
    assert_eq!(assignment.readiness.blocking_count, 1);
    let diagnostic = assignment
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "INVALID_MODEL_SELECTION")
        .unwrap();
    assert_eq!(diagnostic.severity, ModelMappingSeverity::Blocking);
    assert_eq!(
        diagnostic.related_model_id.as_deref(),
        Some("missing_model_id")
    );
}

#[test]
fn project_readiness_summary_counts_ready_placeholder_missing_and_blocking() {
    let service = ComponentModelMappingService::new();
    let library = built_in_component_library();
    let project = project_with_components(vec![
        instance("R1", "generic_resistor"),
        instance("U1", "generic_op_amp"),
        instance("X1", "custom_unknown"),
    ]);

    let readiness = service.evaluate_project_simulation_readiness(&project, &library);

    assert!(!readiness.can_simulate);
    assert_eq!(readiness.component_count, 3);
    assert_eq!(readiness.ready_count, 1);
    assert_eq!(readiness.placeholder_count, 1);
    assert_eq!(readiness.missing_count, 1);
    assert_eq!(readiness.invalid_count, 0);
    assert_eq!(readiness.blocking_count, 1);
    assert_eq!(readiness.warning_count, 1);
}
