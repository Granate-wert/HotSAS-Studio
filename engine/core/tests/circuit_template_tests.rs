use hotsas_core::{rc_low_pass_formula, rc_low_pass_project, rc_low_pass_template};

#[test]
fn rc_low_pass_template_contains_expected_components_and_nets() {
    let template = rc_low_pass_template();
    let component_ids: Vec<_> = template
        .components
        .iter()
        .map(|component| component.instance_id.as_str())
        .collect();

    assert!(component_ids.contains(&"V1"), "template must contain V1");
    assert!(component_ids.contains(&"R1"), "template must contain R1");
    assert!(component_ids.contains(&"C1"), "template must contain C1");
    assert_eq!(
        template.named_nodes.get("Vin").map(String::as_str),
        Some("net_in")
    );
    assert_eq!(
        template.named_nodes.get("Vout").map(String::as_str),
        Some("net_out")
    );
    assert!(
        template
            .compatible_formula_ids
            .contains(&"rc_low_pass_cutoff".to_string()),
        "template must be compatible with rc_low_pass_cutoff"
    );
}

#[test]
fn rc_low_pass_project_wires_expected_signal_path() {
    let project = rc_low_pass_project();

    for net_id in ["net_in", "net_out", "gnd"] {
        assert!(
            project.schematic.nets.iter().any(|net| net.id == net_id),
            "project must contain net {net_id}"
        );
    }

    assert_connection(&project, "V1", "p", "net_in");
    assert_connection(&project, "V1", "n", "gnd");
    assert_connection(&project, "R1", "1", "net_in");
    assert_connection(&project, "R1", "2", "net_out");
    assert_connection(&project, "C1", "1", "net_out");
    assert_connection(&project, "C1", "2", "gnd");
}

#[test]
fn rc_low_pass_formula_is_bound_to_template() {
    let formula = rc_low_pass_formula();

    assert_eq!(formula.id, "rc_low_pass_cutoff");
    assert_eq!(
        formula.linked_circuit_template_id.as_deref(),
        Some("rc_low_pass_template")
    );
    assert!(formula.variables.contains_key("R"));
    assert!(formula.variables.contains_key("C"));
    assert!(formula.outputs.contains_key("fc"));
}

fn assert_connection(
    project: &hotsas_core::CircuitProject,
    component_id: &str,
    pin: &str,
    net: &str,
) {
    let component = project
        .schematic
        .components
        .iter()
        .find(|component| component.instance_id == component_id)
        .unwrap_or_else(|| panic!("missing component {component_id}"));

    assert!(
        component
            .connected_nets
            .iter()
            .any(|connected| connected.pin_id == pin && connected.net_id == net),
        "expected {component_id}.{pin} to connect to {net}"
    );
}
