use hotsas_application::SchematicEditingService;
use hotsas_core::{
    AddComponentRequest, CircuitProject, ConnectPinsRequest, DeleteComponentRequest,
    DeleteWireRequest, MoveComponentRequest, Point, RenameNetRequest, UpdateQuickParameterRequest,
};

fn empty_project() -> CircuitProject {
    CircuitProject {
        id: "test".to_string(),
        name: "Test".to_string(),
        format_version: "1".to_string(),
        engine_version: "0.1.4".to_string(),
        project_type: "schematic".to_string(),
        created_at: "2024-01-01".to_string(),
        updated_at: "2024-01-01".to_string(),
        schematic: hotsas_core::CircuitModel {
            id: "sch1".to_string(),
            title: "Test".to_string(),
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
    }
}

#[test]
fn add_resistor_component_succeeds() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.components.len(), 1);
    assert_eq!(edit.project.schematic.components[0].instance_id, "R1");
    assert_eq!(edit.message, "Added component R1");
}

#[test]
fn duplicate_instance_id_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("duplicate"));
}

#[test]
fn move_component_updates_position() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.move_component(
        &mut project,
        MoveComponentRequest {
            instance_id: "R1".to_string(),
            position: Point::new(300.0, 400.0),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.components[0].position.x, 300.0);
    assert_eq!(edit.project.schematic.components[0].position.y, 400.0);
}

#[test]
fn move_unknown_component_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.move_component(
        &mut project,
        MoveComponentRequest {
            instance_id: "R99".to_string(),
            position: Point::new(300.0, 400.0),
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn delete_component_removes_instance() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R1".to_string(),
        },
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().project.schematic.components.len(), 0);
}

#[test]
fn delete_unknown_component_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R99".to_string(),
        },
    );
    assert!(result.is_err());
}

#[test]
fn connect_pins_creates_net_and_wire() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: Some("net_rc".to_string()),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.nets.len(), 1);
    assert_eq!(edit.project.schematic.wires.len(), 1);
    assert_eq!(edit.project.schematic.nets[0].name, "net_rc");
}

#[test]
fn connect_unknown_component_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: None,
        },
    );
    assert!(result.is_err());
}

#[test]
fn rename_net_succeeds() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    project.schematic.nets.push(hotsas_core::Net {
        id: "n1".to_string(),
        name: "old_name".to_string(),
        connected_pins: vec![],
    });

    let result = svc.rename_net(
        &mut project,
        RenameNetRequest {
            net_id: "n1".to_string(),
            new_name: "new_name".to_string(),
        },
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().project.schematic.nets[0].name, "new_name");
}

#[test]
fn rename_net_empty_name_returns_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    project.schematic.nets.push(hotsas_core::Net {
        id: "n1".to_string(),
        name: "old_name".to_string(),
        connected_pins: vec![],
    });

    let result = svc.rename_net(
        &mut project,
        RenameNetRequest {
            net_id: "n1".to_string(),
            new_name: "   ".to_string(),
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("empty"));
}

#[test]
fn connect_unknown_from_pin_returns_controlled_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "unknown_pin".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: None,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("pin 'unknown_pin' not found"));
}

#[test]
fn connect_unknown_to_pin_returns_controlled_error() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "unknown_pin".to_string(),
            net_name: None,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("pin 'unknown_pin' not found"));
}

#[test]
fn connect_valid_pins_creates_connection() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "2".to_string(),
            net_name: Some("net_rc".to_string()),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.nets.len(), 1);
    assert_eq!(edit.project.schematic.wires.len(), 1);
    let net = &edit.project.schematic.nets[0];
    assert_eq!(net.connected_pins.len(), 2);
    assert!(net
        .connected_pins
        .iter()
        .any(|cp| cp.component_id == "R1" && cp.pin_id == "1"));
    assert!(net
        .connected_pins
        .iter()
        .any(|cp| cp.component_id == "C1" && cp.pin_id == "2"));
}

#[test]
fn delete_connected_component_removes_related_wires() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: Some("net_rc".to_string()),
        },
    )
    .unwrap();

    assert_eq!(project.schematic.wires.len(), 1);

    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R1".to_string(),
        },
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap().project.schematic.wires.len(), 0);
}

#[test]
fn delete_connected_component_removes_stale_net_connected_pins() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: Some("net_rc".to_string()),
        },
    )
    .unwrap();

    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R1".to_string(),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.nets.len(), 1);
    let net = &edit.project.schematic.nets[0];
    assert!(!net.connected_pins.iter().any(|cp| cp.component_id == "R1"));
    assert!(net.connected_pins.iter().any(|cp| cp.component_id == "C1"));
}

#[test]
fn delete_connected_component_returns_floating_net_warning() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: Some("net_rc".to_string()),
        },
    )
    .unwrap();

    let result = svc.delete_component(
        &mut project,
        DeleteComponentRequest {
            instance_id: "R1".to_string(),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    // After deleting R1, C1 remains alone on the net → floating net warning
    assert!(edit
        .validation_warnings
        .iter()
        .any(|w| w.code == "floating_net"));
}

#[test]
fn delete_wire_removes_wire_and_cleans_up_net() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "capacitor".to_string(),
            component_definition_id: None,
            instance_id: Some("C1".to_string()),
            position: Point::new(200.0, 200.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();
    svc.connect_pins(
        &mut project,
        ConnectPinsRequest {
            from_component_id: "R1".to_string(),
            from_pin_id: "1".to_string(),
            to_component_id: "C1".to_string(),
            to_pin_id: "1".to_string(),
            net_name: Some("net_rc".to_string()),
        },
    )
    .unwrap();

    let wire_id = project.schematic.wires[0].id.clone();
    let result = svc.delete_wire(
        &mut project,
        DeleteWireRequest {
            wire_id: wire_id.clone(),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    assert_eq!(edit.project.schematic.wires.len(), 0);
    let net = edit
        .project
        .schematic
        .nets
        .iter()
        .find(|n| n.name == "net_rc")
        .unwrap();
    assert!(net.connected_pins.is_empty());
    let r1 = edit
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    assert!(r1.connected_nets.is_empty());
}

#[test]
fn update_component_quick_parameter_updates_model() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.update_component_quick_parameter(
        &mut project,
        UpdateQuickParameterRequest {
            component_id: "R1".to_string(),
            parameter_id: "resistance".to_string(),
            value: "4.7k".to_string(),
        },
    );
    assert!(result.is_ok());
    let edit = result.unwrap();
    let r1 = edit
        .project
        .schematic
        .components
        .iter()
        .find(|c| c.instance_id == "R1")
        .unwrap();
    let resistance = r1.overridden_parameters.get("resistance").unwrap();
    assert_eq!(resistance.original(), "4.7k");
}

#[test]
fn update_component_quick_parameter_rejects_invalid_value() {
    let svc = SchematicEditingService::new();
    let mut project = empty_project();
    svc.add_component(
        &mut project,
        AddComponentRequest {
            component_kind: "resistor".to_string(),
            component_definition_id: None,
            instance_id: Some("R1".to_string()),
            position: Point::new(100.0, 100.0),
            rotation_deg: 0.0,
        },
    )
    .unwrap();

    let result = svc.update_component_quick_parameter(
        &mut project,
        UpdateQuickParameterRequest {
            component_id: "R1".to_string(),
            parameter_id: "resistance".to_string(),
            value: "invalid".to_string(),
        },
    );
    assert!(result.is_err());
}
