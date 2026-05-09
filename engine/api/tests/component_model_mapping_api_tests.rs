use hotsas_api::{ComponentModelAssignmentDto, SpiceModelReferenceDto};
use hotsas_core::{
    ComponentModelAssignment, ComponentModelAssignmentStatus, SimulationReadiness,
    SpiceModelReference, SpiceModelReferenceKind, SpiceModelSource,
};

#[test]
fn dto_status_values_are_stable_snake_case() {
    let model = SpiceModelReference {
        id: "builtin_resistor_primitive".to_string(),
        display_name: "Builtin resistor primitive".to_string(),
        model_kind: SpiceModelReferenceKind::PrimitiveModel,
        source: SpiceModelSource::Builtin,
        status: ComponentModelAssignmentStatus::AssignedBuiltin,
        limitations: vec![],
        warnings: vec![],
    };

    let dto = SpiceModelReferenceDto::from(&model);

    assert_eq!(dto.model_kind, "primitive_model");
    assert_eq!(dto.source, "builtin");
    assert_eq!(dto.status, "assigned_builtin");
}

#[test]
fn assignment_dto_status_is_stable_snake_case() {
    let assignment = ComponentModelAssignment {
        component_definition_id: "generic_op_amp".to_string(),
        component_instance_id: Some("U1".to_string()),
        model_ref: None,
        pin_mappings: vec![],
        parameter_bindings: vec![],
        status: ComponentModelAssignmentStatus::Placeholder,
        readiness: SimulationReadiness::placeholder(),
        diagnostics: vec![],
    };

    let dto = ComponentModelAssignmentDto::from(&assignment);

    assert_eq!(dto.status, "placeholder");
    assert_eq!(dto.readiness.status_label, "Placeholder model");
}
