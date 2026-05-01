use hotsas_core::{rc_low_pass_project, CircuitQueryService};

#[test]
fn circuit_query_returns_component_and_parameter() {
    let project = rc_low_pass_project();

    let component = CircuitQueryService::get_component(&project, "R1").unwrap();
    let resistance = CircuitQueryService::require_component_parameter(
        &project,
        &component.instance_id,
        "resistance",
    )
    .unwrap();

    assert_eq!(component.instance_id, "R1");
    assert_eq!(resistance.value.original, "10k");
}

#[test]
fn circuit_query_reports_missing_parameter() {
    let project = rc_low_pass_project();

    let error =
        CircuitQueryService::require_component_parameter(&project, "R1", "missing").unwrap_err();

    assert!(error.to_string().contains("missing parameter"));
}
