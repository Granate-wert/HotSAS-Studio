use hotsas_adapters::UserCircuitSpiceNetlistExporter;
use hotsas_core::{
    CircuitModel, CircuitProject, ComponentInstance, ComponentModelAssignment,
    ComponentModelAssignmentStatus, ComponentPinMapping, ComponentPinRole, ConnectedPin, Net,
    Point, SimulationReadiness, SpiceModelReference, SpiceModelReferenceKind, SpiceModelSource,
};
use hotsas_ports::NetlistExporterPort;
use std::collections::BTreeMap;

fn empty_project() -> CircuitProject {
    CircuitProject {
        id: "model-mapping-test".to_string(),
        name: "Model Mapping Test".to_string(),
        format_version: "1".to_string(),
        engine_version: "1".to_string(),
        project_type: "circuit".to_string(),
        created_at: "now".to_string(),
        updated_at: "now".to_string(),
        schematic: CircuitModel {
            id: "schematic".to_string(),
            title: "Model Mapping Test".to_string(),
            components: vec![],
            wires: vec![],
            nets: vec![],
            labels: vec![],
            probes: vec![],
            annotations: vec![],
        },
        simulation_profiles: vec![],
        linked_libraries: vec![],
        reports: vec![],
        imported_model_catalog: None,
        persisted_model_assignments: vec![],
    }
}

fn add_component(project: &mut CircuitProject, instance_id: &str, definition_id: &str) {
    project.schematic.components.push(ComponentInstance {
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
    });
}

fn connect_pin(project: &mut CircuitProject, component_id: &str, pin_id: &str, net_id: &str) {
    if let Some(component) = project
        .schematic
        .components
        .iter_mut()
        .find(|component| component.instance_id == component_id)
    {
        component.connected_nets.push(ConnectedPin {
            component_id: component_id.to_string(),
            pin_id: pin_id.to_string(),
            net_id: net_id.to_string(),
        });
    }

    if !project.schematic.nets.iter().any(|net| net.id == net_id) {
        project.schematic.nets.push(Net {
            id: net_id.to_string(),
            name: net_id.to_string(),
            connected_pins: vec![],
        });
    }
}

#[test]
fn placeholder_op_amp_assignment_exports_warning_comment() {
    let mut project = empty_project();
    add_component(&mut project, "U1", "generic_op_amp");
    project.schematic.components[0].selected_simulation_model_id =
        Some("generic_op_amp_model".to_string());
    connect_pin(&mut project, "U1", "non_inverting", "net_p");
    connect_pin(&mut project, "U1", "inverting", "net_n");
    connect_pin(&mut project, "U1", "output", "net_out");

    let netlist = UserCircuitSpiceNetlistExporter
        .export_spice_netlist(&project)
        .unwrap();

    assert!(netlist.contains("* WARNING: placeholder model for U1"));
    assert!(netlist.contains("generic_op_amp"));
}

#[test]
fn subcircuit_assignment_exports_x_line_nodes_in_model_pin_index_order() {
    let mut project = empty_project();
    add_component(&mut project, "U1", "generic_op_amp");
    connect_pin(&mut project, "U1", "non_inverting", "net_plus");
    connect_pin(&mut project, "U1", "inverting", "net_minus");
    connect_pin(&mut project, "U1", "output", "net_out");
    connect_pin(&mut project, "U1", "vcc", "vdd");
    connect_pin(&mut project, "U1", "vee", "vss");

    let assignment = ComponentModelAssignment {
        component_definition_id: "generic_op_amp".to_string(),
        component_instance_id: Some("U1".to_string()),
        model_ref: Some(SpiceModelReference {
            id: "lm358_custom".to_string(),
            display_name: "LM358 custom".to_string(),
            model_kind: SpiceModelReferenceKind::Subcircuit,
            source: SpiceModelSource::ImportedFile,
            status: ComponentModelAssignmentStatus::AssignedImported,
            limitations: vec![],
            warnings: vec![],
        }),
        pin_mappings: vec![
            ComponentPinMapping {
                component_pin_id: "output".to_string(),
                model_pin_name: "OUT".to_string(),
                model_pin_index: Some(0),
                role: Some(ComponentPinRole::Output),
                required: true,
            },
            ComponentPinMapping {
                component_pin_id: "non_inverting".to_string(),
                model_pin_name: "IN+".to_string(),
                model_pin_index: Some(1),
                role: Some(ComponentPinRole::Positive),
                required: true,
            },
            ComponentPinMapping {
                component_pin_id: "inverting".to_string(),
                model_pin_name: "IN-".to_string(),
                model_pin_index: Some(2),
                role: Some(ComponentPinRole::Negative),
                required: true,
            },
            ComponentPinMapping {
                component_pin_id: "vee".to_string(),
                model_pin_name: "VEE".to_string(),
                model_pin_index: Some(3),
                role: Some(ComponentPinRole::SupplyNegative),
                required: true,
            },
            ComponentPinMapping {
                component_pin_id: "vcc".to_string(),
                model_pin_name: "VCC".to_string(),
                model_pin_index: Some(4),
                role: Some(ComponentPinRole::SupplyPositive),
                required: true,
            },
        ],
        parameter_bindings: vec![],
        status: ComponentModelAssignmentStatus::AssignedImported,
        readiness: SimulationReadiness::ready(),
        diagnostics: vec![],
    };

    let netlist = UserCircuitSpiceNetlistExporter
        .export_spice_netlist_with_assignments(&project, &[assignment])
        .unwrap();

    let x_line = netlist
        .lines()
        .find(|line| line.starts_with("XU1 "))
        .expect("expected subcircuit X-line");
    assert_eq!(x_line, "XU1 net_out net_plus net_minus vss vdd lm358custom");
}
